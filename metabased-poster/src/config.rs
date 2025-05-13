//! The `config` module handles configuration parsing for the metabased poster.

use alloy::primitives::Address;
use clap::Parser;
use shared::parse::{parse_address, parse_url};
use std::{fmt::Debug, time::Duration};
use url::Url;

/// Configuration for the metabased sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// URL of the settlement chain RPC node
    #[arg(short = 's', long, env = "SETTLEMENT_RPC_URL", value_parser = parse_url)]
    pub settlement_rpc_url: Url,

    /// URL of the app-chain RPC node
    #[arg(short = 'a', long, env = "APPCHAIN_RPC_URL", value_parser = parse_url)]
    pub appchain_rpc_url: Url,

    /// Address of the assertion poster contract
    #[arg(short = 'b', long, env = "ASSERTION_POSTER_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub assertion_poster_contract_address: Address,

    /// Private key    
    #[arg(short = 'k', long, env = "POSTER_PRIVATE_KEY")]
    pub private_key: String,

    /// The interval between each block polling
    #[arg( long, env = "POSTER_POLLING_INTERVAL", default_value = "10m", value_parser = humantime::parse_duration )]
    pub polling_interval: Duration,

    /// Port for Poster
    #[arg(short = 'p', long, env = "POSTER_PORT", default_value_t = 8080)]
    pub port: u16,

    /// Port for metrics
    #[arg(short = 'm', long, env = "POSTER_METRICS_PORT", default_value_t = 9292)]
    pub metrics_port: u16,
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}
