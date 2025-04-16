//! This module contains `config` for the `Batcher` service

use alloy::transports::http::reqwest::Error as ReqwestError;
use clap::Parser;
use std::{fmt::Debug, time::Duration};
use thiserror::Error;
use tracing::error;

/// Errors that can occur during Batcher configuration or validation
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Failed to create the HTTP client
    #[error("failed to build HTTP client: {0}")]
    HttpClient(#[from] ReqwestError),

    /// Failed to connect to the given RPC URL
    #[error("failed to connect to RPC URL for chain {0} at {1}")]
    RpcUrlConnection(String, String),

    /// RPC endpoint responded with a non-success status code
    #[error("invalid HTTP status from RPC URL {1} for chain {0}: {2}")]
    RpcUrlInvalidStatus(String, String, String),

    /// RPC endpoint returned a mismatched chain ID
    #[error("RPC URL {0} returned mismatched chain ID. Expected: {1}, Got: {2}")]
    RpcUrlInvalidChainId(String, String, String),

    /// RPC response could not be parsed
    #[error("failed to parse JSON-RPC response: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

/// Configuration for Batcher
#[allow(clippy::doc_markdown)]
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct BatcherConfig {
    /// Redis address to listen on
    /// Example: "0.0.0.0:6379"
    #[arg(short = 'r', long, env = "REDIS_ADDRESS")]
    pub redis_address: String,

    /// Chain ID
    #[arg(short = 'i', long, env = "CHAIN_ID")]
    pub chain_id: u64,

    /// Batch size
    #[arg(short = 'b', long, env = "BATCH_SIZE", default_value_t = 10)]
    pub batch_size: usize,

    /// Polling interval for the batcher in milliseconds
    #[arg( long, env = "BATCHER_POLLING_INTERVAL", default_value = "500ms", value_parser = humantime::parse_duration )]
    pub polling_interval: Duration,

    /// Sequencer client URL
    #[arg(long, env = "SEQUENCER_CLIENT_URL", default_value = "http://localhost:8080")]
    pub sequencer_client_url: String,
}

impl BatcherConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}

impl Default for BatcherConfig {
    fn default() -> Self {
        Self {
            redis_address: "0.0.0.0:6379".to_string(),
            chain_id: 1,
            batch_size: 10,
            polling_interval: Duration::from_millis(500),
            sequencer_client_url: "http://localhost:8080".to_string(),
        }
    }
}
