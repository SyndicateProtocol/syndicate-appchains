//! Configuration for the slotting module

use clap::Parser;
use thiserror::Error;
use tracing::debug;

/// The rollup contract is deployed on block 1, so start slotting at block 2 instead.
pub const FIRST_SLOT: u64 = 2;

/// Configuration for the slotter
#[derive(Parser, Debug, Clone)]
pub struct SlottingConfig {
    /// The duration of a [`Slotter`] slot in milliseconds.
    #[arg(long, env = "SLOTTER_SLOT_DURATION_MS", default_value_t = 2_000)]
    pub slot_duration_ms: u64,

    /// The [`Slotter`] slot number (mchain block) to start from.
    #[arg(long, env = "SLOTTER_START_SLOT_NUMBER", default_value_t = FIRST_SLOT)]
    pub start_slot_number: u64,

    /// The epoch timestamp of the [`Slotter`] slot to start from, in milliseconds.
    /// This is set to the same timestamp as the metachain genesis block by default:
    /// April 7, 2024
    #[arg(long, env = "SLOTTER_START_SLOT_TIMESTAMP", default_value_t = 1712500000 * 1000)]
    pub start_slot_timestamp: u64,
}

impl SlottingConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.slot_duration_ms == 0 {
            return Err(ConfigError::Invalid {
                message: "Slot duration must be greater than 0".to_string(),
            });
        }
        Ok(())
    }

    /// Creates a new [`SlottingConfig`] instance
    pub fn new(
        slot_duration_ms: u64,
        start_slot_number: u64,
        start_slot_timestamp: u64,
    ) -> Result<Self, ConfigError> {
        let config = Self { slot_duration_ms, start_slot_number, start_slot_timestamp };
        config.validate()?;
        debug!("Created slotting config: {:?}", config);
        Ok(config)
    }
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}

impl Default for SlottingConfig {
    fn default() -> Self {
        let config = Self::parse_from([""]);
        debug!("Created default SlottingConfig: {:?}", config);
        config
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_default_slotting_config() {
        let config = SlottingConfig::default();
        assert_eq!(config.slot_duration_ms, 2_000);
        assert_eq!(config.start_slot_number, 2);
        assert_eq!(config.start_slot_timestamp, 1712500000000);
    }

    #[test]
    fn test_default_parsing() {
        let config = SlottingConfig::parse_from(["test"]);
        assert_eq!(config.slot_duration_ms, 2_000);
        assert_eq!(config.start_slot_number, 2);
        assert_eq!(config.start_slot_timestamp, 1712500000000);
    }

    #[test]
    fn test_new_with_validation() {
        // Valid config
        let result = SlottingConfig::new(2_000, 0, 0);
        assert!(result.is_ok());

        // Invalid config
        let result = SlottingConfig::new(0, 0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation() {
        // Test zero duration
        let result = SlottingConfig::new(0, 100, 1_000_000);
        assert_matches!(
            result.unwrap_err(),
            ConfigError::Invalid { message } if message.contains("duration")
        );

        // Test valid config with non-zero values
        let result = SlottingConfig::new(2_000, 100, 1_000_000);
        assert!(result.is_ok());
    }
}
