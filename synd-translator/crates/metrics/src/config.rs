//! Configuration for the block builder service

use clap::Parser;
use thiserror::Error;
use tracing::debug;

/// Configuration for the block builder service
#[derive(Parser, Debug, Clone)]
pub struct MetricsConfig {
    #[allow(missing_docs)]
    #[arg(long, env = "METRICS_PORT", default_value_t = 9090)]
    pub metrics_port: u16,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self { metrics_port: 9090 }
    }
}

impl MetricsConfig {
    /// Creates a new [`MetricsConfig`] instance.
    pub fn new(metrics_port: u16) -> Result<Self, ConfigError> {
        let config = Self { metrics_port };
        debug!("Created translator metrics config: {:?}", config);
        config.validate()?;
        Ok(config)
    }

    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.metrics_port == 0 {
            return Err(ConfigError::InvalidPort("Port cannot be 0".to_string()));
        }

        Ok(())
    }
}

#[allow(missing_docs)]
/// Possible errors that can occur when initializing the metrics configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid metrics_port: {0}")]
    InvalidPort(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_default_parsing() {
        let config = MetricsConfig::parse_from(["test"]);
        assert_eq!(config.metrics_port, 9090);
    }

    #[test]
    fn test_validate() {
        let config = MetricsConfig { metrics_port: 0 };
        assert_matches!(config.validate(), Err(ConfigError::InvalidPort(_)));
    }
}
