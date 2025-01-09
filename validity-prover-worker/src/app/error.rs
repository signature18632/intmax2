use intmax2_interfaces::api::error::ServerError;

#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("Client error: {0}")]
    ClientError(#[from] ServerError),

    #[error("Transition prove failed: {0}")]
    TransitionProveFailed(String),
}
