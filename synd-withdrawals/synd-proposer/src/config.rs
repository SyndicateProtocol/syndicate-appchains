//! The `config` module handles configuration parsing for the Syndicate Proposer.

use alloy::primitives::Address;
use clap::Parser;
use shared::parse::{parse_address, parse_url};
use std::{fmt::Debug, time::Duration};
use url::Url;

/// Configuration for the Syndicate Proposer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// URL of the Ethereum "L1" chain RPC node
    #[arg(short = 'e', long, env = "ETH_RPC_URL", value_parser = parse_url)]
    pub ethereum_rpc_url: Url,

    /// URL of the settlement chain RPC node
    #[arg(short = 's', long, env = "SETTLEMENT_RPC_URL", value_parser = parse_url)]
    pub settlement_rpc_url: Url,

    /// URL of the app-chain RPC node
    #[arg(short = 'q', long, env = "SEQUENCING_RPC_URL", value_parser = parse_url)]
    pub sequencing_rpc_url: Url,

    /// URL of the app-chain RPC node
    #[arg(short = 'a', long, env = "APPCHAIN_RPC_URL", value_parser = parse_url)]
    pub appchain_rpc_url: Url,

    /// URL of the synd-enclave RPC node
    #[arg(short = 'n', long, env = "ENCLAVE_RPC_URL", value_parser = parse_url)]
    pub enclave_rpc_url: Url,

    /// Address of the assertion poster contract
    #[arg(long, env = "ASSERTION_POSTER_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub assertion_poster_contract_address: Address,

    /// Address of the assertion poster contract
    #[arg(short = 't', long, env = "TEE_MODULE_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub tee_module_contract_address: Address,

    /// Bridge address on the L1
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
       value_parser = parse_address)]
    pub arbitrum_bridge_address: Address,

    /// Inbox address on the L1 - for deposits
    #[arg(short = 'i', long, env = "INBOX_ADDRESS",
       value_parser = parse_address)]
    pub inbox_address: Address,

    /// Sequencer inbox address on the L1 - for batches
    #[arg(long, env = "SEQUENCER_INBOX_ADDRESS",
       value_parser = parse_address)]
    pub sequencer_inbox_address: Address,

    /// Private key    
    #[arg(short = 'k', long, env = "PROPOSER_PRIVATE_KEY")]
    pub private_key: String,

    /// The interval between each block polling
    #[arg(long, env = "PROPOSER_POLLING_INTERVAL", default_value = "10m", value_parser = humantime::parse_duration )]
    pub polling_interval: Duration,

    /// Port for Proposer
    #[arg(short = 'p', long, env = "PROPOSER_PORT", default_value_t = 8080)]
    pub port: u16,

    /// Port for metrics
    #[arg(short = 'm', long, env = "PROPOSER_METRICS_PORT", default_value_t = 9292)]
    pub metrics_port: u16,
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}
