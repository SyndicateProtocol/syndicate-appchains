//! Configuration for the `synd-seqchain-verifier`

use alloy::primitives::{keccak256, Address, B256};
use clap::Parser;
use serde::{Deserialize, Serialize};
use shared::parse::parse_address;
use std::fmt::Debug;

/// Configuration for the verifier
#[derive(Parser, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SeqchainVerifierConfig {
    /// Bridge address on the L1
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
       value_parser = parse_address)]
    pub arbitrum_bridge_address: Address,
}

impl SeqchainVerifierConfig {
    /// Hash the verifier config
    #[allow(clippy::unwrap_used)]
    pub fn hash_verifier_config_sha256(&self) -> B256 {
        let encoded = serde_json::to_string(self).unwrap();
        keccak256(encoded)
    }
}
