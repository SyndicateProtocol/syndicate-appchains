//! This module contains `config` for the `synd-maestro` service

use crate::{
    config::ConfigError::RpcUrlInvalidAddress,
    errors::ConfigError,
    valkey::ttl::{waiting_txn::WAITING_TXN_TTL, wallet_nonce::WALLET_NONCE_TTL},
};
use alloy::{
    primitives::ChainId,
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, RootProvider,
    },
};
use clap::Parser;
use derivative::Derivative;
use shared::{
    multi_rpc_provider::MultiRpcProvider,
    parse::{fmt_sanitize_url_for_logging_hashmap, parse_url},
};
use std::{collections::HashMap, time::Duration};
use tracing::{debug, error};
use url::Url;

/// Configuration for Maestro
#[allow(clippy::doc_markdown)]
#[derive(Parser, Derivative, Clone, Default)]
#[derivative(Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8080)]
    pub port: i32,

    /// Metrics port to listen on
    #[arg(short = 'm', long, env = "METRICS_PORT", default_value_t = 8081)]
    pub metrics_port: u16,

    /// Valkey cache address to listen on.
    /// Example: "valkey://0.0.0.0:6379"
    #[arg(short = 'r', long, env = "VALKEY_URL")]
    pub valkey_url: String,

    /// Chain ID to RPC URL mappings as a JSON string object
    /// providers will be used in order of the array, if one fails, the next one is tried.
    /// Example: '{"1": ["https://example.com"], "2": ["https://another.com", "https://backup.com"]}'
    #[derivative(Debug(format_with = "fmt_sanitize_url_for_logging_hashmap"))]
    #[arg(
        short = 'c',
        long,
        env = "CHAIN_RPC_URLS",
        default_value = "{}",
        value_parser = parse_chain_rpc_urls_map
    )]
    pub chain_rpc_urls: HashMap<u64, Vec<Url>>,

    /// Timeout in seconds for connection validation
    #[arg(long, env = "VALIDATION_TIMEOUT", default_value = "5s",
    value_parser = humantime::parse_duration)]
    pub validation_timeout: Duration,

    /// Skip validation of RPC URLs
    #[arg(long, env = "SKIP_VALIDATION", default_value_t = false)]
    pub skip_validation: bool,

    /// Time-to-live (TTL) of waiting transaction Valkey key values
    #[arg(long, env = "WAITING_TXN_TTL", default_value = WAITING_TXN_TTL,
    value_parser = humantime::parse_duration)]
    pub waiting_txn_ttl: Duration,

    /// Time-to-live (TTL) of wallet nonce Valkey key values
    #[arg(long, env = "WALLET_NONCE_TTL", default_value = WALLET_NONCE_TTL,
    value_parser = humantime::parse_duration)]
    pub wallet_nonce_ttl: Duration,

    /// Duration after which a transaction is considered finalized
    #[arg(long, env = "FINALIZATION_DURATION", default_value = "15m",
    value_parser = humantime::parse_duration)]
    pub finalization_duration: Duration,

    /// Interval at which a background task checks for finalized transactions
    #[arg(long, env = "FINALIZATION_CHECKER_INTERVAL", default_value = "1m",
    value_parser = humantime::parse_duration)]
    pub finalization_checker_interval: Duration,

    /// Maximum number of resubmission retries
    #[arg(long, env = "MAX_TRANSACTION_RETRIES", default_value = "3")]
    pub max_transaction_retries: u32,

    /// Skip sender wallet balance check
    #[arg(long, env = "SKIP_BALANCE_CHECK", default_value_t = false)]
    pub skip_balance_check: bool,

    /// Max number of waiting transactions to store for a wallet on a chain
    #[arg(long, env = "MAX_NONCE_BUFFER")]
    pub max_nonce_buffer: Option<u64>,
}

/// Parse the chain ID to URL mappings from the JSON string
fn parse_chain_rpc_urls_map(s: &str) -> Result<HashMap<u64, Vec<Url>>, ConfigError> {
    let map: HashMap<u64, Vec<String>> =
        serde_json::from_str(s).map_err(|e| RpcUrlInvalidAddress(e.to_string()))?;

    map.into_iter()
        .map(|(k, v)| {
            v.into_iter()
                .map(|s| parse_url(&s).map_err(|_| RpcUrlInvalidAddress(s)))
                .collect::<Result<Vec<_>, _>>()
                .map(|urls| (k, urls))
        })
        .collect()
}

#[allow(missing_docs)]
pub type RpcProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }

    /// Validates the configuration
    pub async fn validate(&self) -> Result<HashMap<ChainId, MultiRpcProvider>, ConfigError> {
        // Skip validation if requested
        if self.skip_validation {
            debug!("Skipping config validation");
            return Ok(HashMap::new());
        }
        match self.ping_rpc_urls().await {
            Ok(provider_map) => {
                debug!("All RPC URLs validated successfully");
                Ok(provider_map)
            }
            Err(e) => {
                error!(%e, "RPC URL validation failed");
                Err(e)
            }
        }
    }

    /// Checks that all RPC URLs are accessible by making a test connection. Return usable providers
    #[allow(clippy::cognitive_complexity)]
    async fn ping_rpc_urls(&self) -> Result<HashMap<ChainId, MultiRpcProvider>, ConfigError> {
        // Validate RPC URLs by trying to connect to each one
        let mut provider_map: HashMap<ChainId, MultiRpcProvider> = HashMap::new();

        if self.chain_rpc_urls.is_empty() {
            return Ok(provider_map);
        }

        for (chain_id, urls) in &self.chain_rpc_urls {
            if urls.is_empty() {
                debug!(%chain_id, "No RPC URLs configured for chain, skipping");
                continue;
            }

            debug!(%chain_id, url_count = urls.len(), "Creating MultiRpcProvider for chain");

            match MultiRpcProvider::new(urls.clone(), Some(*chain_id)).await {
                Ok(multi_provider) => {
                    debug!(%chain_id, working_providers = multi_provider.provider_count(), "Successfully created MultiRpcProvider for chain");
                    provider_map.insert(*chain_id, multi_provider);
                }
                Err(e) => {
                    error!(%chain_id, %e, "Failed to create MultiRpcProvider for chain");
                    // Exit immediately when no working providers are found
                    if matches!(e, shared::multi_rpc_provider::MultiRpcError::NoWorkingProviders(_))
                    {
                        error!(%chain_id, "No working providers found for chain - all RPC endpoints failed");
                        std::process::exit(1);
                    }
                    // Continue with other chains for other types of errors
                }
            }
        }
        Ok(provider_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chain_rpc_urls_map_valid_single_url() {
        let input = r#"{"1": ["https://eth-mainnet.g.alchemy.com/v2/demo"]}"#;
        let result = parse_chain_rpc_urls_map(input).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&1));
        assert_eq!(result[&1].len(), 1);
        assert_eq!(result[&1][0].as_str(), "https://eth-mainnet.g.alchemy.com/v2/demo");
    }

    #[test]
    fn test_parse_chain_rpc_urls_map_valid_multiple_urls() {
        let input = r#"{"1": ["https://eth-mainnet.g.alchemy.com/v2/demo", "https://rpc.ankr.com/eth"], "137": ["https://polygon-mainnet.g.alchemy.com/v2/demo"]}"#;
        let result = parse_chain_rpc_urls_map(input).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&1));
        assert!(result.contains_key(&137));
        assert_eq!(result[&1].len(), 2);
        assert_eq!(result[&137].len(), 1);
    }

    #[test]
    fn test_parse_chain_rpc_urls_map_empty_json() {
        let input = "{}";
        let result = parse_chain_rpc_urls_map(input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_chain_rpc_urls_map_invalid_json() {
        let input = r#"{"1": ["https://example.com""#; // Missing closing bracket
        let result = parse_chain_rpc_urls_map(input);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RpcUrlInvalidAddress(_)));
    }

    #[test]
    fn test_parse_chain_rpc_urls_map_invalid_url() {
        let input = r#"{"1": ["not-a-valid-url"]}"#;
        let result = parse_chain_rpc_urls_map(input);
        assert!(matches!(result.unwrap_err(), RpcUrlInvalidAddress(_)));
    }
}
