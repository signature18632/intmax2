use std::sync::Arc;

use intmax2_client_sdk::external_api::validity_prover::ValidityProverClient;
use intmax2_interfaces::api::validity_prover::interface::TransitionProofTask;
use intmax2_zkp::circuits::validity::transition::processor::TransitionProcessor;
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
struct Process {
    task: TransitionProofTask,
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
    process: Arc<RwLock<Option<Process>>>,
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
            process: task,
        }
    }

    async fn work(&self) -> Result<()> {
        if self.process.read().await.is_some() {
            log::info!("Task already assigned");
            return Ok(());
        }
        let task = self.client.assign_task().await?;
        if task.is_none() {
            log::info!("No task available");
            return Ok(());
        }
        let task = task.unwrap();
        log::info!("Task assigned for block_number {}", task.block_number);
        let process = Process {
            task: task.clone(),
            transition_proof: None,
        };
        self.process.write().await.replace(process);

        // generate proof
        let transition_proof = self
            .transition_processor
            .prove(&task.prev_validity_pis, &task.validity_witness)
            .map_err(|e| WorkerError::TransitionProveFailed(format!("{:?}", e)))?;
        self.process
            .write()
            .await
            .as_mut()
            .unwrap()
            .transition_proof
            .replace(transition_proof);
        log::info!("Proof generated for block_number {}", task.block_number);
        Ok(())
    }

    async fn submit(&self) -> Result<()> {
        let process = self.process.read().await.clone();
        if process.is_none() {
            log::info!("No process to submit");
            return Ok(());
        }
        let process = process.unwrap();
        if let Some(transition_proof) = &process.transition_proof {
            // submit proof if available
            self.client
                .complete_task(process.task.block_number, transition_proof.clone())
                .await?;
            self.process.write().await.take(); // clear process
            log::info!(
                "Proof submitted for block_number {}",
                process.task.block_number
            );
        } else {
            // submit heartbeat if proof is not available
            self.client.heartbeat(process.task.block_number).await?;
            log::info!(
                "Heartbeat submitted for block_number {}",
                process.task.block_number
            );
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
