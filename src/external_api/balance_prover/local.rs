use async_trait::async_trait;
use intmax2_zkp::{
    common::witness::{
        receive_deposit_witness::ReceiveDepositWitness,
        receive_transfer_witness::ReceiveTransferWitness, spent_witness::SpentWitness,
        tx_witness::TxWitness, update_witness::UpdateWitness,
    },
    ethereum_types::u256::U256,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{
        circuit_data::VerifierCircuitData, config::PoseidonGoldilocksConfig,
        proof::ProofWithPublicInputs,
    },
};
use std::sync::{Arc, Mutex};

use crate::external_api::common::error::ServerError;

use intmax2_zkp::{
    circuits::balance::balance_processor::BalanceProcessor,
    mock::block_validity_prover::BlockValidityProver,
};

use super::interface::BalanceProverInterface;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub struct LocalBalanceProver {
    pub validity_vd: VerifierCircuitData<F, C, D>,
    pub balance_processor: BalanceProcessor<F, C, D>,
}

impl LocalBalanceProver {
    pub fn new(validity_prover: Arc<Mutex<BlockValidityProver<F, C, D>>>) -> Self {
        let validity_vd = validity_prover
            .lock()
            .unwrap()
            .validity_processor()
            .get_verifier_data();

        let temp = validity_prover.lock().unwrap();
        let validity_circuit = temp.validity_circuit();
        let balance_processor = BalanceProcessor::new(validity_circuit);
        drop(temp);
        Self {
            validity_vd,
            balance_processor,
        }
    }
}

#[async_trait]
impl BalanceProverInterface for LocalBalanceProver {
    async fn prove_spent(
        &self,
        spent_witness: &SpentWitness,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        let spent_proof = self
            .balance_processor
            .balance_transition_processor
            .sender_processor
            .prove_spent(&spent_witness)
            .map_err(|e| ServerError::InternalError(format!("prove_spent failed: {:?}", e)))?;
        Ok(spent_proof)
    }

    async fn prove_send(
        &self,
        pubkey: U256,
        tx_witnes: &TxWitness,
        update_witness: &UpdateWitness<F, C, D>,
        spent_proof: &ProofWithPublicInputs<F, C, D>,
        prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        let balance_proof = self
            .balance_processor
            .prove_send(
                &self.validity_vd,
                pubkey,
                tx_witnes,
                update_witness,
                spent_proof,
                prev_proof,
            )
            .map_err(|e| ServerError::InternalError(format!("prove_send failed: {:?}", e)))?;
        Ok(balance_proof)
    }

    async fn prove_update(
        &self,
        pubkey: U256,
        update_witness: &UpdateWitness<F, C, D>,
        prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        let balance_proof = self
            .balance_processor
            .prove_update(&self.validity_vd, pubkey, update_witness, prev_proof)
            .map_err(|e| ServerError::InternalError(format!("prove_update failed: {:?}", e)))?;
        Ok(balance_proof)
    }

    async fn prove_receive_transfer(
        &self,
        pubkey: U256,
        receive_transfer_witness: &ReceiveTransferWitness<F, C, D>,
        prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        let balance_proof = self
            .balance_processor
            .prove_receive_transfer(pubkey, receive_transfer_witness, prev_proof)
            .map_err(|e| {
                ServerError::InternalError(format!("prove_receive_transfer failed: {:?}", e))
            })?;
        Ok(balance_proof)
    }

    async fn prove_receive_deposit(
        &self,
        pubkey: U256,
        receive_deposit_witness: &ReceiveDepositWitness,
        prev_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    ) -> Result<ProofWithPublicInputs<F, C, D>, ServerError> {
        let balance_proof = self
            .balance_processor
            .prove_receive_deposit(pubkey, &receive_deposit_witness, prev_proof)
            .map_err(|e| {
                ServerError::InternalError(format!("prove_receive_deposit failed: {:?}", e))
            })?;
        Ok(balance_proof)
    }
}
