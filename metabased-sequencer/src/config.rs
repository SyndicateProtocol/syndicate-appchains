//! The `config` module handles configuration parsing for the metabased sequencer.

use alloy::primitives::{Address, B256};
use clap::Parser;
use shared::parse::{parse_address, parse_url};
use std::{fmt::Debug, time::Duration};
use url::Url;

/// Configuration for the metabased sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Address of the sequencing contract
    #[arg(short = 'c', long, env = "SEQUENCING_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub chain_contract_address: Address,

    /// URL of the sequencing chain RPC node
    #[arg(short = 'r', long, env = "SEQUENCING_RPC_URL", value_parser = parse_url)]
    pub chain_rpc_url: Url,

    /// Port to listen on
    #[arg(short = 'p', long, env = "SEQUENCER_PORT", default_value_t = 8456)]
    pub port: u16,

    /// Port for metrics
    #[arg(short = 'm', long, env = "SEQUENCER_METRICS_PORT", default_value_t = 9191)]
    pub metrics_port: u16,

    /// Private key for signing transactions    
    #[arg(short = 'k', long, env = "SEQUENCER_PRIVATE_KEY")]
    pub private_key: B256,

    /// Confirmations to wait for when relaying transaction
    #[arg(long, env = "SEQUENCER_CONFIRMATIONS", default_value_t = 0)]
    pub tx_confirmations: u64,

    /// Timeout when relaying transaction
    #[arg(
        long,
        env = "SEQUENCER_TIMEOUT",
        default_value = "60s",
        value_parser = humantime::parse_duration
    )]
    pub tx_timeout: Duration,
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}
