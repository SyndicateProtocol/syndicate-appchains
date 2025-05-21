//! Configuration for the `synd-appchain-verifier`

use alloy::primitives::{keccak256, Address, B256};
use clap::Parser;
use serde::{Deserialize, Serialize};
use shared::parse::{parse_address, parse_addresses};
use std::fmt::Debug;

/// Configuration for the verifier
#[derive(Parser, Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AppchainVerifierConfig {
    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address, default_value = "0x0000000000000000000000000000000000000000")]
    pub sequencing_contract_address: Address,

    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
       value_parser = parse_address, default_value = "0x0000000000000000000000000000000000000000")]
    pub arbitrum_bridge_address: Address,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
       value_parser = parse_address, default_value = "0x0000000000000000000000000000000000000000")]
    pub arbitrum_inbox_address: Address,

    /// Ignore delayed messages
    #[arg(long, env = "ARBITRUM_IGNORE_DELAYED_MESSAGES", default_value = "false",  action = clap::ArgAction::Set)]
    pub arbitrum_ignore_delayed_messages: bool,

    /// Allowed settlement addresses
    #[arg(long, env = "ALLOWED_SETTLEMENT_ADDRESSES", value_parser = parse_addresses, default_value = "")]
    pub allowed_settlement_addresses: Vec<Address>,

    /// Settlement delay in seconds
    #[arg(long, env = "SETTLEMENT_DELAY", default_value = "0")]
    pub settlement_delay: u64,
}

impl AppchainVerifierConfig {
    /// Hash the verifier config
    #[allow(clippy::unwrap_used)]
    pub fn hash_verifier_config_sha256(&self) -> B256 {
        let encoded = serde_json::to_string(self).unwrap();
        keccak256(encoded)
    }
}

impl Default for AppchainVerifierConfig {
    fn default() -> Self {
        Self {
            sequencing_contract_address: Address::ZERO,
            arbitrum_bridge_address: Address::ZERO,
            arbitrum_inbox_address: Address::ZERO,
            arbitrum_ignore_delayed_messages: false,
            allowed_settlement_addresses: vec![],
            settlement_delay: 0,
        }
    }
}
