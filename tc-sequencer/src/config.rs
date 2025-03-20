//! The `config` module handles configuration parsing for the metabased sequencer.

use alloy::primitives::Address;
use clap::Parser;
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;
use url::Url;

/// Error type for configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidURL(String),
    /// Invalid URL: no host
    #[error("Invalid URL: no host")]
    InvalidURLHost,
    /// Invalid URL scheme
    #[error("Invalid URL scheme: {0}. Only http and https are supported")]
    InvalidURLScheme(String),

    /// Invalid address
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
}

/// Configuration for the tc sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// URL of the TC API
    #[arg(short = 'u', long, env = "TC_URL", value_parser = parse_url)]
    pub tc_url: Url,

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
        default_value = "0xFEA8A2BA8B760348ea95492516620ad45a299d53",
        value_parser = parse_address
    )]
    pub metabased_sequencer_factory_address: Address,

    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8456)]
    pub port: u16,
}

/// Parse a string into an Ethereum `Address`.
fn parse_address(value: &str) -> Result<Address, ConfigError> {
    Address::from_str(value).map_err(|_| ConfigError::InvalidAddress(value.to_string()))
}

/// Parse default string into a valid [`URL`].
fn parse_url(value: &str) -> Result<Url, ConfigError> {
    Url::parse(value).map_or_else(
        |_err| Err(ConfigError::InvalidURL(value.to_string())),
        |url| {
            if !url.has_host() {
                return Err(ConfigError::InvalidURLHost);
            }
            match url.scheme() {
                "http" | "https" => Ok(url),
                _ => Err(ConfigError::InvalidURLScheme(url.scheme().to_string())),
            }
        },
    )
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}
