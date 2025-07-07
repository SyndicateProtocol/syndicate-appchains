//! This module is the `Maestro` service, which receives transaction requests and
//! validates them before submission into the transaction processing pipeline.

use crate::{
    config::{Config, RpcProvider},
    errors::{
        InternalErrorType::{
            Other, RpcFailedToFetchWalletBalance, RpcFailedToFetchWalletNonce, RpcMissing,
            TransactionSubmissionFailed,
        },
        MaestroError::{self, WaitingTransaction},
        MaestroRpcError::{self, InternalError, JsonRpcError},
        WaitingTransactionError::{FailedToDecode, FailedToEnqueue},
    },
    metrics::MaestroMetrics,
    valkey::{
        keys::wallet_nonce::{chain_wallet_nonce_key, ChainWalletNonceKey},
        models::{
            waiting_transaction::{WaitingGapTxnExt, WaitingTransactionId},
            wallet_nonce::WalletNonceExt,
        },
        streams::producer::{CheckFinalizationResult, StreamProducer},
    },
    valkey_metrics::{CacheType, Operation},
    with_cache_metrics,
};
use alloy::{
    consensus::{Transaction, TxEnvelope},
    hex,
    primitives::{keccak256, utils::format_ether, Address, Bytes, ChainId, B256, U256},
    providers::Provider,
};
use derivative::Derivative;
use redis::{aio::ConnectionManager, AsyncCommands};
use shared::{
    json_rpc::{Rejection::NonceTooLow, RpcError::TransactionRejected},
    tracing::SpanKind,
    tx_validation::{check_signature, decode_transaction},
};
use std::{cmp::Ordering, collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, info, instrument, trace, warn, Instrument};

/// The service for filtering and directing transactions
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MaestroService {
    #[derivative(Debug = "ignore")]
    valkey_conn: ConnectionManager,
    // TODO (SEQ-914): Implement distributed lock not local
    chain_wallets: Mutex<HashMap<ChainWalletNonceKey, Arc<Mutex<()>>>>,
    producers: HashMap<ChainId, Arc<StreamProducer>>,
    rpc_providers: HashMap<ChainId, RpcProvider>,
    config: Config,
    pub(crate) metrics: MaestroMetrics,
}

impl MaestroService {
    /// Create a new instance of the Maestro service
    pub async fn new(
        valkey_conn: ConnectionManager,
        config: Config,
        metrics: MaestroMetrics,
    ) -> Result<Self, MaestroError> {
        let rpc_providers = config.validate().await?;
        if rpc_providers.is_empty() {
            warn!("No RPC providers configured. This is probably undesirable");
        }
        let mut res = Self {
            valkey_conn,
            chain_wallets: Mutex::new(HashMap::new()),
            producers: HashMap::new(),
            rpc_providers,
            config,
            metrics,
        };
        res.create_stream_producers().await;

        Ok(res)
    }

    async fn create_stream_producers(&mut self) {
        for (chain_id, provider) in &self.rpc_providers {
            let provider_clone = provider.clone();
            let metrics_clone = self.metrics.clone();
            let valkey_metrics = Arc::new(self.metrics.valkey.clone());

            self.producers.insert(
                *chain_id,
                Arc::new(
                    StreamProducer::new(
                        self.valkey_conn.clone(),
                        *chain_id,
                        self.config.finalization_checker_interval,
                        self.config.finalization_duration,
                        self.config.max_transaction_retries,
                        move |raw_tx: &[u8]| {
                            let provider_clone = provider_clone.clone();
                            let metrics_clone = metrics_clone.clone();
                            let tx_data = raw_tx.to_vec();
                            async move {
                                Self::handle_finalization(tx_data, &provider_clone, &metrics_clone)
                                    .await
                            }
                        },
                        valkey_metrics,
                    )
                    .await,
                ),
            );
        }
    }

    async fn get_chain_wallet_lock(&self, chain_id: ChainId, wallet: Address) -> Arc<Mutex<()>> {
        let key = chain_wallet_nonce_key(chain_id, wallet);
        let mut locks = self.chain_wallets.lock().await;
        locks.entry(key).or_insert_with(|| Arc::new(Mutex::new(()))).clone()
    }

    /// Checks if a given `chain_id` has a corresponding [`RpcProvider`] configured
    pub fn get_rpc_provider(&self, chain_id: &ChainId) -> Result<&RpcProvider, MaestroRpcError> {
        self.rpc_providers
            .get(chain_id)
            .map_or_else(|| Err(InternalError(RpcMissing(*chain_id))), Ok)
    }

    #[allow(clippy::cognitive_complexity)]
    async fn handle_finalization(
        raw_tx: Vec<u8>,
        provider: &RpcProvider,
        metrics: &MaestroMetrics,
    ) -> CheckFinalizationResult {
        let tx_hash = keccak256(raw_tx.clone());
        match provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(_tx_receipt)) => CheckFinalizationResult::Done,
            Ok(None) => {
                // safe to unwrap because we know the tx is valid
                #[allow(clippy::unwrap_used)]
                let tx = decode_transaction(&Bytes::from(raw_tx)).unwrap();
                #[allow(clippy::unwrap_used)]
                let signer = check_signature(&tx).unwrap();

                match provider.get_transaction_count(signer).await {
                    Ok(nonce) => {
                        if nonce == tx.nonce() {
                            warn!(%tx_hash, "Valid transaction is not finalized, resubmitting");
                            metrics.increment_maestro_resubmitted_transactions_total(1);
                            return CheckFinalizationResult::ReSubmit;
                        }
                        warn!(%tx_hash, "Transaction is not finalized, but nonce is not valid anymore, done");
                        CheckFinalizationResult::Done
                    }
                    Err(err) => {
                        error!(%tx_hash, %err, "Failed to query RPC for nonce during finalization check");
                        CheckFinalizationResult::Done
                    }
                }
            }
            Err(err) => {
                error!(%tx_hash, %err, "Failed to query RPC for transaction receipt during finalization check");
                CheckFinalizationResult::Done
            }
        }
    }

    /// Performs gas and balance checks for a transaction.
    ///
    /// Returns `account_balance` of `wallet` in `wei`
    async fn check_sender_wallet_balance(
        &self,
        tx: &TxEnvelope,
        chain_id: ChainId,
        wallet: Address,
    ) -> Result<U256, MaestroRpcError> {
        let account_balance =
            self.get_rpc_provider(&chain_id)?.get_balance(wallet).await.map_err(|e| {
                error!(%chain_id, %wallet, %e, "Failed to fetch account balance for gas check");
                InternalError(RpcFailedToFetchWalletBalance(chain_id, wallet))
            })?;
        Self::balance_check(tx, account_balance)
    }

    fn balance_check(tx: &TxEnvelope, account_balance: U256) -> Result<U256, MaestroRpcError> {
        let tx_hash_str = format!("0x{}", hex::encode(tx.hash()));
        let max_gas_cost = U256::from(tx.gas_limit())
            .checked_mul(U256::from(tx.max_fee_per_gas()))
            .ok_or_else(|| {
                error!(%tx_hash_str, "Overflow calculating max gas cost");
                InternalError(Other)
            })?;
        let total_required = max_gas_cost.checked_add(tx.value()).ok_or_else(|| {
            error!(%tx_hash_str, "Overflow calculating total required funds");
            InternalError(Other)
        })?;

        if account_balance < total_required {
            debug!(%tx_hash_str, %account_balance, %total_required, "Insufficient funds for transaction");
            return Err(JsonRpcError(TransactionRejected(
                shared::json_rpc::Rejection::InsufficientFunds,
            )));
        }

        trace!(%tx_hash_str, %account_balance, %total_required, "Gas check passed");
        Ok(account_balance)
    }

    /// Processes a transaction based on its nonce compared to the wallet's expected nonce
    ///
    /// This method handles the transaction dispatch logic based on nonce comparison:
    /// - If the nonce matches the expected value, the transaction is enqueued immediately, and the
    ///   system checks the cache for waiting transactions with later nonces
    /// - If the nonce is lower than expected, the transaction is rejected as it would be a
    ///   resubmission
    /// - If the nonce is higher than expected, this "waiting" transaction is stored in a cache
    ///   until all preceding transactions with missing nonces arrive
    ///
    /// # Arguments
    /// * `self` - Arc reference to the service to ensure it outlives spawned tasks
    /// * `raw_tx` - The raw transaction bytes to process
    /// * `tx` - The decoded transaction envelope
    /// * `wallet` - The wallet address that signed the transaction
    /// * `chain_id` - The chain identifier the transaction is intended for
    /// * `tx_nonce` - The nonce value included in the transaction
    ///
    /// # Returns
    /// * `Ok(())` - If the transaction was processed successfully (enqueued)
    /// * `Err(MaestroRpcError)` - If the transaction was rejected or processing failed
    ///
    /// # Errors
    /// Returns an error if:
    /// * Transaction nonce is lower than expected (rejected as [`NonceTooLow`])
    /// * Cache operations fail during transaction enqueuing or caching
    /// * Stream write operations fail
    ///
    /// # Notes
    /// When a transaction with the expected nonce is processed, a background task is
    /// spawned to check for and process any waiting transactions with sequential nonces.
    /// This allows the method to return quickly while potentially processing a chain of
    /// transactions in the background.
    #[instrument(
        skip(self, raw_tx, tx),
        err,
        fields(
            otel.kind = ?SpanKind::Producer,
            tx_hash = format!("0x{}", hex::encode(keccak256(raw_tx.as_ref()))),
            chain_id, wallet = %wallet, tx_nonce
        )
    )]
    pub async fn handle_transaction(
        self: &Arc<Self>,
        raw_tx: Bytes,
        tx: &TxEnvelope,
        wallet: Address,
        chain_id: ChainId,
        tx_nonce: u64,
    ) -> Result<(), MaestroRpcError> {
        let chain_wallet_lock = self.get_chain_wallet_lock(chain_id, wallet).await;
        let _guard = chain_wallet_lock.lock().await;
        trace!(chain_id, %wallet, "got lock");

        let expected_nonce = self.get_cached_or_rpc_nonce(wallet, chain_id).await?;

        match tx_nonce.cmp(&expected_nonce) {
            Ordering::Equal => {
                if !self.config.skip_balance_check {
                    // TODO(SEQ-964): Remove lines below once we have tracing
                    let tx_hash_str = format!("0x{}", hex::encode(tx.hash()));
                    trace!(%tx_hash_str, %tx_nonce, sender_wallet=%wallet, %chain_id, "Checking sender wallet balance");
                    let sender_balance_eth = format!(
                        "{} ETH units",
                        format_ether(self.check_sender_wallet_balance(tx, chain_id, wallet).await?)
                    );
                    trace!(%tx_hash_str, %tx_nonce, sender_wallet=%wallet, %sender_balance_eth, %chain_id, "Sender wallet balance is sufficient");
                }

                // 1. update the cache with nonce + 1. Quit if this fails
                let new_nonce = self.increment_wallet_nonce(chain_id, wallet, tx_nonce, tx_nonce+1).await.
                    map_err(|e| {
                        let rejection = NonceTooLow(tx_nonce+1 , tx_nonce);
                        error!(%e, %chain_id, %wallet, %expected_nonce, %tx_nonce, "failed to increment wallet nonce");
                        JsonRpcError(TransactionRejected(rejection))
                    }
                    )?;

                // 2. enqueue the txn
                self.enqueue_raw_transaction(&raw_tx, tx.hash(), chain_id).await?;
                self.metrics.increment_maestro_enqueued_transactions_total(1);

                // 3. check cache for waiting txns (background task)
                let service_clone = self.clone();
                let _handle = tokio::spawn(
                    async move {
                        service_clone
                            .check_for_and_enqueue_waiting_transactions(chain_id, wallet, new_nonce)
                            .await
                    }
                    .in_current_span(),
                );
            }
            Ordering::Less => {
                let rejection = NonceTooLow(expected_nonce, tx_nonce);
                warn!(tx_hash = format!("0x{}", hex::encode(tx.hash())), %chain_id, "Failed to submit forwarded transaction: {}", rejection);
                return Err(JsonRpcError(TransactionRejected(rejection)));
            }
            Ordering::Greater => {
                debug!(tx_hash = format!("0x{}", hex::encode(tx.hash())), %chain_id, "Caching waiting transaction");
                self.cache_waiting_transaction(chain_id, wallet, tx_nonce, raw_tx, tx.hash())
                    .await?;
                self.metrics.increment_maestro_waiting_transactions_total(1);
            }
        }
        Ok(())
    }

    /// Enqueues a raw transaction to the Valkey Stream for a specific chain
    ///
    /// This method uses Valkey stream producers to forward the transaction data.
    /// It creates a new producer if one doesn't exist for the specified chain.
    ///
    /// # Arguments
    /// * `raw_tx` - The raw transaction bytes to enqueue
    /// * `chain_id` - The chain identifier to send the transaction to
    ///
    /// # Returns
    /// * `Ok(())` - If the transaction was successfully enqueued
    /// * `Err(MaestroRpcError)` - If the Valkey operation fails
    ///
    /// # Errors
    /// Returns an error if:
    /// * Valkey connection fails
    /// * Stream write operation fails
    /// * Producer creation fails
    pub async fn enqueue_raw_transaction(
        &self,
        raw_tx: &Bytes,
        tx_hash: &B256,
        chain_id: ChainId,
    ) -> Result<(), MaestroRpcError> {
        let producer = self.producers.get(&chain_id).ok_or_else(|| {
            error!(%chain_id, %tx_hash, "Non existent stream producer for chain id");
            InternalError(RpcMissing(chain_id))
        })?;

        producer.enqueue_transaction(raw_tx).await.map_err(|e| {
            error!(%chain_id, %tx_hash, %e, "failed to enqueue transaction to Valkey Stream");
            InternalError(TransactionSubmissionFailed(tx_hash.to_string()))
        })?;
        Ok(())
    }

    /// Gets a wallet's nonce from cache or RPC provider
    ///
    /// This method attempts to retrieve a wallet's nonce from the cache.
    /// If the nonce is not found or cannot be parsed, it falls back to fetching
    /// it from the RPC provider and updates the cache.
    ///
    /// # Arguments
    /// * `signer` - The wallet address to get the nonce for
    /// * `chain_id` - The chain identifier to query
    ///
    /// # Returns
    /// * `Ok(u64)` - The current nonce for the wallet
    /// * `Err(MaestroRpcError)` - If retrieving the nonce fails
    ///
    /// # Errors
    /// Returns an error if:
    /// * Valkey connection fails
    /// * RPC provider is missing for the specified chain
    /// * RPC request to fetch the nonce fails
    #[instrument(
        skip(self, signer),
        err,
        fields(
            otel.kind = ?SpanKind::Client,
            wallet = %signer,
            chain_id = %chain_id,
        )
    )]
    pub async fn get_cached_or_rpc_nonce(
        &self,
        signer: Address,
        chain_id: ChainId,
    ) -> Result<u64, MaestroRpcError> {
        // TODO(SEQ-885): remove this once tracing makes the below logs redundant
        let chain_wallet_nonce_key = chain_wallet_nonce_key(chain_id, signer);
        let mut conn = self.valkey_conn.clone();

        let nonce = match with_cache_metrics!(
            &self.metrics.valkey,
            conn.get_wallet_nonce(chain_id, signer)
        ) {
            // Cache hit - we have a valid value
            Ok(Some(nonce_str)) => {
                // Parse the nonce string to u64
                match nonce_str.parse::<u64>() {
                    Ok(nonce) => nonce,
                    Err(e) => {
                        warn!(%nonce_str, %chain_wallet_nonce_key, %e, "Failed to parse nonce from Valkey cache, falling back to RPC");
                        self.get_nonce_from_rpc_and_update_cache(chain_id, signer).await?
                    }
                }
            }
            // Cache miss or error - need to get from RPC
            _ => {
                trace!(%signer, %chain_id, %chain_wallet_nonce_key, "No cached nonce found, fetching from RPC");
                self.get_nonce_from_rpc_and_update_cache(chain_id, signer).await?
            }
        };

        Ok(nonce)
    }

    /// Fetches a wallet's nonce from the RPC provider and updates the cache
    ///
    /// This method is called when there is a cache miss or parsing error
    /// when getting nonce from the cache. It queries the RPC provider
    /// for the current nonce and updates the Valkey cache with the result.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to query
    /// * `signer` - The wallet address to get the nonce for
    ///
    /// # Returns
    /// * `Ok(u64)` - The nonce retrieved from the RPC provider
    /// * `Err(MaestroRpcError)` - If retrieving the nonce fails
    ///
    /// # Errors
    /// Return an error if:
    /// * RPC provider is missing for the specified chain
    /// * RPC request to fetch the nonce fails
    async fn get_nonce_from_rpc_and_update_cache(
        &self,
        chain_id: ChainId,
        signer: Address,
    ) -> Result<u64, MaestroRpcError> {
        // TODO(SEQ-885): remove this once tracing makes the below log redundant
        let chain_wallet_nonce_key = chain_wallet_nonce_key(chain_id, signer);
        let provider = self.get_rpc_provider(&chain_id)?;

        // Get nonce
        let rpc_nonce = match provider.get_transaction_count(signer).await {
            Ok(nonce) => nonce,
            Err(e) => {
                error!(%signer, %chain_id, %e, "unable to get nonce from RPC");
                return Err(InternalError(RpcFailedToFetchWalletNonce(chain_id, signer)));
            }
        };

        let mut conn = self.valkey_conn.clone();
        let cache_result = with_cache_metrics!(
            &self.metrics.valkey,
            Operation::Write,
            CacheType::ValkeyCache,
            conn.set_wallet_nonce(chain_id, signer, rpc_nonce, self.config.wallet_nonce_ttl)
        );

        match cache_result {
            Ok(_) => {}
            Err(e) => {
                error!(%chain_wallet_nonce_key, %chain_id, %rpc_nonce, %e, "unable to cache nonce");
            }
        }

        Ok(rpc_nonce)
    }

    /// Updates a wallet's nonce in the cache
    ///
    /// This method updates the wallet's nonce in the cache to the
    /// specified value. Typically used after a transaction is submitted.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier for the wallet
    /// * `wallet_address` - The wallet address to update the nonce for
    /// * `current_nonce` - The current nonce value
    /// * `desired_nonce` - The new nonce value to set
    ///
    /// # Returns
    /// * `Ok(String)` - The result of the Valkey SET operation if successful
    /// * `Err(Error)` - If the Valkey operation fails
    #[instrument(skip(self), err)]
    pub async fn increment_wallet_nonce(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        current_nonce: u64,
        desired_nonce: u64,
    ) -> Result<u64, MaestroError> {
        let mut conn = self.valkey_conn.clone();

        let cached_nonce = with_cache_metrics!(
            &self.metrics.valkey,
            conn.get_wallet_nonce(chain_id, wallet_address)
        )?;

        if let Some(cached_nonce) = cached_nonce {
            let cached_nonce_parsed = cached_nonce.parse::<u64>().map_err(|_e| {
                error!(%desired_nonce, %cached_nonce, %chain_id, %wallet_address, "failed to parse nonce as u64 from cache");
                //TODO clear cache here?
                WaitingTransaction(FailedToEnqueue)
            })?;
            if cached_nonce_parsed.cmp(&current_nonce) != Ordering::Equal {
                error!(%desired_nonce, %cached_nonce_parsed, %current_nonce, %chain_id, %wallet_address, "unexpected cached nonce. likely a concurrency bug");
                //TODO clear cache here?
                return Err(WaitingTransaction(FailedToEnqueue));
            }
        }

        // actual increment
        let old_nonce = with_cache_metrics!(
            &self.metrics.valkey,
            conn.set_wallet_nonce(
                chain_id,
                wallet_address,
                desired_nonce,
                self.config.wallet_nonce_ttl
            )
        )
        .map_err(MaestroError::Valkey)?;

        match old_nonce {
            None => {
                // No value previously set in cache. Happy path.
                Ok(desired_nonce)
            }
            Some(old_nonce) => {
                let old_nonce_parsed = old_nonce.parse::<u64>().
                    map_err(|_e| {
                        error!(%desired_nonce, %old_nonce, %chain_id, %wallet_address, "failed to parse nonce as u64 from cache");
                        //TODO clear cache here?
                        WaitingTransaction(FailedToEnqueue)
                    })?;

                if desired_nonce.cmp(&old_nonce_parsed) != Ordering::Greater {
                    error!(%desired_nonce, %old_nonce_parsed, %current_nonce, %chain_id, %wallet_address, "new nonce not greater than cached. likely a concurrency bug");
                    //TODO clear cache here?
                }
                Ok(desired_nonce)
            }
        }
    }

    /// Checks for and processes waiting transaction in the cache
    ///
    /// This function checks the cache for waiting transactions with
    /// sequential nonces starting from `nonce_to_check`. If any are found, it enqueues
    /// each transaction, updates the wallet's nonce in the cache, and removes the transactions from
    /// the cache.
    ///
    /// # Arguments
    /// * `&self` - reference to the service
    /// * `chain_id` - The chain identifier
    /// * `wallet_address` - The wallet address to check for waiting transactions
    /// * `starting_nonce` - The nonce value to start checking from
    ///
    /// # Returns
    /// * `Result<(), Error>` - Result containing possible [`WaitingTransaction`] errors
    #[instrument(
        name = "enqueue_waiting",
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Producer,
        )
    )]
    pub async fn check_for_and_enqueue_waiting_transactions(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        starting_nonce: u64,
    ) -> Result<(), MaestroError> {
        // Tokio lock is acquired in order, so lock acquisition will occur "next" for a wallet after
        // parent txn function
        let chain_wallet_lock = self.get_chain_wallet_lock(chain_id, wallet_address).await;
        let _guard = chain_wallet_lock.lock().await;

        let waiting_txns = self
            .get_contiguous_waiting_transactions_from_cache(
                chain_id,
                wallet_address,
                starting_nonce,
            )
            .await?;

        let mut _current_waiting_txn_nonce = 0u64;

        match waiting_txns.first() {
            // Early return
            None => return Ok(()),
            Some(waiting_txn) => {
                let tx = decode_transaction(waiting_txn).map_err(|e| {
                    error!(%e, %chain_id, %wallet_address, "Failed to decode raw transaction from cache");
                    WaitingTransaction(FailedToDecode)
                })?;
                _current_waiting_txn_nonce = tx.nonce();
            }
        }

        let mut waiting_txn_ids: Vec<WaitingTransactionId> = Vec::new();

        // Decode txn, enqueue, and remove from cache
        for waiting_txn in waiting_txns {
            let tx = decode_transaction(&waiting_txn).map_err(|e| {
                error!(%e, %chain_id, %wallet_address, "Failed to decode raw transaction from cache");
                WaitingTransaction(FailedToDecode)
            })?;
            _current_waiting_txn_nonce = tx.nonce();
            if let Err(e) = self.enqueue_raw_transaction(&waiting_txn, tx.hash(), chain_id).await {
                let tx_hash = format!("0x{}", hex::encode(tx.hash()));
                error!(%e, %chain_id, %wallet_address, %tx_hash, "Failed to enqueue transaction");
                return Err(WaitingTransaction(FailedToEnqueue));
            }

            waiting_txn_ids.push(WaitingTransactionId {
                chain_id,
                wallet_address,
                nonce: _current_waiting_txn_nonce,
            })
        }

        self.remove_waiting_transactions_from_cache(&waiting_txn_ids).await.map_err(|e|{
            error!(%e, %chain_id, %wallet_address, "Failed to remove waiting transactions from cache");
            WaitingTransaction(FailedToEnqueue)
        })?;

        let new_nonce = self
            .increment_wallet_nonce(
                chain_id,
                wallet_address,
                starting_nonce,
                _current_waiting_txn_nonce + 1,
            )
            .await?;
        debug!(%chain_id, %wallet_address, prev_nonce=%starting_nonce, updated_nonce=%new_nonce, "background waiting txn process updated nonce");

        Ok(())
    }

    /// Checks if contiguous transactions exist in the cache for the given wallet address and nonce
    ///
    /// This method attempts to retrieve a transaction from the waiting gap cache
    /// for a specific wallet, chain, and nonce combination. By "contiguous", we mean transactions
    /// with adjacent nonces starting from `starting_nonce` onward.
    ///
    /// For example, if transactions with nonce 5, 6, 7, 9, 10, and 11 exist in the cache and this
    /// function is called with `starting_nonce` 5, then 3 transactions will be returned - those
    /// with nonces 5, 6, and 7.
    /// If the function were to be called with `starting_nonce` 10, then only 2 transactions would
    /// be returned - those with nonce 9 and 10.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to check
    /// * `wallet_address` - The wallet address to look up
    /// * `starting_nonce` - The specific nonce to check for
    ///
    /// # Returns
    /// * `Ok(Some(Bytes))` - The raw transaction bytes if found in cache and successfully decoded
    /// * `Ok(None)` - If no transaction is found for the given nonce
    /// * `Err(Error::Valkey)` - If there was an error accessing Valkey cache
    /// * `Err(Error::WaitingTransaction(FailedToDecode))` - If the transaction was found but hex
    ///   decoding failed
    ///
    /// # Errors
    /// Returns an error if:
    /// * Valkey connection fails or operation fails during transaction retrieval (wrapped as
    ///   [`MaestroError::Valkey`])
    /// * Hex decoding fails when converting the cached transaction string to bytes (wrapped as
    ///   [`Error::WaitingTransaction(FailedToDecode)`])
    ///
    /// This method will log detailed error information when hex decoding fails, including the
    /// actual hex string that failed to decode.
    pub async fn get_contiguous_waiting_transactions_from_cache(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        starting_nonce: u64,
    ) -> Result<Vec<Bytes>, MaestroError> {
        let mut conn = self.valkey_conn.clone();
        let mut txns: Vec<Bytes> = Vec::new();
        let mut current_nonce = starting_nonce;
        loop {
            match with_cache_metrics!(
                &self.metrics.valkey,
                conn.get_waiting_txn(chain_id, wallet_address, current_nonce)
            )
            .map_err(MaestroError::Valkey)?
            {
                None => {
                    //
                    break;
                }
                Some(tx_hex) => match hex::decode(&tx_hex) {
                    Ok(tx_bytes) => {
                        txns.push(Bytes::from(tx_bytes));
                        current_nonce += 1
                    }
                    Err(e) => {
                        error!(%e, %chain_id, %wallet_address, %starting_nonce, %tx_hex, "Failed to decode hex transaction from cache");
                        return Err(WaitingTransaction(FailedToDecode));
                    }
                },
            }
        }
        Ok(txns)
    }

    /// Stores a waiting transaction in the cache
    ///
    /// When a transaction is received with a nonce higher than expected,
    /// this method stores it in the cache until the preceding
    /// transactions arrive.
    ///
    /// Note: Transactions are cached solely based on which one was seen most recently, so a more
    /// recent transaction for the same chain, wallet, and nonce can overwrite an old one.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier
    /// * `wallet_address` - The wallet address that signed the transaction
    /// * `nonce` - The transaction nonce
    /// * `raw_tx` - The raw transaction bytes
    /// * `tx_hash` - The transaction hash for error reporting
    ///
    /// # Returns
    /// * `Ok(String)` - The Valkey operation result if successful
    /// * `Err(MaestroRpcError)` - If the Valkey operation fails
    ///
    /// # Errors
    /// Returns an error if the Valkey operation fails
    pub async fn cache_waiting_transaction(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce: u64,
        raw_tx: Bytes,
        tx_hash: &B256,
    ) -> Result<String, MaestroRpcError> {
        let mut conn = self.valkey_conn.clone();
        with_cache_metrics!(
            &self.metrics.valkey,
            conn.set_waiting_txn(
                chain_id,
                wallet_address,
                nonce,
                raw_tx,
                self.config.waiting_txn_ttl
            )
        )
        .map_err(|_| InternalError(TransactionSubmissionFailed(tx_hash.to_string())))
    }

    /// Removes an array of waiting transactions from the cache
    ///
    /// This method is called when waiting transactions are successfully processed
    /// and should be removed from the waiting cache.
    ///
    /// # Arguments
    /// * `waiting_txns` - array of [`WaitingTransactionId`], corresponding to desired transactions
    ///   to remove
    ///
    /// # Returns
    /// * `Ok(u64)` - The number of transactions removed if successful
    /// * `Err(MaestroRpcError)` - If the cache operation fails
    ///
    /// # Errors
    /// Returns an error if the cache operation fails
    #[instrument(
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Consumer,
        )
    )]
    pub async fn remove_waiting_transactions_from_cache(
        &self,
        waiting_txns: &[WaitingTransactionId],
    ) -> Result<u64, MaestroRpcError> {
        if waiting_txns.is_empty() {
            error!("No waiting txns to remove");
            return Err(InternalError(Other));
        }

        let mut conn = self.valkey_conn.clone();
        let txn_ids = waiting_txns;
        let result = with_cache_metrics!(&self.metrics.valkey, conn.del_waiting_txn_keys(txn_ids))
            .map_err(|_| InternalError(Other))?;

        let chain_id = waiting_txns[0].chain_id;
        let wallet_address = waiting_txns[0].wallet_address;

        // Log if an incorrect number of keys was removed - likely a bug
        if result != (txn_ids.len() as u64) {
            error!(
            %chain_id, %wallet_address, num_txns_requested = %txn_ids.len(), num_txns_requested = %result,
            "Removed different number of waiting transactions from cache than requested"
            );
        }

        Ok(result)
    }

    /// Checks if the Valkey connection is healthy
    ///
    /// This method attempts to ping the Valkey connection to check if it is healthy.
    ///
    /// # Returns
    /// * `Ok(true)` - If the connection is healthy
    /// * `Err(MaestroError::Valkey)` - If the connection is not healthy
    pub async fn health(&self) -> Result<bool, MaestroError> {
        let mut conn = self.valkey_conn.clone();
        let health: Result<String, _> = conn.ping().await;
        match health {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Valkey connection is not healthy: {:?}", e);
                Err(MaestroError::Valkey(e))
            }
        }
    }

    /// Shuts down the Maestro service and all its components gracefully.
    #[allow(clippy::cognitive_complexity)]
    pub async fn shutdown(&self) {
        info!("Shutting down MaestroService...");

        let mut shutdown_handles = Vec::new();
        for (chain_id, producer) in &self.producers {
            info!(%chain_id, "Initiating shutdown for StreamProducer...");
            let producer_clone = Arc::clone(producer);
            shutdown_handles.push(tokio::spawn(async move {
                producer_clone.shutdown().await;
            }));
        }

        for handle in shutdown_handles {
            if let Err(e) = handle.await {
                error!("Error during StreamProducer shutdown: {:?}", e);
            }
        }

        info!("All StreamProducers shut down.");
        info!("MaestroService shutdown complete.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, valkey::streams::producer::tx_stream_key};
    use alloy::{
        network::{EthereumWallet, TransactionBuilder},
        primitives::{keccak256, utils::parse_ether, Address, Bytes, U256},
        rlp::Encodable,
        rpc::types::TransactionRequest,
    };
    use prometheus_client::registry::Registry;
    use redis::AsyncCommands;
    use serde_json::json;
    use std::time::Duration;
    use test_utils::{
        docker::{start_valkey, E2EProcess},
        transaction::create_legacy_transaction,
        wait_until,
    };
    use tokio::time::sleep;
    use wiremock::{
        matchers::{body_partial_json, method},
        Mock, MockServer, ResponseTemplate,
    };

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    // Helper to create a test service with real Valkey and mock RPC
    async fn create_test_service() -> (MaestroService, MockServer, MockServer, E2EProcess) {
        // Start cache
        let (valkey_container, valkey_url) = start_valkey().await.unwrap();

        // Start mock RPC servers
        let mock_rpc_server_4 = MockServer::start().await;
        set_up_mock_chain_id(&mock_rpc_server_4, 4).await;
        let mock_rpc_server_5 = MockServer::start().await;
        set_up_mock_chain_id(&mock_rpc_server_5, 5).await;

        // Create chain RPC URLs map
        let mut chain_rpc_urls = HashMap::new();
        chain_rpc_urls.insert(4, mock_rpc_server_4.uri());
        chain_rpc_urls.insert(5, mock_rpc_server_5.uri());

        // Create cache connection
        let client = redis::Client::open(valkey_url.as_str()).unwrap();
        let valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Create test config
        let config = Config {
            port: 0,
            valkey_url,
            chain_rpc_urls,
            validation_timeout: Duration::from_secs(1),
            skip_validation: false,
            metrics_port: 8090,
            waiting_txn_ttl: Duration::from_secs(20), // shorter than default
            wallet_nonce_ttl: Duration::from_secs(3),
            finalization_duration: Duration::from_secs(10),
            finalization_checker_interval: Duration::from_secs(1),
            max_transaction_retries: 3,
            skip_balance_check: false,
        };

        let mut registry = Registry::default();
        let metrics = MaestroMetrics::new(&mut registry);

        // Create service
        let service = MaestroService::new(valkey_conn, config, metrics).await.unwrap();

        (service, mock_rpc_server_4, mock_rpc_server_5, valkey_container)
    }

    // Helper to set up default mock responses
    async fn set_up_mock_chain_id(mock_server: &MockServer, chain_id: u64) {
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_chainId"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": format!("0x{:x}", chain_id)
            })))
            .mount(mock_server)
            .await;
    }

    async fn set_up_mock_balance(mock_server: &MockServer, wallet: &str, balance: U256) {
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getBalance",
                "params": [wallet, "latest"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": format!("0x{:x}", balance)
            })))
            .mount(mock_server)
            .await;
    }

    async fn set_up_mock_transaction_count(mock_server: &MockServer, wallet: &str, nonce: u64) {
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount",
                "params": [wallet, "latest"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": format!("0x{:x}", nonce)
            })))
            .mount(mock_server)
            .await;
    }

    #[tokio::test]
    async fn test_get_producer() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test chain ID
        let chain_id = 4u64;

        // Get producer for first time
        let producer1 = service.producers.get(&chain_id).unwrap();

        // Verify stream key is correct
        assert_eq!(producer1.stream_key, format!("maestro:transactions:{chain_id}"));

        // Get producer again
        let producer2 = service.producers.get(&chain_id).unwrap();

        // Verify it's the same producer (by comparing Arc pointers)
        assert!(Arc::ptr_eq(producer1, producer2), "Should reuse existing producer");

        // Try with a different chain ID
        let different_chain_id = 5u64;
        let producer3 = service.producers.get(&different_chain_id).unwrap();

        // Verify it's a different producer
        assert!(
            !Arc::ptr_eq(producer1, producer3),
            "Should create a new producer for different chain"
        );

        // Verify correct stream key
        assert_eq!(producer3.stream_key, format!("maestro:transactions:{different_chain_id}"));
    }

    #[tokio::test]
    async fn test_enqueue_raw_transaction() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let raw_tx = Bytes::from(vec![1, 2, 3, 4, 5]);
        let tx_hash = keccak256(raw_tx.as_ref());

        // Enqueue transaction
        let result = service.enqueue_raw_transaction(&raw_tx, &tx_hash, chain_id).await;

        // Verify success
        assert!(result.is_ok(), "Enqueuing transaction should succeed");

        // Verify transaction was actually stored
        // Connect to Valkey directly to check
        let client = redis::Client::open(service.config.valkey_url.as_str()).unwrap();
        let mut conn = ConnectionManager::new(client).await.unwrap();

        // Check stream exists and has entries
        let stream_key = tx_stream_key(chain_id);
        let len: u64 = conn.xlen(&stream_key).await.unwrap();

        assert!(len > 0, "Stream should contain entries");

        // Read stream entries
        let entries: Vec<(String, HashMap<String, Vec<u8>>)> =
            conn.xrange(&stream_key, "-", "+").await.unwrap();

        // Verify entry data
        assert!(!entries.is_empty(), "Stream should have entries");
        let (_, data_map) = &entries[0];
        assert!(data_map.contains_key("data"), "Entry should contain transaction data");
        assert_eq!(data_map["data"], raw_tx.to_vec(), "Entry data should match raw transaction");
    }

    #[tokio::test]
    async fn test_get_cached_or_rpc_nonce_cache_hit() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let expected_nonce = 42u64;

        // Pre-populate Valkey with nonce
        let mut conn = service.valkey_conn.clone();
        conn.set_wallet_nonce(chain_id, wallet, expected_nonce, service.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Get nonce
        let nonce_result = service.get_cached_or_rpc_nonce(wallet, chain_id).await;

        // Verify result
        assert!(nonce_result.is_ok(), "Getting cached nonce should succeed");
        assert_eq!(
            nonce_result.unwrap(),
            expected_nonce,
            "Retrieved nonce should match the cached value"
        );
    }

    #[tokio::test]
    async fn test_get_cached_or_rpc_nonce_cache_miss() {
        // Create test service
        let (service, mock_server_4, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let expected_nonce = 42u64;

        // Set up mock RPC response
        set_up_mock_transaction_count(&mock_server_4, &wallet_hex, expected_nonce).await;

        // Get nonce (should go to RPC since cache is empty)
        let nonce_result = service.get_cached_or_rpc_nonce(wallet, chain_id).await;

        // Verify result
        assert!(nonce_result.is_ok(), "Getting nonce from RPC should succeed");
        assert_eq!(nonce_result.unwrap(), expected_nonce, "Retrieved nonce should match RPC value");

        // Verify nonce was cached
        let mut conn = service.valkey_conn.clone();
        let cached_nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(cached_nonce, Some(expected_nonce.to_string()), "Nonce should be cached");
    }

    #[tokio::test]
    async fn test_get_nonce_from_rpc_missing_provider() {
        // Create test service with mock servers
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data - use a chain ID that doesn't have a provider
        let chain_id = 9999u64; // Not in our config
        let wallet = Address::from_slice(&[0x42; 20]);

        // Try to get nonce
        let result = service.get_nonce_from_rpc_and_update_cache(chain_id, wallet).await;

        // Verify error
        assert!(result.is_err(), "Should fail when provider is missing");
        match result {
            Err(InternalError(RpcMissing(c))) => {
                assert_eq!(c, chain_id, "Error should reference missing chain ID");
            }
            _ => panic!("Expected RpcMissing error"),
        }
    }

    #[tokio::test]
    async fn test_get_nonce_from_rpc_provider_error() {
        // Create test service
        let (service, mock_server_4, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));

        // Set up mock RPC to return error
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount",
                "params": [wallet_hex, "latest"]
            })))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32000,
                    "message": "Test RPC error"
                }
            })))
            .mount(&mock_server_4)
            .await;

        // Try to get nonce
        let result = service.get_nonce_from_rpc_and_update_cache(chain_id, wallet).await;

        // Verify error
        assert!(result.is_err(), "Should fail when RPC returns error");
        match result {
            Err(InternalError(RpcFailedToFetchWalletNonce(c, w))) => {
                assert_eq!(c, chain_id, "Error should reference correct chain ID");
                assert_eq!(w, wallet, "Error should reference correct wallet");
            }
            _ => panic!("Expected RpcFailedToFetchWalletNonce error"),
        }
    }

    #[tokio::test]
    async fn test_increment_wallet_nonce() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let initial_nonce = 41u64;

        let incremented_nonce = 42u64;

        // Set initial nonce
        {
            let mut conn = service.valkey_conn.clone();
            conn.set_wallet_nonce(chain_id, wallet, initial_nonce, service.config.wallet_nonce_ttl)
                .await
                .unwrap();

            // Verify initial nonce
            let nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
            assert_eq!(nonce, Some(initial_nonce.to_string()), "Initial nonce should be set");
        }

        // Increment nonce
        let result = service
            .increment_wallet_nonce(chain_id, wallet, initial_nonce, initial_nonce + 1)
            .await;

        // Verify success (result should be "OK" from Valkey)
        assert!(result.is_ok(), "Incrementing nonce should succeed");

        // Verify nonce was updated
        let mut conn = service.valkey_conn.clone();
        let updated_nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            updated_nonce,
            Some(incremented_nonce.to_string()),
            "Nonce should be incremented"
        );
    }

    #[tokio::test]
    async fn test_nonce_ttl() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;

        // Set nonce
        {
            let mut conn = service.valkey_conn.clone();
            conn.set_wallet_nonce(chain_id, wallet, nonce, service.config.wallet_nonce_ttl)
                .await
                .unwrap();

            // Verify nonce is initially set
            let initial_nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
            assert_eq!(initial_nonce, Some(nonce.to_string()), "Nonce should be initially set");
        } // Let this connection drop

        // Wait for TTL to expire
        sleep(Duration::from_secs(3 + 1)).await;

        // Create a fresh connection after the sleep
        let client = redis::Client::open(service.config.valkey_url.as_str()).unwrap();
        let mut fresh_conn = ConnectionManager::new(client).await.unwrap();

        // Verify nonce has expired using the fresh connection
        let expired_nonce = fresh_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(expired_nonce, None, "Nonce should expire after TTL period");
    }

    #[tokio::test]
    async fn test_multi_chain_independence() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data for two different chains
        let chain_id1 = 4u64;
        let chain_id2 = 5u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce1 = 42u64;
        let nonce1_plus1 = 43u64;
        let nonce2 = 100u64;
        let nonce2_plus1 = 101u64;

        // Set nonces for both chains
        service
            .increment_wallet_nonce(chain_id1, wallet, nonce1, nonce1_plus1)
            .await
            .expect("Incrementing nonce should succeed");
        service
            .increment_wallet_nonce(chain_id2, wallet, nonce2, nonce2_plus1)
            .await
            .expect("Incrementing nonce should succeed");

        // Verify nonces are chain-specific
        let mut conn = service.valkey_conn.clone();
        let get1 = conn.get_wallet_nonce(chain_id1, wallet).await.unwrap();
        let get2 = conn.get_wallet_nonce(chain_id2, wallet).await.unwrap();

        assert_eq!(get1, Some(nonce1_plus1.to_string()), "Chain 1 nonce should be correct");
        assert_eq!(get2, Some(nonce2_plus1.to_string()), "Chain 2 nonce should be correct");
    }

    #[tokio::test]
    async fn test_invalid_cached_nonce() {
        // Create test service
        let (service, mock_server_4, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let invalid_nonce_str = "not_a_number";
        let rpc_nonce = 42u64;

        // Set up invalid nonce in cache
        {
            // Directly set invalid nonce string
            let key = chain_wallet_nonce_key(chain_id, wallet);
            let mut conn = service.valkey_conn.clone();
            let _: String = conn.set(&key, invalid_nonce_str).await.unwrap();
        }

        // Set up mock RPC response for fallback
        set_up_mock_transaction_count(&mock_server_4, &wallet_hex, rpc_nonce).await;

        // Get nonce (should fall back to RPC due to parsing error)
        let nonce_result = service.get_cached_or_rpc_nonce(wallet, chain_id).await;

        // Verify result
        assert!(nonce_result.is_ok(), "Should successfully fall back to RPC");
        assert_eq!(nonce_result.unwrap(), rpc_nonce, "Should return RPC nonce value");

        // Verify cache was updated with valid nonce
        let mut conn = service.valkey_conn.clone();
        let updated_nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            updated_nonce,
            Some(rpc_nonce.to_string()),
            "Cache should be updated with valid nonce"
        );
    }

    #[tokio::test]
    async fn test_check_cache_for_transaction() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let _tx_hash = "test_hash";

        // Set up test transaction in Valkey
        let mut conn = service.valkey_conn.clone();
        conn.set_waiting_txn(
            chain_id,
            wallet,
            nonce,
            raw_tx.clone(),
            service.config.waiting_txn_ttl,
        )
        .await
        .unwrap();

        // Check the transaction exists in cache
        let cached_tx = service
            .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, nonce)
            .await
            .unwrap();
        assert!(!cached_tx.is_empty(), "Transaction should be found in cache");
        assert_eq!(cached_tx[0], raw_tx, "Retrieved transaction should match the stored one");

        // Check a non-existent transaction returns None
        let non_existent = service
            .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, nonce + 1)
            .await
            .unwrap();
        assert!(non_existent.is_empty(), "Non-existent transaction should return None");
    }

    #[tokio::test]
    async fn test_put_gap_transaction_on_cache() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        // Put transaction in cache
        let result = service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                nonce,
                raw_tx.clone(),
                &keccak256(raw_tx.as_ref()),
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Putting transaction in cache should succeed");

        // Verify transaction was stored
        let mut conn = service.valkey_conn.clone();
        let cached_tx_hex = conn.get_waiting_txn(chain_id, wallet, nonce).await.unwrap();

        assert!(cached_tx_hex.is_some(), "Transaction should be stored in cache");
        let cached_bytes = hex::decode(cached_tx_hex.unwrap()).unwrap();
        assert_eq!(cached_bytes, raw_tx.to_vec(), "Stored transaction should match the original");
    }

    #[tokio::test]
    async fn test_remove_gap_transaction_from_cache() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet_address = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);

        // Set up test transaction in Valkey
        let mut conn = service.valkey_conn.clone();
        conn.set_waiting_txn(
            chain_id,
            wallet_address,
            nonce,
            raw_tx.clone(),
            service.config.waiting_txn_ttl,
        )
        .await
        .unwrap();

        // Verify transaction exists before removal
        assert!(
            !service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet_address, nonce)
                .await
                .unwrap()
                .is_empty(),
            "Transaction should exist before removal"
        );

        // Remove transaction
        let result = service
            .remove_waiting_transactions_from_cache(&[WaitingTransactionId {
                chain_id,
                wallet_address,
                nonce,
            }])
            .await;

        // Verify success
        assert!(result.is_ok(), "Removing transaction from cache should succeed");
        assert_eq!(result.unwrap(), 1, "Should have removed exactly 1 key");

        // Verify transaction is gone
        assert!(
            service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet_address, nonce)
                .await
                .unwrap()
                .is_empty(),
            "Transaction should be removed from cache"
        );

        // Test removing non-existent key
        let non_existent_result = service
            .remove_waiting_transactions_from_cache(&[WaitingTransactionId {
                chain_id,
                wallet_address,
                nonce: nonce + 1,
            }])
            .await;

        assert!(non_existent_result.is_ok(), "Removing non-existent key should not fail");
        assert_eq!(non_existent_result.unwrap(), 0, "Should return 0 for non-existent key");
    }

    #[tokio::test]
    async fn test_check_for_and_enqueue_waiting_transactions_empty() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Check for waiting transactions (shouldn't be any)
        let result = service
            .check_for_and_enqueue_waiting_transactions(chain_id, wallet, current_nonce)
            .await;

        // Verify success with no transactions
        assert!(result.is_ok(), "Function should succeed with no waiting transactions");
    }

    #[tokio::test]
    async fn test_check_for_and_enqueue_waiting_transactions_with_transactions() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Create a simple transaction that can be encoded/decoded
        let mut raw_tx0 = vec![];
        let mut raw_tx1 = vec![];
        let mut raw_tx2 = vec![];
        build_test_txn(&mut raw_tx0, current_nonce).await;
        build_test_txn(&mut raw_tx1, current_nonce + 1).await;
        build_test_txn(&mut raw_tx2, current_nonce + 2).await;

        // Add two sequential transactions to the waiting gap
        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 1,
                Bytes::from(raw_tx1.clone()),
                &keccak256(raw_tx1),
            )
            .await
            .unwrap();

        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2,
                Bytes::from(raw_tx2.clone()),
                &keccak256(raw_tx2),
            )
            .await
            .unwrap();

        // Set up Valkey stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        //Submit txn and increment nonce
        service
            .enqueue_raw_transaction(&Bytes::from(raw_tx0.clone()), &keccak256(raw_tx0), chain_id)
            .await
            .unwrap();
        service
            .increment_wallet_nonce(chain_id, wallet, current_nonce, current_nonce + 1)
            .await
            .unwrap();

        // Process waiting transactions
        let result = service
            .check_for_and_enqueue_waiting_transactions(chain_id, wallet, current_nonce + 1)
            .await;

        // Verify success
        assert!(result.is_ok(), "Processing waiting transactions should succeed");

        // Verify transactions were enqueued (stream length increased)
        let final_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        println!("{initial_len} {final_len}");

        // Should have processed three total transactions
        assert_eq!(final_len, initial_len + 3, "Should have enqueued 3 transactions");

        // Verify nonce was updated in cache
        let updated_nonce =
            service.valkey_conn.clone().get_wallet_nonce(chain_id, wallet).await.unwrap();

        assert_eq!(
            updated_nonce,
            Some((current_nonce + 3).to_string()),
            "Nonce should be incremented three times"
        );

        // Verify transactions are no longer in waiting gap
        assert!(
            service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce + 1)
                .await
                .unwrap()
                .is_empty(),
            "First transaction should be removed from waiting gap"
        );

        assert!(
            service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce + 2)
                .await
                .unwrap()
                .is_empty(),
            "Second transaction should be removed from waiting gap"
        );
    }

    async fn build_test_txn(mut raw_tx0: &mut Vec<u8>, nonce: u64) {
        let signer = alloy::signers::local::PrivateKeySigner::random();
        let signer_wallet = EthereumWallet::from(signer);
        TransactionRequest::default()
            .to(Address::default())
            .value(U256::from(1))
            .nonce(nonce)
            .gas_limit(0)
            .max_fee_per_gas(0)
            .max_priority_fee_per_gas(0)
            .build(&signer_wallet)
            .await
            .expect("Failed to build transaction")
            .encode(&mut raw_tx0);
    }

    #[tokio::test]
    async fn test_check_for_and_enqueue_waiting_transactions_with_non_sequential_nonces() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Create test transactions
        let mut raw_tx1 = vec![];
        let mut raw_tx3 = vec![];
        build_test_txn(&mut raw_tx1, current_nonce).await;
        build_test_txn(&mut raw_tx3, current_nonce + 2).await;

        // Add transactions with nonce gap
        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce,
                Bytes::from(raw_tx1.clone()),
                &keccak256(raw_tx1),
            )
            .await
            .unwrap();

        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2,
                Bytes::from(raw_tx3.clone()),
                &keccak256(raw_tx3),
            )
            .await
            .unwrap();

        // Set up Valkey stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Set cache nonce to be the first one
        valkey_conn
            .set_wallet_nonce(chain_id, wallet, current_nonce, service.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        // Process waiting transactions
        let result = service
            .check_for_and_enqueue_waiting_transactions(chain_id, wallet, current_nonce)
            .await;

        // Verify success
        assert!(result.is_ok(), "Processing waiting transactions should succeed");

        // Verify only the first transaction was enqueued
        let final_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        assert_eq!(final_len, initial_len + 1, "Should have enqueued 1 transaction");

        // Verify nonce was updated only once
        let updated_nonce =
            service.valkey_conn.clone().get_wallet_nonce(chain_id, wallet).await.unwrap();

        assert_eq!(
            updated_nonce,
            Some((current_nonce + 1).to_string()),
            "Nonce should be incremented only once"
        );

        // First transaction should be removed
        assert!(
            service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce)
                .await
                .unwrap()
                .is_empty(),
            "First transaction should be removed from waiting gap"
        );

        // Gap transaction should still be there
        assert!(
            !service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce + 2)
                .await
                .unwrap()
                .is_empty(),
            "Gap transaction should still be in waiting gap"
        );
    }

    #[tokio::test]
    async fn test_check_for_and_enqueue_waiting_transactions_invalid_transaction() {
        // Create test service
        let (service, _, _, _valkey_container_guard) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Create an invalid transaction (can't be properly decoded)
        let invalid_tx = Bytes::from(vec![0x01, 0x02, 0x03]);

        // Add invalid transaction to waiting gap
        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce,
                invalid_tx.clone(),
                &keccak256(invalid_tx),
            )
            .await
            .unwrap();

        // Process waiting transactions
        let result = service
            .check_for_and_enqueue_waiting_transactions(chain_id, wallet, current_nonce)
            .await;

        // Should fail due to invalid transaction
        assert!(result.is_err(), "Processing invalid transaction should fail");

        match result {
            Err(WaitingTransaction(FailedToDecode)) => {
                // Expected error
            }
            _ => panic!("Expected WaitingTransaction(FailedToDecode) error"),
        }

        // Invalid transaction should still be in cache
        assert!(
            !service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce)
                .await
                .unwrap()
                .is_empty(),
            "Invalid transaction should remain in waiting gap after error"
        );
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_equal_nonce() {
        // Create test service
        let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let nonce = 42u64;
        // Create a transaction that can be RLP decoded
        let mut raw_tx_vec = vec![];
        build_test_txn(&mut raw_tx_vec, nonce).await;
        let raw_tx = Bytes::from(raw_tx_vec);

        let balance = parse_ether("100").unwrap();
        set_up_mock_balance(&mock_server, &wallet_hex, balance).await;

        // Set up Valkey stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service_arc.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Set the initial nonce value
        valkey_conn
            .set_wallet_nonce(chain_id, wallet, nonce, service_arc.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        // Submit transaction with equal nonce
        let result = service_arc
            .handle_transaction(
                raw_tx.clone(),
                &decode_transaction(&raw_tx).unwrap(),
                wallet,
                chain_id,
                nonce,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with equal nonce should be accepted");

        wait_until!(
            valkey_conn.xlen::<&String, u64>(&stream_key).await.unwrap().eq(&(initial_len + 1)),
            Duration::from_millis(100)
        );

        // Verify transactions were enqueued (stream length increased)
        let final_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();
        assert_eq!(final_len, initial_len + 1, "Should have enqueued 1 transaction");

        // Verify nonce was incremented
        let updated_nonce = valkey_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            updated_nonce,
            Some((nonce + 1).to_string()),
            "Nonce should be incremented by 1"
        );
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_lower_nonce() {
        // Create test service
        let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let current_nonce = 42u64;
        let lower_nonce = 41u64;
        // Create a transaction that can be RLP decoded
        let mut raw_tx_vec = vec![];
        build_test_txn(&mut raw_tx_vec, lower_nonce).await; // Use lower_nonce for the tx
        let raw_tx = Bytes::from(raw_tx_vec);

        let balance = parse_ether("100").unwrap();
        set_up_mock_balance(&mock_server, &wallet_hex, balance).await;

        // Set up Valkey
        let client = redis::Client::open(service_arc.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Set the initial nonce value
        valkey_conn
            .set_wallet_nonce(chain_id, wallet, current_nonce, service_arc.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Submit transaction with lower nonce
        let result = service_arc
            .handle_transaction(
                raw_tx.clone(),
                &decode_transaction(&raw_tx).unwrap(),
                wallet,
                chain_id,
                lower_nonce,
            )
            .await;

        // Verify transaction was rejected due to lower nonce
        assert!(result.is_err(), "Transaction with lower nonce should be rejected");

        // Check the specific error type
        match result {
            Err(JsonRpcError(TransactionRejected(NonceTooLow(expected, actual)))) => {
                assert_eq!(expected, current_nonce, "Expected nonce should match");
                assert_eq!(actual, lower_nonce, "Actual nonce should match");
            }
            _ => panic!("Expected NonceTooLow rejection"),
        }

        // Verify nonce was NOT incremented
        let unchanged_nonce = valkey_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            unchanged_nonce,
            Some(current_nonce.to_string()),
            "Nonce should not change when transaction is rejected"
        );
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_higher_nonce() {
        // Create test service
        let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let current_nonce = 42u64;
        let higher_nonce = 44u64;
        // Create a transaction that can be RLP decoded
        let mut raw_tx_vec = vec![];
        build_test_txn(&mut raw_tx_vec, higher_nonce).await; // Use higher_nonce for the tx
        let raw_tx = Bytes::from(raw_tx_vec);

        let balance = parse_ether("100").unwrap();
        set_up_mock_balance(&mock_server, &wallet_hex, balance).await;

        // Set up Valkey
        let client = redis::Client::open(service_arc.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Set the initial nonce value
        valkey_conn
            .set_wallet_nonce(chain_id, wallet, current_nonce, service_arc.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Submit transaction with higher nonce
        let result = service_arc
            .handle_transaction(
                raw_tx.clone(),
                &decode_transaction(&raw_tx).unwrap(),
                wallet,
                chain_id,
                higher_nonce,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with higher nonce should be accepted and cached");

        // Verify nonce was NOT incremented
        let unchanged_nonce = valkey_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            unchanged_nonce,
            Some(current_nonce.to_string()),
            "Nonce should not change when transaction is gap-cached"
        );

        // Verify transaction was stored in waiting gap
        let cached_tx = service_arc
            .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, higher_nonce)
            .await
            .unwrap();

        assert!(!cached_tx.is_empty(), "Transaction should be stored in waiting gap");
        assert_eq!(cached_tx[0], raw_tx, "Cached transaction should match the original");
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_with_background_processing() {
        // Create test service
        let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let current_nonce = 41u64;

        // Create transactions for sequential nonces
        let raw_tx0 = create_legacy_transaction(chain_id, current_nonce);
        let raw_tx1 = create_legacy_transaction(chain_id, current_nonce + 1);
        let raw_tx2 = create_legacy_transaction(chain_id, current_nonce + 2);

        // Setup: Store future transactions in waiting gap
        service_arc
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 1,
                raw_tx1.clone(),
                &keccak256(raw_tx1),
            )
            .await
            .unwrap();

        service_arc
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2,
                raw_tx2.clone(),
                &keccak256(raw_tx2),
            )
            .await
            .unwrap();

        // Set up Valkey stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service_arc.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Set the initial nonce value
        valkey_conn
            .set_wallet_nonce(chain_id, wallet, current_nonce, service_arc.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        let balance = parse_ether("100").unwrap();
        set_up_mock_balance(&mock_server, &wallet_hex, balance).await;

        // Submit transaction with current nonce
        let result = service_arc
            .handle_transaction(
                raw_tx0.clone(),
                &decode_transaction(&raw_tx0).unwrap(),
                wallet,
                chain_id,
                current_nonce,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with current nonce should be accepted");

        // Wait for background processing to complete
        wait_until!(
            valkey_conn.xlen::<&String, u64>(&stream_key).await.unwrap().eq(&(initial_len + 3)),
            Duration::from_millis(100)
        );

        // Verify all three transactions were enqueued
        let final_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();
        assert_eq!(final_len, initial_len + 3, "Should have enqueued all 3 transactions");

        // Verify nonce was incremented for all transactions
        let updated_nonce = valkey_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            updated_nonce,
            Some((current_nonce + 3).to_string()),
            "Nonce should be incremented for all transactions"
        );

        // Verify all transactions are removed from waiting gap
        assert!(
            service_arc
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce)
                .await
                .unwrap()
                .is_empty(),
            "Transaction 0 should be removed from waiting gap"
        );

        assert!(
            service_arc
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce + 1)
                .await
                .unwrap()
                .is_empty(),
            "Transaction 1 should be removed from waiting gap"
        );

        assert!(
            service_arc
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce + 2)
                .await
                .unwrap()
                .is_empty(),
            "Transaction 2 should be removed from waiting gap"
        );
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_with_gaps() {
        // Create test service
        let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", hex::encode(wallet.as_slice()));
        let current_nonce = 41u64;

        // Create transactions for non-sequential nonces (gap at nonce 42)
        let mut raw_tx0 = vec![];
        let mut raw_tx2 = vec![];
        build_test_txn(&mut raw_tx0, current_nonce).await;
        build_test_txn(&mut raw_tx2, current_nonce + 2).await;

        // Setup: Store future transaction with a gap
        service_arc
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2, // Skip nonce 42
                raw_tx2.clone().into(),
                &keccak256(raw_tx2),
            )
            .await
            .unwrap();

        // Set up Valkey stream
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service_arc.config.valkey_url.as_str()).unwrap();
        let mut valkey_conn = ConnectionManager::new(client).await.unwrap();

        // Set the initial nonce value
        valkey_conn
            .set_wallet_nonce(chain_id, wallet, current_nonce, service_arc.config.wallet_nonce_ttl)
            .await
            .unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();

        // Submit transaction with current nonce
        let raw_tx0_bytes = Bytes::from(raw_tx0);
        let decoded_tx0_for_test = decode_transaction(&raw_tx0_bytes).unwrap();

        let balance = parse_ether("100").unwrap();
        set_up_mock_balance(&mock_server, &wallet_hex, balance).await;

        let result = service_arc
            .handle_transaction(
                raw_tx0_bytes,
                &decoded_tx0_for_test,
                wallet,
                chain_id,
                current_nonce,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with current nonce should be accepted");

        // Wait for background processing to complete
        wait_until!(
            valkey_conn.xlen::<&String, u64>(&stream_key).await.unwrap().eq(&(initial_len + 1)),
            Duration::from_millis(100)
        );

        // Verify only one transaction was enqueued (the gap prevents processing tx2)
        let final_len: u64 = valkey_conn.xlen(&stream_key).await.unwrap();
        assert_eq!(final_len, initial_len + 1, "Should have enqueued only 1 transaction");

        // Verify nonce was incremented only for the first transaction
        let updated_nonce = valkey_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            updated_nonce,
            Some((current_nonce + 1).to_string()),
            "Nonce should be incremented only for the first transaction"
        );

        // Verify the gapped transaction is still in the waiting gap
        assert!(
            !service_arc
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, current_nonce + 2)
                .await
                .unwrap()
                .is_empty(),
            "Transaction with gap should still be in waiting gap"
        );
    }

    mod handle_finalization_tests {
        use super::*; // Brings in outer test helpers and main module items
        use crate::valkey::streams::producer::CheckFinalizationResult;
        use alloy::primitives::{keccak256, B256};
        use serde_json::json;
        use test_utils::transaction::create_legacy_transaction;
        use wiremock::{matchers::body_partial_json, Mock, ResponseTemplate}; // method is already in scope via super::* if needed

        const TEST_CHAIN_ID_U64: u64 = 4;
        // Address corresponding to the private key used in `create_legacy_transaction`
        const KNOWN_SIGNER_ADDRESS_STR: &str = "0x90f79bf6eb2c4f870365e785982e1f101e93b906";

        async fn setup_mock_receipt_response(
            server: &MockServer,
            tx_hash: B256,
            receipt_json: Option<serde_json::Value>, // None means result is null
            is_rpc_error: bool,
        ) {
            let mock_builder = Mock::given(method("POST")).and(body_partial_json(json!({
                "method": "eth_getTransactionReceipt",
                "params": [format!("{:#x}", tx_hash)]
            })));

            if is_rpc_error {
                mock_builder
                    .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "error": {"code": -32000, "message": "Receipt RPC error"},
                    })))
                    .mount(server)
                    .await;
            } else {
                match receipt_json {
                    Some(data) => {
                        mock_builder
                            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                                "jsonrpc": "2.0",
                                "id": 1,
                                "result": data
                            })))
                            .mount(server)
                            .await;
                    }
                    None => {
                        // Ok(None) from provider.get_transaction_receipt
                        mock_builder
                            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                                "jsonrpc": "2.0",
                                "id": 1,
                                "result": null
                            })))
                            .mount(server)
                            .await;
                    }
                }
            }
        }

        // Using existing set_up_mock_transaction_count for success cases
        // Adding a specific helper for error cases for getTransactionCount
        async fn setup_mock_nonce_error_response(server: &MockServer, wallet_address_str: &str) {
            Mock::given(method("POST"))
                .and(body_partial_json(json!({
                    "method": "eth_getTransactionCount",
                    "params": [wallet_address_str, "latest"]
                })))
                .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {"code": -32000, "message": "Nonce RPC error"}
                })))
                .mount(server)
                .await;
        }

        #[tokio::test]
        async fn test_receipt_found() {
            let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
            let rpc_provider = service.rpc_providers.get(&TEST_CHAIN_ID_U64).unwrap();
            let tx_nonce = 10u64;
            let raw_tx_bytes = create_legacy_transaction(TEST_CHAIN_ID_U64, tx_nonce);
            let raw_tx_vec = raw_tx_bytes.to_vec();
            let tx_hash = keccak256(&raw_tx_vec);

            let mock_receipt_data =
                json!({"status": "0x1", "transactionHash": format!("{:#x}", tx_hash)});
            setup_mock_receipt_response(&mock_server, tx_hash, Some(mock_receipt_data), false)
                .await;

            let result =
                MaestroService::handle_finalization(raw_tx_vec, rpc_provider, &service.metrics)
                    .await;
            assert_eq!(result, CheckFinalizationResult::Done);
        }

        #[tokio::test]
        async fn test_receipt_not_found_nonce_matches_resubmit() {
            let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
            let rpc_provider = service.rpc_providers.get(&TEST_CHAIN_ID_U64).unwrap();
            let tx_nonce = 11u64;
            let raw_tx_bytes = create_legacy_transaction(TEST_CHAIN_ID_U64, tx_nonce);
            let raw_tx_vec = raw_tx_bytes.to_vec();
            let tx_hash = keccak256(&raw_tx_vec);

            // Decode the transaction to get the actual signer address.
            // This is necessary because create_legacy_transaction might use a random signer,
            // making KNOWN_SIGNER_ADDRESS_STR unreliable for this specific mock.
            let tx_for_signer_derivation = decode_transaction(&raw_tx_bytes)
                .expect("Test setup: Failed to decode transaction for signer derivation");
            let actual_signer = check_signature(&tx_for_signer_derivation)
                .expect("Test setup: Failed to derive signer from transaction");
            // Format as 0x-prefixed lowercase hex, which is standard for RPC calls.
            let actual_signer_hex = format!("{actual_signer:#x}");

            // Receipt is null
            // Use the dynamically derived signer address for the mock setup.
            setup_mock_receipt_response(&mock_server, tx_hash, None, false).await;
            set_up_mock_transaction_count(&mock_server, &actual_signer_hex, tx_nonce).await;

            let result =
                MaestroService::handle_finalization(raw_tx_vec, rpc_provider, &service.metrics)
                    .await;
            assert_eq!(result, CheckFinalizationResult::ReSubmit);
        }

        #[tokio::test]
        async fn test_receipt_not_found_nonce_differs_done() {
            let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
            let rpc_provider = service.rpc_providers.get(&TEST_CHAIN_ID_U64).unwrap();
            let tx_nonce = 12u64;
            let rpc_nonce = 13u64; // Different from tx_nonce
            let raw_tx_bytes = create_legacy_transaction(TEST_CHAIN_ID_U64, tx_nonce);
            let raw_tx_vec = raw_tx_bytes.to_vec();
            let tx_hash = keccak256(&raw_tx_vec);

            setup_mock_receipt_response(&mock_server, tx_hash, None, false).await; // Receipt is null
            set_up_mock_transaction_count(&mock_server, KNOWN_SIGNER_ADDRESS_STR, rpc_nonce).await; // Nonce differs

            let result =
                MaestroService::handle_finalization(raw_tx_vec, rpc_provider, &service.metrics)
                    .await;
            assert_eq!(result, CheckFinalizationResult::Done);
        }

        #[tokio::test]
        async fn test_receipt_not_found_get_nonce_fails_done() {
            let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
            let rpc_provider = service.rpc_providers.get(&TEST_CHAIN_ID_U64).unwrap();
            let tx_nonce = 14u64;
            let raw_tx_bytes = create_legacy_transaction(TEST_CHAIN_ID_U64, tx_nonce);
            let raw_tx_vec = raw_tx_bytes.to_vec();
            let tx_hash = keccak256(&raw_tx_vec);

            setup_mock_receipt_response(&mock_server, tx_hash, None, false).await; // Receipt is null
            setup_mock_nonce_error_response(&mock_server, KNOWN_SIGNER_ADDRESS_STR).await; // Nonce call fails

            let result =
                MaestroService::handle_finalization(raw_tx_vec, rpc_provider, &service.metrics)
                    .await;
            assert_eq!(result, CheckFinalizationResult::Done);
        }

        #[tokio::test]
        async fn test_get_receipt_fails_done() {
            let (service, mock_server, _, _valkey_container_guard) = create_test_service().await;
            let rpc_provider = service.rpc_providers.get(&TEST_CHAIN_ID_U64).unwrap();
            let tx_nonce = 15u64;
            let raw_tx_bytes = create_legacy_transaction(TEST_CHAIN_ID_U64, tx_nonce);
            let raw_tx_vec = raw_tx_bytes.to_vec();
            let tx_hash = keccak256(&raw_tx_vec);

            setup_mock_receipt_response(&mock_server, tx_hash, None, true).await; // Receipt call fails

            let result =
                MaestroService::handle_finalization(raw_tx_vec, rpc_provider, &service.metrics)
                    .await;
            assert_eq!(result, CheckFinalizationResult::Done);
        }
    }

    // Helper to create a dummy transaction for testing
    async fn create_dummy_tx(gas_limit: u64, max_fee_per_gas: u128, value: U256) -> TxEnvelope {
        let signer = alloy::signers::local::PrivateKeySigner::random();
        let signer_wallet = EthereumWallet::from(signer);
        TransactionRequest::default()
            .with_gas_limit(gas_limit)
            .with_max_fee_per_gas(max_fee_per_gas)
            .with_max_priority_fee_per_gas(0)
            .with_value(value)
            .with_to(Address::ZERO)
            .with_nonce(0)
            .with_chain_id(123)
            .build(&signer_wallet)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_balance_check_sufficient_funds() {
        let tx = create_dummy_tx(21000u64, 10_000_000_000u128, parse_ether("1").unwrap()).await;
        let account_balance = parse_ether("2").unwrap();
        let result = MaestroService::balance_check(&tx, account_balance);
        let balance_check_result = result.unwrap();
        assert_eq!(balance_check_result, account_balance);
    }

    #[tokio::test]
    async fn test_balance_check_exact_funds() {
        let gas_limit_u64 = 21000u64;
        let max_fee_per_gas_u128 = 10_000_000_000u128; // 10 gwei
        let value = parse_ether("1").unwrap();
        let tx = create_dummy_tx(gas_limit_u64, max_fee_per_gas_u128, value).await;

        let max_gas_cost = U256::from(gas_limit_u64) * U256::from(max_fee_per_gas_u128);
        let account_balance = max_gas_cost + value;

        let result = MaestroService::balance_check(&tx, account_balance);
        assert_eq!(result.unwrap(), account_balance);
    }

    #[tokio::test]
    async fn test_balance_check_insufficient_funds() {
        let tx = create_dummy_tx(21000u64, 10_000_000_000u128, parse_ether("1").unwrap()).await;
        let account_balance = parse_ether("0.5").unwrap();
        let result = MaestroService::balance_check(&tx, account_balance);
        assert!(result.is_err());
        match result.unwrap_err() {
            JsonRpcError(TransactionRejected(shared::json_rpc::Rejection::InsufficientFunds)) => {}
            _ => panic!("Expected InsufficientFunds error"),
        }
    }

    #[tokio::test]
    async fn test_balance_check_insufficient_funds_for_max_gas_cost() {
        // gas_limit * max_fee_per_gas (u64::MAX * u128::MAX) does NOT overflow U256,
        // but it results in a very large gas cost.
        let tx = create_dummy_tx(u64::MAX, u128::MAX, U256::from(1u64)).await;
        let account_balance = U256::ZERO; // Set balance to 0 to make it insufficient
        let result = MaestroService::balance_check(&tx, account_balance);
        assert!(result.is_err());
        match result.unwrap_err() {
            JsonRpcError(TransactionRejected(shared::json_rpc::Rejection::InsufficientFunds)) => {}
            _ => {
                panic!("Expected InsufficientFunds error due to maximal gas cost and zero balance")
            }
        }
    }

    #[tokio::test]
    async fn test_balance_check_total_required_overflow() {
        // (gas_limit * max_fee_per_gas) is small, but adding value overflows U256
        let tx = create_dummy_tx(1u64, 1u128, U256::MAX).await;
        let account_balance = U256::MAX;
        let result = MaestroService::balance_check(&tx, account_balance);
        assert!(result.is_err());
        match result.unwrap_err() {
            InternalError(Other) => {} // Expecting Other due to checked_add failing
            _ => panic!("Expected InternalError(Other) for total_required overflow"),
        }
    }
}
