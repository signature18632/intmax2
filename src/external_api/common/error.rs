#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
