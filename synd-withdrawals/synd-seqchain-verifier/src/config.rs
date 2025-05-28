//! Configuration for the `synd-seqchain-verifier`

use alloy::primitives::Address;
use clap::Parser;
use serde::{Deserialize, Serialize};
use shared::parse::parse_address;
use std::fmt::Debug;

/// Configuration for the verifier
#[derive(Parser, Clone, Debug, Hash, Serialize, Deserialize)]
pub struct SeqchainVerifierConfig {
    /// Bridge address on the L1
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
       value_parser = parse_address, default_value = "0x0000000000000000000000000000000000000000")]
    pub arbitrum_bridge_address: Address,

    /// Inbox address on the L1
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
       value_parser = parse_address, default_value = "0x0000000000000000000000000000000000000000")]
    pub arbitrum_inbox_address: Address,
}

impl Default for SeqchainVerifierConfig {
    fn default() -> Self {
        Self { arbitrum_bridge_address: Address::ZERO, arbitrum_inbox_address: Address::ZERO }
    }
}
