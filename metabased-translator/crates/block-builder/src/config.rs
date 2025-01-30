//! Configuration for the block builder service
use alloy::{
    primitives::Address,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use clap::{Parser, ValueEnum};
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;

const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

/// Configuration for the block builder service
#[derive(Parser, Clone)]
#[allow(missing_docs)]
pub struct BlockBuilderConfig {
    #[arg(short = 'p', long, env = "BLOCK_BUILDER_PORT", default_value_t = 8888)]
    pub port: u16,

    #[arg(
        short = 'g',
        long,
        env = "BLOCK_BUILDER_GENESIS_TIMESTAMP",
        default_value_t = 1712500000
    )]
    pub genesis_timestamp: u64,

    #[arg(short = 'c', long, env = "BLOCK_BUILDER_TARGET_CHAIN_ID", default_value_t = 13331370)]
    pub target_chain_id: u64,

    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address,
        default_value = "0x0000000000000000000000000000000000000000")]
    pub sequencing_contract_address: Address,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "BLOCK_BUILDER_TARGET_ROLLUP", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,

    /// Arbitrum rollup address on the m-chain
    #[arg(short = 'm', long, env = "BLOCK_BUILDER_ARBITRUM_MCHAIN_ROLLUP_ADDRESS",
        value_parser = parse_address,
        // TODO (SEQ-534): Use a default value if possible
        default_value = "0x0000000000000000000000000000000000000000")]
    pub mchain_rollup_address: Address,

    /// Delayed inbox address on the settlement chain
    #[arg(short = 'd', long, env = "BLOCK_BUILDER_ARBITRUM_DELAYED_INBOX_ADDRESS",
        value_parser = parse_address,
        default_value = "0x0000000000000000000000000000000000000000")]
    pub delayed_inbox_address: Address,

    // path to the directory where anvil will keep its state
    #[arg(long, env = "BLOCK_BUILDER_ANVIL_STATE_PATH", default_value = "")]
    pub anvil_state_path: String,

    // interval at which anvil saves state to disk (in seconds)
    #[arg(long, env = "BLOCK_BUILDER_ANVIL_STATE_INTERVAL", default_value_t = 1)]
    pub anvil_state_interval: u64,

    // number of states to be kept in memory by anvil
    #[arg(long, env = "BLOCK_BUILDER_ANVIL_PRUNE_HISTORY", default_value_t = 50)]
    pub anvil_prune_history: u64,
}

/// Possible target rollup types for the [`block-builder`]
#[allow(missing_docs)]
#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum TargetRollupType {
    OPTIMISM,
    ARBITRUM,
}

/// Parse a string into an Ethereum `Address`.
fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value).map_err(|_| format!("Invalid address: {}", value))
}

/// Parse default string into a `PrivateKeySigner`.
pub fn get_default_private_key_signer() -> LocalSigner<SigningKey> {
    PrivateKeySigner::from_str(DEFAULT_PRIVATE_KEY_SIGNER)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}

impl Debug for BlockBuilderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBuilderConfig")
            .field("port", &self.port)
            .field("genesis_timestamp", &self.genesis_timestamp)
            .field("target_chain_id", &self.target_chain_id)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("target_rollup_type", &self.target_rollup_type)
            .field("mchain_rollup_address", &self.mchain_rollup_address)
            .field("delayed_inbox_address", &self.delayed_inbox_address)
            .field("signer_key", &"<private>") // Skip showing private key
            .field("anvil_state_path", &self.anvil_state_path)
            .field("anvil_state_interval", &self.anvil_state_interval)
            .field("anvil_prune_history", &self.anvil_prune_history)
            .finish()
    }
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        let default_address = Address::from_str("0x1234000000000000000000000000000000000000")
            .unwrap_or_else(|err| {
                panic!("Failed to parse default address: {}", err);
            });
        Self {
            port: 8888,
            genesis_timestamp: 1712500000,
            target_chain_id: 13331370,
            target_rollup_type: TargetRollupType::ARBITRUM,
            sequencing_contract_address: default_address,
            mchain_rollup_address: default_address,
            delayed_inbox_address: default_address,
            anvil_state_path: String::new(),
            anvil_state_interval: 1,
            anvil_prune_history: 50,
        }
    }
}

impl BlockBuilderConfig {
    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::InvalidPort("Port cannot be 0".to_string()));
        }

        if self.target_chain_id == 0 {
            return Err(ConfigError::InvalidChainId("Chain ID cannot be 0".to_string()));
        }

        if self.sequencing_contract_address == Address::ZERO {
            return Err(ConfigError::InvalidAddress(
                "Sequencing contract address cannot be 0".to_string(),
            ));
        }

        match self.target_rollup_type {
            // Validate Arbitrum specific configuration
            TargetRollupType::ARBITRUM => {
                if self.mchain_rollup_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "MChain rollup address cannot be 0".to_string(),
                    ));
                }
                if self.delayed_inbox_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "Delayed inbox address cannot be 0".to_string(),
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

        if self.anvil_state_path.is_empty() {
            return Err(ConfigError::InvalidAnvilStatePath(
                "Anvil state path cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[allow(missing_docs)]
/// Possible errors that can occur when initializing the block builder configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid port: {0}")]
    InvalidPort(String),
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

    #[test]
    fn test_default_parsing() {
        let config = BlockBuilderConfig::parse_from(["test"]);
        assert_eq!(config.port, 8888);
        assert_eq!(config.genesis_timestamp, 1712500000);
        assert_eq!(config.target_chain_id, 13331370);
        assert_eq!(
            config.sequencing_contract_address.to_string(),
            "0x0000000000000000000000000000000000000000"
        );
        assert_eq!(config.anvil_state_path, "");
        assert_eq!(config.anvil_state_interval, 1);
        assert_eq!(config.anvil_prune_history, 50);
    }

    #[test]
    fn test_validate_port() {
        let config = BlockBuilderConfig { port: 0, ..Default::default() };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidPort(_))));
    }

    #[test]
    fn test_validate_chain_id() {
        let config = BlockBuilderConfig { target_chain_id: 0, ..Default::default() };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidChainId(_))));
    }

    #[test]
    fn test_validate_rollup_type() {
        let config = BlockBuilderConfig {
            target_rollup_type: TargetRollupType::OPTIMISM,
            ..Default::default()
        };
        assert!(matches!(config.validate(), Err(ConfigError::UnsupportedRollupType(_))));
    }

    #[test]
    fn test_validate_mchain_rollup_address() {
        let config =
            BlockBuilderConfig { mchain_rollup_address: Address::ZERO, ..Default::default() };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidAddress(_))));
    }

    #[test]
    fn test_validate_delayed_inbox_address() {
        let config =
            BlockBuilderConfig { delayed_inbox_address: Address::ZERO, ..Default::default() };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidAddress(_))));
    }

    #[test]
    fn test_validate_sequencing_contract_address() {
        let config =
            BlockBuilderConfig { sequencing_contract_address: Address::ZERO, ..Default::default() };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidAddress(_))));
    }
}
