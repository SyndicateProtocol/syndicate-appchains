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
       value_parser = parse_address)]
    pub arbitrum_bridge_address: Address,
}
