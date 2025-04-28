//! The `errors` module contains the error types for Maestro.

use alloy::{
    primitives::{Address, ChainId},
    transports::TransportError,
};
use jsonrpsee::types::{ErrorCode, ErrorObjectOwned};
use redis::RedisError;
use thiserror::Error;
use tracing::error;

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
/// Primary error type for the metabased sequencer, following JSON-RPC error code mapping
#[derive(Debug, Error)]
pub enum Error {
    /// Error relating to Redis
    #[error(transparent)]
    Redis(#[from] RedisError),

    /// Error relating to Config
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
}

/// Error types relating to Config
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to connect to chain RPC URL: {0} expected chain ID: {1} got {2}")]
    RpcUrlInvalidChainId(String, String, String),

    #[error("Invalid RPC address: {0}")]
    RpcUrlInvalidAddress(String),

    #[error("failed to connect to chain RPC URL: {0}")]
    RpcUrlConnection(#[from] TransportError),
}

/// JSON-RPC specific error types
#[derive(Debug, Error)]
pub enum MaestroRpcError {
    /// Internal Maestro error
    #[error("internal error: {0}")]
    Internal(InternalError),
}

/// Known internal Maestro errors
/// NOTE: These are client-facing
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InternalError {
    /// No RPC provider for a transaction
    #[error("chain {0} is unsupported")]
    RpcMissing(ChainId),

    /// Failed to fetch wallet nonce from RPC
    #[error("chain {0} failed to return wallet {1} nonce")]
    RpcFailedToFetchWalletNonce(ChainId, Address),

    /// Failed ot submit transaction
    #[error("transaction submission failed")]
    TransactionSubmissionFailed,
}

impl From<MaestroRpcError> for ErrorObjectOwned {
    fn from(error: MaestroRpcError) -> Self {
        match error {
            MaestroRpcError::Internal(e) => ErrorObjectOwned::owned(
                ErrorCode::InternalError.code(),
                format!("internal error: {}", e),
                None::<()>,
            ),
        }
    }
}
