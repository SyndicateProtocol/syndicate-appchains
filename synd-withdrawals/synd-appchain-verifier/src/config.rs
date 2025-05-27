//! Configuration for the `synd-appchain-verifier`

use alloy::primitives::{keccak256, Address, B256};
use clap::Parser;
use serde::{Deserialize, Serialize};
use shared::parse::parse_address;
use std::fmt::Debug;

/// Configuration for the verifier
#[derive(Parser, Clone, Debug, Hash, Serialize, Deserialize, Default)]
pub struct AppchainVerifierConfig {
    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address, default_value = "0x0000000000000000000000000000000000000000")]
    pub sequencing_contract_address: Address,

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
