//! Configuration for the verifier

use alloy::primitives::Address;
use clap::Parser;
use shared::parse::{parse_address, parse_addresses};
use std::fmt::Debug;

/// Configuration for the verifier
#[derive(Parser, Clone, Debug)]
#[allow(missing_docs)]
pub struct VerifierConfig {
    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address)]
    pub sequencing_contract_address: Address,

    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
       value_parser = parse_address)]
    pub arbitrum_bridge_address: Address,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
       value_parser = parse_address)]
    pub arbitrum_inbox_address: Address,

    /// Ignore delayed messages
    #[arg(long, env = "ARBITRUM_IGNORE_DELAYED_MESSAGES")]
    pub arbitrum_ignore_delayed_messages: bool,

    /// Allowed settlement addresses
    #[arg(long, env = "ALLOWED_SETTLEMENT_ADDRESSES", value_parser = parse_addresses)]
    pub allowed_settlement_addresses: Vec<Address>,
}

impl Default for VerifierConfig {
    fn default() -> Self {
        Self {
            sequencing_contract_address: Address::ZERO,
            arbitrum_bridge_address: Address::ZERO,
            arbitrum_inbox_address: Address::ZERO,
            arbitrum_ignore_delayed_messages: false,
            allowed_settlement_addresses: vec![],
        }
    }
}
