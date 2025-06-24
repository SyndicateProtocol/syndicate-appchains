//! Configuration module for the chain ingestor service

use crate::eth_client::EthClient;
use clap::{command, Parser};
use humantime::parse_duration;
use std::time::Duration;

/// CLI args for the chain ingestor executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
pub struct Config {
    #[arg(long, env = "WS_URL")]
    pub ws_url: String,
    #[arg(long, env = "DB_FILE")]
    pub db_file: String,
    #[arg(long, env = "START_BLOCK", default_value_t = 0)]
    pub start_block: u64,
    #[arg(long, env = "CHANNEL_SIZE", default_value_t = 1024)]
    pub channel_size: usize,
    #[arg(long, env = "PARALLEL_SYNC_REQUESTS", default_value_t = 190)]
    pub parallel_sync_requests: u64,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    pub port: u16,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    pub metrics_port: u16,
    #[arg(
        long,
        env = "REQUEST_TIMEOUT",
        default_value = "10s",
        value_parser = parse_duration
    )]
    pub request_timeout: Duration,
}

impl Config {
    /// Creates a new [`EthClient`] provider from the configuration
    pub async fn new_provider(&self) -> EthClient {
        EthClient::new(
            &self.ws_url,
            self.request_timeout,
            Duration::from_secs(300),
            self.channel_size,
        )
        .await
    }
}
