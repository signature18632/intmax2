#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Network error: {0}")]
    NetworkError(String),
}
