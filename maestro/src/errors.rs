//! The `errors` module contains the error types for Maestro.

use alloy::transports::http::reqwest;
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
    Config(ConfigError),
}

/// Error types relating to Config
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("unable to connect to server: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("failed to connect to chain ID: {0} RPC URL: {1}")]
    RpcUrlConnection(String, String),

    #[error("bad response code from chain ID: {0} RPC URL: {1} response status: {2}")]
    RpcUrlInvalidStatus(String, String, String),

    #[error("failed to connect to chain RPC URL: {0} expected chain ID: {1} got {2}")]
    RpcUrlInvalidChainId(String, String, String),
}
