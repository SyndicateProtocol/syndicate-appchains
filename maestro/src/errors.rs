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
    #[error("unable to parse Arbitrum Nitro URLs map: {0}")]
    ChainIdNitroUrlParse(String),

    #[error("unable to connect to server: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("failed to connect to chain id: {0} nitro URL: {1} error: {2}")]
    NitroUrlConnection(String, String, reqwest::Error),

    #[error("failed to connect to chain id: {0} nitro URL: {1} status: {2}")]
    NitroUrlInvalidStatus(String, String, String),
}
