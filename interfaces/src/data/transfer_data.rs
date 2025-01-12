use ark_std::Zero;
use serde::{Deserialize, Serialize};

use intmax2_zkp::{
    common::{
        signature::key_set::KeySet,
        transfer::Transfer,
        trees::{transfer_tree::TransferMerkleProof, tx_tree::TxMerkleProof},
        tx::Tx,
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
};

use super::encryption::{decrypt, encrypt};

// backup data for receiving transfers
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferData {
    // Ephemeral key to query the sender's prev balance proof and spent proof
    pub ephemeral_privkey: U256,

    // Info to update the sender's balance proof
    pub sender: U256,
    pub tx: Tx,
    pub tx_index: u32,
    pub tx_merkle_proof: TxMerkleProof,
    pub tx_tree_root: Bytes32,

    // Used for updating receiver's balance proof
    pub transfer: Transfer,
    pub transfer_index: u32,
    pub transfer_merkle_proof: TransferMerkleProof,
}

impl TransferData {
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
            &self.tx,
            self.tx_index as u64,
            self.tx_tree_root.try_into()?,
        )?;
        self.transfer_merkle_proof
            .verify(
                &self.transfer,
                self.transfer_index as u64,
                self.tx.transfer_tree_root,
            )
            .map_err(|e| anyhow::anyhow!("transfer merkle proof validation failed: {}", e))?;
        Ok(())
    }
}
