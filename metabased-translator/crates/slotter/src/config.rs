//! Configuration for the slotter module

use clap::Parser;
use humantime::parse_duration;
use thiserror::Error;
use tracing::debug;

/// Configuration for the slotter
#[derive(Parser, Debug, Clone)]
pub struct SlotterConfig {
    /// Delay applied to settlement chain blocks (in seconds)
    /// This helps sequencing chain blocks to be processed sooner
    /// This delay , like the slot duration, and start blocks, must be set at genesis and never
    /// changed
    #[arg(long, env = "SETTLEMENT_DELAY", default_value_t = 60)]
    pub settlement_delay: u64,

    /// Maximum allowed timestamp difference between chains
    /// If the timestamp difference exceeds this value, the slotter will stop consuming blocks from
    /// the chain that is ahead. A value of 0 disables this feature.
    /// Accepts human-readable time format like "10s", "5m", "1h", etc.
    #[arg(long, env = "MAX_SOURCE_CHAIN_TIME_GAP", default_value = "7days", value_parser = parse_duration_to_seconds)]
    pub max_source_chain_time_gap: u64,
}

// Helper function to parse humantime duration to seconds
fn parse_duration_to_seconds(s: &str) -> Result<u64, humantime::DurationError> {
    parse_duration(s).map(|d| d.as_secs())
}

impl SlotterConfig {
    /// Validates the configuration
    pub const fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }

    /// Creates a new [`SlotterConfig`] instance
    pub fn new(settlement_delay: u64, max_source_chain_time_gap: u64) -> Result<Self, ConfigError> {
        let config = Self { settlement_delay, max_source_chain_time_gap };
        config.validate()?;
        debug!("Created slotter config: {:?}", config);
        Ok(config)
    }
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}

impl Default for SlotterConfig {
    fn default() -> Self {
        Self::parse_from([""])
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_default_slotter_config() {
        let config = SlotterConfig::default();
        assert_eq!(config.settlement_delay, 60);
        assert_eq!(config.max_source_chain_time_gap, 7 * 24 * 60 * 60);
    }

    #[test]
    fn test_default_parsing() {
        let config = SlotterConfig::parse_from(["test"]);
        assert_eq!(config.settlement_delay, 60);
        assert_eq!(config.max_source_chain_time_gap, 7 * 24 * 60 * 60);
    }

    #[test]
    fn test_humantime_parsing() {
        let config = SlotterConfig::parse_from(["test", "--max-source-chain-time-gap", "5m"]);
        assert_eq!(config.max_source_chain_time_gap, 300); // 5 minutes = 300 seconds

        let config = SlotterConfig::parse_from(["test", "--max-source-chain-time-gap", "1h"]);
        assert_eq!(config.max_source_chain_time_gap, 3600); // 1 hour = 3600 seconds

        let config = SlotterConfig::parse_from(["test", "--max-source-chain-time-gap", "30s"]);
        assert_eq!(config.max_source_chain_time_gap, 30); // 30 seconds
    }
}
