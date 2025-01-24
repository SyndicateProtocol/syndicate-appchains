//! Configuration for the block builder service

use alloy::primitives::Address;
use clap::Parser;
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;
use tracing::debug;

/// Configuration for the block builder service
#[derive(Parser, Debug, Clone)]
pub struct BlockBuilderConfig {
    #[allow(missing_docs)]
    #[arg(long, env = "BLOCK_BUILDER_MCHAIN_ID", default_value_t = 13331370)]
    pub mchain_id: u64,

    #[allow(missing_docs)]
    #[arg(short = 'p', long, env = "BLOCK_BUILDER_PORT", default_value_t = 8888)]
    pub port: u16,

    #[allow(missing_docs)]
    #[arg(
        short = 'g',
        long,
        env = "BLOCK_BUILDER_GENESIS_TIMESTAMP",
        default_value_t = 1712500000
    )]
    pub genesis_timestamp: u64,

    #[allow(missing_docs)]
    #[arg(short = 'c', long, env = "BLOCK_BUILDER_CHAIN_ID", default_value_t = 84532)]
    pub chain_id: u64,

    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address,
        default_value = "0x1234000000000000000000000000000000000000")]
    pub sequencing_contract_address: Address,
}

/// Parse a string into an Ethereum `Address`.
fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value).map_err(|_| format!("Invalid address: {}", value))
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        Self {
            mchain_id: 13331370,
            port: 8888,
            genesis_timestamp: 1712500000,
            chain_id: 84532,
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
        mchain_id: u64,
        port: u16,
        genesis_timestamp: u64,
        chain_id: u64,
        sequencing_contract_address: Address,
    ) -> Result<Self, ConfigError> {
        let config =
            Self { mchain_id, port, genesis_timestamp, chain_id, sequencing_contract_address };
        debug!("Created block builder config: {:?}", config);
        config.validate()?;
        Ok(config)
    }

    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::InvalidPort("Port cannot be 0".to_string()));
        }

        if self.chain_id == 0 {
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
        assert_eq!(config.port, 8888);
        assert_eq!(config.genesis_timestamp, 1712500000);
        assert_eq!(config.chain_id, 84532);
        assert_eq!(
            config.sequencing_contract_address.to_string(),
            "0x1234000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_validate() {
        let config = BlockBuilderConfig {
            mchain_id: 1234,
            port: 0,
            genesis_timestamp: 1000000,
            chain_id: 12345,
            sequencing_contract_address: Address::from_str(
                "0x1234000000000000000000000000000000000000",
            )
            .unwrap(),
        };
        assert!(matches!(config.validate(), Err(ConfigError::InvalidPort(_))));
    }
}
