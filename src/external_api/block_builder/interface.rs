use async_trait::async_trait;
use intmax2_zkp::{
    common::{
        block_builder::BlockProposal, signature::flatten::FlatG2, tx::Tx,
        witness::transfer_witness::TransferWitness,
    },
    ethereum_types::u256::U256,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

use crate::external_api::common::error::ServerError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub struct FeeProof {
    pub spent_proof: ProofWithPublicInputs<F, C, D>,
    pub prev_balance_proof: ProofWithPublicInputs<F, C, D>,
    pub transfer_witness: TransferWitness,
}

#[async_trait]
pub trait BlockBuilderInterface {
    async fn initialize_tx(
        &self,
        pubkey: U256,
        tx: Tx,
        fee_proof: FeeProof,
    ) -> Result<(), ServerError>;

    async fn query_tx(&self, pubkey: U256, tx: Tx) -> Result<Option<BlockProposal>, ServerError>;

    async fn post_signature(
        &self,
        pubkey: U256,
        tx: Tx,
        signature: FlatG2,
    ) -> Result<(), ServerError>;
}
