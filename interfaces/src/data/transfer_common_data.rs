use intmax2_zkp::{
    circuits::balance::{balance_pis::BalancePublicInputs, send::spent_circuit::SpentPublicInputs},
    common::{signature::key_set::KeySet, trees::tx_tree::TxMerkleProof, tx::Tx},
    ethereum_types::{bytes32::Bytes32, u256::U256},
    utils::poseidon_hash_out::PoseidonHashOut,
};
use serde::{Deserialize, Serialize};

use crate::utils::circuit_verifiers::CircuitVerifiers;

use super::{
    encryption::algorithm::{decrypt, encrypt},
    error::DataError,
    proof_compression::{CompressedBalanceProof, CompressedSpentProof},
};

type Result<T> = std::result::Result<T, DataError>;

/// Common data for all transfers in a batch transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferCommonData {
    pub tx: Tx,
    pub tx_index: u32,
    pub tx_merkle_proof: TxMerkleProof,
    pub tx_tree_root: Bytes32,
    pub spent_proof: CompressedSpentProof,
    pub prev_balance_proof: CompressedBalanceProof,
}

impl TransferCommonData {
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
            .verify(&self.tx, self.tx_index as u64, tx_tree_root)
            .map_err(|_| DataError::ValidationError("Invalid tx_merkle_proof".to_string()))?;

        // skip spent proof verification because spent circuit cannot be deserialized for now.
        let spent_proof = self.spent_proof.decompress()?;
        let prev_balance_proof = self.prev_balance_proof.decompress()?;
        let balance_vd = CircuitVerifiers::load().get_balance_vd();
        balance_vd
            .verify(prev_balance_proof.clone())
            .map_err(|_| DataError::ValidationError("Invalid prev_balance_proof".to_string()))?;
        let spent_pis = SpentPublicInputs::from_pis(&spent_proof.public_inputs);
        let prev_balance_pis = BalancePublicInputs::from_pis(&prev_balance_proof.public_inputs);

        // Validation of public inputs
        if spent_pis.tx != self.tx {
            return Err(DataError::ValidationError(format!(
                "Invalid spent proof: tx mismatch: {:?} != {:?}",
                spent_pis.tx, self.tx
            )));
        }
        if !spent_pis.is_valid {
            return Err(DataError::ValidationError(
                "Invalid spent proof: is_valid is false".to_string(),
            ));
        }
        if spent_pis.prev_private_commitment != prev_balance_pis.private_commitment {
            return Err(DataError::ValidationError(format!(
                "Invalid spent proof: prev_private_commitment mismatch: {} != {}",
                spent_pis.prev_private_commitment, prev_balance_pis.private_commitment
            )));
        }

        Ok(())
    }
}
