use std::sync::Arc;

use intmax2_client_sdk::external_api::validity_prover::ValidityProverClient;
use intmax2_zkp::{
    circuits::validity::transition::processor::TransitionProcessor,
    common::witness::validity_witness::ValidityWitness,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use tokio::sync::RwLock;

use crate::EnvVar;

use super::error::WorkerError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

type Result<T> = std::result::Result<T, WorkerError>;

#[derive(Clone)]
struct Task {
    block_number: u32,
    witness: ValidityWitness,
    transition_proof: Option<ProofWithPublicInputs<F, C, D>>,
}

#[derive(Clone)]
struct Config {
    work_interval: u64,
    submit_interval: u64,
}

#[derive(Clone)]
pub struct Worker {
    config: Config,
    client: ValidityProverClient,
    transition_processor: Arc<TransitionProcessor<F, C, D>>,
    task: Arc<RwLock<Option<Task>>>,
}

impl Worker {
    pub fn new(env: &EnvVar) -> Worker {
        let config = Config {
            work_interval: env.work_interval,
            submit_interval: env.submit_interval,
        };
        let client = ValidityProverClient::new(&env.validity_prover_base_url);
        let transition_processor = Arc::new(TransitionProcessor::new());
        let task = Arc::new(RwLock::new(None));
        Worker {
            config,
            client,
            transition_processor,
            task,
        }
    }

    async fn work(&self) -> Result<()> {
        if self.task.read().await.is_some() {
            log::info!("Task already assigned");
            return Ok(());
        }
        let task = self.client.assign_task().await?;
        if task.is_none() {
            log::info!("No task available");
            return Ok(());
        }
        let (block_number, validity_witness) = task.unwrap();
        let task = Task {
            block_number,
            witness: validity_witness.clone(),
            transition_proof: None,
        };
        self.task.write().await.replace(task);

        // generate proof
        let transition_proof = self
            .transition_processor
            .prove(todo!(), &validity_witness)
            .map_err(|e| WorkerError::TransitionProveFailed(format!("{:?}", e)))?;
        self.task
            .write()
            .await
            .as_mut()
            .unwrap()
            .transition_proof
            .replace(transition_proof);
        Ok(())
    }

    async fn submit(&self) -> Result<()> {
        let task = self.task.read().await.clone();
        if task.is_none() {
            log::info!("No task assigned");
            return Ok(());
        }
        let task = task.unwrap();

        if let Some(transition_proof) = &task.transition_proof {
            // submit proof if available
            self.client
                .complete_task(task.block_number, transition_proof.clone())
                .await?;
            self.task.write().await.take(); // clear task
        } else {
            // submit heartbeat if proof is not available
            self.client.heartbeat(task.block_number).await?;
        }
        Ok(())
    }

    fn work_job(self) {
        tokio::spawn(async move {
            loop {
                match self.work().await {
                    Ok(_) => {}
                    Err(e) => log::error!("Error while working: {:?}", e),
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(self.config.work_interval))
                    .await;
            }
        });
    }

    fn submit_job(self) {
        tokio::spawn(async move {
            loop {
                match self.submit().await {
                    Ok(_) => {}
                    Err(e) => log::error!("Error while submitting: {:?}", e),
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    self.config.submit_interval,
                ))
                .await;
            }
        });
    }

    pub fn run(&self) {
        self.clone().work_job();
        self.clone().submit_job();
    }
}
