use intmax2_zkp::{
    circuits::balance::{balance_pis::BalancePublicInputs, send::spent_circuit::SpentPublicInputs},
    common::signature::key_set::KeySet,
    ethereum_types::u256::U256,
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
pub struct SenderProofSet {
    pub spent_proof: CompressedSpentProof,
    pub prev_balance_proof: CompressedBalanceProof,
}

impl SenderProofSet {
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
