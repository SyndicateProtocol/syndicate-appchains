//! The `config` module handles configuration parsing for the metabased sequencer.

use alloy::primitives::Address;
use clap::Parser;
use shared::parse::parse_address;
use std::{collections::HashMap, fmt::Debug, str::FromStr};
use thiserror::Error;
use url::Url;

// TODO [SEQ-792]: Get the sequencing addresses map from a config file
/// Get the sequencing addresses map
pub fn get_sequencing_addresses_default() -> HashMap<u64, Address> {
    let mut map = HashMap::new();
    // Manchego Chain
    map.insert(510000, get_address("0x180972BF154c9Aea86c43149D83B7Ea078c33f48"));
    // Burrata Chain
    map.insert(510001, get_address("0xC1cacFC14624c4E241286Ade61DF545b90f850B4"));
    // IRL Chain
    map.insert(63821, get_address("0x536EA7C009ebE418501a1DB133b281a4a01d50f5"));
    // Commerce Chain
    map.insert(63822, get_address("0x7C8d3922298AbbEF7beE5F3dACC4238326482789"));
    // Dream Chain
    map.insert(63823, get_address("0x62B82d1AF6D61DdfE5b4af38Eb5dE982A7f7565f"));
    // Amino Chain
    map.insert(63824, get_address("0x8CcaC248CcFCA1283981678B7291F48f6e26ad39"));
    // Eco Chain
    map.insert(63825, get_address("0x47ec452FA5035C24217daCC66aA305802F1d0fbe"));
    // Playground Chain
    map.insert(63826, get_address("0x4e001110D16bE154EB586e73d2da823721E1a9cD"));
    map
}

// TODO [SEQ-792]: Remove once we get this from config file
/// Get an address from a string [TEMPORARY]
fn get_address(string: &str) -> Address {
    Address::from_str(string).unwrap_or_else(|_| panic!("Invalid address: {}", string))
}

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
    /// Invalid sequencing addresses mapping
    #[error("Invalid sequencing addresses mapping: {0}")]
    InvalidSequencingAddressesMapping(String),
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

    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8456)]
    pub port: u16,

    /// Mapping of chain IDs to their corresponding sequencing addresses
    #[arg(short = 'a', long, env = "SEQUENCING_ADDRESSES", value_parser = parse_sequencing_addresses)]
    pub sequencing_addresses: HashMap<u64, Address>,

    /// Address of the wallet pool contract
    #[arg(short = 'p', long, env = "WALLET_POOL_ADDRESS", value_parser = parse_address, default_value = "0x9d9E8B09C1f7d9cC1Cdd4a843e695fD580a390E8")]
    pub wallet_pool_address: Address,
}

/// Parse a string into a map of chain IDs to their corresponding sequencing addresses
fn parse_sequencing_addresses(value: &str) -> Result<HashMap<u64, Address>, ConfigError> {
    let mut map = get_sequencing_addresses_default();
    for line in value.split(',') {
        let parts = line.split('=').collect::<Vec<_>>();
        let chain_id = parts[0].parse::<u64>().map_err(|e| {
            ConfigError::InvalidSequencingAddressesMapping(format!("Invalid chain ID: {}", e))
        })?;
        let address = parts[1].parse::<Address>().map_err(|e| {
            ConfigError::InvalidSequencingAddressesMapping(format!("Invalid address: {}", e))
        })?;
        map.insert(chain_id, address);
    }
    Ok(map)
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
            port: 8456,
            wallet_pool_address: Address::ZERO,
            sequencing_addresses: HashMap::new(),
        }
    }
}
