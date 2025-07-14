//! This module contains `config` for the `Batcher` service

use alloy::{
    primitives::{Address, ChainId},
    providers::{Provider, ProviderBuilder},
    transports::TransportError,
};
use clap::Parser;
use shared::parse::{parse_address, parse_url};
use std::{fmt::Debug, time::Duration};
use thiserror::Error;
use url::Url;

/// Error types relating to Config
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to connect to chain RPC URL: {0} expected chain ID: {1} got {2}")]
    InvalidChainId(String, String, String),

    #[error("failed to connect to chain RPC URL: {0}")]
    ConnectionError(#[from] TransportError),
}

/// Configuration for Batcher
#[allow(clippy::doc_markdown)]
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct BatcherConfig {
    /// Valkey address to listen on
    /// Example: valkey://0.0.0.0:6379
    #[arg(short = 'r', long, env = "VALKEY_URL")]
    pub valkey_url: String,

    /// Chain ID
    #[arg(short = 'c', long, env = "CHAIN_ID")]
    pub chain_id: ChainId,

    /// Enable compression
    #[arg(short = 'z', long, env = "COMPRESSION_ENABLED", default_value = "true")]
    pub compression_enabled: bool,

    /// Max batch size in bytes
    #[arg(long, env = "MAX_BATCH_SIZE", default_value = "90KB")] // 90 kilobytes
    pub max_batch_size: byte_unit::Byte,

    /// Timeout for the batcher in milliseconds
    #[arg( long, env = "BATCHER_TIMEOUT", default_value = "200ms", value_parser = humantime::parse_duration )]
    pub timeout: Duration,

    /// Batcher private key
    ///
    /// This is the private key for the wallet responsible for signing transaction
    /// batches submitted by the Batcher service. The corresponding wallet must be
    /// funded with sufficient native tokens (e.g., ETH) to cover gas costs when
    /// submitting transactions on the sequencing chain.
    #[arg(short = 'k', long, env = "BATCHER_PRIVATE_KEY")]
    pub private_key: String,

    /// URL of the sequencing chain RPC node
    #[arg(short = 'u', long, env = "SEQUENCING_RPC_URL", value_parser = parse_url)]
    pub sequencing_rpc_url: Url,

    /// The maximum number of retries for rate limit errors
    #[arg(long, env = "RPC_MAX_RETRIES", default_value = "16")]
    pub rpc_max_retries: u32,

    /// The initial backoff in milliseconds
    #[arg(long, env = "RPC_INITIAL_BACKOFF_MS", default_value = "100")]
    pub rpc_initial_backoff_ms: u64,

    /// The number of compute units per second for this provider (CU values are usually specified
    /// in the provider's documentation)
    #[arg(long, env = "RPC_COMPUTE_UNITS_PER_SECOND", default_value = "1000")]
    pub rpc_compute_units_per_second: u64,

    /// The average cost of a request in compute units
    #[arg(long, env = "RPC_COMPUTE_UNITS_AVG_REQUEST_COST", default_value = "17")]
    pub rpc_compute_units_avg_request_cost: u64,

    /// The contract address that the sequencer will use to submit batches
    #[arg(short = 's', long, env = "SEQUENCING_ADDRESS", value_parser = parse_address)]
    pub sequencing_address: Address,

    /// Server port
    #[arg(long, env = "PORT", default_value_t = 8082)]
    pub port: u16,

    /// whether to wait for the receipt of the batch submission
    #[arg(long, env = "WAIT_FOR_RECEIPT", default_value_t = false)]
    pub wait_for_receipt: bool,
}

impl BatcherConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }

    /// Validates the configuration
    pub async fn validate(&self) -> Result<(), ConfigError> {
        ping_sequencing_rpc_url(&self.sequencing_rpc_url).await?;
        Ok(())
    }
}

async fn ping_sequencing_rpc_url(url: &Url) -> Result<(), ConfigError> {
    let provider =
        ProviderBuilder::new().connect(url.as_str()).await.map_err(ConfigError::ConnectionError)?;
    provider.get_chain_id().await?;
    Ok(())
}

impl Default for BatcherConfig {
    fn default() -> Self {
        Self {
            valkey_url: "redis://127.0.0.1:6379".to_string(),
            chain_id: 1,
            compression_enabled: true,
            max_batch_size: byte_unit::Byte::from_u64(90 * 1024),
            timeout: Duration::from_millis(200),
            private_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap_or_else(|_| {
                panic!("Failed to parse default sequencing RPC URL");
            }),
            rpc_max_retries: 10,
            rpc_initial_backoff_ms: 100,
            rpc_compute_units_per_second: 1000,
            rpc_compute_units_avg_request_cost: 17,
            sequencing_address: Address::ZERO,
            port: 8082,
            wait_for_receipt: false,
        }
    }
}
