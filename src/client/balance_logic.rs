use intmax2_zkp::{
    circuits::balance::balance_processor::get_prev_balance_pis,
    common::{
        private_state::FullPrivateState,
        salt::Salt,
        witness::{
            deposit_witness::DepositWitness, private_transition_witness::PrivateTransitionWitness,
            receive_deposit_witness::ReceiveDepositWitness,
        },
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
    mock::data::deposit_data::DepositData,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

use crate::external_api::{
    balance_prover::interface::BalanceProverInterface,
    block_validity_prover::interface::BlockValidityInterface,
};

use super::error::ClientError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub async fn process_deposit<V: BlockValidityInterface, B: BalanceProverInterface>(
    validity_prover: &V,
    balance_processor: &B,
    pubkey: U256,
    full_private_state: &mut FullPrivateState,
    new_salt: Salt,
    prev_balance_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    receive_block_number: u32,
    deposit_data: &DepositData,
) -> Result<ProofWithPublicInputs<F, C, D>, ClientError> {
    // update balance proof up to the deposit block
    let before_balance_proof = update_balance_proof(
        validity_prover,
        balance_processor,
        pubkey,
        prev_balance_proof,
        receive_block_number,
    )
    .await?;

    // Generate witness
    let (deposit_index, deposit_block_number) = validity_prover
        .get_deposit_index_and_block_number(deposit_data.deposit_hash())
        .await?
        .ok_or(ClientError::InternalError(
            "Deposit index and block number not found".to_string(),
        ))?;
    if deposit_block_number > receive_block_number {
        return Err(ClientError::InternalError(
            "Deposit block number is greater than receive block number".to_string(),
        ));
    }
    let deposit_merkle_proof = validity_prover
        .get_deposit_merkle_proof(receive_block_number, deposit_index)
        .await?;
    let deposit_witness = DepositWitness {
        deposit_salt: deposit_data.deposit_salt,
        deposit_index: deposit_index as usize,
        deposit: deposit_data.deposit.clone(),
        deposit_merkle_proof,
    };
    let deposit = deposit_data.deposit.clone();
    let nullifier: Bytes32 = deposit.poseidon_hash().into();
    let private_transition_witness = PrivateTransitionWitness::new(
        full_private_state,
        deposit.token_index,
        deposit.amount,
        nullifier,
        new_salt,
    )
    .map_err(|e| ClientError::WitnessGenerationError(format!("PrivateTransitionWitness {}", e)))?;
    let receive_deposit_witness = ReceiveDepositWitness {
        deposit_witness,
        private_transition_witness,
    };

    // prove deposit
    let balance_proof = balance_processor
        .prove_receive_deposit(
            pubkey,
            &receive_deposit_witness,
            &Some(before_balance_proof),
        )
        .await?;

    Ok(balance_proof)
}

// Inner function to update balance proof
async fn update_balance_proof<V: BlockValidityInterface, B: BalanceProverInterface>(
    validity_prover: &V,
    balance_processor: &B,
    pubkey: U256,
    prev_balance_proof: &Option<ProofWithPublicInputs<F, C, D>>,
    block_number: u32,
) -> Result<ProofWithPublicInputs<F, C, D>, ClientError> {
    // sync check
    if block_number > validity_prover.block_number().await? {
        return Err(ClientError::InternalError(
            "Validity prover is not up to date".to_string(),
        ));
    }
    if block_number == 0 {
        return Err(ClientError::InternalError(
            "Block number should be greater than 0".to_string(),
        ));
    }

    let prev_balance_pis = get_prev_balance_pis(pubkey, prev_balance_proof);
    if block_number == prev_balance_pis.public_state.block_number {
        // no need to update balance proof
        return Ok(prev_balance_proof.clone().unwrap());
    }

    // get update witness
    let update_witness = validity_prover
        .get_update_witness(
            pubkey,
            block_number,
            prev_balance_pis.public_state.block_number,
            false,
        )
        .await?;
    let last_block_number = update_witness.get_last_block_number();
    if last_block_number > block_number {
        return Err(ClientError::InternalError(
            "There is a sent tx after prev balance proof".to_string(),
        ));
    }
    let balance_proof = balance_processor
        .prove_update(pubkey, &update_witness, &prev_balance_proof)
        .await?;
    Ok(balance_proof)
}
