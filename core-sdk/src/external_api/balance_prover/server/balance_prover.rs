use async_trait::async_trait;
use intmax2_zkp::{
    common::witness::{
        receive_deposit_witness::ReceiveDepositWitness,
        receive_transfer_witness::ReceiveTransferWitness, spent_witness::SpentWitness,
        tx_witness::TxWitness, update_witness::UpdateWitness,
        withdrawal_witness::WithdrawalWitness,
    },
    ethereum_types::u256::U256,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

use crate::external_api::{
    balance_prover::interface::BalanceProverInterface, common::error::ServerError,
};

pub struct BalanceProver;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[async_trait(?Send)]
impl BalanceProverInterface for BalanceProver {
    async fn prove_spent(
        &self,
        _spent_witness: &SpentWitness,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        todo!()
    }

    async fn prove_send(
        &self,
        _pubkey: U256,
        _tx_witnes: &TxWitness,
        _update_witness: &UpdateWitness<F, C, D>,
        _spent_proof: &ProofWithPublicInputs<F, C, D>,
        _prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        todo!()
    }

    async fn prove_update(
        &self,
        _pubkey: U256,
        _update_witness: &UpdateWitness<F, C, D>,
        _prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        todo!()
    }

    async fn prove_receive_transfer(
        &self,
        _pubkey: U256,
        _receive_transfer_witness: &ReceiveTransferWitness<F, C, D>,
        _prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        todo!()
    }

    async fn prove_receive_deposit(
        &self,
        _pubkey: U256,
        _receive_deposit_witness: &ReceiveDepositWitness,
        _prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        todo!()
    }

    async fn prove_single_withdrawal(
        &self,
        _withdrawal_witness: &WithdrawalWitness<F, C, D>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        todo!()
    }
}
