use thiserror::Error;
use tracing_appender::rolling::InitError;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt as _,
    util::{SubscriberInitExt as _, TryInitError},
    EnvFilter,
};

use crate::{env::Env, health_check::load_name_and_version};
use common::env::EnvType;

#[derive(Error, Debug)]
pub enum InitLoggerError {
    #[error("Failed to initialize logger: {0}")]
    SetGlobalSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error("Failed to build file appender: {0}")]
    FailedToBuildFileAppender(#[from] InitError),

    #[error("Failed to initialize logger: {0}")]
    TryInitError(#[from] TryInitError),
}

pub fn init_logger() -> Result<(), InitLoggerError> {
    // Get package info for log file naming
    let (_name, _version) = load_name_and_version();

    dotenv::dotenv().ok();
    let env = envy::from_env::<Env>().expect("Failed to load environment variables");
    let env_filter = EnvFilter::new(env.app_log);

    // Initialize the global subscriber
    if env.env == EnvType::Local {
        // Log to stdout
        let subscriber = fmt::Layer::new().with_line_number(true);
        tracing_subscriber::registry()
            .with(subscriber)
            .with(env_filter)
            .try_init()?;
    } else {
        // Log to stdout
        let subscriber = fmt::Layer::new().with_target(false).json();
        tracing_subscriber::registry()
            .with(subscriber)
            .with(env_filter)
            .try_init()?;
    }
    Ok(())
}
