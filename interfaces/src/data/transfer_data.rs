use serde::{Deserialize, Serialize};

use intmax2_zkp::{
    common::{
        signature::key_set::KeySet, transfer::Transfer, trees::transfer_tree::TransferMerkleProof,
        tx::Tx,
    },
    ethereum_types::u256::U256,
};

use super::{
    encryption::algorithm::{decrypt, encrypt},
    error::DataError,
};

type Result<T> = std::result::Result<T, DataError>;

/// Backup data for receiving transfers
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferData {
    // Ephemeral key to query the transfer common data
    pub ephemeral_common_key: U256,

    pub sender: U256,
    pub tx: Tx,
    pub transfer: Transfer,
    pub transfer_index: u32,
    pub transfer_merkle_proof: TransferMerkleProof,
}

impl TransferData {
    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let data = bincode::deserialize(bytes)?;
        Ok(data)
    }

    pub fn encrypt(&self, pubkey: U256) -> Vec<u8> {
        encrypt(pubkey, &self.to_bytes())
    }

    pub fn decrypt(bytes: &[u8], key: KeySet) -> Result<Self> {
        let data = decrypt(key, bytes).map_err(|e| DataError::DecryptionError(e.to_string()))?;
        let data = Self::from_bytes(&data)?;
        data.validate(key)?;
        Ok(data)
    }

    pub fn validate(&self, _key: KeySet) -> Result<()> {
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
