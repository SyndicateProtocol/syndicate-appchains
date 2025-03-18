//! The `types` module handles types for the metabased poster.

use alloy::primitives::{B256, U256};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct NitroBlock {
    number: U256,
    l1_block_number: U256,
    timestamp: U256,
    hash: B256,
    send_root: B256,
}

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum RuntimeError {
    #[error("Failed to initialize component: {0}")]
    Initialization(String),

    #[error("Component shutdown error: {0}")]
    Shutdown(String),

    #[error("Invalid config")]
    InvalidConfig(String),

    #[error("Task recoverable error: {0}")]
    TaskFailedRecoverable(String),

    #[error("Task unrecoverable error: {0}")]
    TaskFailedUnrecoverable(String),
}
