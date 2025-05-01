//! This module is the `Maestro` service, which receives transaction requests and
//! validates them before submission into the transaction processing pipeline.

use crate::{
    config::{Config, RpcProvider},
    errors::{
        Error,
        Error::WaitingTransaction,
        InternalError::{RpcFailedToFetchWalletNonce, RpcMissing, TransactionSubmissionFailed},
        MaestroRpcError,
        MaestroRpcError::{Internal, JsonRpcError},
        WaitingTransactionError::{FailedToDecode, FailedToEnqueue},
    },
    redis::{
        keys::{wallet_nonce::chain_wallet_nonce_key, ChainWalletNonceKey},
        models::{waiting_transaction::WaitingGapTxnExt, wallet_nonce::WalletNonceExt},
        streams::producer::StreamProducer,
    },
};
use alloy::{
    consensus::Transaction,
    hex,
    primitives::{Address, Bytes, ChainId},
    providers::Provider,
};
use redis::aio::MultiplexedConnection;
use shared::{
    json_rpc::{Rejection::NonceTooLow, RpcError::TransactionRejected},
    tx_validation::decode_transaction,
};
use std::{cmp::Ordering, collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, trace, warn};

/// The service for filtering and directing transactions
#[derive(Debug)]
pub struct MaestroService {
    redis_conn: MultiplexedConnection,
    chain_wallets: Mutex<HashMap<ChainWalletNonceKey, Arc<Mutex<()>>>>,
    producers: HashMap<ChainId, Arc<StreamProducer>>,
    rpc_providers: HashMap<ChainId, RpcProvider>,
    _config: Config,
}

// TODO check stale locks
// pub async fn clean_stale_locks(service: Arc<MaestroService>) {
//     const STALE_LO*CK_THRESHOLD: Duration = Duration::from_secs(1);
//
//     loop {
//         tokio::time::sleep(Duration::from_secs(1)).await;
//
//         let now = std::time::Instant::now();
//         let mut stale_count = 0;
//
//         service.chain_wallets.retain(|_, (_, last_updated)| {
//             let retain = now.duration_since(*last_updated) < STALE_LOCK_THRESHOLD;
//             if !retain {
//                 stale_count += 1;
//             }
//             retain
//         });
//
//         if stale_count > 0 {
//             warn!("Cleaned {} stale locks", stale_count);
//         }
//     }
// }

impl MaestroService {
    /// Create a new instance of the Maestro service
    pub async fn new(redis_conn: MultiplexedConnection, config: Config) -> Result<Self, Error> {
        let rpc_providers = config.validate().await?;
        if rpc_providers.is_empty() {
            warn!("No RPC providers configured. This is probably undesirable");
        }
        let producers = Self::create_redis_producers(&redis_conn, &config, &rpc_providers);

        Ok(Self {
            redis_conn,
            chain_wallets: Mutex::new(HashMap::new()),
            producers,
            rpc_providers,
            _config: config,
        })
    }

    fn create_redis_producers(
        redis_conn: &MultiplexedConnection,
        config: &Config,
        rpc_providers: &HashMap<ChainId, RpcProvider>,
    ) -> HashMap<ChainId, Arc<StreamProducer>> {
        let mut producers = HashMap::new();
        for chain_id in rpc_providers.keys() {
            producers.insert(
                *chain_id,
                Arc::new(StreamProducer::new(
                    redis_conn.clone(),
                    *chain_id,
                    config.prune_interval,
                    config.prune_max_age,
                )),
            );
        }
        producers
    }
    // TODO revisit timings
    // pub async fn clean_stale_locks(self: Arc<Self>) {
    //     const STALE_LOCK_THRESHOLD: Duration = Duration::from_secs(1);
    //
    //     loop {
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //
    //         let now = std::time::Instant::now();
    //         let mut stale_count = 0;
    //
    //         self.chain_wallets.retain(|_, (_, last_updated)| {
    //             let retain = now.duration_since(*last_updated) < STALE_LOCK_THRESHOLD;
    //             if !retain {
    //                 stale_count += 1;
    //             }
    //             retain
    //         });
    //
    //         if stale_count > 0 {
    //             warn!("Cleaned {} stale locks", stale_count);
    //         }
    //     }
    // }

    async fn get_chain_wallet_lock(&self, chain_id: ChainId, wallet: Address) -> Arc<Mutex<()>> {
        let key = chain_wallet_nonce_key(chain_id, wallet);
        let mut locks = self.chain_wallets.lock().await;
        locks.entry(key).or_insert_with(|| Arc::new(Mutex::new(()))).clone()
    }

    // TODO update docs
    /// Processes a transaction based on its nonce compared to the wallet's expected nonce
    ///
    /// This method handles the transaction dispatch logic based on nonce comparison:
    /// - If the nonce matches the expected value, the transaction is enqueued immediately and the
    ///   system checks the cache for waiting transactions with subsequent nonces
    /// - If the nonce is lower than expected, the transaction is rejected as it would be a
    ///   resubmission
    /// - If the nonce is higher than expected, this "waiting" transaction is stored in a cache
    ///   until all preceding transactions with missing nonces arrive
    ///
    /// # Arguments
    /// * `self` - Arc reference to the service to ensure it outlives spawned tasks
    /// * `raw_tx` - The raw transaction bytes to process
    /// * `signer` - The wallet address that signed the transaction
    /// * `chain_id` - The chain identifier the transaction is intended for
    /// * `tx_nonce` - The nonce value included in the transaction
    /// * `tx_hash` - The transaction hash for logging and error reporting
    /// * `expected_nonce` - The expected nonce value from the system's perspective
    ///
    /// # Returns
    /// * `Ok(())` - If the transaction was processed successfully (enqueued)
    /// * `Err(MaestroRpcError)` - If the transaction was rejected or processing failed
    ///
    /// # Errors
    /// Returns an error if:
    /// * Transaction nonce is lower than expected (rejected as [`NonceTooLow`])
    /// * Redis operations fail during transaction enqueuing or caching
    /// * Stream write operations fail
    ///
    /// # Notes
    /// When a transaction with the expected nonce is processed, a background task is
    /// spawned to check for and process any waiting transactions with sequential nonces.
    /// This allows the method to return quickly while potentially processing a chain of
    /// transactions in the background.
    pub async fn handle_transaction_and_manage_nonces(
        self: &Arc<Self>,
        raw_tx: Bytes,
        wallet: Address,
        chain_id: ChainId,
        tx_nonce: u64,
        tx_hash: &String,
    ) -> Result<(), MaestroRpcError> {
        // Handle nonce comparison
        // Get wallet-specific lock
        let chain_wallet_lock = self.get_chain_wallet_lock(chain_id, wallet).await;
        let _guard = chain_wallet_lock.lock().await;

        // getting this after acquiring lock should eliminate concurrency?
        let expected_nonce = self.get_cached_or_rpc_nonce(wallet, chain_id).await?;

        match tx_nonce.cmp(&expected_nonce) {
            Ordering::Equal => {
                // 1. enqueue the txn
                self.enqueue_raw_transaction(raw_tx, chain_id, tx_hash).await?;

                // 2. update the cache with nonce + 1. Quit if this fails
                let new_nonce = self.increment_wallet_nonce(chain_id, wallet, tx_nonce, tx_nonce+1).await.
                    map_err(|e| {
                        let rejection = NonceTooLow(tx_nonce+1 , tx_nonce);
                        trace!(%e, %chain_id, %wallet, %expected_nonce, %tx_nonce, "failed to increment wallet nonce");
                        JsonRpcError(TransactionRejected(rejection))
                    }
                    )?;

                // 3. check cache for waiting txns (background task)
                let service_clone = self.clone();
                let _handle = tokio::spawn(async move {
                    service_clone
                        .check_for_and_enqueue_waiting_transactions(chain_id, wallet, new_nonce)
                        .await
                });
            }
            Ordering::Less => {
                let rejection = NonceTooLow(expected_nonce, tx_nonce);
                warn!(%tx_hash, %chain_id, "Failed to submit forwarded transaction: {}", rejection);
                return Err(JsonRpcError(TransactionRejected(rejection)))
            }
            Ordering::Greater => {
                self.cache_waiting_transaction(chain_id, wallet, tx_nonce, raw_tx, tx_hash).await?;
            }
        }
        Ok(())
    }

    /// Enqueues a raw transaction to the Redis stream for a specific chain
    ///
    /// This method use Redis stream producers to forward the transaction data.
    /// It creates a new producer if one doesn't exist for the specified chain.
    ///
    /// # Arguments
    /// * `raw_tx` - The raw transaction bytes to enqueue
    /// * `chain_id` - The chain identifier to send the transaction to
    /// * `chain_id` - The hash of the raw transaction, for logging
    ///
    /// # Returns
    /// * `Ok(())` - If the transaction was successfully enqueued
    /// * `Err(ErrorObjectOwned)` - If the Redis operation fails
    ///
    /// # Errors
    /// Returns an error if:
    /// * Redis connection fails
    /// * Stream write operation fails
    /// * Producer creation fails
    pub async fn enqueue_raw_transaction(
        &self,
        raw_tx: Bytes,
        chain_id: ChainId,
        tx_hash: &str,
    ) -> Result<(), MaestroRpcError> {
        // Get or create producer while holding lock
        let producer = self.producers.get(&chain_id).ok_or({
            error!(%chain_id, %tx_hash, "Non existent stream producer for chain id");
            Internal(RpcMissing(chain_id))
        })?;

        // Release lock before making async call
        producer.enqueue_transaction(raw_tx.into()).await.map_err(|e| {
            error!(%chain_id, %tx_hash, %e, "failed to enqueue transaction to Redis Stream");
            Internal(TransactionSubmissionFailed(tx_hash.to_string()))
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
    /// * Redis connection fails
    /// * RPC provider is missing for the specified chain
    /// * RPC request to fetch the nonce fails
    pub async fn get_cached_or_rpc_nonce(
        &self,
        signer: Address,
        chain_id: ChainId,
    ) -> Result<u64, MaestroRpcError> {
        // TODO(SEQ-885): remove this once tracing makes the below logs redundant
        let chain_wallet_nonce_key = chain_wallet_nonce_key(chain_id, signer);
        let mut conn = self.redis_conn.clone();

        let nonce = match conn.get_wallet_nonce(chain_id, signer).await {
            // Cache hit - we have a valid value
            Ok(Some(nonce_str)) => {
                // Parse the nonce string to u64
                match nonce_str.parse::<u64>() {
                    Ok(nonce) => nonce,
                    Err(e) => {
                        warn!(%nonce_str, %chain_wallet_nonce_key, %e, "Failed to parse nonce from Redis cache, falling back to RPC");
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

    /// Fetches a wallet's nonce from RPC provider and updates the cache
    ///
    /// This method is called when there is a cache miss or parsing error
    /// when getting a nonce from the cache. It queries the RPC provider
    /// for the current nonce and updates the Redis cache with the result.
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
    /// Returns an error if:
    /// * RPC provider is missing for the specified chain
    /// * RPC request to fetch the nonce fails
    async fn get_nonce_from_rpc_and_update_cache(
        &self,
        chain_id: ChainId,
        signer: Address,
    ) -> Result<u64, MaestroRpcError> {
        // TODO(SEQ-885): remove this once tracing makes the below log redundant
        let chain_wallet_nonce_key = chain_wallet_nonce_key(chain_id, signer);
        let provider = self.rpc_providers.get(&chain_id).ok_or_else(|| {
            error!("No RPC provider for chain {}", chain_id);
            Internal(RpcMissing(chain_id))
        })?;

        // Get nonce
        let rpc_nonce = match provider.get_transaction_count(signer).await {
            Ok(nonce) => nonce,
            Err(e) => {
                error!(%signer, %chain_id, %e, "unable to get nonce from RPC");
                return Err(Internal(RpcFailedToFetchWalletNonce(chain_id, signer)))
            }
        };

        // Update cache
        let mut conn = self.redis_conn.clone();
        match conn.set_wallet_nonce(chain_id, signer, rpc_nonce).await {
            Ok(_) => {}
            Err(e) => {
                error!(%chain_wallet_nonce_key, %chain_id, %rpc_nonce, %e, "unable to cache nonce");
            }
        }

        Ok(rpc_nonce)
    }

    // TODO update docs
    /// Increments a wallet's nonce in the Redis cache
    ///
    /// This method updates the wallet's nonce in the Redis cache to the
    /// specified value + 1. Typically used after a transaction is submitted.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier for the wallet
    /// * `wallet_address` - The wallet address to update the nonce for
    /// * `current_nonce` - The current nonce value
    ///
    /// # Returns
    /// * `Ok(String)` - The result of the Redis SET operation if successful
    /// * `Err(Error)` - If the Redis operation fails
    pub async fn increment_wallet_nonce(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        current_nonce: u64,
        desired_nonce: u64,
    ) -> Result<u64, Error> {
        let mut conn = self.redis_conn.clone();

        let cached_nonce = conn.get_wallet_nonce(chain_id, wallet_address).await?;
        if let Some(cached_nonce) = cached_nonce {
            let cached_nonce_parsed = cached_nonce.parse::<u64>().map_err(|_e| {
                error!(%desired_nonce, %cached_nonce, %chain_id, %wallet_address, "failed to parse nonce as u64 from Redis cache");
                //TODO clear cache here?
                WaitingTransaction(FailedToEnqueue)
            })?;
            if cached_nonce_parsed.cmp(&current_nonce) != Ordering::Equal {
                error!(%desired_nonce, %cached_nonce_parsed, %current_nonce, %chain_id, %wallet_address, "unexpected cached nonce. likely a concurrency bug");
                //TODO clear cache here?
                return Err(WaitingTransaction(FailedToEnqueue))
            }
        }

        // actual increment
        let old_nonce = conn
            .set_wallet_nonce(chain_id, wallet_address, desired_nonce)
            .await
            .map_err(Error::Redis)?;

        match old_nonce {
            None => {
                // No value previously set in cache. Happy path.
                println!("X_X_X_X: no nonce case, {}", desired_nonce); // TODO delete me
                Ok(desired_nonce)
            }
            Some(old_nonce) => {
                let old_nonce_parsed = old_nonce.parse::<u64>().
                    map_err(|_e| {
                        error!(%desired_nonce, %old_nonce, %chain_id, %wallet_address, "failed to parse nonce as u64 from Redis cache");
                        //TODO clear cache here?
                        WaitingTransaction(FailedToEnqueue)
                    })?;

                if desired_nonce.cmp(&old_nonce_parsed) != Ordering::Greater {
                    error!(%desired_nonce, %old_nonce_parsed, %current_nonce, %chain_id, %wallet_address, "new nonce not greater than cached. likely a concurrency bug");
                    //TODO clear cache here?
                    // return Err(WaitingTransaction(FailedToEnqueue)) // TODO delete this? Maybe
                    // let execution proceed
                }
                println!(
                    "X_X_X_X: expected increment case, desired {} cached {}",
                    desired_nonce, old_nonce_parsed
                ); // TODO delete me
                Ok(desired_nonce)
            }
        }
    }

    // TODO docs
    /// Checks for and processes waiting transaction in the cache
    ///
    /// This function checks for transactions with
    /// sequential nonces starting from `nonce_to_check`. If found, it processes
    /// each transaction, updates the wallet's nonce in the cache, and continues checking for
    /// the next nonce until no more transactions are found.
    ///
    /// # Arguments
    /// * `&self` - reference to the service
    /// * `chain_id` - The chain identifier
    /// * `wallet_address` - The wallet address to check for waiting transactions
    /// * `nonce_to_check` - The nonce value to start checking from
    ///
    /// # Returns
    /// * `Result<(), Error>` - Result containing possible [`WaitingTransaction`] errors
    pub async fn check_for_and_enqueue_waiting_transactions(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        starting_nonce: u64,
    ) -> Result<(), Error> {
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

        // Early return
        match waiting_txns.first() {
            None => return Ok(()),
            Some(waiting_txn) => {
                let tx = decode_transaction(waiting_txn).map_err(|e| {
                    error!(%e, %chain_id, %wallet_address, "Failed to decode raw transaction from cache");
                    WaitingTransaction(FailedToDecode)
                })?;
                _current_waiting_txn_nonce = tx.nonce();
            }
        }

        // Decode txn, enqueue, and remove from cache
        for waiting_txn in waiting_txns {
            // TODO unnecessary decode here
            let tx = decode_transaction(&waiting_txn).map_err(|e| {
                error!(%e, %chain_id, %wallet_address, "Failed to decode raw transaction from cache");
                WaitingTransaction(FailedToDecode)
            })?;
            _current_waiting_txn_nonce = tx.nonce();
            let tx_hash = format!("0x{}", hex::encode(tx.hash()));

            if let Err(e) = self.enqueue_raw_transaction(waiting_txn, chain_id, &tx_hash).await {
                error!(%e, %chain_id, %wallet_address, %tx_hash, "Failed to enqueue transaction");
                return Err(WaitingTransaction(FailedToEnqueue))
            }

            // TODO delete array of keys
            self.remove_waiting_transaction_from_cache(chain_id, wallet_address, _current_waiting_txn_nonce, &tx_hash).await.map_err(|e|{
                error!(%e, %chain_id, %wallet_address, %tx_hash, nonce=%_current_waiting_txn_nonce, "Failed to remove waiting transaction from cache");
                WaitingTransaction(FailedToEnqueue)
            })?;
        }

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

    // TODO update doc that it returns multiple
    /// Checks if a transaction exists in the cache for the given wallet address and nonce
    ///
    /// This method attempts to retrieve a transaction from the waiting gap cache
    /// for a specific wallet, chain, and nonce combination. It also handles decoding
    /// the hex-encoded transaction data from Redis into raw bytes.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to check
    /// * `wallet_address` - The wallet address to look up
    /// * `nonce` - The specific nonce to check for
    ///
    /// # Returns
    /// * `Ok(Some(Bytes))` - The raw transaction bytes if found in cache and successfully decoded
    /// * `Ok(None)` - If no transaction is found for the given nonce
    /// * `Err(Error::Redis)` - If there was an error accessing Redis
    /// * `Err(Error::WaitingTransaction(FailedToDecode))` - If the transaction was found but hex
    ///   decoding failed
    ///
    /// # Errors
    /// Returns an error if:
    /// * Redis connection fails or operation fails during transaction retrieval (wrapped as
    ///   [`Error::Redis`])
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
    ) -> Result<Vec<Bytes>, Error> {
        let mut conn = self.redis_conn.clone();
        let mut txns: Vec<Bytes> = Vec::new();
        let mut current_nonce = starting_nonce; // TODO unit test this off by one logic
        loop {
            match conn
                .get_waiting_txn(chain_id, wallet_address, current_nonce)
                .await
                .map_err(Error::Redis)?
            {
                None => {
                    //
                    break
                }
                Some(tx_hex) => match hex::decode(&tx_hex) {
                    Ok(tx_bytes) => {
                        txns.push(Bytes::from(tx_bytes));
                        current_nonce += 1
                    }
                    Err(e) => {
                        error!(%e, %chain_id, %wallet_address, %starting_nonce, %tx_hex, "Failed to decode hex transaction from cache");
                        return Err(WaitingTransaction(FailedToDecode))
                    }
                },
            }
        }
        // TODO starting_nonce + len(txns) must be == current_nonce-1
        Ok(txns)
        // let result = option
        //     .map_or_else(
        //         || Ok(None),
        //         |tx_hex| match hex::decode(&tx_hex) {
        //             Ok(bytes) => Ok(Some(Bytes::from(bytes))),
        //             Err(e) => {
        //                 error!(%e, %chain_id, %wallet_address, %starting_nonce, %tx_hex, "Failed
        // to decode hex transaction from cache");
        // Err(WaitingTransaction(FailedToDecode))             }
        //         }
        //     );
        // result
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
    /// * `Ok(String)` - The Redis operation result if successful
    /// * `Err(MaestroRpcError)` - If the Redis operation fails
    ///
    /// # Errors
    /// Returns an error if the Redis operation fails
    pub async fn cache_waiting_transaction(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce: u64,
        raw_tx: Bytes,
        tx_hash: &str,
    ) -> Result<String, MaestroRpcError> {
        let mut conn = self.redis_conn.clone();
        conn.set_waiting_txn(chain_id, wallet_address, nonce, raw_tx)
            .await
            .map_err(|_| Internal(TransactionSubmissionFailed(tx_hash.to_string())))
    }

    /// Removes a waiting transaction from the cache
    ///
    /// This method is called when a waiting transaction is successfully processed
    /// and should be removed from the waiting cache.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier
    /// * `wallet_address` - The wallet address that signed the transaction
    /// * `nonce` - The transaction nonce to remove
    /// * `tx_hash` - The transaction hash for error reporting
    ///
    /// # Returns
    /// * `Ok(u64)` - The number of keys removed if successful
    /// * `Err(MaestroRpcError)` - If the Redis operation fails
    ///
    /// # Errors
    /// Returns an error if the Redis operation fails
    pub async fn remove_waiting_transaction_from_cache(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce: u64,
        tx_hash: &str,
    ) -> Result<u64, MaestroRpcError> {
        let mut conn = self.redis_conn.clone();
        let result = conn
            .del_waiting_txn_key(chain_id, wallet_address, nonce)
            .await
            .map_err(|_| Internal(TransactionSubmissionFailed(tx_hash.to_string())))?;

        // Log if more than one key was removed - likely a bug
        if result > 1 {
            error!(
            %chain_id, %wallet_address, %nonce, %tx_hash, keys_removed=%result,
            "Removed multiple keys when deleting 1 waiting transaction"
            );
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, redis::streams::producer::tx_stream_key};
    use alloy::{
        network::{EthereumWallet, TransactionBuilder},
        primitives::{Address, Bytes},
        rlp::Encodable,
        rpc::types::TransactionRequest,
    };
    use redis::AsyncCommands;
    use serde_json::json;
    use std::time::Duration;
    use test_utils::{
        docker::{start_redis, Docker},
        wait_until,
    };
    use tokio::time::sleep;
    use wiremock::{
        matchers::{body_partial_json, method},
        Mock, MockServer, ResponseTemplate,
    };

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    // Helper to create a test service with real Redis and mock RPC
    async fn create_test_service() -> (MaestroService, MockServer, MockServer, Docker) {
        // Start Redis
        let (redis_container, redis_url) = start_redis().await.unwrap();

        // Start mock RPC servers
        let mock_rpc_server_4 = MockServer::start().await;
        set_up_mock_chain_id(&mock_rpc_server_4, 4).await;
        let mock_rpc_server_5 = MockServer::start().await;
        set_up_mock_chain_id(&mock_rpc_server_5, 5).await;

        // Create chain RPC URLs map
        let mut chain_rpc_urls = HashMap::new();
        chain_rpc_urls.insert("4".to_string(), mock_rpc_server_4.uri());
        chain_rpc_urls.insert("5".to_string(), mock_rpc_server_5.uri());

        // Create Redis connection
        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Create test config
        let config = Config {
            port: 0,
            redis_url,
            chain_rpc_urls,
            validation_timeout: Duration::from_secs(1),
            skip_validation: false,
            prune_interval: Duration::from_secs(1),
            prune_max_age: Duration::from_secs(1),
            metrics_port: 8090,
        };

        // Create service
        let service = MaestroService::new(redis_conn, config).await.unwrap();

        (service, mock_rpc_server_4, mock_rpc_server_5, redis_container)
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
        let (service, _, _, _) = create_test_service().await;

        // Test chain ID
        let chain_id = 4u64;

        // Get producer for first time
        let producer1 = service.producers.get(&chain_id).unwrap();

        // Verify stream key is correct
        assert_eq!(producer1.stream_key, format!("maestro:transactions:{}", chain_id));

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
        assert_eq!(producer3.stream_key, format!("maestro:transactions:{}", different_chain_id));
    }

    #[tokio::test]
    async fn test_enqueue_raw_transaction() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let raw_tx = Bytes::from(vec![1, 2, 3, 4, 5]);

        // Enqueue transaction
        let result = service.enqueue_raw_transaction(raw_tx.clone(), chain_id, "").await;

        // Verify success
        assert!(result.is_ok(), "Enqueuing transaction should succeed");

        // Verify transaction was actually stored
        // Connect to Redis directly to check
        let client = redis::Client::open(service._config.redis_url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();

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
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let expected_nonce = 42u64;

        // Pre-populate Redis with nonce
        let mut conn = service.redis_conn.clone();
        conn.set_wallet_nonce(chain_id, wallet, expected_nonce).await.unwrap();

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
        let (service, mock_server_4, _, _) = create_test_service().await;

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
        let mut conn = service.redis_conn.clone();
        let cached_nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(cached_nonce, Some(expected_nonce.to_string()), "Nonce should be cached");
    }

    #[tokio::test]
    async fn test_get_nonce_from_rpc_missing_provider() {
        // Create test service with mock servers
        let (service, _, _, _) = create_test_service().await;

        // Test data - use a chain ID that doesn't have a provider
        let chain_id = 9999u64; // Not in our config
        let wallet = Address::from_slice(&[0x42; 20]);

        // Try to get nonce
        let result = service.get_nonce_from_rpc_and_update_cache(chain_id, wallet).await;

        // Verify error
        assert!(result.is_err(), "Should fail when provider is missing");
        match result {
            Err(Internal(RpcMissing(c))) => {
                assert_eq!(c, chain_id, "Error should reference missing chain ID");
            }
            _ => panic!("Expected RpcMissing error"),
        }
    }

    #[tokio::test]
    async fn test_get_nonce_from_rpc_provider_error() {
        // Create test service
        let (service, mock_server_4, _, _) = create_test_service().await;

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
            Err(Internal(RpcFailedToFetchWalletNonce(c, w))) => {
                assert_eq!(c, chain_id, "Error should reference correct chain ID");
                assert_eq!(w, wallet, "Error should reference correct wallet");
            }
            _ => panic!("Expected RpcFailedToFetchWalletNonce error"),
        }
    }

    #[tokio::test]
    async fn test_increment_wallet_nonce() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let initial_nonce = 41u64;

        let incremented_nonce = 42u64;

        // Set initial nonce
        {
            let mut conn = service.redis_conn.clone();
            conn.set_wallet_nonce(chain_id, wallet, initial_nonce).await.unwrap();

            // Verify initial nonce
            let nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
            assert_eq!(nonce, Some(initial_nonce.to_string()), "Initial nonce should be set");
        }

        // Increment nonce
        let result = service
            .increment_wallet_nonce(chain_id, wallet, initial_nonce, initial_nonce + 1)
            .await;

        // Verify success (result should be "OK" from Redis)
        assert!(result.is_ok(), "Incrementing nonce should succeed");

        // Verify nonce was updated
        let mut conn = service.redis_conn.clone();
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
        let (service, _, _, redis) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;

        // Set nonce
        {
            let mut conn = service.redis_conn.clone();
            conn.set_wallet_nonce(chain_id, wallet, nonce).await.unwrap();

            // Verify nonce is initially set
            let initial_nonce = conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
            assert_eq!(initial_nonce, Some(nonce.to_string()), "Nonce should be initially set");
        } // Let this connection drop

        // Wait for TTL to expire
        sleep(Duration::from_secs(3 + 1)).await;

        // Create a fresh connection after the sleep
        let client = redis::Client::open(service._config.redis_url.as_str()).unwrap();
        let mut fresh_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Verify nonce has expired using the fresh connection
        let expired_nonce = fresh_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(expired_nonce, None, "Nonce should expire after TTL period");

        // Keep redis in scope to avoid closed connections
        drop(redis)
    }

    #[tokio::test]
    async fn test_multi_chain_independence() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

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
        let mut conn = service.redis_conn.clone();
        let get1 = conn.get_wallet_nonce(chain_id1, wallet).await.unwrap();
        let get2 = conn.get_wallet_nonce(chain_id2, wallet).await.unwrap();

        assert_eq!(get1, Some(nonce1_plus1.to_string()), "Chain 1 nonce should be correct");
        assert_eq!(get2, Some(nonce2_plus1.to_string()), "Chain 2 nonce should be correct");
    }

    #[tokio::test]
    async fn test_invalid_cached_nonce() {
        // Create test service
        let (service, mock_server_4, _, _) = create_test_service().await;

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
            let mut conn = service.redis_conn.clone();
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
        let mut conn = service.redis_conn.clone();
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
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let _tx_hash = "test_hash";

        // Set up test transaction in Redis
        let mut conn = service.redis_conn.clone();
        conn.set_waiting_txn(chain_id, wallet, nonce, raw_tx.clone()).await.unwrap();

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
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let tx_hash = "test_hash";

        // Put transaction in cache
        let result = service
            .cache_waiting_transaction(chain_id, wallet, nonce, raw_tx.clone(), tx_hash)
            .await;

        // Verify success
        assert!(result.is_ok(), "Putting transaction in cache should succeed");

        // Verify transaction was stored
        let mut conn = service.redis_conn.clone();
        let cached_tx_hex = conn.get_waiting_txn(chain_id, wallet, nonce).await.unwrap();

        assert!(cached_tx_hex.is_some(), "Transaction should be stored in cache");
        let cached_bytes = hex::decode(cached_tx_hex.unwrap()).unwrap();
        assert_eq!(cached_bytes, raw_tx.to_vec(), "Stored transaction should match the original");
    }

    #[tokio::test]
    async fn test_remove_gap_transaction_from_cache() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let tx_hash = "test_hash";

        // Set up test transaction in Redis
        let mut conn = service.redis_conn.clone();
        conn.set_waiting_txn(chain_id, wallet, nonce, raw_tx.clone()).await.unwrap();

        // Verify transaction exists before removal
        assert!(
            !service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, nonce)
                .await
                .unwrap()
                .is_empty(),
            "Transaction should exist before removal"
        );

        // Remove transaction
        let result =
            service.remove_waiting_transaction_from_cache(chain_id, wallet, nonce, tx_hash).await;

        // Verify success
        assert!(result.is_ok(), "Removing transaction from cache should succeed");
        assert_eq!(result.unwrap(), 1, "Should have removed exactly 1 key");

        // Verify transaction is gone
        assert!(
            service
                .get_contiguous_waiting_transactions_from_cache(chain_id, wallet, nonce)
                .await
                .unwrap()
                .is_empty(),
            "Transaction should be removed from cache"
        );

        // Test removing non-existent key
        let non_existent_result = service
            .remove_waiting_transaction_from_cache(chain_id, wallet, nonce + 1, tx_hash)
            .await;

        assert!(non_existent_result.is_ok(), "Removing non-existent key should not fail");
        assert_eq!(non_existent_result.unwrap(), 0, "Should return 0 for non-existent key");
    }

    #[tokio::test]
    async fn test_check_for_and_enqueue_waiting_transactions_empty() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

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
        let (service, _, _, redis) = create_test_service().await;

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
                Bytes::from(raw_tx1),
                "tx1",
            )
            .await
            .unwrap();

        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2,
                Bytes::from(raw_tx2),
                "tx2",
            )
            .await
            .unwrap();

        // Set up Redis stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        //Submit txn and increment nonce
        service.enqueue_raw_transaction(Bytes::from(raw_tx0), chain_id, "raw_tx0").await.unwrap();
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
        let final_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        println!("{} {}", initial_len, final_len);

        // Should have processed three total transactions
        assert_eq!(final_len, initial_len + 3, "Should have enqueued 3 transactions");

        // Verify nonce was updated in Redis
        let updated_nonce =
            service.redis_conn.clone().get_wallet_nonce(chain_id, wallet).await.unwrap();

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

        // Keep redis in scope to avoid closed connections
        drop(redis)
    }

    async fn build_test_txn(mut raw_tx0: &mut Vec<u8>, nonce: u64) {
        let signer = alloy::signers::local::PrivateKeySigner::random();
        let signer_wallet = EthereumWallet::from(signer);
        TransactionRequest::default()
            .to(Address::default())
            .value(alloy::primitives::U256::from(1))
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
        let (service, _, _, redis) = create_test_service().await;

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
            .cache_waiting_transaction(chain_id, wallet, current_nonce, Bytes::from(raw_tx1), "tx1")
            .await
            .unwrap();

        service
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2,
                Bytes::from(raw_tx3),
                "tx3",
            )
            .await
            .unwrap();

        // Set up Redis stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Set cache nonce to be the first one
        redis_conn.set_wallet_nonce(chain_id, wallet, current_nonce).await.unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        // Process waiting transactions
        let result = service
            .check_for_and_enqueue_waiting_transactions(chain_id, wallet, current_nonce)
            .await;

        // Verify success
        assert!(result.is_ok(), "Processing waiting transactions should succeed");

        // Verify only the first transaction was enqueued
        let final_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        assert_eq!(final_len, initial_len + 1, "Should have enqueued 1 transaction");

        // Verify nonce was updated only once
        let updated_nonce =
            service.redis_conn.clone().get_wallet_nonce(chain_id, wallet).await.unwrap();

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

        // Keep redis in scope to avoid closed connections
        drop(redis)
    }

    #[tokio::test]
    async fn test_check_for_and_enqueue_waiting_transactions_invalid_transaction() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Create an invalid transaction (can't be properly decoded)
        let invalid_tx = Bytes::from(vec![0x01, 0x02, 0x03]);

        // Add invalid transaction to waiting gap
        service
            .cache_waiting_transaction(chain_id, wallet, current_nonce, invalid_tx, "invalid_tx")
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
        let (service, _, _, _) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let tx_hash = "test_hash".to_string();

        // Set up Redis stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service_arc._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Set the initial nonce value
        redis_conn.set_wallet_nonce(chain_id, wallet, nonce).await.unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        // Submit transaction with equal nonce
        let result = service_arc
            .handle_transaction_and_manage_nonces(raw_tx, wallet, chain_id, nonce, &tx_hash)
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with equal nonce should be accepted");

        wait_until!(
            redis_conn.xlen::<&String, u64>(&stream_key).await.unwrap().eq(&(initial_len + 1)),
            Duration::from_millis(100)
        );

        // Verify transactions were enqueued (stream length increased)
        let final_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();
        assert_eq!(final_len, initial_len + 1, "Should have enqueued 1 transaction");

        // Verify nonce was incremented
        let updated_nonce = redis_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            updated_nonce,
            Some((nonce + 1).to_string()),
            "Nonce should be incremented by 1"
        );
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_lower_nonce() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 42u64;
        let lower_nonce = 41u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let tx_hash = "test_hash".to_string();

        // Set up Redis
        let client = redis::Client::open(service_arc._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Set the initial nonce value
        redis_conn.set_wallet_nonce(chain_id, wallet, current_nonce).await.unwrap();

        // Submit transaction with lower nonce
        let result = service_arc
            .handle_transaction_and_manage_nonces(raw_tx, wallet, chain_id, lower_nonce, &tx_hash)
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
        let unchanged_nonce = redis_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(
            unchanged_nonce,
            Some(current_nonce.to_string()),
            "Nonce should not change when transaction is rejected"
        );
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_higher_nonce() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 42u64;
        let higher_nonce = 44u64;
        let raw_tx = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);
        let tx_hash = "test_hash".to_string();

        // Set up Redis
        let client = redis::Client::open(service_arc._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Set the initial nonce value
        redis_conn.set_wallet_nonce(chain_id, wallet, current_nonce).await.unwrap();

        // Submit transaction with higher nonce
        let result = service_arc
            .handle_transaction_and_manage_nonces(
                raw_tx.clone(),
                wallet,
                chain_id,
                higher_nonce,
                &tx_hash,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with higher nonce should be accepted and cached");

        // Verify nonce was NOT incremented
        let unchanged_nonce = redis_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
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
        let (service, _, _, redis) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Create transactions for sequential nonces
        let mut raw_tx0 = vec![];
        let mut raw_tx1 = vec![];
        let mut raw_tx2 = vec![];
        build_test_txn(&mut raw_tx0, current_nonce).await;
        build_test_txn(&mut raw_tx1, current_nonce + 1).await;
        build_test_txn(&mut raw_tx2, current_nonce + 2).await;

        // Setup: Store future transactions in waiting gap
        service_arc
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 1,
                Bytes::from(raw_tx1.clone()),
                "tx1",
            )
            .await
            .unwrap();

        service_arc
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2,
                Bytes::from(raw_tx2.clone()),
                "tx2",
            )
            .await
            .unwrap();

        // Set up Redis stream for checking enqueued transactions
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service_arc._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Set the initial nonce value
        redis_conn.set_wallet_nonce(chain_id, wallet, current_nonce).await.unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        // Submit transaction with current nonce
        let tx_hash = "tx0".to_string();
        let result = service_arc
            .handle_transaction_and_manage_nonces(
                Bytes::from(raw_tx0),
                wallet,
                chain_id,
                current_nonce,
                &tx_hash,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with current nonce should be accepted");

        // Wait for background processing to complete
        wait_until!(
            redis_conn.xlen::<&String, u64>(&stream_key).await.unwrap().eq(&(initial_len + 3)),
            Duration::from_millis(100)
        );

        // Verify all three transactions were enqueued
        let final_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();
        assert_eq!(final_len, initial_len + 3, "Should have enqueued all 3 transactions");

        // Verify nonce was incremented for all transactions
        let updated_nonce = redis_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
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

        // Keep redis in scope to avoid closed connections
        drop(redis)
    }

    #[tokio::test]
    async fn test_handle_transaction_and_manage_nonces_with_gaps() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;
        let service_arc = Arc::new(service);

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let current_nonce = 41u64;

        // Create transactions for non-sequential nonces (gap at nonce 42)
        let mut raw_tx0 = vec![];
        let mut raw_tx2 = vec![];
        build_test_txn(&mut raw_tx0, current_nonce).await;
        // build_test_txn(&mut raw_tx1, 1).await;
        build_test_txn(&mut raw_tx2, current_nonce + 2).await;

        // Setup: Store future transaction with a gap
        service_arc
            .cache_waiting_transaction(
                chain_id,
                wallet,
                current_nonce + 2, // Skip nonce 42
                Bytes::from(raw_tx2.clone()),
                "tx2",
            )
            .await
            .unwrap();

        // Set up Redis stream
        let stream_key = tx_stream_key(chain_id);
        let client = redis::Client::open(service_arc._config.redis_url.as_str()).unwrap();
        let mut redis_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Set the initial nonce value
        redis_conn.set_wallet_nonce(chain_id, wallet, current_nonce).await.unwrap();

        // Get initial count of stream entries
        let initial_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();

        // Submit transaction with current nonce
        let tx_hash = "tx0".to_string();
        let result = service_arc
            .handle_transaction_and_manage_nonces(
                Bytes::from(raw_tx0),
                wallet,
                chain_id,
                current_nonce,
                &tx_hash,
            )
            .await;

        // Verify success
        assert!(result.is_ok(), "Transaction with current nonce should be accepted");

        // Wait for background processing to complete
        wait_until!(
            redis_conn.xlen::<&String, u64>(&stream_key).await.unwrap().eq(&(initial_len + 1)),
            Duration::from_millis(100)
        );

        // Verify only one transaction was enqueued (the gap prevents processing tx2)
        let final_len: u64 = redis_conn.xlen(&stream_key).await.unwrap();
        assert_eq!(final_len, initial_len + 1, "Should have enqueued only 1 transaction");

        // Verify nonce was incremented only for the first transaction
        let updated_nonce = redis_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
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
}
