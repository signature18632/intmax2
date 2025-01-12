use serde::{Deserialize, Serialize};

use intmax2_zkp::{
    common::{
        signature::key_set::KeySet, trees::tx_tree::TxMerkleProof,
        witness::spent_witness::SpentWitness,
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
    utils::poseidon_hash_out::PoseidonHashOut,
};

type Result<T> = std::result::Result<T, DataError>;

use super::{
    encryption::algorithm::{decrypt, encrypt},
    error::DataError,
};

// tx data for syncing sender's balance proof
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxData {
    pub tx_index: u32,
    pub tx_merkle_proof: TxMerkleProof,
    pub tx_tree_root: Bytes32,
    pub spent_witness: SpentWitness, // to update sender's private state

    // Ephemeral key to query the transfer common data
    // This is not necessary for sender but added for logging purpose
    pub ephemeral_common_key: U256,
}

impl TxData {
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
        let tx_tree_root: PoseidonHashOut = self
            .tx_tree_root
            .try_into()
            .map_err(|_| DataError::ValidationError("Invalid tx_tree_root".to_string()))?;
        self.tx_merkle_proof
            .verify(&self.spent_witness.tx, self.tx_index as u64, tx_tree_root)
            .map_err(|_| DataError::ValidationError("Invalid tx_merkle_proof".to_string()))?;
        Ok(())
    }
}
