//! The `errors` module contains the error types for Maestro.

use alloy::transports::TransportError;
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
