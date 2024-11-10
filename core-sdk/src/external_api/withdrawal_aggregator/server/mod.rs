use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

use crate::external_api::common::error::ServerError;

use super::interface::{Fee, WithdrawalAggregatorInterface};

pub struct WithdrawalAggregatorServer;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

impl WithdrawalAggregatorServer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl WithdrawalAggregatorInterface for WithdrawalAggregatorServer {
    async fn fee(&self) -> Result<Fee, ServerError> {
        todo!()
    }

    async fn request_withdrawal(
        &self,
        _single_withdrawal_proof: &ProofWithPublicInputs<F, C, D>,
    ) -> Result<(), ServerError> {
        todo!()
    }
}
