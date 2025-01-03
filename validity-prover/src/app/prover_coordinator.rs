use std::time::Duration;

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

// CREATE TABLE IF NOT EXISTS prover_tasks (
//     block_number INTEGER PRIMARY KEY,
//     assigned BOOLEAN NOT NULL,
//     assigned_at TIMESTAMP,
//     completed BOOLEAN NOT NULL,
//     completed_at TIMESTAMP,
//     transition_proof JSONB
// );

pub struct ProverCoordinator {
    pub transition_processor: TransitionProcessor<F, C, D>,
    pub validity_circuit: ValidityCircuit<F, C, D>,
    pub pool: PgPool,
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
        Ok(Self {
            transition_processor,
            validity_circuit,
            pool,
        })
    }

    // Assign the task with the smallest block number among the unassigned tasks
    pub async fn assign_task(&self) -> Result<Option<u32>> {
        let record = sqlx::query!(
            r#"
            UPDATE prover_tasks
            SET assigned = TRUE, assigned_at = NOW()
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
}
