use std::{sync::Arc, time::Duration};

use intmax2_zkp::circuits::validity::{
    transition::processor::TransitionProcessor, validity_circuit::ValidityCircuit,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::Env;

use super::error::ProverCoordinatorError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

type Result<T> = std::result::Result<T, ProverCoordinatorError>;

// CREATE TABLE IF NOT EXISTS validity_proofs (
//     block_number INTEGER PRIMARY KEY,
//     proof JSONB NOT NULL
// );

// CREATE TABLE IF NOT EXISTS prover_tasks (
//     block_number INTEGER PRIMARY KEY,
//     assigned BOOLEAN NOT NULL,
//     assigned_at TIMESTAMP,
//     last_heartbeat TIMESTAMP,
//     completed BOOLEAN NOT NULL,
//     completed_at TIMESTAMP,
//     transition_proof JSONB
// );

#[derive(Clone)]
pub struct Config {
    pub heartbeat_interval: u64,
    pub cleanup_interval: u64,
    pub validity_proof_interval: u64,
}

#[derive(Clone)]
pub struct ProverCoordinator {
    pub transition_processor: Arc<TransitionProcessor<F, C, D>>,
    pub validity_circuit: Arc<ValidityCircuit<F, C, D>>,
    pub pool: PgPool,
    pub config: Config,
}

impl ProverCoordinator {
    pub async fn new(env: &Env) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(env.database_max_connections)
            .idle_timeout(Duration::from_secs(env.database_timeout))
            .connect(&env.database_url)
            .await?;
        let transition_processor = TransitionProcessor::new();
        let validity_circuit = ValidityCircuit::new(
            &transition_processor
                .transition_wrapper_circuit
                .data
                .verifier_data(),
        );
        let heartbeat_config = Config {
            heartbeat_interval: env.heartbeat_interval,
            cleanup_interval: env.cleanup_interval,
            validity_proof_interval: env.validity_proof_interval,
        };
        Ok(Self {
            transition_processor: Arc::new(transition_processor),
            validity_circuit: Arc::new(validity_circuit),
            config: heartbeat_config,
            pool,
        })
    }

    // Assign the task with the smallest block number among the unassigned tasks
    pub async fn assign_task(&self) -> Result<Option<u32>> {
        let record = sqlx::query!(
            r#"
            UPDATE prover_tasks
            SET assigned = TRUE, assigned_at = NOW(), last_heartbeat = NOW()
            WHERE block_number = (
                SELECT block_number
                FROM prover_tasks
                WHERE assigned = FALSE
                ORDER BY block_number
                LIMIT 1
            )
            RETURNING block_number
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        let block_number = record.map(|r| r.block_number as u32);
        Ok(block_number)
    }

    pub async fn heartbeat(&self, block_number: u32) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE prover_tasks
            SET last_heartbeat = NOW()
            WHERE block_number = $1
            "#,
            block_number as i32,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Set the task to assigned = FALSE if the task has not received a heartbeat for the last heartbeat_interval
    pub async fn clean_up(&self) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE prover_tasks
            SET assigned = FALSE
            WHERE assigned = TRUE AND last_heartbeat < NOW() - INTERVAL '1 second' * $1
            "#,
            self.config.heartbeat_interval as i64,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Complete the task with the given block number
    pub async fn complete_task(
        &self,
        block_number: u32,
        transition_proof: &ProofWithPublicInputs<F, C, D>,
    ) -> Result<()> {
        let transition_proof = serde_json::to_value(transition_proof)?;
        sqlx::query!(
            r#"
            UPDATE prover_tasks
            SET assigned = FALSE, completed = TRUE, completed_at = NOW(), transition_proof = $1
            WHERE block_number = $2
            "#,
            transition_proof,
            block_number as i32,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn generate_validity_proof(&self) -> Result<()> {
        // Get the largest block_number and its proof from the validity_proofs table that already exists
        let record = sqlx::query!(
            r#"
            SELECT block_number, proof
            FROM validity_proofs
            ORDER BY block_number DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        let (mut last_validity_proof_block_number, mut prev_proof) = match record {
            Some(record) => (
                record.block_number as u32,
                serde_json::from_value(record.proof.clone())?,
            ),
            None => (0, None),
        };

        // Get records from the prover_tasks table with block_number greater than last_validity_proof_block_number and completed = TRUE
        let records = sqlx::query!(
            r#"
            SELECT block_number, transition_proof
            FROM prover_tasks
            WHERE block_number > $1 AND completed = TRUE
            ORDER BY block_number
            "#,
            last_validity_proof_block_number as i32,
        )
        .fetch_all(&self.pool)
        .await?;

        for record in records.iter() {
            let block_number = record.block_number as u32;
            if block_number != last_validity_proof_block_number + 1 {
                break;
            }
            last_validity_proof_block_number = block_number;

            let transition_proof: ProofWithPublicInputs<F, C, D> =
                serde_json::from_value(record.transition_proof.clone().unwrap())?;
            let validity_proof = self
                .validity_circuit
                .prove(&transition_proof, &prev_proof)
                .map_err(|e| {
                    ProverCoordinatorError::FailedToGenerateValidityProof(e.to_string())
                })?;

            // Add a new validity proof to the validity_proofs table
            let validity_proof_value = serde_json::to_value(&validity_proof)?;
            sqlx::query!(
                r#"
                INSERT INTO validity_proofs (block_number, proof)
                VALUES ($1, $2)
                ON CONFLICT (block_number)
                DO UPDATE SET proof = $2
                "#,
                block_number as i32,
                validity_proof_value,
            )
            .execute(&self.pool)
            .await?;
            prev_proof = Some(validity_proof);
        }

        Ok(())
    }

    fn clean_up_job(self) {
        tokio::spawn(async move {
            loop {
                self.clean_up().await.unwrap();
                tokio::time::sleep(Duration::from_secs(self.config.cleanup_interval)).await;
            }
        });
    }

    fn generate_validity_proof_job(self) {
        tokio::spawn(async move {
            loop {
                self.generate_validity_proof().await.unwrap();
                tokio::time::sleep(Duration::from_secs(self.config.validity_proof_interval)).await;
            }
        });
    }

    pub fn job(&self) {
        self.clone().clean_up_job();
        self.clone().generate_validity_proof_job();
    }
}
