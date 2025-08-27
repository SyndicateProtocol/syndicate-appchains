//! Multi-RPC Provider with failover support
//!
//! This module provides a `MultiRpcProvider` that supports automatic failover between
//! multiple RPC endpoints for the same chain.

use crate::{
    parse::{sanitize_url_for_logging, sanitize_url_vec_to_string},
    types::FilledProvider,
};
use alloy::{
    consensus::Account,
    eips::{BlockId, BlockNumberOrTag},
    network::{Ethereum, Network},
    primitives::{Address, BlockHash, Bytes, ChainId, StorageKey, TxHash, B256, U128, U256, U64},
    providers::{
        Caller, EthCall, EthCallMany, EthCallManyParams, EthCallParams, EthGetBlock,
        FilterPollerBuilder, GetSubscription, PendingTransaction, PendingTransactionBuilder,
        PendingTransactionConfig, PendingTransactionError, Provider, ProviderBuilder, ProviderCall,
        RootProvider, RpcWithBlock, SendableTx, SubFullBlocks, WalletProvider, WatchBlocks,
    },
    rpc::{
        client::{NoParams, RpcClient},
        json_rpc::{RpcRecv, RpcSend},
        types::{
            erc4337::TransactionConditional,
            pubsub::{Params, SubscriptionKind},
            simulate::{SimulatePayload, SimulatedBlock},
            AccountInfo, Block, Bundle, EIP1186AccountProofResponse, EthCallResponse, FeeHistory,
            Filter, FilterChanges, Index, Log, TransactionReceipt, TransactionRequest,
        },
    },
    transports::{layers::RetryBackoffLayer, RpcError, TransportErrorKind, TransportResult},
};
use serde_json::value::RawValue;
use std::{
    borrow::Cow,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::{debug, error, info, warn};
use url::Url;

/// Multi-RPC provider that handles multiple RPC endpoints with automatic failover
#[allow(missing_debug_implementations)]
pub struct MultiRpcProvider {
    /// List of RPC providers for this chain, ordered by priority
    providers: Vec<Arc<FilledProvider>>,
    /// Index of the currently active provider
    active_provider_index: Arc<AtomicUsize>,
}

impl Clone for MultiRpcProvider {
    fn clone(&self) -> Self {
        Self {
            providers: self.providers.clone(),
            active_provider_index: Arc::clone(&self.active_provider_index),
        }
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
    pub const fn new(
        max_retries: u32,
        initial_backoff_ms: u64,
        compute_units_per_second: u64,
        avg_request_cost: u64,
    ) -> Self {
        Self { max_retries, initial_backoff_ms, compute_units_per_second, avg_request_cost }
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
    NoWorkingProviders(Vec<Url>),
}

impl MultiRpcProvider {
    /// Create a new `MultiRpcProvider` from a list of RPC URLs (backward compatibility)
    /// Note: This creates providers without wallet functionality
    /// Note `ChainID` is optional, if not provided, the provider's chainID will not be verified
    pub async fn new(urls: Vec<Url>, chain_id: Option<ChainId>) -> Result<Self, MultiRpcError> {
        // Create a dummy wallet for callers that don't have a wallet
        use alloy::signers::local::PrivateKeySigner;
        let dummy_signer = PrivateKeySigner::random();
        let dummy_wallet = alloy::network::EthereumWallet::from(dummy_signer);

        Self::new_with_wallet(urls, chain_id, dummy_wallet).await
    }

    /// Create a new `MultiRpcProvider` from a list of RPC URLs with wallet functionality
    /// Note `ChainID` is optional, if not provided, the provider's chainID will not be verified
    pub async fn new_with_wallet(
        urls: Vec<Url>,
        chain_id: Option<ChainId>,
        wallet: alloy::network::EthereumWallet,
    ) -> Result<Self, MultiRpcError> {
        Self::new_with_wallet_and_retry(urls, chain_id, wallet, None).await
    }

    /// Create a new `MultiRpcProvider` from a list of RPC URLs with wallet functionality and retry
    /// configuration
    /// Note `ChainID` is optional, if not provided, the provider's chainID will not be verified
    pub async fn new_with_wallet_and_retry(
        urls: Vec<Url>,
        chain_id: Option<ChainId>,
        wallet: alloy::network::EthereumWallet,
        retry_config: Option<RetryConfig>,
    ) -> Result<Self, MultiRpcError> {
        if urls.is_empty() {
            return Err(MultiRpcError::NoProvidersConfigured);
        }

        let mut providers = Vec::new();
        let mut working_urls = Vec::new();

        for (index, url) in urls.iter().enumerate() {
            if let Some(provider) = Self::try_connect_wallet_provider(
                url,
                chain_id,
                wallet.clone(),
                retry_config.as_ref(),
                index,
            )
            .await
            {
                providers.push(Arc::new(provider));
                working_urls.push(url.clone());
            }
        }

        if providers.is_empty() {
            return Err(MultiRpcError::NoWorkingProviders(urls));
        }

        info!(
            working_urls = ?sanitize_url_vec_to_string(&working_urls),
            chain_id = chain_id,
            "Working URLs for chain"
        );

        Ok(Self { providers, active_provider_index: Arc::new(AtomicUsize::new(0)) })
    }

    /// Verify that a provider's chain ID matches the expected chain ID
    async fn verify_chain_id<P: Provider>(
        provider: &P,
        expected_chain_id: ChainId,
        url: &Url,
        index: usize,
    ) -> bool {
        match provider.get_chain_id().await {
            Ok(provider_chain_id) => {
                if provider_chain_id == expected_chain_id {
                    debug!(chain_id = %expected_chain_id, url = %sanitize_url_for_logging(url), index, "Successfully connected to RPC provider");
                    true
                } else {
                    warn!(
                        chain_id = %expected_chain_id,
                        provider_chain_id = %provider_chain_id,
                        url = %sanitize_url_for_logging(url),
                        index,
                        "Chain ID mismatch for RPC provider, url will be ignored"
                    );
                    false
                }
            }
            Err(e) => {
                warn!(chain_id = %expected_chain_id, url = %sanitize_url_for_logging(url), index, error = %e, "Failed to verify chain ID for RPC provider, url will be ignored");
                false
            }
        }
    }

    /// Attempt to connect to a single RPC URL with wallet and verify its chain ID
    async fn try_connect_wallet_provider(
        url: &Url,
        chain_id: Option<ChainId>,
        wallet: alloy::network::EthereumWallet,
        retry_config: Option<&RetryConfig>,
        index: usize,
    ) -> Option<FilledProvider> {
        debug!(chain_id = chain_id, url = %sanitize_url_for_logging(url), index, "Connecting to RPC provider with wallet");

        let result = if let Some(config) = retry_config {
            // Create RPC client with retry layer
            let retry_layer = RetryBackoffLayer::new(
                config.max_retries,
                config.initial_backoff_ms,
                config.compute_units_per_second,
            )
            .with_avg_unit_cost(config.avg_request_cost);
            let rpc_client = RpcClient::builder().layer(retry_layer).connect(url.as_str()).await;

            rpc_client.map(|client| ProviderBuilder::new().wallet(wallet).connect_client(client))
        } else {
            // Create provider without retry layer
            ProviderBuilder::new().wallet(wallet).connect(url.as_str()).await
        };

        match result {
            Ok(provider) => {
                if let Some(chain_id) = chain_id {
                    Self::verify_chain_id(&provider, chain_id, url, index).await.then_some(provider)
                } else {
                    Some(provider)
                }
            }
            Err(e) => {
                warn!(chain_id = chain_id, url = %sanitize_url_for_logging(url), index, error = %e, "Failed to connect to RPC provider");
                None
            }
        }
    }

    /// Get the default signer address for the active provider
    pub fn signer_address(&self) -> Address {
        self.active_provider().0.default_signer_address()
    }

    /// Create from already instantiated providers
    pub fn from_providers(providers: Vec<Arc<FilledProvider>>) -> Self {
        Self { providers, active_provider_index: Arc::new(AtomicUsize::new(0)) }
    }

    /// Get the number of providers
    pub const fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Get the currently active provider
    pub fn active_provider(&self) -> (Arc<FilledProvider>, usize) {
        let index = self.active_provider_index.load(Ordering::Acquire);
        let provider = self.providers[index].clone();
        (provider, index)
    }

    /// Determine if an RPC error should trigger a fallback to the next provider
    fn should_failover(error: &RpcError<TransportErrorKind>) -> bool {
        match error {
            // classes of errors that are likely to be provider-specific
            RpcError::Transport(_) |
            RpcError::SerError(_) |
            RpcError::UnsupportedFeature(_) |
            RpcError::DeserError { .. } |
            RpcError::NullResp |
            RpcError::LocalUsageError(_) => true,

            // These are legitimate RPC responses/errors - don't retry
            // (e.g., invalid tx, gas estimation failed)
            // https://github.com/MetaMask/rpc-errors/blob/main/src/error-constants.ts
            RpcError::ErrorResp(error) => ![
                -32000, // invalid input
                -32003, // tx rejected
                -32602, // invalid params
                -32600, // invalid request
            ]
            .contains(&error.code),
        }
    }

    /// Helper method to retry an operation with automatic failover
    async fn execute_with_provider_failover<T, F, Fut>(
        &self,
        operation: F,
    ) -> Result<T, RpcError<TransportErrorKind>>
    where
        F: Fn(Arc<FilledProvider>) -> Fut + Send,
        Fut: Future<Output = Result<T, RpcError<TransportErrorKind>>>,
    {
        let start_provider_index = self.active_provider_index.load(Ordering::Acquire);

        // Try each provider exactly once, starting from current active
        for i in 0..self.providers.len() {
            let provider_index = (start_provider_index + i) % self.providers.len();
            let provider = self.providers[provider_index].clone();

            match operation(provider).await {
                Ok(result) => {
                    debug!(
                        start_index = start_provider_index,
                        successful_index = provider_index,
                        "Request succeeded with provider"
                    );

                    // update the active provider if different that the starting one
                    if i > 0 {
                        let _ = self.active_provider_index.compare_exchange_weak(
                            start_provider_index,
                            provider_index,
                            Ordering::Release,
                            Ordering::Acquire,
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    if !Self::should_failover(&e) {
                        debug!(
                            provider_index = provider_index,
                            error = %e,
                            "RPC error response, returning immediately"
                        );
                        return Err(e);
                    }

                    debug!(
                        provider_index = provider_index,
                        error = %e,
                        "Transport/connectivity error, trying next provider"
                    );
                    // Continue to next provider
                }
            }
        }

        Err(RpcError::LocalUsageError("All providers failed - exhausted all options".into()))
    }

    fn with_failover<Params, Resp, Output, Map, Fut, F>(
        &self,
        operation: F,
    ) -> ProviderCall<Params, Resp, Output, Map>
    where
        F: Fn(Arc<FilledProvider>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<Output, RpcError<TransportErrorKind>>> + Send,
        Params: RpcSend,
        Resp: RpcRecv,
        Map: Fn(Resp) -> Output,
    {
        let multi_provider = self.clone();
        let future: Pin<
            Box<dyn Future<Output = Result<Output, RpcError<TransportErrorKind>>> + Send>,
        > = Box::pin(async move { multi_provider.execute_with_provider_failover(operation).await });
        ProviderCall::from(future)
    }
}

// Implement Provider trait with REAL failover logic that cycles through providers
#[allow(clippy::type_complexity)]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl Provider<Ethereum> for MultiRpcProvider {
    fn root(&self) -> &RootProvider<Ethereum> {
        self.providers[self.active_provider_index.load(Ordering::Acquire)].root()
    }

    fn get_accounts(&self) -> ProviderCall<NoParams, Vec<Address>> {
        self.with_failover(|provider| provider.get_accounts())
    }

    fn get_blob_base_fee(&self) -> ProviderCall<NoParams, U128, u128> {
        self.with_failover(|provider| provider.get_blob_base_fee())
    }

    fn get_block_number(&self) -> ProviderCall<NoParams, U64, u64> {
        self.with_failover(|provider| provider.get_block_number())
    }

    fn call(&self, tx: TransactionRequest) -> EthCall<Ethereum, Bytes> {
        EthCall::call(self.clone(), tx).block(BlockNumberOrTag::Pending.into())
    }

    fn call_many<'req>(
        &self,
        bundles: &'req [Bundle],
    ) -> EthCallMany<'req, Ethereum, Vec<Vec<EthCallResponse>>> {
        EthCallMany::new(self.clone(), bundles)
    }

    fn simulate<'req>(
        &self,
        payload: &'req SimulatePayload,
    ) -> RpcWithBlock<&'req SimulatePayload, Vec<SimulatedBlock<Block>>> {
        // For complex RpcWithBlock methods with lifetime parameters, delegate to active provider
        // This is still better than always using the first provider
        self.active_provider().0.simulate(payload)
    }

    fn get_chain_id(&self) -> ProviderCall<NoParams, U64, u64> {
        self.with_failover(|provider| provider.get_chain_id())
    }

    fn create_access_list<'a>(
        &self,
        request: &'a TransactionRequest,
    ) -> RpcWithBlock<&'a TransactionRequest, alloy::rpc::types::AccessListResult> {
        // For complex RpcWithBlock methods with lifetime parameters, delegate to active provider
        // This is still better than always using the first provider
        self.active_provider().0.create_access_list(request)
    }

    fn estimate_gas(&self, tx: TransactionRequest) -> EthCall<Ethereum, U64, u64> {
        EthCall::gas_estimate(self.clone(), tx)
            .block(BlockNumberOrTag::Pending.into())
            .map_resp(|r| r.to::<u64>())
    }

    async fn get_fee_history(
        &self,
        block_count: u64,
        last_block: BlockNumberOrTag,
        reward_percentiles: &[f64],
    ) -> TransportResult<FeeHistory> {
        self.execute_with_provider_failover(|provider| {
            let percentiles = reward_percentiles.to_owned();
            async move { provider.get_fee_history(block_count, last_block, &percentiles).await }
        })
        .await
    }

    fn get_gas_price(&self) -> ProviderCall<NoParams, U128, u128> {
        self.with_failover(|provider| provider.get_gas_price())
    }

    fn get_account_info(&self, address: Address) -> RpcWithBlock<Address, AccountInfo> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            multi_provider.with_failover(move |provider| async move {
                provider.get_account_info(address).block_id(block_id).await
            })
        })
    }

    fn get_account(&self, address: Address) -> RpcWithBlock<Address, Account> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            multi_provider.with_failover(move |provider| async move {
                provider.get_account(address).block_id(block_id).await
            })
        })
    }

    fn get_balance(&self, address: Address) -> RpcWithBlock<Address, U256> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            multi_provider.with_failover(move |provider| async move {
                provider.get_balance(address).block_id(block_id).await
            })
        })
    }

    fn get_block(&self, block: BlockId) -> EthGetBlock<<Ethereum as Network>::BlockResponse> {
        let multi_provider = self.clone();
        EthGetBlock::new_provider(
            block,
            Box::new(move |block_txs_kind| {
                multi_provider.with_failover(move |provider| async move {
                    provider.get_block(block).kind(block_txs_kind).await
                })
            }),
        )
    }

    fn get_block_by_hash(
        &self,
        hash: BlockHash,
    ) -> EthGetBlock<<Ethereum as Network>::BlockResponse> {
        let multi_provider = self.clone();
        EthGetBlock::new_provider(
            BlockId::Hash(hash.into()),
            Box::new(move |block_txs_kind| {
                multi_provider.with_failover(move |provider| async move {
                    provider.get_block_by_hash(hash).kind(block_txs_kind).await
                })
            }),
        )
    }

    fn get_block_by_number(
        &self,
        number: BlockNumberOrTag,
    ) -> EthGetBlock<<Ethereum as Network>::BlockResponse> {
        let multi_provider = self.clone();
        EthGetBlock::new_provider(
            BlockId::Number(number),
            Box::new(move |block_txs_kind| {
                multi_provider.with_failover(move |provider| async move {
                    provider.get_block_by_number(number).kind(block_txs_kind).await
                })
            }),
        )
    }

    async fn get_block_transaction_count_by_hash(
        &self,
        hash: BlockHash,
    ) -> TransportResult<Option<u64>> {
        self.execute_with_provider_failover(|provider| async move {
            provider.get_block_transaction_count_by_hash(hash).await
        })
        .await
    }

    async fn get_block_transaction_count_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<Option<u64>> {
        self.execute_with_provider_failover(|provider| async move {
            provider.get_block_transaction_count_by_number(block_number).await
        })
        .await
    }

    fn get_block_receipts(
        &self,
        block: BlockId,
    ) -> ProviderCall<(BlockId,), Option<Vec<<Ethereum as Network>::ReceiptResponse>>> {
        self.with_failover(move |provider| provider.get_block_receipts(block))
    }

    fn get_code_at(&self, address: Address) -> RpcWithBlock<Address, Bytes> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            multi_provider.with_failover(move |provider| async move {
                provider.get_code_at(address).block_id(block_id).await
            })
        })
    }

    async fn watch_blocks(&self) -> TransportResult<FilterPollerBuilder<B256>> {
        panic!("not implemented");
    }
    async fn watch_full_blocks(
        &self,
    ) -> TransportResult<WatchBlocks<<Ethereum as Network>::BlockResponse>> {
        panic!("not implemented");
    }

    async fn watch_pending_transactions(&self) -> TransportResult<FilterPollerBuilder<B256>> {
        panic!("not implemented");
    }
    async fn watch_logs(&self, _filter: &Filter) -> TransportResult<FilterPollerBuilder<Log>> {
        panic!("not implemented");
    }

    async fn watch_full_pending_transactions(
        &self,
    ) -> TransportResult<FilterPollerBuilder<<Ethereum as Network>::TransactionResponse>> {
        panic!("not implemented");
    }

    async fn get_filter_changes<R: RpcRecv>(&self, _id: U256) -> TransportResult<Vec<R>>
    where
        Self: Sized,
    {
        panic!("not implemented");
    }

    async fn get_filter_changes_dyn(&self, _id: U256) -> TransportResult<FilterChanges> {
        panic!("not implemented");
    }

    async fn get_filter_logs(&self, _id: U256) -> TransportResult<Vec<Log>> {
        panic!("not implemented");
    }

    async fn uninstall_filter(&self, _id: U256) -> TransportResult<bool> {
        panic!("not implemented");
    }

    #[inline]
    async fn watch_pending_transaction(
        &self,
        _config: PendingTransactionConfig,
    ) -> Result<PendingTransaction, PendingTransactionError> {
        panic!("not implemented");
    }

    async fn get_logs(&self, filter: &Filter) -> TransportResult<Vec<Log>> {
        self.execute_with_provider_failover(
            |provider| async move { provider.get_logs(filter).await },
        )
        .await
    }

    fn get_proof(
        &self,
        address: Address,
        keys: Vec<StorageKey>,
    ) -> RpcWithBlock<(Address, Vec<StorageKey>), EIP1186AccountProofResponse> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            let keys_clone = keys.clone();
            multi_provider.with_failover(move |provider| {
                let value = keys_clone.clone();
                async move { provider.get_proof(address, value).block_id(block_id).await }
            })
        })
    }

    fn get_storage_at(&self, address: Address, key: U256) -> RpcWithBlock<(Address, U256), U256> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            multi_provider.with_failover(move |provider| async move {
                provider.get_storage_at(address, key).block_id(block_id).await
            })
        })
    }

    fn get_transaction_by_sender_nonce(
        &self,
        sender: Address,
        nonce: u64,
    ) -> ProviderCall<(Address, U64), Option<<Ethereum as Network>::TransactionResponse>> {
        self.with_failover(move |provider| provider.get_transaction_by_sender_nonce(sender, nonce))
    }

    fn get_transaction_by_hash(
        &self,
        hash: TxHash,
    ) -> ProviderCall<(TxHash,), Option<<Ethereum as Network>::TransactionResponse>> {
        self.with_failover(move |provider| provider.get_transaction_by_hash(hash))
    }

    fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: B256,
        index: usize,
    ) -> ProviderCall<(B256, Index), Option<<Ethereum as Network>::TransactionResponse>> {
        self.with_failover(move |provider| {
            provider.get_transaction_by_block_hash_and_index(block_hash, index)
        })
    }

    fn get_raw_transaction_by_block_hash_and_index(
        &self,
        block_hash: B256,
        index: usize,
    ) -> ProviderCall<(B256, Index), Option<Bytes>> {
        self.with_failover(move |provider| {
            provider.get_raw_transaction_by_block_hash_and_index(block_hash, index)
        })
    }

    /// Gets a transaction by block number and transaction index position.
    fn get_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumberOrTag,
        index: usize,
    ) -> ProviderCall<(BlockNumberOrTag, Index), Option<<Ethereum as Network>::TransactionResponse>>
    {
        self.with_failover(move |provider| {
            provider.get_transaction_by_block_number_and_index(block_number, index)
        })
    }

    /// Gets a raw transaction by block number and transaction index position.
    fn get_raw_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumberOrTag,
        index: usize,
    ) -> ProviderCall<(BlockNumberOrTag, Index), Option<Bytes>> {
        self.with_failover(move |provider| {
            provider.get_raw_transaction_by_block_number_and_index(block_number, index)
        })
    }

    fn get_raw_transaction_by_hash(&self, hash: TxHash) -> ProviderCall<(TxHash,), Option<Bytes>> {
        self.with_failover(move |provider| provider.get_raw_transaction_by_hash(hash))
    }

    fn get_transaction_count(&self, address: Address) -> RpcWithBlock<Address, U64, u64> {
        let multi_provider = self.clone();
        RpcWithBlock::new_provider(move |block_id| {
            multi_provider.with_failover(move |provider| async move {
                provider.get_transaction_count(address).block_id(block_id).await
            })
        })
    }

    fn get_transaction_receipt(
        &self,
        hash: B256,
    ) -> ProviderCall<(B256,), Option<TransactionReceipt>> {
        let hash_copy = hash;
        self.with_failover(move |provider| provider.get_transaction_receipt(hash_copy))
    }

    async fn get_uncle(
        &self,
        _tag: BlockId,
        _idx: u64,
    ) -> TransportResult<Option<<Ethereum as Network>::BlockResponse>> {
        panic!("not implemented");
    }

    /// Gets the number of uncles for the block specified by the tag [`BlockId`].
    async fn get_uncle_count(&self, _tag: BlockId) -> TransportResult<u64> {
        panic!("not implemented");
    }

    fn get_max_priority_fee_per_gas(&self) -> ProviderCall<NoParams, U128, u128> {
        self.with_failover(|provider| provider.get_max_priority_fee_per_gas())
    }

    async fn new_block_filter(&self) -> TransportResult<U256> {
        self.execute_with_provider_failover(
            |provider| async move { provider.new_block_filter().await },
        )
        .await
    }

    async fn new_filter(&self, filter: &Filter) -> TransportResult<U256> {
        self.execute_with_provider_failover(|provider| {
            let filter = filter.clone();
            async move { provider.new_filter(&filter).await }
        })
        .await
    }

    async fn new_pending_transactions_filter(&self, full: bool) -> TransportResult<U256> {
        self.execute_with_provider_failover(|provider| async move {
            provider.new_pending_transactions_filter(full).await
        })
        .await
    }

    async fn send_raw_transaction(
        &self,
        encoded_tx: &[u8],
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        self.execute_with_provider_failover(|provider| async move {
            provider.send_raw_transaction(encoded_tx).await
        })
        .await
    }

    async fn send_raw_transaction_conditional(
        &self,
        encoded_tx: &[u8],
        conditional: TransactionConditional,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        self.execute_with_provider_failover(move |provider| {
            let conditional = conditional.clone();
            async move { provider.send_raw_transaction_conditional(encoded_tx, conditional).await }
        })
        .await
    }

    async fn send_transaction_internal(
        &self,
        tx: SendableTx<Ethereum>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        self.execute_with_provider_failover(move |provider| {
            let tx = tx.clone();
            async move { provider.send_transaction_internal(tx).await }
        })
        .await
    }

    async fn sign_transaction(
        &self,
        tx: <Ethereum as Network>::TransactionRequest,
    ) -> TransportResult<Bytes> {
        self.execute_with_provider_failover(move |provider| {
            let tx = tx.clone();
            async move { provider.sign_transaction(tx).await }
        })
        .await
    }

    fn subscribe_blocks(
        &self,
    ) -> GetSubscription<(SubscriptionKind,), <Ethereum as Network>::HeaderResponse> {
        panic!("not implemented")
    }

    fn subscribe_full_blocks(&self) -> SubFullBlocks<Ethereum> {
        panic!("not implemented")
    }

    fn subscribe_pending_transactions(&self) -> GetSubscription<(SubscriptionKind,), B256> {
        panic!("not implemented")
    }

    fn subscribe_full_pending_transactions(
        &self,
    ) -> GetSubscription<(SubscriptionKind, Params), <Ethereum as Network>::TransactionResponse>
    {
        panic!("not implemented")
    }

    fn subscribe_logs(&self, _filter: &Filter) -> GetSubscription<(SubscriptionKind, Params), Log> {
        panic!("not implemented")
    }

    fn subscribe<P, R>(&self, _params: P) -> GetSubscription<P, R>
    where
        P: RpcSend,
        R: RpcRecv,
        Self: Sized,
    {
        panic!("not implemented")
    }

    /// Cancels a subscription given the subscription ID.
    async fn unsubscribe(&self, _id: B256) -> TransportResult<()> {
        panic!("not implemented")
    }

    fn syncing(&self) -> ProviderCall<NoParams, alloy::rpc::types::SyncStatus> {
        self.with_failover(|provider| provider.syncing())
    }

    fn get_client_version(&self) -> ProviderCall<NoParams, String> {
        self.with_failover(|provider| provider.get_client_version())
    }

    fn get_sha3(&self, data: &[u8]) -> ProviderCall<(String,), B256> {
        let data = data.to_owned();
        self.with_failover(move |provider| provider.get_sha3(&data))
    }

    fn get_net_version(&self) -> ProviderCall<NoParams, U64, u64> {
        self.with_failover(|provider| provider.get_net_version())
    }

    async fn raw_request<P, R>(&self, method: Cow<'static, str>, params: P) -> TransportResult<R>
    where
        P: RpcSend,
        R: RpcRecv,
        Self: Sized,
    {
        self.execute_with_provider_failover(move |provider| {
            let method = method.clone();
            let params = params.clone();
            async move { provider.raw_request(method, params).await }
        })
        .await
    }

    async fn raw_request_dyn(
        &self,
        method: Cow<'static, str>,
        params: &RawValue,
    ) -> TransportResult<Box<RawValue>> {
        self.execute_with_provider_failover(move |provider| {
            let method = method.clone();
            async move { provider.raw_request_dyn(method, params).await }
        })
        .await
    }
}

impl<N, Resp> Caller<N, Resp> for MultiRpcProvider
where
    N: Network,
    Resp: RpcRecv,
{
    fn call(
        &self,
        params: EthCallParams<N>,
    ) -> TransportResult<ProviderCall<EthCallParams<N>, Resp>> {
        Ok(self.with_failover(move |provider| {
            let params = params.clone();
            async move {
                let weak_client = provider.root().weak_client();
                let call_result = Caller::call(&weak_client, params)?;
                call_result.await
            }
        }))
    }

    fn estimate_gas(
        &self,
        params: EthCallParams<N>,
    ) -> TransportResult<ProviderCall<EthCallParams<N>, Resp>> {
        Ok(self.with_failover(move |provider| {
            let params = params.clone();
            async move {
                let weak_client = provider.weak_client();
                let call_result = Caller::estimate_gas(&weak_client, params)?;
                call_result.await
            }
        }))
    }

    fn call_many(
        &self,
        params: EthCallManyParams<'_>,
    ) -> TransportResult<ProviderCall<EthCallManyParams<'static>, Resp>> {
        let owned_params = params.into_owned();
        Ok(self.with_failover(move |provider| {
            let cloned_params = owned_params.clone();
            async move {
                let weak_client = provider.root().weak_client();
                let call_result = <_ as Caller<N, Resp>>::call_many(&weak_client, cloned_params)?;
                call_result.await
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::setup_global_logging;

    #[ctor::ctor]
    fn init() {
        setup_global_logging();
    }

    #[tokio::test]
    async fn test_empty_urls() {
        let result = MultiRpcProvider::new(vec![], Some(1)).await;
        assert!(matches!(result, Err(MultiRpcError::NoProvidersConfigured)));
    }

    #[tokio::test]
    async fn test_invalid_urls() {
        let result =
            MultiRpcProvider::new(vec![Url::parse("invalid://url").unwrap()], Some(1)).await;
        assert!(matches!(result, Err(MultiRpcError::NoWorkingProviders(_))));
    }

    #[tokio::test]
    async fn test_individual_provider_failure() {
        // Test that the failing provider actually fails when called directly
        let (failing_provider, _server) = create_mock_provider_with_transport_error().await;

        let result = failing_provider.get_block_number().await;
        println!("Direct call to failing provider result: {result:?}");
        assert!(result.is_err(), "Expected failing provider to actually fail");
    }

    #[tokio::test]
    async fn test_failover_on_transport_error() {
        // Create mock providers that simulate transport errors
        let (failing_provider, _failing_server) = create_mock_provider_with_transport_error().await;
        let (working_provider, _working_server) = create_mock_provider_working().await;

        let multi_provider = MultiRpcProvider::from_providers(vec![
            Arc::new(failing_provider),
            Arc::new(working_provider),
        ]);

        // Verify initial state - should be using first provider (index 0)
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 0);

        // Make a call that should trigger failover due to transport error
        let result = multi_provider.get_block_number().await;
        assert!(result.is_ok(), "Expected result to be Ok after failover, got: {result:?}");

        // Should have switched to second provider (index 1)
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_no_failover_on_rpc_error_response() {
        //1st rpc will return an internal error, should failover (-32603 = internal server error)
        let (rpc_internal_error_provider, _rpc_server) =
            create_mock_provider_with_rpc_error(-32603).await;
        //2nd rpc will return an expected API error, should not failover (-32000 = invalid input)
        let (rpc_error_provider, _rpc_server) = create_mock_provider_with_rpc_error(-32000).await;
        // 3rd should never be reached
        let (working_provider, _working_server) = create_mock_provider_working().await;

        let multi_provider = MultiRpcProvider::from_providers(vec![
            Arc::new(rpc_internal_error_provider),
            Arc::new(rpc_error_provider),
            Arc::new(working_provider),
        ]);

        // Verify initial state
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 0);

        // Make a call that should NOT trigger failover due to RPC error response
        let result = multi_provider.get_block_number().await;

        // Should fail with the RPC error
        assert!(result.is_err(), "Expected result to be Err after failover, got: {result:?}");

        // index 0 should be kept as the active provider
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn test_all_providers_fail() {
        // Create multiple failing providers
        let (failing_provider1, _server1) = create_mock_provider_with_transport_error().await;
        let (failing_provider2, _server2) = create_mock_provider_with_transport_error().await;

        let multi_provider = MultiRpcProvider::from_providers(vec![
            Arc::new(failing_provider1),
            Arc::new(failing_provider2),
        ]);

        // Make a call - should try all providers and fail
        let result = multi_provider.get_block_number().await;

        // Should fail after exhausting all providers
        assert!(result.is_err());

        // Check error message indicates all providers failed
        if let Err(error) = result {
            assert!(matches!(error, RpcError::LocalUsageError(_)));
        }
    }

    #[tokio::test]
    async fn test_cloned_provider_failover() {
        // Create mock providers that simulate transport errors
        let (failing_provider, _failing_server) = create_mock_provider_with_transport_error().await;
        let (working_provider, _working_server) = create_mock_provider_working().await;

        let multi_provider = MultiRpcProvider::from_providers(vec![
            Arc::new(failing_provider),
            Arc::new(working_provider),
        ]);

        let cloned_provider = multi_provider.clone();

        // Verify initial state - should be using first provider (index 0)
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 0);

        // Make a call that should trigger failover due to transport error
        let result = multi_provider.get_block_number().await;
        assert!(result.is_ok(), "Expected result to be Ok after failover, got: {result:?}");

        // Should have switched to second provider (index 1)
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 1);
        // assert the cloned provider also has switched to second provider (index 1)
        assert_eq!(cloned_provider.active_provider_index.load(Ordering::SeqCst), 1);

        assert!(multi_provider.active_provider().0.get_block_number().await.is_ok());
        assert!(cloned_provider.active_provider().0.get_block_number().await.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_provider_switching() {
        use std::collections::HashSet;
        use tokio::sync::Barrier;

        // Create three providers: one failing, two working
        let (failing_provider, _failing_server) = create_mock_provider_with_transport_error().await;
        let (working_provider1, _working_server1) = create_mock_provider_working().await;
        let (working_provider2, _working_server2) = create_mock_provider_working().await;

        let multi_provider = Arc::new(MultiRpcProvider::from_providers(vec![
            Arc::new(failing_provider),
            Arc::new(working_provider1),
            Arc::new(working_provider2),
        ]));

        // Verify initial state - should be using first provider (index 0)
        assert_eq!(multi_provider.active_provider_index.load(Ordering::SeqCst), 0);

        // Use a barrier to ensure all threads start simultaneously
        let num_threads = 10;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        for i in 0..num_threads {
            let provider_clone = multi_provider.clone();
            let barrier_clone = barrier.clone();

            let handle = tokio::spawn(async move {
                barrier_clone.wait().await;

                let result = provider_clone.get_block_number().await;
                assert!(
                    result.is_ok(),
                    "Thread {}: Expected result to be Ok after failover, got: {:?}",
                    i,
                    result
                );
                // Return the final provider index each thread observed
                provider_clone.active_provider_index.load(Ordering::SeqCst)
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete and collect results
        let mut final_indices = Vec::new();
        for handle in handles {
            final_indices.push(handle.await.unwrap());
        }

        // All threads should observe the same final provider index after switching
        let unique_indices: HashSet<usize> = final_indices.into_iter().collect();
        assert_eq!(
            unique_indices.len(),
            1,
            "All threads should observe the same final provider index after switching"
        );
        assert_eq!(
            multi_provider.active_provider_index.load(Ordering::SeqCst),
            1,
            "Should have switched away from failing provider"
        );
    }

    // Helper functions to create test providers
    async fn create_mock_provider_with_transport_error() -> (FilledProvider, mockito::ServerGuard) {
        use alloy::signers::local::PrivateKeySigner;

        // Create a provider with an invalid URL to guarantee transport errors
        let dummy_signer = PrivateKeySigner::random();
        let dummy_wallet = alloy::network::EthereumWallet::from(dummy_signer);

        // Use a URL that will definitely cause connection errors
        let provider = ProviderBuilder::new()
            .wallet(dummy_wallet)
            .connect("http://127.0.0.1:1") // Port 1 is reserved and typically not available
            .await
            .expect("Should connect but fail at runtime");

        // Return a dummy server that's not actually used
        let server = mockito::Server::new_async().await;
        (provider, server)
    }

    async fn create_mock_provider_with_rpc_error(
        error_code: i64,
    ) -> (FilledProvider, mockito::ServerGuard) {
        use alloy::signers::local::PrivateKeySigner;
        use serde_json::json;

        let mut server = mockito::Server::new_async().await;

        // Mock server returns JSON-RPC error (not transport error)
        let error_response = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "error": {
                "code": error_code,
                "message": "Internal error"
            }
        });

        server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(error_response.to_string())
            .create_async()
            .await;

        let dummy_signer = PrivateKeySigner::random();
        let dummy_wallet = alloy::network::EthereumWallet::from(dummy_signer);

        let provider = ProviderBuilder::new()
            .wallet(dummy_wallet)
            .connect(&server.url())
            .await
            .expect("Should connect to RPC error mock server");

        (provider, server)
    }

    async fn create_mock_provider_working() -> (FilledProvider, mockito::ServerGuard) {
        use alloy::signers::local::PrivateKeySigner;
        use serde_json::json;

        let mut server = mockito::Server::new_async().await;

        // Mock server returns successful JSON-RPC responses
        let success_response = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "result": "0x1"
        });

        server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(success_response.to_string())
            .create_async()
            .await;

        let dummy_signer = PrivateKeySigner::random();
        let dummy_wallet = alloy::network::EthereumWallet::from(dummy_signer);

        let provider = ProviderBuilder::new()
            .wallet(dummy_wallet)
            .connect(&server.url())
            .await
            .expect("Should connect to working mock server");

        (provider, server)
    }
}
