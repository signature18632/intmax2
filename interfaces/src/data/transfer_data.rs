use serde::{Deserialize, Serialize};

use intmax2_zkp::{
    common::{
        signature::key_set::KeySet,
        transfer::Transfer,
        trees::{transfer_tree::TransferMerkleProof, tx_tree::TxMerkleProof},
        tx::Tx,
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
    utils::poseidon_hash_out::PoseidonHashOut,
};

use super::{
    encryption::Encryption, error::DataError, sender_proof_set::SenderProofSet,
    validation::Validation,
};

/// Backup data for receiving transfers
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferData {
    // Ephemeral key to query the sender proof set
    pub sender_proof_set_ephemeral_key: U256,
    // After fetching sender proof set, this will be filled
    pub sender_proof_set: Option<SenderProofSet>,

    pub sender: U256,
    pub tx: Tx,
    pub tx_index: u32,
    pub tx_merkle_proof: TxMerkleProof,
    pub tx_tree_root: Bytes32,
    pub transfer: Transfer,
    pub transfer_index: u32,
    pub transfer_merkle_proof: TransferMerkleProof,
}

impl TransferData {
    pub fn set_sender_proof_set(&mut self, sender_proof_set: SenderProofSet) {
        self.sender_proof_set = Some(sender_proof_set);
    }
}

impl Encryption for TransferData {}

impl Validation for TransferData {
    fn validate(&self, _key: KeySet) -> Result<(), DataError> {
        let tx_tree_root: PoseidonHashOut = self
            .tx_tree_root
            .try_into()
            .map_err(|_| DataError::ValidationError("Invalid tx_tree_root".to_string()))?;
        self.tx_merkle_proof
            .verify(&self.tx, self.tx_index as u64, tx_tree_root)
            .map_err(|_| DataError::ValidationError("Invalid tx_merkle_proof".to_string()))?;
        self.transfer_merkle_proof
            .verify(
                &self.transfer,
                self.transfer_index as u64,
                self.tx.transfer_tree_root,
            )
            .map_err(|_| DataError::ValidationError("Invalid transfer_merkle_proof".to_string()))?;
        Ok(())
    }
}
