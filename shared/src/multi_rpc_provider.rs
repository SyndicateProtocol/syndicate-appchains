//! Multi-RPC Provider with failover support
//!
//! This module provides a `MultiRpcProvider` that supports automatic failover between
//! multiple RPC endpoints for the same chain.

use alloy::{
    eips::BlockNumberOrTag, network::Network, primitives::{Address, ChainId, B256, U256}, providers::{
         Caller, EthCall, EthCallManyParams, EthCallParams, Provider, ProviderBuilder, ProviderCall, RootProvider, WalletProvider 
    }, rpc::{client::RpcClient, json_rpc::RpcRecv, types::{TransactionReceipt, TransactionRequest}}, transports::{layers::RetryBackoffLayer, RpcError, TransportErrorKind, TransportResult}
};
use arc_swap::ArcSwap;
use std::{fmt, future::Future, pin::Pin, sync::{atomic::{AtomicUsize, Ordering}, Arc}};
use tracing::{debug, error, info, warn};

use crate::types::FilledProvider;

/// Multi-RPC provider that handles multiple RPC endpoints with automatic failover
pub struct MultiRpcProvider {
    /// List of RPC providers for this chain, ordered by priority
    providers: Vec<Arc<FilledProvider>>,
    /// Currently active provider
    active_provider: ArcSwap<FilledProvider>,
    /// Index of currently active provider
    active_provider_index: AtomicUsize,
}

impl Clone for MultiRpcProvider {
    fn clone(&self) -> Self {
        Self {
            providers: self.providers.clone(),
            active_provider: ArcSwap::new(self.active_provider()),
            active_provider_index: AtomicUsize::new(self.active_provider_index.load(Ordering::Relaxed)),
        }
    }
}

impl fmt::Debug for MultiRpcProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MultiRpcProvider")
            .field("provider_count", &self.providers.len())
            .finish()
    }
}

/// Configuration for retry and rate limiting
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries for rate limit errors
    pub max_retries: u32,
    /// Initial backoff in milliseconds
    pub initial_backoff_ms: u64,
    /// Compute units per second for rate limiting
    pub compute_units_per_second: u64,
    /// Average cost of a request in compute units
    pub avg_request_cost: u64,
}

impl RetryConfig {
    /// Create a new retry configuration
    pub const fn new(max_retries: u32, initial_backoff_ms: u64, compute_units_per_second: u64, avg_request_cost: u64) -> Self {
        Self {
            max_retries,
            initial_backoff_ms,
            compute_units_per_second,
            avg_request_cost,
        }
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
    async fn verify_chain_id<P: Provider>(
        provider: &P,
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
                        "Chain ID mismatch for RPC provider, url will be ignored"
                    );
                    false
                }
            }
            Err(e) => {
                warn!(chain_id = %expected_chain_id, url = %url, index, error = %e, "Failed to verify chain ID for RPC provider, url will be ignored");
                false
            }
        }
    }

    /// Attempt to connect to a single RPC URL with wallet and verify its chain ID
    async fn try_connect_wallet_provider(
        url: &str,
        chain_id: ChainId,
        wallet: alloy::network::EthereumWallet,
        retry_config: Option<&RetryConfig>,
        index: usize,
    ) -> Option<FilledProvider> {
        debug!(chain_id = %chain_id, url = %url, index, "Connecting to RPC provider with wallet");

        let result = if let Some(config) = retry_config {
            // Create RPC client with retry layer
            let rpc_client = {
                RetryBackoffLayer::new(
                    config.max_retries,
                    config.initial_backoff_ms,
                    config.compute_units_per_second,
                ).with_avg_unit_cost(config.avg_request_cost);
                RpcClient::builder()
                .layer(())
            }
                .connect(url)
                .await;
                
                rpc_client.map(|client| {
                    ProviderBuilder::new()
                        .wallet(wallet)
                        .connect_client(client)
                })
        } else {
            // Create provider without retry layer
            ProviderBuilder::new().wallet(wallet).connect(url).await
        };

        match result {
            Ok(provider) => {
                Self::verify_chain_id(&provider, chain_id, url, index).await.then_some(provider)
            }
            Err(e) => {
                warn!(chain_id = %chain_id, url = %url, index, error = %e, "Failed to connect to RPC provider");
                None
            }
        }
    }

    /// Create a new `MultiRpcProvider` from a list of RPC URLs (backward compatibility)
    /// Note: This creates providers without wallet functionality
    pub async fn new(urls: Vec<String>, chain_id: ChainId) -> Result<Self, MultiRpcError> {
        // Create a dummy wallet for for callers that don't have a wallet
        use alloy::signers::local::PrivateKeySigner;
        let dummy_signer = PrivateKeySigner::random();
        let dummy_wallet = alloy::network::EthereumWallet::from(dummy_signer);
        
        Self::new_with_wallet(urls, chain_id, dummy_wallet).await
    }

    /// Create a new `MultiRpcProvider` from a list of RPC URLs with wallet functionality
    pub async fn new_with_wallet(
        urls: Vec<String>, 
        chain_id: ChainId,
        wallet: alloy::network::EthereumWallet,
    ) -> Result<Self, MultiRpcError> {
        Self::new_with_wallet_and_retry(urls, chain_id, wallet, None).await
    }

    /// Create a new `MultiRpcProvider` from a list of RPC URLs with wallet functionality and retry configuration
    pub async fn new_with_wallet_and_retry(
        urls: Vec<String>, 
        chain_id: ChainId,
        wallet: alloy::network::EthereumWallet,
        retry_config: Option<RetryConfig>,
    ) -> Result<Self, MultiRpcError> {
        if urls.is_empty() {
            return Err(MultiRpcError::NoProvidersConfigured);
        }

        let mut providers = Vec::new();
        let mut working_urls = Vec::new();

        for (index, url) in urls.iter().enumerate() {
            if let Some(provider) = Self::try_connect_wallet_provider(url, chain_id, wallet.clone(), retry_config.as_ref(), index).await {
                providers.push(Arc::new(provider));
                working_urls.push(url.clone());
            }
        }

        if providers.is_empty() {
            return Err(MultiRpcError::NoWorkingProviders(urls));
        }

        debug!(chain_id = %chain_id, working_count = providers.len(), total_count = urls.len(), "Created MultiRpcProvider");
        info!(working_urls = ?working_urls, chain_id = %chain_id, "Working URLs for chain");

        Ok(Self { 
            active_provider: ArcSwap::new(providers[0].clone()),
            providers, 
            active_provider_index: AtomicUsize::new(0),
        })
    }

    /// Create from already instantiated providers
    pub fn from_providers(
        providers: Vec<Arc<FilledProvider>>,
    ) -> Self {
        Self { 
            active_provider: ArcSwap::new(providers[0].clone()),
            providers, 
            active_provider_index: AtomicUsize::new(0),
        }
    }

    /// Get the number of providers
    pub const fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Get the currently active provider
    pub fn active_provider(&self) -> Arc<FilledProvider> {
        (*self.active_provider.load()).clone()
    }

    /// Try to switch to the next available provider, returns false if no more providers are available
    fn next_provider(&self) -> bool {
        let current = self.active_provider_index.load(Ordering::Relaxed);
        let next = current + 1;
        
        if next >= self.providers.len() {
            return false;
        }

        self.active_provider_index.store(next, Ordering::Relaxed);
        self.active_provider.store(self.providers[next].clone());
        debug!(
            from_index = current,
            to_index = next,
            "Switched to fallback provider"
        );
        true
    }

    /// Reset to the primary provider (index 0)
    pub fn reset_to_initial_provider(&self) {
        self.active_provider_index.store(0, Ordering::Relaxed);
        self.active_provider.store(self.providers[0].clone());
    }

    /// Determine if an RPC error should trigger a fallback to the next provider
    const fn should_retry_with_next_provider(error: &RpcError<TransportErrorKind>) -> bool {
        match error {
            // Transport errors indicate connectivity/availability issues - try next provider
            RpcError::Transport(_) |
            // Serialization/deserialization errors could be provider-specific - try next provider
            RpcError::SerError(_) | 
            RpcError::DeserError { .. } => true,
            
            // These are legitimate RPC responses/errors - don't retry
            RpcError::ErrorResp(_) |        // Server returned error (e.g., invalid tx, gas estimation failed)
            RpcError::NullResp |            // Server returned null response
            RpcError::UnsupportedFeature(_) | // RPC method not supported
            RpcError::LocalUsageError(_) => false,  // Local preprocessing error
        }
    }

    /// Handle an error and determine if we should try the next provider
    /// Returns Ok(true) if we switched to next provider, Ok(false) if no more providers, Err if immediate error  
    fn handle_error(&self, error: RpcError<TransportErrorKind>) -> ControlFlow {
        let current_index = self.active_provider_index.load(Ordering::Relaxed);
        
        if !Self::should_retry_with_next_provider(&error) {
            debug!(
                provider_index = current_index,
                error = %error,
                "RPC error response, returning immediately"
            );

            self.reset_to_initial_provider();
            // Error is related to the RPC call itself, not the provider
            return ControlFlow::Err(error)
           
        } 

        debug!(
            provider_index = current_index,
            error = %error,
            "Transport/connectivity error, trying next provider"
        );
        if self.next_provider() {
            ControlFlow::RetryWithNextProvider
        } else {
            ControlFlow::ErrAllProvidersFailed
        }
    }


    /// Helper method to retry an operation with automatic failover
    async fn retry_with_fallback<T, F, Fut>(&self, operation: F) -> Result<T, RpcError<TransportErrorKind>>
    where
        F: Fn(Arc<FilledProvider>) -> Fut,
        Fut: Future<Output = Result<T, RpcError<TransportErrorKind>>>,
    {
        loop {
            let provider = self.active_provider();
            match operation(provider).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    match self.handle_error(e) {
                        ControlFlow::RetryWithNextProvider => {},
                        ControlFlow::ErrAllProvidersFailed => return Err(RpcError::LocalUsageError("All providers failed".to_string().into())),
                        ControlFlow::Err(e) => return Err(e),
                    }
                }
            }
        }
    }
}


#[allow(missing_docs)]
#[derive(Debug)]
pub enum ControlFlow {
    RetryWithNextProvider,
    ErrAllProvidersFailed,
    Err(RpcError<TransportErrorKind>),
}


// Implement Provider trait with REAL failover logic that cycles through providers
#[allow(clippy::type_complexity)]
impl Provider<alloy::network::Ethereum> for MultiRpcProvider {
    fn root(&self) -> &RootProvider<alloy::network::Ethereum> {
        self.providers[0].root() // TODO can this be improved?
    }

    fn get_block_number(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::primitives::U64, u64> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<u64, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.get_block_number().await.map_err(|e| {
                RpcError::LocalUsageError(e.to_string().into())
            })
        });
        ProviderCall::from(future)
    }

    // TODO fix this and all the other fns that use active_provider()
    fn call(&self, tx: TransactionRequest) -> EthCall<alloy::network::Ethereum, alloy::primitives::Bytes> {
        EthCall::call(self.clone(),tx).block(BlockNumberOrTag::Pending.into())
    }

    fn estimate_gas(&self, tx: TransactionRequest) -> EthCall<alloy::network::Ethereum, alloy::primitives::U64, u64> {
        // For EthCall, delegate to active provider for now (still better than always first provider)
        // EthCall has complex construction requirements that would need more work
        self.active_provider().estimate_gas(tx)
    }


    fn get_transaction_by_hash(&self, hash: B256) -> ProviderCall<(B256,), Option<alloy::rpc::types::Transaction>> {
        // For now, delegate to active provider (still better than always first provider)
        // Would need to implement custom failover loop with proper error handling
        self.active_provider().get_transaction_by_hash(hash)
    }

    fn simulate<'req>(
        &self,
        payload: &'req alloy::rpc::types::simulate::SimulatePayload,
    ) -> alloy::providers::RpcWithBlock<&'req alloy::rpc::types::simulate::SimulatePayload, Vec<alloy::rpc::types::simulate::SimulatedBlock<alloy::rpc::types::Block>>> {
        // For complex RpcWithBlock methods with lifetime parameters, delegate to active provider
        // This is still better than always using the first provider
        self.active_provider().simulate(payload)
    }

    fn create_access_list<'a>(
        &self,
        request: &'a TransactionRequest,
    ) -> alloy::providers::RpcWithBlock<&'a TransactionRequest, alloy::rpc::types::AccessListResult> {
        // For complex RpcWithBlock methods with lifetime parameters, delegate to active provider
        // This is still better than always using the first provider
        self.active_provider().create_access_list(request)
    }


    fn get_chain_id(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::primitives::U64, u64> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<u64, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_chain_id()).await
        });
        ProviderCall::from(future)
    }

    fn get_gas_price(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::primitives::U128, u128> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<u128, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_gas_price()).await
        });
        ProviderCall::from(future)
    }

    fn get_transaction_receipt(&self, hash: B256) -> ProviderCall<(B256,), Option<TransactionReceipt>> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<Option<TransactionReceipt>, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.get_transaction_receipt(hash).await.map_err(|e| {
                RpcError::LocalUsageError(e.to_string().into())
            })
        });
        ProviderCall::from(future)
    }

    fn get_balance(&self, address: Address) -> alloy::providers::RpcWithBlock<Address, U256> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let future: Pin<Box<dyn Future<Output = Result<U256, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| async move {
                    provider.get_balance(address).block_id(block_id).await
                }).await.map_err(|e| {
                    RpcError::LocalUsageError(e.to_string().into())
                })
            });
            ProviderCall::from(future)
        })
    }

    fn get_transaction_count(&self, address: Address) -> alloy::providers::RpcWithBlock<Address, alloy::primitives::U64, u64> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let future: Pin<Box<dyn Future<Output = Result<u64, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| async move {
                    provider.get_transaction_count(address).block_id(block_id).await
                }).await
            });
            ProviderCall::from(future)
        })
    }

    fn get_accounts(&self) -> ProviderCall<alloy::rpc::client::NoParams, Vec<Address>> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<Vec<Address>, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_accounts()).await
        });
        ProviderCall::from(future)
    }

    fn get_blob_base_fee(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::primitives::U128, u128> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<u128, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_blob_base_fee()).await
        });
        ProviderCall::from(future)
    }

    fn get_max_priority_fee_per_gas(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::primitives::U128, u128> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<u128, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_max_priority_fee_per_gas()).await
        });
        ProviderCall::from(future)
    }

    fn syncing(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::rpc::types::SyncStatus> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<alloy::rpc::types::SyncStatus, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.syncing()).await
        });
        ProviderCall::from(future)
    }

    fn get_client_version(&self) -> ProviderCall<alloy::rpc::client::NoParams, String> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<String, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_client_version()).await
        });
        ProviderCall::from(future)
    }

    fn get_net_version(&self) -> ProviderCall<alloy::rpc::client::NoParams, alloy::primitives::U64, u64> {
        let multi_provider = self.clone();
        let future: Pin<Box<dyn Future<Output = Result<u64, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
            multi_provider.retry_with_fallback(|provider| provider.get_net_version()).await
        });
        ProviderCall::from(future)
    }

    fn get_account_info(
        &self,
        address: Address,
    ) -> alloy::providers::RpcWithBlock<Address, alloy::rpc::types::AccountInfo> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let future: Pin<Box<dyn Future<Output = Result<alloy::rpc::types::AccountInfo, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| async move {
                    provider.get_account_info(address).block_id(block_id).await
                }).await
            });
            ProviderCall::from(future)
        })
    }

    fn get_account(&self, address: Address) -> alloy::providers::RpcWithBlock<Address, alloy::consensus::Account> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let future: Pin<Box<dyn Future<Output = Result<alloy::consensus::Account, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| async move {
                    provider.get_account(address).block_id(block_id).await
                }).await
            });
            ProviderCall::from(future)
        })
    }

    fn get_code_at(&self, address: Address) -> alloy::providers::RpcWithBlock<Address, alloy::primitives::Bytes> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let future: Pin<Box<dyn Future<Output = Result<alloy::primitives::Bytes, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| async move {
                    provider.get_code_at(address).block_id(block_id).await
                }).await
            });
            ProviderCall::from(future)
        })
    }

    fn get_proof(
        &self,
        address: Address,
        keys: Vec<alloy::primitives::StorageKey>,
    ) -> alloy::providers::RpcWithBlock<(Address, Vec<alloy::primitives::StorageKey>), alloy::rpc::types::EIP1186AccountProofResponse> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let keys = keys.clone();
            let future: Pin<Box<dyn Future<Output = Result<alloy::rpc::types::EIP1186AccountProofResponse, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| {
                    let keys = keys.clone();
                    async move {
                        provider.get_proof(address, keys).block_id(block_id).await
                    }
                }).await
            });
            ProviderCall::from(future)
        })
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: U256,
    ) -> alloy::providers::RpcWithBlock<(Address, U256), U256> {
        let multi_provider = self.clone();
        alloy::providers::RpcWithBlock::new_provider(move |block_id| {
            let multi_provider = multi_provider.clone();
            let future: Pin<Box<dyn Future<Output = Result<U256, RpcError<TransportErrorKind>>> + Send>> = Box::pin(async move {
                multi_provider.retry_with_fallback(|provider| async move {
                    provider.get_storage_at(address, key).block_id(block_id).await
                }).await
            });
            ProviderCall::from(future)
        })
    }


    // TODO implement missing methods
}

// TODO implement this to be able to use it in eth_call, estimate_gas, etc.
impl<N, Resp> Caller<N, Resp> for MultiRpcProvider 
where
    N: Network,
    Resp: RpcRecv,
{
    fn call(
        &self,
        params: EthCallParams<N>,
    ) -> TransportResult<ProviderCall<EthCallParams<N>, Resp>>{
        todo!()
    }

    fn estimate_gas(
        &self,
        params: EthCallParams<N>,
    ) -> TransportResult<ProviderCall<EthCallParams<N>, Resp>>{
        todo!()
    }

    fn call_many(
        &self,
        params: EthCallManyParams<'_>,
    ) -> TransportResult<ProviderCall<EthCallManyParams<'static>, Resp>>{
        todo!()
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
