//! The `config` module handles configuration parsing for the batch sequencer.

use alloy::primitives::Address;
use batcher::config::{BatcherConfig, ConfigError as BatcherConfigError};
use clap::Parser;
use shared::parse::parse_address;
use std::fmt::Debug;
use thiserror::Error;

/// Error types relating to Config
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    /// `BatcherConfig` error
    #[error("BatcherConfig error: {0}")]
    Batcher(#[from] BatcherConfigError),
}

/// Config struct for the Batch Sequencer.
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct BatchSequencerConfig {
    /// The contract address that the sequencer will use to submit batches
    #[arg(short = 's', long, env = "SEQUENCING_ADDRESS", value_parser = parse_address)]
    pub sequencing_address: Address,

    #[command(flatten)]
    /// The batcher config
    pub batcher: BatcherConfig,

    /// Metrics port to listen on
    #[arg(short = 'm', long, env = "METRICS_PORT", default_value_t = 8082)]
    pub metrics_port: u16,
}

impl BatchSequencerConfig {
    /// Initialize the config from the CLI arguments and environment variables.
    pub fn initialize() -> Self {
        <Self as Parser>::parse()
    }

    /// Validate the config
    pub async fn validate(&self) -> Result<(), ConfigError> {
        self.batcher.validate().await?;
        Ok(())
    }
}

impl Default for BatchSequencerConfig {
    fn default() -> Self {
        Self::initialize()
    }
}
