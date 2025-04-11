//! The `config` module contains the configuration types for the Metabased Translator's ingestion
//! pipeline

use clap::Parser;
use std::time::Duration;
use thiserror::Error;
use tracing::debug;

/// Configuration for a generic chain ingestor
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ChainIngestorConfig {
    pub buffer_size: usize,
    pub polling_interval: Duration,
    pub rpc_url: String,
    pub rpc_timeout: Duration,
    pub start_block: u64,
    pub syncing_batch_size: u64,
    pub backoff_initial_interval: Duration,
    pub backoff_scaling_factor: u64,
    pub max_backoff: Duration,
}

// Due to `clap` not supporting prefixes, we need to redefine the SequencingChainConfig and
// SettlementChainConfig here

/// Configuration for the sequencing chain
#[derive(Parser, Debug, Clone, Default)]
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

    /// The number of parallel requests used for syncing the m-chain
    #[arg(
        long = "sequencing-syncing-batch-size",
        env = "SEQUENCING_SYNCING_BATCH_SIZE",
        default_value_t = 50
    )]
    pub sequencing_syncing_batch_size: u64,

    /// The initial backoff interval for retries
    #[arg(
        long = "sequencing-backoff-initial-interval",
        env = "SEQUENCING_BACKOFF_INITIAL_INTERVAL",
        default_value = "50ms",
        value_parser = humantime::parse_duration
    )]
    pub sequencing_backoff_initial_interval: Duration,

    /// The scaling factor for exponential backoff
    #[arg(
        long = "sequencing-backoff-scaling-factor",
        env = "SEQUENCING_BACKOFF_SCALING_FACTOR",
        default_value_t = 2
    )]
    pub sequencing_backoff_scaling_factor: u64,

    /// The maximum backoff interval for retries
    #[arg(
        long = "sequencing-max-backoff",
        env = "SEQUENCING_MAX_BACKOFF",
        default_value = "60s",
        value_parser = humantime::parse_duration
    )]
    pub sequencing_max_backoff: Duration,
}

/// Configuration for the settlement chain
#[derive(Parser, Debug, Clone, Default)]
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

    /// The number of parallel requests used for syncing the m-chain
    #[arg(
        long = "settlement-syncing-batch-size",
        env = "SETTLEMENT_SYNCING_BATCH_SIZE",
        default_value_t = 50
    )]
    pub settlement_syncing_batch_size: u64,

    /// The initial backoff interval for retries
    #[arg(
        long = "settlement-backoff-initial-interval",
        env = "SETTLEMENT_BACKOFF_INITIAL_INTERVAL",
        default_value = "50ms",
        value_parser = humantime::parse_duration
    )]
    pub settlement_backoff_initial_interval: Duration,

    /// The scaling factor for exponential backoff
    #[arg(
        long = "settlement-backoff-scaling-factor",
        env = "SETTLEMENT_BACKOFF_SCALING_FACTOR",
        default_value_t = 2
    )]
    pub settlement_backoff_scaling_factor: u64,

    /// The maximum backoff interval for retries
    #[arg(
        long = "settlement-max-backoff",
        env = "SETTLEMENT_MAX_BACKOFF",
        default_value = "30s",
        value_parser = humantime::parse_duration
    )]
    pub settlement_max_backoff: Duration,
}

// impl From<ChainIngestorConfig> for SequencingChainConfig {
//     fn from(config: ChainIngestorConfig) -> Self {
//         Self {
//             sequencing_buffer_size: config.buffer_size,
//             sequencing_polling_interval: config.polling_interval,
//             sequencing_rpc_url: config.rpc_url,
//             sequencing_rpc_timeout: config.rpc_timeout,
//             sequencing_start_block: config.start_block,
//             sequencing_syncing_batch_size: config.syncing_batch_size,
//             sequencing_backoff_initial_interval: config.backoff_initial_interval,
//             sequencing_backoff_scaling_factor: config.backoff_scaling_factor,
//             sequencing_max_backoff: config.max_backoff,
//         }
//     }
// }

impl From<SequencingChainConfig> for ChainIngestorConfig {
    fn from(config: SequencingChainConfig) -> Self {
        Self {
            buffer_size: config.sequencing_buffer_size,
            polling_interval: config.sequencing_polling_interval,
            rpc_url: config.sequencing_rpc_url.unwrap_or_default(), // TODO come back to this
            rpc_timeout: config.sequencing_rpc_timeout,
            start_block: config.sequencing_start_block.unwrap_or(0), // TODO come back to this
            syncing_batch_size: config.sequencing_syncing_batch_size,
            backoff_initial_interval: config.sequencing_backoff_initial_interval,
            backoff_scaling_factor: config.sequencing_backoff_scaling_factor,
            max_backoff: config.sequencing_max_backoff,
        }
    }
}

// impl From<ChainIngestorConfig> for SettlementChainConfig {
//     fn from(config: ChainIngestorConfig) -> Self {
//         Self {
//             settlement_buffer_size: config.buffer_size,
//             settlement_polling_interval: config.polling_interval,
//             settlement_rpc_url: config.rpc_url,
//             settlement_rpc_timeout: config.rpc_timeout,
//             settlement_start_block: Some(config.start_block),
//             settlement_syncing_batch_size: config.syncing_batch_size,
//             settlement_backoff_initial_interval: config.backoff_initial_interval,
//             settlement_backoff_scaling_factor: config.backoff_scaling_factor,
//             settlement_max_backoff: config.max_backoff,
//         }
//     }
// }

impl From<SettlementChainConfig> for ChainIngestorConfig {
    fn from(config: SettlementChainConfig) -> Self {
        Self {
            buffer_size: config.settlement_buffer_size,
            polling_interval: config.settlement_polling_interval,
            rpc_url: config.settlement_rpc_url,
            rpc_timeout: config.settlement_rpc_timeout,
            start_block: config.settlement_start_block.unwrap_or(0), // TODO come back to this
            syncing_batch_size: config.settlement_syncing_batch_size,
            backoff_initial_interval: config.settlement_backoff_initial_interval,
            backoff_scaling_factor: config.settlement_backoff_scaling_factor,
            max_backoff: config.settlement_max_backoff,
        }
    }
}

// impl From<&SequencingChainConfig> for ChainIngestorConfig {
//     fn from(config: &SequencingChainConfig) -> Self {
//         Self {
//             buffer_size: config.sequencing_buffer_size,
//             polling_interval: config.sequencing_polling_interval,
//             rpc_url: config.sequencing_rpc_url.clone(),
//             rpc_timeout: config.sequencing_rpc_timeout,
//             start_block: config.sequencing_start_block,
//             syncing_batch_size: config.sequencing_syncing_batch_size,
//             backoff_initial_interval: config.sequencing_backoff_initial_interval,
//             backoff_scaling_factor: config.sequencing_backoff_scaling_factor,
//             max_backoff: config.sequencing_max_backoff,
//         }
//     }
// }

// impl From<&SettlementChainConfig> for ChainIngestorConfig {
//     fn from(config: &SettlementChainConfig) -> Self {
//         Self {
//             buffer_size: config.settlement_buffer_size,
//             polling_interval: config.settlement_polling_interval,
//             rpc_url: config.settlement_rpc_url.clone(),
//             rpc_timeout: config.settlement_rpc_timeout,
//             start_block: Some(config.settlement_start_block),
//             syncing_batch_size: config.settlement_syncing_batch_size,
//             backoff_initial_interval: config.settlement_backoff_initial_interval,
//             backoff_scaling_factor: config.settlement_backoff_scaling_factor,
//             max_backoff: config.settlement_max_backoff,
//         }
//     }
// }

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
    #[error("Empty rpc url")]
    EmptyRpcUrl(),
    #[error("Invalid rpc timeout: {0}")]
    InvalidRpcTimeout(String),
    #[error("Invalid batch size: {0}")]
    InvalidBatchSize(String),
    #[error("Invalid backoff interval: {0}")]
    InvalidBackoffInterval(String),
    #[error("Invalid backoff scaling factor: {0}")]
    InvalidBackoffScalingFactor(String),
    #[error("Invalid max backoff interval: {0}")]
    InvalidMaxBackoff(String),
}

impl Default for ChainIngestorConfig {
    fn default() -> Self {
        Self {
            buffer_size: 100,
            polling_interval: Duration::from_secs(1),
            rpc_url: "http://localhost:8545".to_string(),
            rpc_timeout: Duration::from_secs(5),
            start_block: 1,
            syncing_batch_size: 50,
            backoff_initial_interval: Duration::from_millis(50),
            backoff_scaling_factor: 2,
            max_backoff: Duration::from_secs(30),
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
        buffer_size: usize,
        polling_interval: Duration,
        syncing_batch_size: u64,
        backoff_initial_interval: Duration,
        backoff_scaling_factor: u64,
        max_backoff: Duration,
    ) -> Result<Self, ConfigError> {
        let config = Self {
            rpc_url,
            rpc_timeout,
            start_block,
            buffer_size,
            polling_interval,
            syncing_batch_size,
            backoff_initial_interval,
            backoff_scaling_factor,
            max_backoff,
        };
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

        if self.rpc_url.is_empty() {
            return Err(ConfigError::EmptyRpcUrl());
        }

        if self.rpc_timeout.is_zero() {
            return Err(ConfigError::InvalidRpcTimeout(
                "RPC timeout must be greater than 0".to_string(),
            ));
        }

        // See docs for more context https://docs.alchemy.com/reference/batch-requests
        if self.syncing_batch_size > 1000 {
            return Err(ConfigError::InvalidBatchSize(format!(
                "Batch size must not be greater than 1000, found: {}",
                self.syncing_batch_size
            )));
        }

        if self.backoff_initial_interval.is_zero() {
            return Err(ConfigError::InvalidBackoffInterval(
                "Backoff initial interval must be greater than 0".to_string(),
            ));
        }

        if self.backoff_scaling_factor == 0 {
            return Err(ConfigError::InvalidBackoffScalingFactor(
                "Backoff scaling factor must be greater than 0".to_string(),
            ));
        }

        if self.max_backoff.is_zero() {
            return Err(ConfigError::InvalidMaxBackoff(
                "Max backoff interval must be greater than 0".to_string(),
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
        ChainIngestorConfig::new(
            "http://test:8545".to_string(),
            Duration::from_secs(10),
            100,
            10,
            Duration::from_secs(5),
            50,
            Duration::from_millis(50),
            2,
            Duration::from_secs(30),
        )
        .unwrap()
    }

    #[test]
    fn test_chain_ingestor_config_validation() {
        // Valid config
        let config = ChainIngestorConfig::new(
            "http://localhost:8545".to_string(),
            Duration::from_secs(5),
            100,
            10,
            Duration::from_secs(5),
            50,
            Duration::from_millis(50),
            2,
            Duration::from_secs(30),
        )
        .unwrap();
        assert!(config.validate().is_ok());

        // Invalid polling interval
        let mut config = test_chain_ingestor_config();
        config.polling_interval = Duration::from_secs(0);
        assert_matches!(config.validate(), Err(ConfigError::InvalidPollingInterval(_)));

        // Invalid buffer size
        let mut config = test_chain_ingestor_config();
        config.buffer_size = 0;
        assert_matches!(config.validate(), Err(ConfigError::InvalidBufferSize(_)));

        // Invalid RPC timeout
        let mut config = test_chain_ingestor_config();
        config.rpc_timeout = Duration::from_secs(0);
        assert_matches!(config.validate(), Err(ConfigError::InvalidRpcTimeout(_)));

        // Invalid batch size
        let mut config = test_chain_ingestor_config();
        config.syncing_batch_size = 1001;
        assert_matches!(config.validate(), Err(ConfigError::InvalidBatchSize(_)));
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
        assert_eq!(config.rpc_timeout, cloned.rpc_timeout);
        assert_eq!(config.start_block, cloned.start_block);
        assert_eq!(config.buffer_size, cloned.buffer_size);
        assert_eq!(config.polling_interval, cloned.polling_interval);
        assert_eq!(config.syncing_batch_size, cloned.syncing_batch_size);
        assert_eq!(config.backoff_initial_interval, cloned.backoff_initial_interval);
        assert_eq!(config.backoff_scaling_factor, cloned.backoff_scaling_factor);
        assert_eq!(config.max_backoff, cloned.max_backoff);
    }
}
