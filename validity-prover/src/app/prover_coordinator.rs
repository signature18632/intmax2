use std::time::Duration;

use intmax2_zkp::circuits::validity::{
    transition::processor::TransitionProcessor, validity_circuit::ValidityCircuit,
};
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::Env;

use super::error::ProverCoordinatorError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

type Result<T> = std::result::Result<T, ProverCoordinatorError>;

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

    // 既に生成されているtransition proofを使ってvalidity proofを生成する
    pub fn wrap_task(&self) {
        todo!()
    }
}
