//! Configuration for the slotter module

use clap::Parser;
use thiserror::Error;
use tracing::debug;

/// Configuration for the slotter
#[derive(Parser, Debug, Clone)]
pub struct SlotterConfig {
    /// Delay applied to settlement chain blocks (in seconds)
    /// This helps sequencing chain blocks to be processed sooner
    /// This delay , like the slot duration, and start blocks, must be set at genesis and never
    /// changed
    #[arg(long, env = "SLOTTER_SETTLEMENT_DELAY", default_value_t = 60)]
    pub settlement_delay: u64,
}

impl SlotterConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }

    /// Creates a new [`SlotterConfig`] instance
    pub fn new(settlement_delay: u64) -> Result<Self, ConfigError> {
        let config = Self { settlement_delay };
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
    }

    #[test]
    fn test_default_parsing() {
        let config = SlotterConfig::parse_from(["test"]);
        assert_eq!(config.settlement_delay, 60);
    }
}
