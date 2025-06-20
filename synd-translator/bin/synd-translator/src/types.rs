use eyre::Report;
use synd_block_builder::config::ConfigError;
use synd_config::config::IngestorConfigError;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to initialize component: {0}")]
    Initialization(String),

    #[error("Component shutdown error: {0}")]
    Shutdown(String),

    #[error("Task recoverable error: {0}")]
    TaskFailedRecoverable(String),

    #[error("Task unrecoverable error: {0}")]
    TaskFailedUnrecoverable(String),

    #[error("Invalid config: {0}")]
    InvalidConfig(String),

    #[error(transparent)]
    BlockBuilderConfig(#[from] ConfigError),

    #[error(transparent)]
    IngestorConfig(#[from] IngestorConfigError),

    #[error(transparent)]
    TranslatorConfig(#[from] synd_config::config::ConfigError),

    #[error(transparent)]
    Other(#[from] Report),
}
