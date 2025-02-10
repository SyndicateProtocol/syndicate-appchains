//! Configuration for the slotter module

use clap::Parser;
use thiserror::Error;
use tracing::debug;

/// Configuration for the slotter
#[derive(Parser, Debug, Clone)]
pub struct SlotterConfig {
    /// The duration of a [`Slotter`] slot in seconds.
    #[arg(long, env = "SLOTTER_SLOT_DURATION", default_value_t = 2)]
    pub slot_duration: u64,

    /// The epoch timestamp of the [`Slotter`] slot to start from, in seconds.
    /// This is dynamically set at runtime.
    #[arg(skip)]
    pub start_slot_timestamp: u64,

    /// Delay applied to settlement chain blocks (in seconds)
    #[arg(long, env = "SLOTTER_SETTLEMENT_DELAY", default_value_t = 60)]
    pub settlement_delay: u64,
}

impl SlotterConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.slot_duration == 0 {
            return Err(ConfigError::Invalid {
                message: "Slot duration must be greater than 0".to_string(),
            });
        }
        Ok(())
    }

    /// Creates a new [`SlotterConfig`] instance
    pub fn new(
        slot_duration: u64,
        start_slot_timestamp: u64,
        settlement_delay: u64,
    ) -> Result<Self, ConfigError> {
        let config = Self { slot_duration, start_slot_timestamp, settlement_delay };
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
        let mut config = Self::parse_from([""]);
        config.start_slot_timestamp = 1712500000;
        config
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_default_slotter_config() {
        let config = SlotterConfig::default();
        assert_eq!(config.slot_duration, 2);
        assert_eq!(config.start_slot_timestamp, 1712500000);
    }

    #[test]
    fn test_default_parsing() {
        let config = SlotterConfig::parse_from(["test"]);
        assert_eq!(config.slot_duration, 2);
        assert_eq!(config.start_slot_timestamp, 0);
    }

    #[test]
    fn test_new_with_validation() {
        // Valid config
        let result = SlotterConfig::new(2_000, 0, 60);
        assert!(result.is_ok());

        // Invalid config
        let result = SlotterConfig::new(0, 0, 60);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation() {
        // Test zero duration
        let result = SlotterConfig::new(0, 1_000_000, 60);
        assert_matches!(
            result.unwrap_err(),
            ConfigError::Invalid { message } if message.contains("duration")
        );

        // Test valid config with non-zero values
        let result = SlotterConfig::new(2_000, 1_000_000, 60);
        assert!(result.is_ok());
    }
}
