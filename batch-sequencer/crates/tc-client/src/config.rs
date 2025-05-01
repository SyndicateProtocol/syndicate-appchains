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
    /// Invalid address
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    /// Invalid sequencing addresses
    #[error("Invalid sequencing addresses: {0}")]
    InvalidSequencingAddresses(String),
}

/// Configuration for the tc sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct TCConfig {
    /// Endpoint of the TC API
    #[arg(short = 'e', long, env = "TC_ENDPOINT", default_value = "staging", value_parser = TCEndpoint::parse)]
    pub tc_endpoint: TCEndpoint,

    /// Project ID for the TC API
    #[arg(short = 'i', long, env = "TC_PROJECT_ID", default_value = "project-id")]
    pub tc_project_id: String,

    /// API key for the TC API
    #[arg(short = 'y', long, env = "TC_API_KEY", default_value = "api-key")]
    pub tc_api_key: String,
}

impl From<shared::parse::Error> for ConfigError {
    fn from(error: shared::parse::Error) -> Self {
        match error {
            shared::parse::Error::URL(_) => {
                unreachable!("parse_address should only return Error::EthereumAddress")
            }
            shared::parse::Error::EthereumAddress(error) => Self::InvalidAddress(error),
            shared::parse::Error::InvalidMap(error) => Self::InvalidSequencingAddresses(error),
        }
    }
}

impl TCConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}

impl Default for TCConfig {
    fn default() -> Self {
        Self {
            tc_endpoint: TCEndpoint::Staging,
            tc_project_id: String::new(),
            tc_api_key: String::new(),
        }
    }
}
