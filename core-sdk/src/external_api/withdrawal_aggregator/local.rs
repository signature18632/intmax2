use async_trait::async_trait;
use intmax2_zkp::{
    ethereum_types::address::Address,
    mock::withdrawal_aggregator::WithdrawalAggregator as InnerWithdrawalAggregator,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use std::sync::{Arc, Mutex};

use crate::{external_api::common::error::ServerError, utils::circuit_verifiers::CircuitVerifiers};

use super::interface::{Fee, WithdrawalAggregatorInterface};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub struct LocalWithdrawalAggregator(pub Arc<Mutex<InnerWithdrawalAggregator<F, C, D>>>);

impl LocalWithdrawalAggregator {
    pub fn new() -> anyhow::Result<Self> {
        let verifiers = CircuitVerifiers::load();
        let inner_withdrawal_aggregator =
            InnerWithdrawalAggregator::new(&verifiers.get_balance_vd().common);
        Ok(Self(Arc::new(Mutex::new(inner_withdrawal_aggregator))))
    }

    // finalize the withdrawal
    pub async fn wrap(&mut self) -> anyhow::Result<()> {
        let mut inner = self.0.lock().unwrap();
        let (_withdrawals, _wrap_proof) = inner
            .wrap(Address::default())
            .map_err(|e| anyhow::anyhow!("Failed to wrap {}", e))?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl WithdrawalAggregatorInterface for LocalWithdrawalAggregator {
    async fn fee(&self) -> Result<Fee, ServerError> {
        Ok(Fee {
            native_fee: 0,
            erc20_fee: 0,
            erc721_fee: 0,
            erc1155_fee: 0,
        })
    }

    async fn request_withdrawal(
        &self,
        single_withdrawal_proof: &ProofWithPublicInputs<F, C, D>,
    ) -> Result<(), ServerError> {
        let mut inner = self.0.lock().unwrap();
        inner
            .add(single_withdrawal_proof)
            .map_err(|e| ServerError::InternalError(format!("Failed to add proof {}", e)))?;
        Ok(())
    }
}
