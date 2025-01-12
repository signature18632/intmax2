use super::proof_compression::ProofCompressionError;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Proof compression error: {0}")]
    ProofCompressionError(#[from] ProofCompressionError),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Deserialization error: {0}")]
    DeserializeError(#[from] bincode::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
