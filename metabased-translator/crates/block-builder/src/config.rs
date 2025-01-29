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
    #[arg(short = 'f', long, env = "BLOCK_BUILDER_SNAPSHOT_FILE", default_value = "")]
    pub file: String,

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
        default_value = ROLLUP_CONTRACT_ADDRESS)]
    pub sequencing_contract_address: Address,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "BLOCK_BUILDER_TARGET_ROLLUP", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,

    /// Arbitrum rollup address on the m-chain
    #[arg(short = 'm', long, env = "BLOCK_BUILDER_ARBITRUM_MCHAIN_ROLLUP_ADDRESS",
        value_parser = parse_address,
        default_value = ROLLUP_CONTRACT_ADDRESS)]
    pub mchain_rollup_address: Address,

    /// Delayed inbox address on the settlement chain
    #[arg(short = 'd', long, env = "BLOCK_BUILDER_ARBITRUM_DELAYED_INBOX_ADDRESS",
        value_parser = parse_address,
        default_value = ROLLUP_CONTRACT_ADDRESS)]
    pub delayed_inbox_address: Address,
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

#[allow(missing_docs)]
pub fn get_rollup_contract_address() -> Address {
    get_default_private_key_signer().address().create(0)
}

const ROLLUP_CONTRACT_ADDRESS: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

impl Debug for BlockBuilderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBuilderConfig")
            .field("file", &self.file)
            .field("port", &self.port)
            .field("genesis_timestamp", &self.genesis_timestamp)
            .field("target_chain_id", &self.target_chain_id)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("target_rollup_type", &self.target_rollup_type)
            .field("mchain_rollup_address", &self.mchain_rollup_address)
            .field("delayed_inbox_address", &self.delayed_inbox_address)
            .field("signer_key", &"<private>") // Skip showing private key
            .finish()
    }
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        Self::parse_from([""])
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_parsing() {
        let config = BlockBuilderConfig::parse_from(["test"]);
        assert_eq!(config.file, "");
        assert_eq!(config.port, 8888);
        assert_eq!(config.genesis_timestamp, 1712500000);
        assert_eq!(config.target_chain_id, 13331370);
        assert_eq!(config.sequencing_contract_address.to_string(), ROLLUP_CONTRACT_ADDRESS);
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
