//! The `config` module handles configuration parsing for the metabased sequencer.

use crate::bytecode::{is_supported_factory_address, V1_FACTORY_ADDRESS};
use alloy::primitives::Address;
use clap::Parser;
use std::{fmt::Debug, str::FromStr};
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
    /// Unsupported factory address
    #[error("Unsupported factory address: {0}")]
    UnsupportedFactoryAddress(String),
}

/// Configuration for the tc sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Endpoint of the TC API
    #[arg(short = 'e', long, env = "TC_ENDPOINT", default_value = "staging", value_parser = TCEndpoint::parse)]
    pub tc_endpoint: TCEndpoint,

    /// Project ID for the TC API
    #[arg(short = 'i', long, env = "TC_PROJECT_ID")]
    pub tc_project_id: String,

    /// API key for the TC API
    #[arg(short = 'k', long, env = "TC_API_KEY")]
    pub tc_api_key: String,

    /// Metabased sequencer factory address
    #[arg(
        short = 'f',
        long,
        env = "METABASED_SEQUENCER_FACTORY_ADDRESS",
        // Default value is the address of the metabased sequencer factory on Exo
        // https://syndicate-exo.explorer.alchemy.com/address/0xFEA8A2BA8B760348ea95492516620ad45a299d53
        default_value = V1_FACTORY_ADDRESS,
        value_parser = parse_and_check_supported_address
    )]
    pub metabased_sequencer_factory_address: Address,

    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8456)]
    pub port: u16,
}

/// Parse a string into an Ethereum `Address` and check if it is supported.
fn parse_and_check_supported_address(value: &str) -> Result<Address, ConfigError> {
    let address = shared::parse::parse_address(value)?;
    if !is_supported_factory_address(address) {
        return Err(ConfigError::UnsupportedFactoryAddress(value.to_string()));
    }
    Ok(address)
}

impl From<shared::parse::Error> for ConfigError {
    fn from(error: shared::parse::Error) -> Self {
        match error {
            shared::parse::Error::URL(_) => {
                unreachable!("parse_address should only return Error::EthereumAddress")
            }
            shared::parse::Error::EthereumAddress(error) => Self::InvalidAddress(error),
        }
    }
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tc_endpoint: TCEndpoint::Staging,
            tc_project_id: String::new(),
            tc_api_key: String::new(),
            #[allow(clippy::expect_used)]
            metabased_sequencer_factory_address: Address::from_str(V1_FACTORY_ADDRESS)
                .expect("Failed to parse default factory address"),
            port: 8456,
        }
    }
}
