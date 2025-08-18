//! This module describes the required functionality for Maestro to interact with consumer last id
//! values in the Valkey cache.

use crate::valkey::keys::consumer_last_id::chain_consumer_last_id_key;
use alloy::primitives::ChainId;
use redis::{aio::ConnectionManager, AsyncCommands, RedisResult, SetOptions};
use std::future::Future;

/// Extension trait for Valkey connections to work with consumer last id
pub trait ConsumerLastIdExt {
    /// Get a consumer last id from the Valkey cache
    fn get_consumer_last_id(
        &mut self,
        chain_id: ChainId,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send;

    /// Set a consumer last id in the Valkey cache with TTL
    fn set_consumer_last_id(
        &mut self,
        chain_id: ChainId,
        last_id: String,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send;
}

/// Public trait which is non-async and instead returns a [`Future`]. This provides greater
/// flexibility to trait users
impl ConsumerLastIdExt for ConnectionManager {
    fn get_consumer_last_id(
        &mut self,
        chain_id: ChainId,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send {
        let key = chain_consumer_last_id_key(chain_id);
        self.get(key)
    }

    /// Returns previous last id value before update
    fn set_consumer_last_id(
        &mut self,
        chain_id: ChainId,
        last_id: String,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send {
        let key = chain_consumer_last_id_key(chain_id);
        self.set_options(key, last_id, SetOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valkey::{
        keys::consumer_last_id::CONSUMER_LAST_ID_KEY_PREFIX,
        test_utils::init_valkey_and_get_connection,
    };
    use ctor::ctor;
    use shared::tracing::setup_global_logging;

    #[ctor]
    fn init() {
        setup_global_logging();
    }

    #[tokio::test]
    async fn test_consumer_last_id_key_format() {
        let chain_id = 1u64;

        let key = chain_consumer_last_id_key(chain_id);
        assert_eq!(key, format!("{CONSUMER_LAST_ID_KEY_PREFIX}:{chain_id}"));
    }

    // Avoid having to spin up unique Valkey containers this way
    #[tokio::test]
    async fn test_cache() -> Result<(), eyre::Error> {
        let (conn, valkey_url, _valkey) = init_valkey_and_get_connection().await;
        println!("valkey url is {valkey_url}");

        test_set_get_consumer_last_id(conn.clone()).await;
        test_consumer_set_last_id(conn.clone()).await;
        test_update_existing_consumer_last_id(conn.clone()).await;
        test_non_existent_consumer_last_id(conn.clone()).await;

        Ok(())
    }

    async fn test_set_get_consumer_last_id(mut conn: ConnectionManager) {
        let chain_id = 4u64;
        let last_id = "42-42".to_string();

        // Test set_consumer_last_id
        let set_result = conn.set_consumer_last_id(chain_id, last_id.clone()).await;
        assert!(set_result.is_ok(), "Failed to set consumer last id: {:?}", set_result.err());

        // Test get_consumer_last_id after setting
        let get_result = conn.get_consumer_last_id(chain_id).await.unwrap();
        assert_eq!(get_result, Some(last_id), "Retrieved last id doesn't match set last id");
    }

    async fn test_consumer_set_last_id(mut conn: ConnectionManager) {
        let chain_id = 5u64;
        let last_id = "123-123".to_string();

        // Set the last id
        let set_result = conn.set_consumer_last_id(chain_id, last_id.clone()).await;
        assert!(set_result.is_ok(), "Failed to set consumer last id: {:?}", set_result.err());

        // Verify it exists
        let get_result = conn.get_consumer_last_id(chain_id).await.unwrap();
        assert_eq!(get_result, Some(last_id), "Last id should exist immediately after setting");
    }

    async fn test_update_existing_consumer_last_id(mut conn: ConnectionManager) {
        let chain_id = 6u64;
        let initial_last_id = "10-10".to_string();
        let updated_last_id = "11-11".to_string();

        // Set initial last id
        conn.set_consumer_last_id(chain_id, initial_last_id.clone()).await.unwrap();

        // Verify initial last id
        let initial_get = conn.get_consumer_last_id(chain_id).await.unwrap();
        assert_eq!(initial_get, Some(initial_last_id.clone()), "Initial last id not set correctly");

        // Update last id
        let result = conn.set_consumer_last_id(chain_id, updated_last_id.clone()).await;
        assert!(result.is_ok(), "Failed to update consumer last id: {:?}", result.err());

        // Verify updated last id
        let updated_get = conn.get_consumer_last_id(chain_id).await.unwrap();
        assert_eq!(updated_get, Some(updated_last_id), "Last id not updated correctly");
    }

    async fn test_non_existent_consumer_last_id(mut conn: ConnectionManager) {
        let chain_id = 7u64;

        // Try to get a last id that doesn't exist
        let get_result = conn.get_consumer_last_id(chain_id).await.unwrap();
        assert_eq!(get_result, None, "Non-existent consumer last id should return None");
    }
}
