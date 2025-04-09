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
    pub sequencing_contract_address: Address,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "TARGET_ROLLUP_TYPE", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,

    // TODO(SEQ-555): make bridge and inbox addresses specific to arbitrum
    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
        value_parser = parse_address)]
    pub arbitrum_bridge_address: Address,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
        value_parser = parse_address)]
    pub arbitrum_inbox_address: Address,

    // TODO (SEQ-567): Move ignore_delayed_messages config value on-chain
    /// Flag used to ignore delayed messages besides deposits
    /// Default is false
    #[arg(long, env = "ARBITRUM_IGNORE_DELAYED_MESSAGES", default_value = "false")]
    pub arbitrum_ignore_delayed_messages: bool,
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
        let zero = Address::ZERO.to_string();
        Self::parse_from(["", "-s", &zero, "-b", &zero, "-i", &zero, "--mchain-rpc-url", ""])
    }
}

impl BlockBuilderConfig {
    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.sequencing_contract_address == Address::ZERO {
            return Err(ConfigError::InvalidAddress(
                "Sequencing contract address cannot be 0".to_string(),
            ));
        }

        match self.target_rollup_type {
            // Validate Arbitrum specific configuration
            TargetRollupType::ARBITRUM => {
                if self.arbitrum_bridge_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "Bridge address cannot be 0".to_string(),
                    ));
                }
                if self.arbitrum_inbox_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "Inbox address cannot be 0".to_string(),
                    ));
                }
            }
            // Validate Optimism specific configuration
            TargetRollupType::OPTIMISM => {
                return Err(ConfigError::UnsupportedRollupType(
                    "Optimism is not supported yet".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[allow(missing_docs)]
/// Possible errors that can occur when initializing the block builder configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidURL(String),
    #[error("Invalid URL: no host")]
    InvalidURLHost,
    #[error("Invalid URL scheme: {0}")]
    InvalidURLScheme(String),
    #[error("Invalid chain ID: {0}")]
    InvalidChainId(String),
    #[error("Unsupported rollup type: {0}")]
    UnsupportedRollupType(String),
    #[error("Invalid configuration address: {0}")]
    InvalidAddress(String),
    #[error("Invalid anvil state path: {0}")]
    InvalidAnvilStatePath(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;
    use assert_matches::assert_matches;

    #[test]
    fn test_validate_rollup_type() {
        let config = BlockBuilderConfig {
            target_rollup_type: TargetRollupType::OPTIMISM,
            sequencing_contract_address: address!("0x0000000000000000000000000000000000000001"),
            ..Default::default()
        };
        assert_matches!(config.validate(), Err(ConfigError::UnsupportedRollupType(_)));
    }

    #[test]
    fn test_validate_bridge_address() {
        let config =
            BlockBuilderConfig { arbitrum_bridge_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
    }

    #[test]
    fn test_validate_inbox_address() {
        let config =
            BlockBuilderConfig { arbitrum_inbox_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
    }

    #[test]
    fn test_validate_sequencing_contract_address() {
        let config =
            BlockBuilderConfig { sequencing_contract_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
    }
}
