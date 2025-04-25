//! Configuration for the block builder service
use alloy::primitives::Address;
use clap::{Parser, ValueEnum};
use shared::parse::parse_address;
use std::fmt::Debug;
use thiserror::Error;

/// Configuration for the block builder service
#[derive(Parser, Clone)]
#[allow(missing_docs)]
pub struct BlockBuilderConfig {
    #[arg(long, env = "MCHAIN_RPC_URL")]
    pub mchain_rpc_url: String,

    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address)]
    pub sequencing_contract_address: Option<Address>,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "TARGET_ROLLUP_TYPE", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,

    // TODO(SEQ-555): make bridge and inbox addresses specific to arbitrum
    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
        value_parser = parse_address)]
    pub arbitrum_bridge_address: Option<Address>,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
        value_parser = parse_address)]
    pub arbitrum_inbox_address: Option<Address>,

    /// Flag used to ignore delayed messages besides deposits
    /// Default is false
    #[arg(long, env = "ARBITRUM_IGNORE_DELAYED_MESSAGES")]
    pub arbitrum_ignore_delayed_messages: Option<bool>,
}

/// Possible target rollup types for the [`block-builder`]
#[allow(missing_docs)]
#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum TargetRollupType {
    OPTIMISM,
    ARBITRUM,
}

impl Debug for BlockBuilderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBuilderConfig")
            .field("mchain_rpc_url", &self.mchain_rpc_url)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("target_rollup_type", &self.target_rollup_type)
            .field("arbitrum_bridge_address", &self.arbitrum_bridge_address)
            .field("arbitrum_inbox_address", &self.arbitrum_inbox_address)
            .field("signer_key", &"<private>") // Skip showing private key
            .field("arbitrum_ignore_delayed_messages", &self.arbitrum_ignore_delayed_messages)
            .finish()
    }
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        Self {
            mchain_rpc_url: String::new(),
            sequencing_contract_address: Some(Address::ZERO),
            target_rollup_type: TargetRollupType::ARBITRUM,
            arbitrum_bridge_address: Some(Address::ZERO),
            arbitrum_inbox_address: Some(Address::ZERO),
            arbitrum_ignore_delayed_messages: Some(false),
        }
    }
}

impl BlockBuilderConfig {
    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        match self.target_rollup_type {
            // Validate Arbitrum specific configuration
            TargetRollupType::ARBITRUM => {}
            // Validate Optimism specific configuration
            TargetRollupType::OPTIMISM => {
                return Err(ConfigError::UnsupportedRollupType(
                    "Optimism is not supported yet".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), ConfigError> {
        self.validate()?;
        if self.sequencing_contract_address.is_none() {
            return Err(ConfigError::SequencingContractAddressMissing);
        }
        if self.arbitrum_bridge_address.is_none() {
            return Err(ConfigError::ArbitrumBridgeAddressMissing);
        }
        if self.arbitrum_inbox_address.is_none() {
            return Err(ConfigError::ArbitrumInboxAddressMissing);
        }
        if self.arbitrum_ignore_delayed_messages.is_none() {
            return Err(ConfigError::ArbitrumIgnoreDelayedMessagesMissing);
        }
        Ok(())
    }
}

#[allow(missing_docs)]
/// Possible errors that can occur when initializing the block builder configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Unsupported rollup type: {0}")]
    UnsupportedRollupType(String),

    #[error("Sequencing contract address is missing")]
    SequencingContractAddressMissing,

    #[error("Arbitrum inbox address is missing")]
    ArbitrumInboxAddressMissing,

    #[error("Arbitrum bridge address is missing")]
    ArbitrumBridgeAddressMissing,

    #[error("Arbitrum ignore delayed messages is missing")]
    ArbitrumIgnoreDelayedMessagesMissing,
}
