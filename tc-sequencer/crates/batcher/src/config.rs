//! This module contains `config` for the `Batcher` service

use alloy::primitives::ChainId;
use byte_unit::Byte;
use clap::Parser;
use std::{fmt::Debug, time::Duration};

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
            redis_url: "0.0.0.0:6379".to_string(),
            chain_id: 1,
            max_batch_size: Byte::from_u64(90 * 1024),
            polling_interval: Duration::from_millis(500),
        }
    }
}
