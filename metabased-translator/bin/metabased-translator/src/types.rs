use common::eth_client::{ClientError, RPCClientError};
use eyre::Report;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to initialize component: {0}")]
    Initialization(String),

    #[error("Component shutdown error: {0}")]
    Shutdown(String),

    #[error("Task unrecoverable error: {0}")]
    TaskFailedUnrecoverable(String),

    #[error("Invalid config")]
    InvalidConfig(String),

    #[error(transparent)]
    BlockBuilderConfig(#[from] block_builder::config::ConfigError),

    #[error(transparent)]
    IngestorConfig(#[from] ingestor::config::ConfigError),

    #[error(transparent)]
    TranslatorConfig(#[from] crate::config::ConfigError),

    #[error(transparent)]
    RPCClient(#[from] RPCClientError),

    #[error(transparent)]
    InitClientError(#[from] ClientError),

    #[error(transparent)]
    Other(#[from] Report),
}
