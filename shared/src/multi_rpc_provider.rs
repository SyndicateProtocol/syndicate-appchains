//! Multi-RPC Provider with failover support
//!
//! This module provides a `MultiRpcProvider` that supports automatic failover between
//! multiple RPC endpoints for the same chain.

use alloy::{
    primitives::{Address, ChainId, B256, U256},
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::{TransactionReceipt, TransactionRequest},
    transports::{RpcError, TransportErrorKind},
};
use std::fmt;
use tracing::{debug, error, warn};

/// Type alias for a standard RPC provider
pub type StandardProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

/// Multi-RPC provider that handles multiple RPC endpoints with automatic failover
#[derive(Clone)]
pub struct MultiRpcProvider {
    /// List of RPC providers for this chain, ordered by priority
    providers: Vec<StandardProvider>,
    /// Chain ID for this provider
    chain_id: ChainId,
    /// URLs for debugging/logging
    urls: Vec<String>,
}

impl fmt::Debug for MultiRpcProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MultiRpcProvider")
            .field("chain_id", &self.chain_id)
            .field("provider_count", &self.providers.len())
            .field("urls", &self.urls)
            .finish()
    }
}

/// Error type for multi-RPC operations and creation
#[derive(Debug, thiserror::Error)]
pub enum MultiRpcError {
    /// All configured RPC providers failed during an operation
    #[error("All RPC providers failed: {0}")]
    AllProvidersFailed(String),

    /// Transport error from the underlying RPC provider
    #[error("Transport error: {0}")]
    Transport(#[from] RpcError<TransportErrorKind>),

    /// No providers were configured during creation
    #[error("No providers configured")]
    NoProvidersConfigured,

    /// No working providers found from the provided URLs during creation
    #[error("No working providers found from URLs: {0:?}")]
    NoWorkingProviders(Vec<String>),
}

impl MultiRpcProvider {
    /// Verify that a provider's chain ID matches the expected chain ID
    async fn verify_chain_id(
        provider: &StandardProvider,
        expected_chain_id: ChainId,
        url: &str,
        index: usize,
    ) -> bool {
        match provider.get_chain_id().await {
            Ok(provider_chain_id) => {
                if provider_chain_id == expected_chain_id {
                    debug!(chain_id = %expected_chain_id, url = %url, index, "Successfully connected to RPC provider");
                    true
                } else {
                    warn!(
                        chain_id = %expected_chain_id,
                        provider_chain_id = %provider_chain_id,
                        url = %url,
                        index,
                        "Chain ID mismatch for RPC provider"
                    );
                    false
                }
            }
            Err(e) => {
                warn!(chain_id = %expected_chain_id, url = %url, index, error = %e, "Failed to verify chain ID for RPC provider");
                false
            }
        }
    }

    /// Attempt to connect to a single RPC URL and verify its chain ID
    async fn try_connect_provider(
        url: &str,
        chain_id: ChainId,
        index: usize,
    ) -> Option<StandardProvider> {
        debug!(chain_id = %chain_id, url = %url, index, "Connecting to RPC provider");

        match ProviderBuilder::new().connect(url).await {
            Ok(provider) => {
                Self::verify_chain_id(&provider, chain_id, url, index).await.then_some(provider)
            }
            Err(e) => {
                warn!(chain_id = %chain_id, url = %url, index, error = %e, "Failed to connect to RPC provider");
                None
            }
        }
    }

    /// Create a new `MultiRpcProvider` from a list of RPC URLs
    pub async fn new(urls: Vec<String>, chain_id: ChainId) -> Result<Self, MultiRpcError> {
        if urls.is_empty() {
            return Err(MultiRpcError::NoProvidersConfigured);
        }

        let mut providers = Vec::new();
        let mut working_urls = Vec::new();

        for (index, url) in urls.iter().enumerate() {
            if let Some(provider) = Self::try_connect_provider(url, chain_id, index).await {
                providers.push(provider);
                working_urls.push(url.clone());
            }
        }

        if providers.is_empty() {
            return Err(MultiRpcError::NoWorkingProviders(urls));
        }

        debug!(chain_id = %chain_id, working_count = providers.len(), total_count = urls.len(), "Created MultiRpcProvider");

        Ok(Self { providers, chain_id, urls: working_urls })
    }

    /// Create from already instantiated providers
    pub const fn from_providers(
        providers: Vec<StandardProvider>,
        chain_id: ChainId,
        urls: Vec<String>,
    ) -> Self {
        Self { providers, chain_id, urls }
    }

    /// Get the number of providers
    pub const fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Get the URLs for this provider
    pub fn urls(&self) -> &[String] {
        &self.urls
    }

    /// Execute an operation with automatic failover across all providers
    async fn execute_with_fallback<T, F, Fut>(&self, operation: F) -> Result<T, MultiRpcError>
    where
        F: Fn(StandardProvider) -> Fut,
        Fut: std::future::Future<Output = Result<T, RpcError<TransportErrorKind>>>,
    {
        let mut last_error = None;

        for (index, provider) in self.providers.iter().enumerate() {
            match operation(provider.clone()).await {
                Ok(result) => {
                    if index > 0 {
                        debug!(
                            chain_id = %self.chain_id,
                            fallback_index = index,
                            url = self.urls.get(index).map_or("unknown", |s| s.as_str()),
                            "RPC fallback succeeded"
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    debug!(
                        chain_id = %self.chain_id,
                        provider_index = index,
                        url = self.urls.get(index).map_or("unknown", |s| s.as_str()),
                        error = %e,
                        "RPC provider failed, trying next"
                    );
                    last_error = Some(e);
                }
            }
        }

        error!(chain_id = %self.chain_id, "All RPC providers failed");
        Err(MultiRpcError::AllProvidersFailed(
            last_error.map_or_else(|| "No providers available".to_string(), |e| e.to_string()),
        ))
    }

    /// Get transaction count for an address
    pub async fn get_transaction_count(&self, address: Address) -> Result<u64, MultiRpcError> {
        self.execute_with_fallback(move |provider| async move {
            provider.get_transaction_count(address).await
        })
        .await
    }

    /// Get balance for an address
    pub async fn get_balance(&self, address: Address) -> Result<U256, MultiRpcError> {
        self.execute_with_fallback(
            move |provider| async move { provider.get_balance(address).await },
        )
        .await
    }

    /// Get transaction receipt
    pub async fn get_transaction_receipt(
        &self,
        hash: B256,
    ) -> Result<Option<TransactionReceipt>, MultiRpcError> {
        self.execute_with_fallback(move |provider| async move {
            provider.get_transaction_receipt(hash).await
        })
        .await
    }

    /// Get chain ID
    pub async fn get_chain_id(&self) -> Result<ChainId, MultiRpcError> {
        // Return the configured chain ID without calling providers
        // since we've already verified this during construction
        Ok(self.chain_id)
    }

    /// Get the current block number
    pub async fn get_block_number(&self) -> Result<u64, MultiRpcError> {
        self.execute_with_fallback(move |provider| async move { provider.get_block_number().await })
            .await
    }

    /// Send a raw transaction
    pub async fn send_raw_transaction(&self, encoded_tx: &[u8]) -> Result<B256, MultiRpcError> {
        self.execute_with_fallback(move |provider| {
            let encoded_tx = encoded_tx.to_vec();
            async move {
                let pending = provider.send_raw_transaction(&encoded_tx).await?;
                Ok(*pending.tx_hash())
            }
        })
        .await
    }

    /// Call a contract method without creating a transaction
    pub async fn call(
        &self,
        tx: TransactionRequest,
    ) -> Result<alloy::primitives::Bytes, MultiRpcError> {
        self.execute_with_fallback(move |provider| {
            let tx = tx.clone();
            async move { provider.call(tx).await }
        })
        .await
    }

    /// Estimate gas for a transaction
    pub async fn estimate_gas(&self, tx: TransactionRequest) -> Result<u64, MultiRpcError> {
        self.execute_with_fallback(move |provider| {
            let tx = tx.clone();
            async move { provider.estimate_gas(tx).await }
        })
        .await
    }

    /// Get gas price
    pub async fn get_gas_price(&self) -> Result<u128, MultiRpcError> {
        self.execute_with_fallback(move |provider| async move { provider.get_gas_price().await })
            .await
    }

    /// Send a transaction and wait for receipt
    pub async fn send_transaction(&self, tx: TransactionRequest) -> Result<B256, MultiRpcError> {
        self.execute_with_fallback(move |provider| {
            let tx = tx.clone();
            async move {
                let pending = provider.send_transaction(tx).await?;
                Ok(*pending.tx_hash())
            }
        })
        .await
    }

    /// Get the first working provider (for compatibility with existing code)
    pub fn first_provider(&self) -> Option<&StandardProvider> {
        self.providers.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_urls() {
        let result = MultiRpcProvider::new(vec![], 1).await;
        assert!(matches!(result, Err(MultiRpcError::NoProvidersConfigured)));
    }

    #[tokio::test]
    async fn test_invalid_urls() {
        let result = MultiRpcProvider::new(vec!["invalid://url".to_string()], 1).await;
        assert!(matches!(result, Err(MultiRpcError::NoWorkingProviders(_))));
    }
}
