use ark_std::Zero;
use serde::{Deserialize, Serialize};

use intmax2_zkp::{
    common::{
        signature::key_set::KeySet, trees::tx_tree::TxMerkleProof,
        witness::spent_witness::SpentWitness,
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
};

use super::encryption::{decrypt, encrypt};

// tx data for syncing sender's balance proof
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxData {
    pub tx_index: u32,
    pub tx_merkle_proof: TxMerkleProof,
    pub tx_tree_root: Bytes32,
    pub spent_witness: SpentWitness, // to update sender's private state
}

impl TxData {
    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let data = bincode::deserialize(bytes)?;
        Ok(data)
    }

    pub fn encrypt(&self, pubkey: U256) -> Vec<u8> {
        encrypt(pubkey, &self.to_bytes())
    }

    pub fn decrypt(bytes: &[u8], key: KeySet) -> anyhow::Result<Self> {
        if key.privkey.is_zero() {
            anyhow::bail!("Invalid private key");
        }

        let data = decrypt(key, bytes)?;
        let data = Self::from_bytes(&data)?;
        data.validate(key)?;
        Ok(data)
    }

    pub fn validate(&self, _key: KeySet) -> anyhow::Result<()> {
        self.tx_merkle_proof.verify(
            &self.spent_witness.tx,
            self.tx_index as u64,
            self.tx_tree_root.try_into()?,
        )?;
        Ok(())
    }
}
