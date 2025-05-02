//! The `config` module handles configuration parsing for the tc client.

use clap::Parser;
use std::fmt::Debug;
use thiserror::Error;
use url::Url;

/// The environment of the TC API
#[derive(Debug, Clone)]
pub enum TCEndpoint {
    /// The staging environment
    Staging,
    /// The production environment
    Production,
    /// A raw URL
    Raw(Url),
}

impl TCEndpoint {
    /// Parse a string into a `TCEnvironment`
    pub fn parse(value: &str) -> Result<Self, ConfigError> {
        match value.to_lowercase().as_str() {
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            url => shared::parse::parse_url(url)
                .map(Self::Raw)
                .map_err(|err| ConfigError::InvalidTCEndpoint(format!("for {}, {}", url, err))),
        }
    }

    /// Get the URL for the given environment
    pub fn get_url(&self) -> Url {
        match self {
            Self::Staging =>
            {
                #[allow(clippy::expect_used)]
                Url::parse("https://staging-api.syndicate.io").expect("Failed to parse staging URL")
            }
            Self::Production =>
            {
                #[allow(clippy::expect_used)]
                Url::parse("https://api.syndicate.io").expect("Failed to parse production URL")
            }
            Self::Raw(url) => url.clone(),
        }
    }
}

/// Error type for configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid transaction cloud endpoint
    #[error("Invalid transaction cloud endpoint: {0}")]
    InvalidTCEndpoint(String),

    /// Invalid TC project ID
    #[error("Invalid TC project ID: {0}")]
    InvalidTCProjectID(String),

    /// Invalid TC API key  
    #[error("Invalid TC API key: {0}")]
    InvalidTCAPIKey(String),
}

/// Configuration for the tc sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct TCConfig {
    /// Endpoint of the TC API
    #[arg(short = 'e', long, env = "TC_ENDPOINT", default_value = "staging", value_parser = TCEndpoint::parse)]
    pub tc_endpoint: Option<TCEndpoint>,

    /// Project ID for the TC API
    #[arg(short = 'i', long, env = "TC_PROJECT_ID")]
    pub tc_project_id: Option<String>,

    /// API key for the TC API
    #[arg(short = 'y', long, env = "TC_API_KEY")]
    pub tc_api_key: Option<String>,
}

impl TCConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.tc_endpoint.is_none() {
            return Err(ConfigError::InvalidTCEndpoint(
                "TC endpoint is required when --use_tc is set".into(),
            ));
        }
        if self.tc_project_id.is_none() {
            return Err(ConfigError::InvalidTCProjectID(
                "TC project ID is required when --use_tc is set".into(),
            ));
        }
        if self.tc_api_key.is_none() {
            return Err(ConfigError::InvalidTCAPIKey(
                "TC API key is required when --use_tc is set".into(),
            ));
        }
        Ok(())
    }
}

/// Default implementation for `TCConfig`
impl Default for TCConfig {
    fn default() -> Self {
        Self { tc_endpoint: None, tc_project_id: None, tc_api_key: None }
    }
}
