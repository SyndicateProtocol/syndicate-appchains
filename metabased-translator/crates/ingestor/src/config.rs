//! The `config` module contains the configuration types for the Metabased Translator's ingestion
//! pipeline

use clap::Parser;
use std::time::Duration;
use thiserror::Error;
use tracing::debug;

/// Configuration for the ingestion pipeline of the Metabased Translator. This includes the
/// configuration for ingesting the sequencer and settlement chains
#[derive(Parser, Debug, Clone)]
pub struct IngestionPipelineConfig {
    /// Configuration for the sequencer chain
    #[command(flatten)]
    pub sequencing: SequencingChainConfig,
    /// Configuration for the settlement chain
    #[command(flatten)]
    pub settlement: SettlementChainConfig,
}

/// Configuration for a generic chain ingestor
#[allow(missing_docs)]
#[derive(Parser, Debug, Clone)]
pub struct ChainIngestorConfig {
    #[arg(long, env, default_value_t = 100)]
    pub buffer_size: usize,

    #[arg(long, env, default_value = "1s", value_parser = humantime::parse_duration)]
    pub polling_interval: Duration,

    #[arg(long, env, default_value = "http://localhost:8545")]
    pub rpc_url: String,

    #[arg(long, env, default_value_t = 0)]
    pub start_block: u64,
}

// Due to `clap` not supporting prefixes, we need to redefine the SequencingChainConfig and
// SettlementChainConfig here

/// Configuration for the sequencing chain
#[derive(Parser, Debug, Clone)]
pub struct SequencingChainConfig {
    /// The size of the buffer used to store incoming blocks
    #[arg(long = "sequencing-buffer-size", env = "SEQUENCING_BUFFER_SIZE", default_value_t = 100)]
    pub sequencing_buffer_size: usize,

    /// The interval between each block polling on the sequencing chain
    #[arg(
        long = "sequencing-polling-interval",
        env = "SEQUENCING_POLLING_INTERVAL",
        default_value = "1s",
        value_parser = humantime::parse_duration
    )]
    pub sequencing_polling_interval: Duration,

    /// The RPC URL of the sequencing chain
    #[arg(
        long = "sequencing-rpc-url",
        env = "SEQUENCING_RPC_URL",
        default_value = "http://localhost:8545"
    )]
    pub sequencing_rpc_url: String,

    /// The block number to start polling from on the sequencing chain
    #[arg(long = "sequencing-start-block", env = "SEQUENCING_START_BLOCK", default_value_t = 0)]
    pub sequencing_start_block: u64,
}

/// Configuration for the settlement chain
#[derive(Parser, Debug, Clone)]
pub struct SettlementChainConfig {
    /// The size of the buffer used to store incoming blocks
    #[arg(long = "settlement-buffer-size", env = "SETTLEMENT_BUFFER_SIZE", default_value_t = 100)]
    pub settlement_buffer_size: usize,

    /// The interval between each block polling on the settlement chain
    #[arg(
        long = "settlement-polling-interval",
        env = "SETTLEMENT_POLLING_INTERVAL",
        default_value = "1s",
        value_parser = humantime::parse_duration
    )]
    pub settlement_polling_interval: Duration,

    /// The RPC URL of the settlement chain
    #[arg(
        long = "settlement-rpc-url",
        env = "SETTLEMENT_RPC_URL",
        default_value = "http://localhost:8546"
    )]
    pub settlement_rpc_url: String,

    /// The block number to start polling from on the settlement chain
    #[arg(long = "settlement-start-block", env = "SETTLEMENT_START_BLOCK", default_value_t = 0)]
    pub settlement_start_block: u64,
}

impl From<ChainIngestorConfig> for SequencingChainConfig {
    fn from(config: ChainIngestorConfig) -> Self {
        Self {
            sequencing_buffer_size: config.buffer_size,
            sequencing_polling_interval: config.polling_interval,
            sequencing_rpc_url: config.rpc_url,
            sequencing_start_block: config.start_block,
        }
    }
}

impl From<SequencingChainConfig> for ChainIngestorConfig {
    fn from(config: SequencingChainConfig) -> Self {
        Self {
            buffer_size: config.sequencing_buffer_size,
            polling_interval: config.sequencing_polling_interval,
            rpc_url: config.sequencing_rpc_url,
            start_block: config.sequencing_start_block,
        }
    }
}

impl From<ChainIngestorConfig> for SettlementChainConfig {
    fn from(config: ChainIngestorConfig) -> Self {
        Self {
            settlement_buffer_size: config.buffer_size,
            settlement_polling_interval: config.polling_interval,
            settlement_rpc_url: config.rpc_url,
            settlement_start_block: config.start_block,
        }
    }
}

impl From<SettlementChainConfig> for ChainIngestorConfig {
    fn from(config: SettlementChainConfig) -> Self {
        Self {
            buffer_size: config.settlement_buffer_size,
            polling_interval: config.settlement_polling_interval,
            rpc_url: config.settlement_rpc_url,
            start_block: config.settlement_start_block,
        }
    }
}

impl SequencingChainConfig {
    #[allow(missing_docs)]
    pub fn validate(&self) -> Result<(), ConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate()
    }
}

impl SettlementChainConfig {
    #[allow(missing_docs)]
    pub fn validate(&self) -> Result<(), ConfigError> {
        let generic_config: ChainIngestorConfig = self.clone().into();
        generic_config.validate()
    }
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid polling interval: {0}")]
    InvalidPollingInterval(String),
    #[error("Invalid buffer size: {0}")]
    InvalidBufferSize(String),
}

// uses clap defaults
impl Default for IngestionPipelineConfig {
    fn default() -> Self {
        Self::parse_from([""])
    }
}

impl IngestionPipelineConfig {
    /// Creates a new `IngestionPipelineConfig` instance given the configs the sequencer and
    /// settlement chain
    pub fn new(
        sequencing: ChainIngestorConfig,
        settlement: ChainIngestorConfig,
    ) -> Result<Self, ConfigError> {
        let config = Self { sequencing: sequencing.into(), settlement: settlement.into() };
        debug!("Created ingestor config: {:?}", config);
        config.validate()?;
        Ok(config)
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.sequencing.validate()?;
        self.settlement.validate()?;
        Ok(())
    }
}

// uses clap defaults
impl Default for ChainIngestorConfig {
    fn default() -> Self {
        Self::parse_from([""])
    }
}

impl ChainIngestorConfig {
    /// Creates a new [`ChainIngestorConfig`] instance
    pub fn new(
        rpc_url: String,
        start_block: u64,
        buffer_size: usize,
        polling_interval: Duration,
    ) -> Result<Self, ConfigError> {
        let config = Self { rpc_url, start_block, buffer_size, polling_interval };
        debug!("Created chain ingestor config: {:?}", config);
        config.validate()?;
        Ok(config)
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.polling_interval.is_zero() {
            return Err(ConfigError::InvalidPollingInterval(
                "Polling interval must be greater than 0".to_string(),
            ));
        }

        if self.buffer_size == 0 {
            return Err(ConfigError::InvalidBufferSize(
                "Buffer size must be greater than 0".to_string(),
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
        ChainIngestorConfig::new("http://test:8545".to_string(), 100, 10, Duration::from_secs(5))
            .unwrap()
    }

    #[test]
    fn test_chain_ingestor_config_validation() {
        // Valid config
        let config = ChainIngestorConfig::new(
            "http://localhost:8545".to_string(),
            100,
            10,
            Duration::from_secs(5),
        )
        .unwrap();
        assert!(config.validate().is_ok());

        // Invalid polling interval
        let config = ChainIngestorConfig {
            rpc_url: "http://localhost:8545".to_string(),
            start_block: 100,
            buffer_size: 10,
            polling_interval: Duration::from_secs(0),
        };
        assert_matches!(config.validate(), Err(ConfigError::InvalidPollingInterval(_)));

        // Invalid buffer size
        let config = ChainIngestorConfig {
            rpc_url: "http://localhost:8545".to_string(),
            start_block: 100,
            buffer_size: 0,
            polling_interval: Duration::from_secs(5),
        };
        assert_matches!(config.validate(), Err(ConfigError::InvalidBufferSize(_)));
    }

    #[test]
    fn test_pipeline_validation() {
        let valid_config = test_chain_ingestor_config();

        let invalid_config = ChainIngestorConfig {
            rpc_url: "http://test:8545".to_string(),
            start_block: 100,
            buffer_size: 0, // Invalid
            polling_interval: Duration::from_secs(5),
        };

        // Pipeline should fail validation if any component fails
        let result = IngestionPipelineConfig::new(valid_config, invalid_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_chain_ingestor_config_polling_interval() {
        let config = test_chain_ingestor_config();
        assert_eq!(config.polling_interval, Duration::from_secs(5));
    }

    #[test]
    fn test_chain_ingestor_config_clone() {
        let config = test_chain_ingestor_config();
        #[allow(clippy::redundant_clone)] // want to test `polling_interval_secs`
        let cloned = config.clone();

        assert_eq!(config.rpc_url, cloned.rpc_url);
        assert_eq!(config.start_block, cloned.start_block);
        assert_eq!(config.buffer_size, cloned.buffer_size);
        assert_eq!(config.polling_interval, cloned.polling_interval);
    }

    #[test]
    fn test_chain_config_conversions() {
        let generic = ChainIngestorConfig {
            buffer_size: 100,
            polling_interval: Duration::from_secs(1),
            rpc_url: "http://sequencer:8545".to_string(),
            start_block: 0,
        };

        let specific: SequencingChainConfig = generic.into();
        assert_eq!(specific.sequencing_buffer_size, 100);
        assert_eq!(specific.sequencing_rpc_url, "http://sequencer:8545");
    }
}
