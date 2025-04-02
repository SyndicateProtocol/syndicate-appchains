//! The `errors` module contains the error types for Maestro.

use redis::RedisError;

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
/// Primary error type for the `maestro` service
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error relating to Redis
    #[error(transparent)]
    Redis(#[from] RedisError),

    /// Error relating to JSON de/serialization
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
