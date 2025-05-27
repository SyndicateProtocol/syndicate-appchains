//! Configuration for the verifier

use alloy::primitives::Address;
use clap::Parser;
use shared::parse::parse_address;
use std::fmt::Debug;

/// Configuration for the verifier
#[derive(Parser, Clone, Debug)]
pub struct VerifierConfig {
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
}

impl Default for VerifierConfig {
    fn default() -> Self {
        Self {
            sequencing_contract_address: Address::ZERO,
            arbitrum_bridge_address: Address::ZERO,
            arbitrum_inbox_address: Address::ZERO,
        }
    }
}
