//! Configuration for the `synd-block-builder` service
use alloy::primitives::Address;
use clap::Parser;
use shared::parse::parse_address;
use std::fmt::Debug;
use thiserror::Error;

/// Configuration for the block builder service
#[derive(Parser, Clone, Default)]
#[allow(missing_docs)]
pub struct BlockBuilderConfig {
    #[arg(long, env = "MCHAIN_WS_URL")]
    pub mchain_ws_url: String,

    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address)]
    pub sequencing_contract_address: Option<Address>,

    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
        value_parser = parse_address)]
    pub arbitrum_bridge_address: Option<Address>,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
        value_parser = parse_address)]
    pub arbitrum_inbox_address: Option<Address>,
}

impl Debug for BlockBuilderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBuilderConfig")
            .field("mchain_ws_url", &self.mchain_ws_url)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("arbitrum_bridge_address", &self.arbitrum_bridge_address)
            .field("arbitrum_inbox_address", &self.arbitrum_inbox_address)
            .field("signer_key", &"<private>") // Skip showing private key
            .finish()
    }
}

impl BlockBuilderConfig {
    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub const fn validate(&self) -> Result<(), ConfigError> {
        if self.sequencing_contract_address.is_none() {
            return Err(ConfigError::SequencingContractAddressMissing);
        }
        if self.arbitrum_bridge_address.is_none() {
            return Err(ConfigError::ArbitrumBridgeAddressMissing);
        }
        if self.arbitrum_inbox_address.is_none() {
            return Err(ConfigError::ArbitrumInboxAddressMissing);
        }
        Ok(())
    }
}

#[allow(missing_docs)]
/// Possible errors that can occur when initializing the block builder configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Sequencing contract address is missing")]
    SequencingContractAddressMissing,

    #[error("Arbitrum inbox address is missing")]
    ArbitrumInboxAddressMissing,

    #[error("Arbitrum bridge address is missing")]
    ArbitrumBridgeAddressMissing,
}
