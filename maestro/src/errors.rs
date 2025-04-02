//! The `errors` module contains the error types for Maestro.

use redis::RedisError;
use std::str::Utf8Error;

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

    /// Error relating to byte to string conversion
    #[error(transparent)]
    ByteConversion(#[from] Utf8Error),
}
