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
use tracing::debug;

const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

/// Configuration for the block builder service
#[derive(Parser, Clone)]
#[allow(missing_docs)]
pub struct BlockBuilderConfig {
    #[arg(short = 'f', long, env = "BLOCK_BUILDER_SNAPSHOT_FILE", default_value_t = String::new())]
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
        default_value = "0x1234000000000000000000000000000000000000")]
    pub sequencing_contract_address: Address,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "BLOCK_BUILDER_TARGET_ROLLUP", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,
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
            .field("file", &self.file)
            .field("port", &self.port)
            .field("genesis_timestamp", &self.genesis_timestamp)
            .field("target_chain_id", &self.target_chain_id)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("target_rollup_type", &self.target_rollup_type)
            .field("signer_key", &"<private>") // Skip showing private key
            .finish()
    }
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        Self {
            file: String::new(),
            port: 8888,
            genesis_timestamp: 1712500000,
            target_chain_id: 13331370,
            target_rollup_type: TargetRollupType::ARBITRUM,
            sequencing_contract_address: Address::from_str(
                "0x1234000000000000000000000000000000000000",
            )
            .unwrap_or_else(|err| {
                panic!("Failed to parse default address: {}", err);
            }),
        }
    }
}

impl BlockBuilderConfig {
    /// Creates a new [`BlockBuilderConfig`] instance.
    pub fn new(
        file: String,
        port: u16,
        genesis_timestamp: u64,
        target_chain_id: u64,
        sequencing_contract_address: Address,
        target_rollup_type: TargetRollupType,
    ) -> Result<Self, ConfigError> {
        let config = Self {
            file,
            port,
            genesis_timestamp,
            target_chain_id,
            sequencing_contract_address,
            target_rollup_type,
        };
        debug!("Created block builder config: {:?}", config);
        config.validate()?;
        Ok(config)
    }

    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::InvalidPort("Port cannot be 0".to_string()));
        }

        if self.target_chain_id == 0 {
            return Err(ConfigError::InvalidChainId("Chain ID cannot be 0".to_string()));
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
        assert_eq!(
            config.sequencing_contract_address.to_string(),
            "0x1234000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_validate() {
        let config = BlockBuilderConfig {
            file: String::new(),
            port: 0,
            genesis_timestamp: 1000000,
            target_chain_id: 12345,
            sequencing_contract_address: Address::from_str(
                "0x1234000000000000000000000000000000000000",
            )
            .unwrap(),
            target_rollup_type: TargetRollupType::ARBITRUM,
        };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidPort(_))));
    }
}
