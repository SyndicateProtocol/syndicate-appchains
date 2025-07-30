//! Common configuration for the `synd-translator`
//!
//! This module contains all possible configuration options for the `synd-translator`. Different
//! crates each inherit a subset of these options to configure themselves

use alloy::primitives::Address;
use clap::Parser;
use common::types::Chain;
use eyre::Result;
use shared::parse::parse_address;
use std::{fmt::Debug, time::Duration};
use synd_block_builder::config::BlockBuilderConfig;
use thiserror::Error;
use tracing::{debug, error};

/// Configuration for a generic chain ingestor
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ChainIngestorConfig {
    pub ws_url: String,
    pub start_block: u64,
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum IngestorConfigError {
    #[error("Empty websocket url")]
    EmptyWSUrl,
    #[error("Invalid start block: {0}")]
    InvalidStartBlock(String),
}

impl Default for ChainIngestorConfig {
    fn default() -> Self {
        Self { ws_url: "wss://localhost:8545".to_string(), start_block: 1 }
    }
}

impl ChainIngestorConfig {
    /// Creates a new [`ChainIngestorConfig`] instance
    pub fn new(ws_url: String, start_block: u64) -> Self {
        let config = Self { ws_url, start_block };
        debug!("Created chain ingestor config: {:?}", config);
        config
    }

    /// Validates the configuration
    pub fn validate(&self, chain: Chain) -> Result<(), IngestorConfigError> {
        if chain == Chain::Settlement && self.ws_url.is_empty() {
            // only the settlement ws url is mandatory
            return Err(IngestorConfigError::EmptyWSUrl);
        }

        Ok(())
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), IngestorConfigError> {
        if self.ws_url.is_empty() {
            return Err(IngestorConfigError::EmptyWSUrl);
        }

        if self.start_block == 0 {
            return Err(IngestorConfigError::InvalidStartBlock(
                "Start block must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Block builder configuration error: {0}")]
    BlockBuilder(#[from] synd_block_builder::config::ConfigError),

    #[error("Ingestor chain configuration error: {0}")]
    Ingestor(#[from] IngestorConfigError),
}

// Due to `clap` not supporting prefixes, we need to redefine the SequencingChainConfig and
// SettlementChainConfig here

/// Configuration for the sequencing chain
#[derive(Parser, Debug, Clone, Default)]
pub struct SequencingChainConfig {
    /// The WS URL of the sequencing chain ingestor
    #[arg(long = "sequencing-ws-url", env = "SEQUENCING_WS_URL")]
    pub sequencing_ws_url: Option<String>,

    /// The block number to start polling from on the sequencing chain
    #[arg(long = "sequencing-start-block", env = "SEQUENCING_START_BLOCK")]
    pub sequencing_start_block: Option<u64>,
}

/// Configuration for the settlement chain
#[derive(Parser, Debug, Clone, Default)]
pub struct SettlementChainConfig {
    /// The WS URL of the settlement chain ingestor
    #[arg(long = "settlement-ws-url", env = "SETTLEMENT_WS_URL")]
    pub settlement_ws_url: String,

    /// The block number to start polling from on the settlement chain
    #[arg(long = "settlement-start-block", env = "SETTLEMENT_START_BLOCK")]
    pub settlement_start_block: Option<u64>,
}

impl From<SequencingChainConfig> for ChainIngestorConfig {
    fn from(config: SequencingChainConfig) -> Self {
        Self {
            ws_url: config.sequencing_ws_url.unwrap_or_default(),
            start_block: config.sequencing_start_block.unwrap_or(0),
        }
    }
}

impl From<SettlementChainConfig> for ChainIngestorConfig {
    fn from(config: SettlementChainConfig) -> Self {
        Self {
            ws_url: config.settlement_ws_url,
            start_block: config.settlement_start_block.unwrap_or(0),
        }
    }
}

impl SequencingChainConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), IngestorConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate(Chain::Sequencing)
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), IngestorConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate_strict()
    }
}

impl SettlementChainConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), IngestorConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate(Chain::Settlement)
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), IngestorConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate_strict()
    }
}

/// Common config struct for the `synd-translator`. This contains all possible config options
/// which other crates can use
#[derive(Parser, Debug, Clone, Default)]
#[command(version, about)]
pub struct TranslatorConfig {
    #[command(flatten)]
    pub block_builder: BlockBuilderConfig,

    #[command(flatten)]
    pub sequencing: SequencingChainConfig,

    #[command(flatten)]
    pub settlement: SettlementChainConfig,

    #[arg(long, env = "PORT", default_value_t = 9090)]
    pub port: u16,

    /// The delay to be applied to settlement chain blocks (expressed in seconds)
    #[arg(long, env = "SETTLEMENT_DELAY")]
    pub settlement_delay: Option<u64>,

    /// The chain ID of the Appchain rollup (not the mchain)
    #[arg(long, env = "APPCHAIN_CHAIN_ID")]
    pub appchain_chain_id: u64,

    /// The address of the ConfigManager contract on the settlement chain
    #[arg(
        long = "config-manager-address",
        env = "CONFIG_MANAGER_ADDRESS",
        value_parser = parse_address
    )]
    pub config_manager_address: Option<Address>,

    /// The URL of the Appchain block explorer
    #[arg(long, env = "APPCHAIN_BLOCK_EXPLORER_URL")]
    pub appchain_block_explorer_url: Option<String>,

    /// The timeout for WebSocket requests
    #[arg(long, env = "WS_REQUEST_TIMEOUT", value_parser=humantime::parse_duration, default_value="10s")]
    pub ws_request_timeout: Duration,

    /// Manually override maximum message buffer capacity per WebSocket subscription
    #[arg(long, env = "WS_MAX_BUFFER_CAPACITY_PER_SUBSCRIPTION", default_value = "1024")]
    pub max_buffer_capacity_per_subscription: usize,

    /// Manually override maximum response size for memory safety (default: 4GB)
    #[arg(long, env = "WS_MAX_RESPONSE_SIZE", default_value = "4294967295")] // u32::MAX
    pub max_response_size: u32,

    /// The timeout for eth_getLogs RPC requests
    #[arg(long, env = "GET_LOGS_TIMEOUT", value_parser=humantime::parse_duration, default_value="60s")]
    pub get_logs_timeout: Duration,

    /// The interval to retry RPC requests
    #[arg(
        long,
        env = "RPC_RETRY_INTERVAL",
        default_value = "1s",
        value_parser = humantime::parse_duration
    )]
    pub rpc_retry_interval: Duration,

    /// The interval to wait for the ingestors to be ready if not ready yet
    #[arg(long, env = "WAIT_FOR_INGESTOR_RETRY_INTERVAL", value_parser=humantime::parse_duration, default_value="1s")]
    pub wait_for_ingestor_retry_interval: Duration,
}

impl TranslatorConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        <Self as Parser>::parse()
    }

    /// Validate [`TranslatorConfig`]
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.sequencing.validate().map_err(ConfigError::Ingestor)?;
        self.settlement.validate().map_err(ConfigError::Ingestor)?;
        Ok(())
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), ConfigError> {
        self.validate()?;
        self.block_builder.validate().map_err(ConfigError::BlockBuilder)?;
        self.sequencing.validate_strict().map_err(ConfigError::Ingestor)?;
        self.settlement.validate_strict().map_err(ConfigError::Ingestor)?;
        Ok(())
    }

    /// Generates a sample command with all possible config fields
    pub fn generate_sample_command() {
        let mut cmd = String::from("cargo run --bin synd-translator -- \\\n");

        // Recursively get all fields from flattened configs
        fn add_fields<T: Parser + 'static>(cmd: &mut String) {
            let app = T::command();
            for arg in app.get_arguments() {
                if let Some(long) = arg.get_long() {
                    cmd.push_str(&format!("  --{} <{}> \\\n", long, long.to_uppercase()));
                }
            }
        }

        add_fields::<BlockBuilderConfig>(&mut cmd);
        add_fields::<SequencingChainConfig>(&mut cmd);
        add_fields::<SettlementChainConfig>(&mut cmd);

        // Remove the trailing slash and newline
        cmd.truncate(cmd.len() - 2);
        println!("{cmd}");
    }
}

#[cfg(test)]
mod tests {
    use super::{ChainIngestorConfig, TranslatorConfig};
    use common::types::Chain;

    #[test]
    fn test_generate_command() {
        TranslatorConfig::generate_sample_command();
    }

    #[test]
    fn test_chain_ingestor_config_validation() {
        // Valid config
        let config = ChainIngestorConfig::new("http://localhost:8545".to_string(), 100);
        assert!(config.validate(Chain::Settlement).is_ok());
    }
}
