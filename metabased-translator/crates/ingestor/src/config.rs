//! The `config` module contains the configuration types for the Metabased Translator's ingestion
//! pipeline

use clap::Parser;
use common::types::Chain;
use std::time::Duration;
use thiserror::Error;
use tracing::debug;

/// Configuration for a generic chain ingestor
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ChainIngestorConfig {
    pub rpc_url: String,
    pub rpc_timeout: Duration,
    pub start_block: u64,
}

// Due to `clap` not supporting prefixes, we need to redefine the SequencingChainConfig and
// SettlementChainConfig here

/// Configuration for the sequencing chain
#[derive(Parser, Debug, Clone, Default)]
pub struct SequencingChainConfig {
    /// The RPC URL of the sequencing chain ingestor
    #[arg(long = "sequencing-rpc-url", env = "SEQUENCING_RPC_URL")]
    pub sequencing_rpc_url: Option<String>,

    /// The timeout duration for RPC requests to the sequencing chain
    #[arg(
        long = "sequencing-rpc-timeout",
        env = "SEQUENCING_RPC_TIMEOUT",
        default_value = "30s",
        value_parser = humantime::parse_duration
    )]
    pub sequencing_rpc_timeout: Duration,

    /// The block number to start polling from on the sequencing chain
    #[arg(long = "sequencing-start-block", env = "SEQUENCING_START_BLOCK")]
    pub sequencing_start_block: Option<u64>,
}

/// Configuration for the settlement chain
#[derive(Parser, Debug, Clone, Default)]
pub struct SettlementChainConfig {
    /// The RPC URL of the settlement chain ingestor
    #[arg(long = "settlement-rpc-url", env = "SETTLEMENT_RPC_URL")]
    pub settlement_rpc_url: String,

    /// The timeout duration for RPC requests to the settlement chain
    #[arg(
        long = "settlement-rpc-timeout",
        env = "SETTLEMENT_RPC_TIMEOUT",
        default_value = "30s",
        value_parser = humantime::parse_duration
    )]
    pub settlement_rpc_timeout: Duration,

    /// The block number to start polling from on the settlement chain
    #[arg(long = "settlement-start-block", env = "SETTLEMENT_START_BLOCK")]
    pub settlement_start_block: Option<u64>,
}

impl From<SequencingChainConfig> for ChainIngestorConfig {
    fn from(config: SequencingChainConfig) -> Self {
        Self {
            rpc_url: config.sequencing_rpc_url.unwrap_or_default(), // TODO come back to this
            rpc_timeout: config.sequencing_rpc_timeout,
            start_block: config.sequencing_start_block.unwrap_or(0), // TODO come back to this
        }
    }
}

impl From<SettlementChainConfig> for ChainIngestorConfig {
    fn from(config: SettlementChainConfig) -> Self {
        Self {
            rpc_url: config.settlement_rpc_url,
            rpc_timeout: config.settlement_rpc_timeout,
            start_block: config.settlement_start_block.unwrap_or(0), // TODO come back to this
        }
    }
}

impl SequencingChainConfig {
    #[allow(missing_docs)]
    pub fn validate(&self) -> Result<(), ConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate(Chain::Sequencing)
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), ConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate_strict()
    }
}

impl SettlementChainConfig {
    #[allow(missing_docs)]
    pub fn validate(&self) -> Result<(), ConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate(Chain::Settlement)
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), ConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate_strict()
    }
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Empty rpc url")]
    EmptyRpcUrl(),
    #[error("Invalid rpc timeout: {0}")]
    InvalidRpcTimeout(String),
    #[error("Invalid start block: {0}")]
    InvalidStartBlock(String),
}

impl Default for ChainIngestorConfig {
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8545".to_string(),
            rpc_timeout: Duration::from_secs(5),
            start_block: 1,
        }
    }
}

impl ChainIngestorConfig {
    /// Creates a new [`ChainIngestorConfig`] instance
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rpc_url: String,
        rpc_timeout: Duration,
        start_block: u64,
    ) -> Result<Self, ConfigError> {
        let config = Self { rpc_url, rpc_timeout, start_block };
        debug!("Created chain ingestor config: {:?}", config);
        Ok(config)
    }

    /// Validates the configuration
    pub fn validate(&self, chain: Chain) -> Result<(), ConfigError> {
        if chain == Chain::Settlement && self.rpc_url.is_empty() {
            // only the settlement rpc url is mandatory
            return Err(ConfigError::EmptyRpcUrl());
        }

        if self.rpc_timeout.is_zero() {
            return Err(ConfigError::InvalidRpcTimeout(
                "RPC timeout must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), ConfigError> {
        if self.rpc_url.is_empty() {
            return Err(ConfigError::EmptyRpcUrl());
        }

        if self.start_block == 0 {
            return Err(ConfigError::InvalidStartBlock(
                "Start block must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    fn test_chain_ingestor_config() -> ChainIngestorConfig {
        ChainIngestorConfig::new("http://test:8545".to_string(), Duration::from_secs(10), 100)
            .unwrap()
    }

    #[test]
    fn test_chain_ingestor_config_validation() {
        // Valid config
        let config = ChainIngestorConfig::new(
            "http://localhost:8545".to_string(),
            Duration::from_secs(5),
            100,
        )
        .unwrap();
        assert!(config.validate(Chain::Settlement).is_ok());

        // Invalid RPC timeout
        let mut config = test_chain_ingestor_config();
        config.rpc_timeout = Duration::from_secs(0);
        assert_matches!(config.validate(Chain::Settlement), Err(ConfigError::InvalidRpcTimeout(_)));
    }

    #[test]
    fn test_chain_ingestor_config_clone() {
        let config = test_chain_ingestor_config();
        #[allow(clippy::redundant_clone)] // want to test `polling_interval_secs`
        let cloned = config.clone();

        assert_eq!(config.rpc_url, cloned.rpc_url);
        assert_eq!(config.rpc_timeout, cloned.rpc_timeout);
        assert_eq!(config.start_block, cloned.start_block);
    }
}
