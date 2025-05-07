//! This module contains `config` for the `Batcher` service

use alloy::{
    primitives::ChainId,
    providers::{Provider, ProviderBuilder},
    transports::TransportError,
};
use clap::Parser;
use shared::parse::parse_url;
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
    /// Redis address to listen on
    /// Example: redis://0.0.0.0:6379
    #[arg(short = 'r', long, env = "REDIS_URL")]
    pub redis_url: String,

    /// Chain ID
    #[arg(short = 'c', long, env = "CHAIN_ID")]
    pub chain_id: ChainId,

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
            redis_url: "redis://127.0.0.1:6379".to_string(),
            chain_id: 1,
            max_batch_size: byte_unit::Byte::from_u64(90 * 1024),
            timeout: Duration::from_millis(200),
            private_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap_or_else(|_| {
                panic!("Failed to parse default sequencing RPC URL");
            }),
        }
    }
}
