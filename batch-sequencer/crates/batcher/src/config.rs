//! This module contains `config` for the `Batcher` service

use alloy::primitives::{Address, ChainId};
use byte_unit::Byte;
use clap::Parser;
use shared::parse::{parse_address, parse_url};
use std::{fmt::Debug, time::Duration};
use url::Url;

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
    pub max_batch_size: Byte,

    /// Polling interval for the batcher in milliseconds
    #[arg( long, env = "BATCHER_POLLING_INTERVAL", default_value = "100ms", value_parser = humantime::parse_duration )]
    pub polling_interval: Duration,

    /// Batcher private key
    #[arg(short = 'k', long, env = "BATCHER_PRIVATE_KEY")]
    pub private_key: String,

    /// Address of the wallet pool
    #[arg(short = 'b', long, env = "WALLET_POOL_ADDRESS", value_parser = parse_address)]
    pub wallet_pool_address: Address,

    /// Address of the sequencing contract
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub sequencing_contract_address: Address,

    /// URL of the sequencing chain RPC node
    #[arg(short = 's', long, env = "SEQUENCING_RPC_URL", value_parser = parse_url)]
    pub sequencing_rpc_url: Url,
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
            redis_url: "redis://127.0.0.1:6379".to_string(),
            chain_id: 1,
            max_batch_size: Byte::from_u64(90 * 1024),
            polling_interval: Duration::from_millis(500),
            private_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            wallet_pool_address: Address::ZERO,
            sequencing_contract_address: Address::ZERO,
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap_or_else(|_| {
                panic!("Failed to parse default sequencing RPC URL");
            }),
        }
    }
}
