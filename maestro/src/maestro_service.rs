//! This module is the `Maestro` service, which receives transaction requests and
//! validates them before submission into the transaction processing pipeline.

use crate::{
    config::{Config, RpcProvider},
    errors::{
        Error,
        InternalError::{RpcFailedToFetchWalletNonce, RpcMissing, TransactionSubmissionFailed},
        MaestroRpcError,
        MaestroRpcError::Internal,
    },
    redis::{
        producer::StreamProducer,
        string::{wallet_nonce_key, WalletNonceExt},
    },
};
use alloy::{
    primitives::{Address, Bytes, ChainId},
    providers::Provider,
};
use redis::aio::MultiplexedConnection;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// The service for filtering and directing transactions
#[derive(Debug)]
pub struct MaestroService {
    redis_conn: MultiplexedConnection,
    producers: Mutex<HashMap<ChainId, Arc<StreamProducer>>>,
    rpc_providers: HashMap<ChainId, RpcProvider>,
    config: Config,
}

impl MaestroService {
    /// Create a new instance of the Maestro service
    pub async fn new(redis_conn: MultiplexedConnection, config: Config) -> Result<Self, Error> {
        let rpc_providers = config.validate().await?;
        if rpc_providers.is_empty() {
            warn!("No RPC providers configured. This is probably undesirable");
        }
        Ok(Self { redis_conn, producers: Mutex::new(HashMap::new()), rpc_providers, config })
    }

    async fn get_or_create_producer(&self, chain_id: ChainId) -> Arc<StreamProducer> {
        let mut producers = self.producers.lock().await;
        producers
            .entry(chain_id)
            .or_insert_with(|| {
                Arc::new(StreamProducer::new(
                    self.redis_conn.clone(),
                    chain_id,
                    self.config.prune_interval,
                    self.config.prune_max_age,
                ))
            })
            .clone()
    }

    /// Enqueues a raw transaction to the Redis stream for a specific chain
    ///
    /// This method use Redis stream producers to forward the transaction data.
    /// It creates a new producer if one doesn't exist for the specified chain.
    ///
    /// # Arguments
    /// * `raw_tx` - The raw transaction bytes to enqueue
    /// * `chain_id` - The chain identifier to send the transaction to
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
    ) -> Result<(), jsonrpsee::types::ErrorObjectOwned> {
        // Get or create producer while holding lock
        let producer = self.get_or_create_producer(chain_id).await;

        // Release lock before making async call
        producer.enqueue_transaction(raw_tx.into()).await.map_err(|e| {
            error!(%chain_id, %e, "failed to enqueue transaction to Redis Stream");
            Internal(TransactionSubmissionFailed)
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
        let chain_wallet_nonce_key = wallet_nonce_key(chain_id, signer);
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
                info!(%signer, %chain_id, %chain_wallet_nonce_key, "No cached nonce found, fetching from RPC");
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
        let chain_wallet_nonce_key = wallet_nonce_key(chain_id, signer);
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

    /// Increments a wallet's nonce in the Redis cache
    ///
    /// This method updates the wallet's nonce in the Redis cache to the
    /// specified value. Typically used after a transaction is submitted
    /// to increment the nonce by 1.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier for the wallet
    /// * `wallet_address` - The wallet address to update the nonce for
    /// * `nonce_to_set` - The new nonce value to set
    ///
    /// # Returns
    /// * `String` - The result of the Redis SET operation, or "-1" if it fails
    pub async fn increment_wallet_nonce(
        &self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce_to_set: u64,
    ) -> String {
        let mut conn = self.redis_conn.clone();
        conn.set_wallet_nonce(chain_id, wallet_address, nonce_to_set).await.unwrap_or_else(|e| {
            warn!(%chain_id, %wallet_address, %e, "failed to set updated wallet nonce");
            "-1".to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use alloy::primitives::{Address, Bytes};
    use serde_json::json;
    use std::time::Duration;
    use test_utils::docker::{start_redis, Docker};
    use tokio::time::sleep;
    use wiremock::{
        matchers::{body_partial_json, method},
        Mock, MockServer, ResponseTemplate,
    };

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
    async fn test_get_or_create_producer() {
        // Create test service
        let (service, _, _, _) = create_test_service().await;

        // Test chain ID
        let chain_id = 4u64;

        // Get producer for first time
        let producer1 = service.get_or_create_producer(chain_id).await;

        // Verify stream key is correct
        assert_eq!(producer1.stream_key, format!("maestro:transactions:{}", chain_id));

        // Get producer again
        let producer2 = service.get_or_create_producer(chain_id).await;

        // Verify it's the same producer (by comparing Arc pointers)
        assert!(Arc::ptr_eq(&producer1, &producer2), "Should reuse existing producer");

        // Try with a different chain ID
        let different_chain_id = 5u64;
        let producer3 = service.get_or_create_producer(different_chain_id).await;

        // Verify it's a different producer
        assert!(
            !Arc::ptr_eq(&producer1, &producer3),
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
        let result = service.enqueue_raw_transaction(raw_tx.clone(), chain_id).await;

        // Verify success
        assert!(result.is_ok(), "Enqueuing transaction should succeed");

        // Verify transaction was actually stored
        // Connect to Redis directly to check
        let client = redis::Client::open(service.config.redis_url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();

        // Check stream exists and has entries
        let stream_key = format!("maestro:transactions:{}", chain_id);
        let len: usize = redis::cmd("XLEN").arg(&stream_key).query_async(&mut conn).await.unwrap();

        assert!(len > 0, "Stream should contain entries");

        // Read stream entries
        let entries: Vec<(String, HashMap<String, Vec<u8>>)> = redis::cmd("XRANGE")
            .arg(&stream_key)
            .arg("-")
            .arg("+")
            .query_async(&mut conn)
            .await
            .unwrap();

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
        let wallet_hex = format!("0x{}", alloy::hex::encode(wallet.as_slice()));
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
        let wallet_hex = format!("0x{}", alloy::hex::encode(wallet.as_slice()));

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
        let result = service.increment_wallet_nonce(chain_id, wallet, incremented_nonce).await;

        // Verify success (result should be "OK" from Redis)
        assert_ne!(result, "-1", "Incrementing nonce should succeed");

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
        let (service, _, _, redis_container) = create_test_service().await;

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
        let client = redis::Client::open(service.config.redis_url.as_str()).unwrap();
        let mut fresh_conn = client.get_multiplexed_async_connection().await.unwrap();

        // Verify nonce has expired using the fresh connection
        let expired_nonce = fresh_conn.get_wallet_nonce(chain_id, wallet).await.unwrap();
        assert_eq!(expired_nonce, None, "Nonce should expire after TTL period");

        drop(redis_container);
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
        let nonce2 = 100u64;

        // Set nonces for both chains
        service.increment_wallet_nonce(chain_id1, wallet, nonce1).await;
        service.increment_wallet_nonce(chain_id2, wallet, nonce2).await;

        // Verify nonces are chain-specific
        let mut conn = service.redis_conn.clone();
        let get1 = conn.get_wallet_nonce(chain_id1, wallet).await.unwrap();
        let get2 = conn.get_wallet_nonce(chain_id2, wallet).await.unwrap();

        assert_eq!(get1, Some(nonce1.to_string()), "Chain 1 nonce should be correct");
        assert_eq!(get2, Some(nonce2.to_string()), "Chain 2 nonce should be correct");
    }

    #[tokio::test]
    async fn test_invalid_cached_nonce() {
        // Create test service
        let (service, mock_server_4, _, _) = create_test_service().await;

        // Test data
        let chain_id = 4u64;
        let wallet = Address::from_slice(&[0x42; 20]);
        let wallet_hex = format!("0x{}", alloy::hex::encode(wallet.as_slice()));
        let invalid_nonce_str = "not_a_number";
        let rpc_nonce = 42u64;

        // Set up invalid nonce in cache
        {
            // Directly set invalid nonce string
            let key = wallet_nonce_key(chain_id, wallet);
            let mut conn = service.redis_conn.clone();
            redis::cmd("SET")
                .arg(&key)
                .arg(invalid_nonce_str)
                .query_async::<String>(&mut conn)
                .await
                .unwrap();
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
}
