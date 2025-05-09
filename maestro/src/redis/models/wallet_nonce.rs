//! This module describes the required functionality for Maestro to interact with wallet nonce
//! values in the Redis cache.

use crate::redis::{
    keys::wallet_nonce::chain_wallet_nonce_key, ttl::wallet_nonce::WALLET_NONCE_TTL_SEC,
};
use alloy::primitives::{Address, ChainId};
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult, SetExpiry::EX, SetOptions};
use std::future::Future;

/// Extension trait for Redis connections to work with wallet nonces
pub trait WalletNonceExt {
    /// Get a wallet nonce from the Redis cache
    fn get_wallet_nonce(
        &mut self,
        chain_id: ChainId,
        wallet_address: Address,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send;

    /// Set a wallet nonce in the Redis cache with TTL
    fn set_wallet_nonce(
        &mut self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce: u64,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send;
}

/// Public trait which is non-async and instead returns a [`Future`]. This provides greater
/// flexibility to trait users
impl WalletNonceExt for MultiplexedConnection {
    fn get_wallet_nonce(
        &mut self,
        chain_id: ChainId,
        wallet_address: Address,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send {
        let key = chain_wallet_nonce_key(chain_id, wallet_address);
        self.get(key)
    }

    /// Returns previous nonce value before update
    fn set_wallet_nonce(
        &mut self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce: u64,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send {
        let key = chain_wallet_nonce_key(chain_id, wallet_address);
        let opts = SetOptions::default().with_expiration(EX(WALLET_NONCE_TTL_SEC)).get(true);
        Box::pin(self.set_options(key, nonce.to_string(), opts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::redis::{
        keys::wallet_nonce::WALLET_NONCE_KEY_PREFIX,
        models::wallet_nonce::WalletNonceExt,
        test_utils::{get_redis_connection, initialize_redis},
    };
    use alloy::primitives::Address;
    use ctor::ctor;
    use test_utils::docker::start_redis;
    use tokio::time::{sleep, Duration};

    #[ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    #[tokio::test]
    async fn test_wallet_nonce_key_format() {
        let chain_id = 1u64;
        let wallet_address = Address::from_slice(&[0x42; 20]);

        let key = chain_wallet_nonce_key(chain_id, wallet_address);

        assert_eq!(
            key,
            format!(
                "{}:{}_0x4242424242424242424242424242424242424242",
                WALLET_NONCE_KEY_PREFIX, chain_id
            )
        );
    }

    #[tokio::test]
    async fn test_set_get_wallet_nonce() {
        let mut conn = get_redis_connection().await;

        let chain_id = 4u64;
        let wallet_address = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;

        // Test set_wallet_nonce
        let set_result = conn.set_wallet_nonce(chain_id, wallet_address, nonce).await;
        assert!(set_result.is_ok(), "Failed to set wallet nonce: {:?}", set_result.err());
        assert!(set_result.unwrap().is_none(), "no value there previously");

        // Test get_wallet_nonce
        let get_result = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
        assert_eq!(get_result, Some(nonce.to_string()), "Retrieved nonce doesn't match set nonce");
    }

    #[tokio::test]
    async fn test_nonce_expiration() {
        // Setup Redis connection
        let (redis, redis_url) = start_redis().await.unwrap();

        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();

        let chain_id = 5u64;
        let wallet_address = Address::from_slice(&[0x24; 20]);
        let nonce = 123u64;

        // Set the nonce
        let set_result = conn.set_wallet_nonce(chain_id, wallet_address, nonce).await;
        assert!(set_result.is_ok(), "Failed to set wallet nonce: {:?}", set_result.err());

        // Verify it exists
        let get_result = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
        assert_eq!(
            get_result,
            Some(nonce.to_string()),
            "Nonce should exist immediately after setting"
        );

        // Wait for TTL to expire (add a small buffer)
        sleep(Duration::from_secs(WALLET_NONCE_TTL_SEC + 1)).await;

        // Verify it has expired
        let expired_result = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
        assert_eq!(expired_result, None, "Nonce should have expired after TTL");

        // Keep Redis in scope or else connection is dropped
        drop(redis)
    }

    #[tokio::test]
    async fn test_update_existing_nonce() {
        let mut conn = get_redis_connection().await;

        let chain_id = 6u64;
        let wallet_address = Address::from_slice(&[0x36; 20]);
        let initial_nonce = 10u64;
        let updated_nonce = 11u64;

        // Set initial nonce
        conn.set_wallet_nonce(chain_id, wallet_address, initial_nonce).await.unwrap();

        // Verify initial nonce
        let initial_get = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
        assert_eq!(initial_get, Some(initial_nonce.to_string()), "Initial nonce not set correctly");

        // Update nonce
        let result =
            conn.set_wallet_nonce(chain_id, wallet_address, updated_nonce).await.unwrap().unwrap();
        assert_eq!(result, initial_nonce.to_string(), "old nonce was in cache");
        assert_eq!(
            result.parse::<u64>().unwrap(),
            updated_nonce - 1,
            "nonce should be incremented by 1"
        );

        // Verify updated nonce
        let updated_get = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
        assert_eq!(updated_get, Some(updated_nonce.to_string()), "Nonce not updated correctly");
    }

    #[tokio::test]
    async fn test_non_existent_nonce() {
        let mut conn = get_redis_connection().await;

        let chain_id = 7u64;
        let wallet_address = Address::from_slice(&[0x48; 20]);

        // Try to get a nonce that doesn't exist
        let get_result = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
        assert_eq!(get_result, None, "Non-existent nonce should return None");
    }

    #[tokio::test]
    async fn test_multiple_wallets_independence() {
        let mut conn = get_redis_connection().await;

        let chain_id = 8u64;
        let wallet_address1 = Address::from_slice(&[0x5A; 20]);
        let wallet_address2 = Address::from_slice(&[0x5B; 20]);
        let nonce1 = 200u64;
        let nonce2 = 300u64;

        // Set nonces for different wallets
        conn.set_wallet_nonce(chain_id, wallet_address1, nonce1).await.unwrap();
        conn.set_wallet_nonce(chain_id, wallet_address2, nonce2).await.unwrap();

        // Verify each wallet has its own independent nonce
        let get1 = conn.get_wallet_nonce(chain_id, wallet_address1).await.unwrap();
        let get2 = conn.get_wallet_nonce(chain_id, wallet_address2).await.unwrap();

        assert_eq!(get1, Some(nonce1.to_string()), "Wallet 1 nonce incorrect");
        assert_eq!(get2, Some(nonce2.to_string()), "Wallet 2 nonce incorrect");
    }

    #[tokio::test]
    async fn test_multiple_chains_independence() {
        let mut conn = get_redis_connection().await;

        let chain_id1 = 9u64;
        let chain_id2 = 10u64;
        let wallet_address = Address::from_slice(&[0x6C; 20]);
        let nonce1 = 400u64;
        let nonce2 = 500u64;

        // Set nonces for same wallet on different chains
        conn.set_wallet_nonce(chain_id1, wallet_address, nonce1).await.unwrap();
        conn.set_wallet_nonce(chain_id2, wallet_address, nonce2).await.unwrap();

        // Verify each chain has its own independent nonce for the same wallet
        let get1 = conn.get_wallet_nonce(chain_id1, wallet_address).await.unwrap();
        let get2 = conn.get_wallet_nonce(chain_id2, wallet_address).await.unwrap();

        assert_eq!(get1, Some(nonce1.to_string()), "Chain 1 nonce incorrect");
        assert_eq!(get2, Some(nonce2.to_string()), "Chain 2 nonce incorrect");
    }

    #[tokio::test]
    async fn test_high_load_scenario() {
        // Setup Redis connection
        let (_redis, redis_url) = start_redis().await.unwrap();

        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();

        let chain_id = 11u64;
        let wallet_address = Address::from_slice(&[0x7D; 20]);

        // Set and get nonce in rapid succession
        for i in 0..100 {
            let nonce = i;

            // Set nonce
            conn.set_wallet_nonce(chain_id, wallet_address, nonce).await.unwrap();

            // Get nonce
            let get_result = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();
            assert_eq!(get_result, Some(nonce.to_string()), "Nonce mismatch at iteration {}", i);
        }
    }

    #[tokio::test]
    async fn test_parallel_operations() {
        let redis_url = initialize_redis().await;
        // Create multiple connections for parallel operations
        let mut handles = vec![];

        for i in 0..10 {
            let chain_id = 12u64;
            let wallet_bytes = [i as u8; 20]; // Create different addresses
            let redis_url_clone = redis_url.clone();

            let handle = tokio::spawn(async move {
                let client = redis::Client::open(redis_url_clone.as_str()).unwrap();
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();

                let wallet_address = Address::from_slice(&wallet_bytes);
                let nonce = (i + 1) * 100;

                // Set and get nonce
                conn.set_wallet_nonce(chain_id, wallet_address, nonce).await.unwrap();
                let result = conn.get_wallet_nonce(chain_id, wallet_address).await.unwrap();

                (wallet_address, nonce, result)
            });

            handles.push(handle);
        }

        // Await and validate results
        for handle in handles {
            let (address, expected_nonce, result) = handle.await.unwrap();
            assert_eq!(
                result,
                Some(expected_nonce.to_string()),
                "Parallel operation failed for address {}",
                address
            );
        }
    }
}
