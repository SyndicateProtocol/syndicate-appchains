//! This module contains `config` for the `synd-maestro` service

use crate::{
    config::ConfigError::RpcUrlInvalidAddress,
    errors::ConfigError,
    redis::ttl::{waiting_txn::WAITING_TXN_TTL, wallet_nonce::WALLET_NONCE_TTL},
};
use alloy::{
    primitives::ChainId,
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, Provider, ProviderBuilder, RootProvider,
    },
};
use clap::Parser;
use std::{collections::HashMap, time::Duration};
use tracing::{debug, error};

/// Configuration for Maestro
#[allow(clippy::doc_markdown)]
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8080)]
    pub port: i32,

    /// Metrics port to listen on
    #[arg(short = 'm', long, env = "METRICS_PORT", default_value_t = 8081)]
    pub metrics_port: u16,

    /// Redis address to listen on
    /// Example: "redis://0.0.0.0:6379"
    #[arg(short = 'r', long, env = "REDIS_URL")]
    pub redis_url: String,

    /// Chain ID to RPC URL mappings as a JSON string object
    /// Example: '{"1": "https://example.com", "2": "https://another.com"}'
    #[arg(
        short = 'c',
        long,
        env = "CHAIN_RPC_URLS",
        default_value = "{}",
        value_parser = parse_chain_rpc_urls_map
    )]
    pub chain_rpc_urls: HashMap<String, String>,

    /// Timeout in seconds for connection validation
    #[arg(long, env = "VALIDATION_TIMEOUT", default_value = "5s",
    value_parser = humantime::parse_duration)]
    pub validation_timeout: Duration,

    /// Skip validation of RPC URLs
    #[arg(long, env = "SKIP_VALIDATION", default_value_t = false)]
    pub skip_validation: bool,

    /// Time-to-live (TTL) of waiting transaction Redis key values
    #[arg(long, env = "WAITING_TXN_TTL", default_value = WAITING_TXN_TTL,
    value_parser = humantime::parse_duration)]
    pub waiting_txn_ttl: Duration,

    /// Time-to-live (TTL) of wallet nonce Redis key values
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
}

/// Parse the chain ID to URL mappings from the JSON string
fn parse_chain_rpc_urls_map(s: &str) -> Result<HashMap<String, String>, ConfigError> {
    let map: HashMap<String, String> =
        serde_json::from_str(s).map_err(|e| RpcUrlInvalidAddress(e.to_string()))?;
    Ok(map)
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
    pub async fn validate(&self) -> Result<HashMap<ChainId, RpcProvider>, ConfigError> {
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
    async fn ping_rpc_urls(&self) -> Result<HashMap<ChainId, RpcProvider>, ConfigError> {
        // Validate RPC URLs by trying to connect to each one
        let mut provider_map: HashMap<ChainId, RpcProvider> = HashMap::new();

        if self.chain_rpc_urls.is_empty() {
            return Ok(provider_map)
        }

        for (chain_id, url) in &self.chain_rpc_urls {
            debug!(%chain_id, %url, "Sending test JSON-RPC request");

            let provider = match ProviderBuilder::new().connect(url).await {
                Ok(provider) => provider,
                Err(e) => {
                    error!(%chain_id, %url, %e, "Unable to connect to configured RPC provider. Transactions on this chain will fail");
                    return Err(RpcUrlInvalidAddress(e.to_string()));
                }
            };

            let resp_chain_id = match provider.get_chain_id().await {
                Ok(id) => id,
                Err(e) => {
                    error!(%e, %chain_id, %url, "Unable to connect to configured RPC provider. Transactions on this chain will fail");
                    return Err(RpcUrlInvalidAddress(e.to_string()));
                }
            };

            if resp_chain_id.to_string() != *chain_id {
                return Err(ConfigError::RpcUrlInvalidChainId(
                    url.to_string(),
                    chain_id.to_string(),
                    resp_chain_id.to_string(),
                ));
            }

            debug!(%chain_id, %url, "Successful JSON-RPC request to RPC provider");
            provider_map.insert(resp_chain_id, provider);
        }
        Ok(provider_map)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            metrics_port: 8081,
            redis_url: String::new(),
            chain_rpc_urls: HashMap::new(),
            validation_timeout: Duration::from_secs(5),
            skip_validation: false,
            waiting_txn_ttl: Default::default(),
            wallet_nonce_ttl: Default::default(),
            finalization_duration: Duration::from_secs(5 * 60),
            finalization_checker_interval: Duration::from_secs(5 * 60),
        }
    }
}
