//! This module describes the required functionality for Maestro to interact with waiting gap
//! transactions in the Redis cache.

use crate::redis::keys::waiting_txn::waiting_gap_txns_key;
use alloy::{
    hex,
    primitives::{Address, Bytes, ChainId},
};
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult, SetExpiry::EX, SetOptions};
use std::{future::Future, time::Duration};

/// Extension trait for Redis connections to work with waiting transactions
pub trait WaitingGapTxnExt {
    /// Get a waiting transaction from the Redis cache
    fn get_waiting_txn(
        &mut self,
        chain_id: ChainId,
        signer: Address,
        nonce: u64,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send;

    /// Set a waiting transaction in the Redis cache with TTL
    fn set_waiting_txn(
        &mut self,
        chain_id: ChainId,
        signer: Address,
        nonce: u64,
        raw_txn: Bytes,
        ttl: Duration,
    ) -> impl Future<Output = RedisResult<String>> + Send;

    /// Delete a waiting transaction by key from the Redis cache
    fn del_waiting_txn_keys(
        &mut self,
        waiting_txns: &[WaitingTransactionId],
    ) -> impl Future<Output = RedisResult<u64>> + Send;
}

impl WaitingGapTxnExt for MultiplexedConnection {
    /// note: returns hex encoded String
    fn get_waiting_txn(
        &mut self,
        chain_id: ChainId,
        wallet_address: Address,
        nonce: u64,
    ) -> impl Future<Output = RedisResult<Option<String>>> + Send {
        let key = waiting_gap_txns_key(chain_id, wallet_address, nonce);
        self.get(key)
    }

    fn set_waiting_txn(
        &mut self,
        chain_id: ChainId,
        signer: Address,
        nonce: u64,
        raw_txn: Bytes,
        ttl: Duration,
    ) -> impl Future<Output = RedisResult<String>> + Send {
        let encoded_txn = hex::encode(raw_txn);
        let key = waiting_gap_txns_key(chain_id, signer, nonce);
        let opts = SetOptions::default().with_expiration(EX(ttl.as_secs()));
        self.set_options(key, encoded_txn, opts)
    }

    fn del_waiting_txn_keys(
        &mut self,
        waiting_txns: &[WaitingTransactionId],
    ) -> impl Future<Output = RedisResult<u64>> + Send {
        let mut keys = Vec::new();
        for txn_id in waiting_txns {
            let key = waiting_gap_txns_key(txn_id.chain_id, txn_id.wallet_address, txn_id.nonce);
            keys.push(key);
        }
        self.del(keys)
    }
}

/// Identifiers for a transaction that, combined, make it unique in the cache
#[derive(Debug)]
#[allow(missing_docs)]
pub struct WaitingTransactionId {
    pub chain_id: ChainId,
    pub wallet_address: Address,
    pub nonce: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::redis::{
        keys::waiting_txn::WAITING_GAP_KEY_PREFIX, models::waiting_transaction::WaitingGapTxnExt,
        test_utils::init_redis_and_get_connection,
    };
    use alloy::primitives::Address;
    use ctor::ctor;

    #[ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    const TTL: Duration = Duration::from_secs(15 * 60);

    #[tokio::test]
    async fn test_waiting_gap_txns_key_format() {
        let chain_id = 1u64;
        let signer = Address::from_slice(&[0x42; 20]);
        let nonce = 5u64;

        let key = waiting_gap_txns_key(chain_id, signer, nonce);

        assert_eq!(
            key,
            format!(
                "{}:{}_0x4242424242424242424242424242424242424242_5",
                WAITING_GAP_KEY_PREFIX, chain_id
            )
        );
    }

    // Avoid having to spin up unique Redis containers this way
    #[tokio::test]
    async fn test_cache() -> Result<(), eyre::Error> {
        let (conn, _, _redis) = init_redis_and_get_connection().await;

        test_set_get_waiting_txn(conn.clone()).await;
        test_del_waiting_txn_key(conn.clone()).await;
        test_multiple_waiting_txns_independence(conn.clone()).await;
        test_same_signer_different_nonces(conn.clone()).await;
        test_overwrite_existing_waiting_txn(conn.clone()).await;
        test_different_chains_independence(conn.clone()).await;
        test_delete_nonexistent_key(conn.clone()).await;
        test_waiting_txn_expiration(conn.clone()).await;
        test_del_multiple_waiting_txn_key(conn.clone()).await;

        Ok(())
    }

    async fn test_set_get_waiting_txn(mut conn: MultiplexedConnection) {
        let chain_id = 4u64;
        let signer = Address::from_slice(&[0x42; 20]);
        let nonce = 42u64;
        let raw_txn = Bytes::from(vec![0x01, 0x02, 0x03, 0x04]);

        // Test set_waiting_txn
        let set_result = conn.set_waiting_txn(chain_id, signer, nonce, raw_txn.clone(), TTL).await;
        assert!(set_result.is_ok(), "Failed to set waiting txn: {:?}", set_result.err());

        // Test get_waiting_txn
        let get_result = conn.get_waiting_txn(chain_id, signer, nonce).await.unwrap();
        assert_eq!(
            get_result,
            Some(hex::encode(raw_txn.clone())),
            "Retrieved transaction doesn't match set transaction"
        );
    }

    async fn test_del_waiting_txn_key(mut conn: MultiplexedConnection) {
        let chain_id = 5u64;
        let wallet_address = Address::from_slice(&[0x24; 20]);
        let nonce = 123u64;
        let raw_txn = Bytes::from(vec![0x05, 0x06, 0x07, 0x08]);

        // Set the transaction
        let set_result =
            conn.set_waiting_txn(chain_id, wallet_address, nonce, raw_txn.clone(), TTL).await;
        assert!(set_result.is_ok(), "Failed to set waiting txn: {:?}", set_result.err());

        // Verify it exists
        let get_result = conn.get_waiting_txn(chain_id, wallet_address, nonce).await.unwrap();
        assert_eq!(
            get_result,
            Some(hex::encode(raw_txn.clone())),
            "Transaction should exist after setting"
        );

        // Delete the transaction
        let txn_ids = vec![WaitingTransactionId { chain_id, wallet_address, nonce }];
        let del_result = conn.del_waiting_txn_keys(&txn_ids).await;
        assert!(del_result.is_ok(), "Failed to delete waiting txn: {:?}", del_result.err());
        assert_eq!(del_result.unwrap(), 1, "Should have deleted exactly one key");

        // Verify it's gone
        let get_after_del = conn.get_waiting_txn(chain_id, wallet_address, nonce).await.unwrap();
        assert_eq!(get_after_del, None, "Transaction should be deleted");
    }

    async fn test_del_multiple_waiting_txn_key(mut conn: MultiplexedConnection) {
        let chain_id = 5u64;
        let wallet_address = Address::from_slice(&[0x24; 20]);
        let nonce = 123u64;
        let raw_txn = Bytes::from(vec![0x05, 0x06, 0x07, 0x08]);

        // Set the transaction
        let set_result =
            conn.set_waiting_txn(chain_id, wallet_address, nonce, raw_txn.clone(), TTL).await;
        assert!(set_result.is_ok(), "Failed to set waiting txn: {:?}", set_result.err());
        let _ =
            conn.set_waiting_txn(chain_id, wallet_address, nonce + 1, raw_txn.clone(), TTL).await;
        let _ =
            conn.set_waiting_txn(chain_id, wallet_address, nonce + 2, raw_txn.clone(), TTL).await;

        // Verify it exists
        let get_result = conn.get_waiting_txn(chain_id, wallet_address, nonce).await.unwrap();
        assert_eq!(
            get_result,
            Some(hex::encode(raw_txn.clone())),
            "Transaction should exist after setting"
        );

        // Delete the transaction
        let txn_ids = vec![
            WaitingTransactionId { chain_id, wallet_address, nonce },
            WaitingTransactionId { chain_id, wallet_address, nonce: nonce + 1 },
            WaitingTransactionId { chain_id, wallet_address, nonce: nonce + 2 },
        ];
        let del_result = conn.del_waiting_txn_keys(&txn_ids).await;
        assert!(del_result.is_ok(), "Failed to delete waiting txns: {:?}", del_result.err());
        assert_eq!(del_result.unwrap(), 3, "Should have deleted exactly 3 keys");

        // Verify it's gone
        let get_after_del =
            conn.get_waiting_txn(chain_id, wallet_address, nonce + 2).await.unwrap();
        assert_eq!(get_after_del, None, "Transaction should be deleted");
    }

    async fn test_waiting_txn_expiration(mut conn: MultiplexedConnection) {
        // For testing TTL, we'll use a much shorter TTL than the default

        let chain_id = 6u64;
        let signer = Address::from_slice(&[0x36; 20]);
        let nonce = 10u64;
        let raw_txn = Bytes::from(vec![0x0A, 0x0B, 0x0C, 0x0D]);

        let shorter_ttl = Duration::from_secs(2);

        // Set the transaction
        conn.set_waiting_txn(chain_id, signer, nonce, raw_txn.clone(), shorter_ttl).await.unwrap();

        // Verify it exists
        let get_result = conn.get_waiting_txn(chain_id, signer, nonce).await.unwrap();
        assert_eq!(
            get_result,
            Some(hex::encode(raw_txn.clone())),
            "Transaction should exist immediately after setting"
        );

        // Check that the key has a TTL (we can't easily wait for 15 minutes in a test)
        let key = waiting_gap_txns_key(chain_id, signer, nonce);
        let ttl: Option<i64> = conn.ttl(&key).await.unwrap();

        // TTL should be positive (key exists and has an expiry) and less than or equal to the
        // defined constant
        assert!(ttl.is_some(), "Key should have a TTL");
        assert!(ttl.unwrap() > 0, "TTL should be positive");
        assert!(
            ttl.unwrap() <= shorter_ttl.as_secs() as i64,
            "TTL should be at most the defined constant"
        );
    }

    async fn test_multiple_waiting_txns_independence(mut conn: MultiplexedConnection) {
        let chain_id = 7u64;
        let signer1 = Address::from_slice(&[0x5A; 20]);
        let signer2 = Address::from_slice(&[0x5B; 20]);
        let nonce1 = 200u64;
        let nonce2 = 300u64;
        let raw_txn1 = Bytes::from(vec![0x11, 0x22, 0x33]);
        let raw_txn2 = Bytes::from(vec![0x44, 0x55, 0x66]);

        // Set transactions for different signers
        conn.set_waiting_txn(chain_id, signer1, nonce1, raw_txn1.clone(), TTL).await.unwrap();
        conn.set_waiting_txn(chain_id, signer2, nonce2, raw_txn2.clone(), TTL).await.unwrap();

        // Verify each transaction is stored independently
        let get1 = conn.get_waiting_txn(chain_id, signer1, nonce1).await.unwrap();
        let get2 = conn.get_waiting_txn(chain_id, signer2, nonce2).await.unwrap();

        assert_eq!(get1, Some(hex::encode(raw_txn1.clone())), "Transaction 1 incorrect");
        assert_eq!(get2, Some(hex::encode(raw_txn2.clone())), "Transaction 2 incorrect");

        // Delete one transaction and verify the other remains
        let txn_ids =
            vec![WaitingTransactionId { chain_id, wallet_address: signer1, nonce: nonce1 }];
        conn.del_waiting_txn_keys(&txn_ids).await.unwrap();

        let get1_after_del = conn.get_waiting_txn(chain_id, signer1, nonce1).await.unwrap();
        let get2_after_del = conn.get_waiting_txn(chain_id, signer2, nonce2).await.unwrap();

        assert_eq!(get1_after_del, None, "Transaction 1 should be deleted");
        assert_eq!(
            get2_after_del,
            Some(hex::encode(raw_txn2.clone())),
            "Transaction 2 should still exist"
        );
    }

    async fn test_same_signer_different_nonces(mut conn: MultiplexedConnection) {
        let chain_id = 8u64;
        let signer = Address::from_slice(&[0x6C; 20]);
        let nonce1 = 100u64;
        let nonce2 = 101u64;
        let raw_txn1 = Bytes::from(vec![0xA1, 0xA2, 0xA3]);
        let raw_txn2 = Bytes::from(vec![0xB1, 0xB2, 0xB3]);

        // Set transactions with different nonces
        conn.set_waiting_txn(chain_id, signer, nonce1, raw_txn1.clone(), TTL).await.unwrap();
        conn.set_waiting_txn(chain_id, signer, nonce2, raw_txn2.clone(), TTL).await.unwrap();

        // Verify different nonces are stored independently
        let get1 = conn.get_waiting_txn(chain_id, signer, nonce1).await.unwrap();
        let get2 = conn.get_waiting_txn(chain_id, signer, nonce2).await.unwrap();

        assert_eq!(get1, Some(hex::encode(raw_txn1.clone())), "Transaction for nonce1 incorrect");
        assert_eq!(get2, Some(hex::encode(raw_txn2.clone())), "Transaction for nonce2 incorrect");
    }

    async fn test_overwrite_existing_waiting_txn(mut conn: MultiplexedConnection) {
        let chain_id = 9u64;
        let signer = Address::from_slice(&[0x7D; 20]);
        let nonce = 42u64;
        let raw_txn1 = Bytes::from(vec![0xC1, 0xC2, 0xC3]);
        let raw_txn2 = Bytes::from(vec![0xD1, 0xD2, 0xD3]);

        // Set initial transaction
        conn.set_waiting_txn(chain_id, signer, nonce, raw_txn1.clone(), TTL).await.unwrap();

        // Verify initial transaction
        let initial_get = conn.get_waiting_txn(chain_id, signer, nonce).await.unwrap();
        assert_eq!(
            initial_get,
            Some(hex::encode(raw_txn1.clone())),
            "Initial transaction not set correctly"
        );

        // Overwrite with new transaction
        conn.set_waiting_txn(chain_id, signer, nonce, raw_txn2.clone(), TTL).await.unwrap();

        // Verify updated transaction
        let updated_get = conn.get_waiting_txn(chain_id, signer, nonce).await.unwrap();
        assert_eq!(
            updated_get,
            Some(hex::encode(raw_txn2.clone())),
            "Transaction not updated correctly"
        );
    }

    async fn test_different_chains_independence(mut conn: MultiplexedConnection) {
        let chain_id1 = 10u64;
        let chain_id2 = 11u64;
        let signer = Address::from_slice(&[0x8E; 20]);
        let nonce = 50u64;
        let raw_txn1 = Bytes::from(vec![0xE1, 0xE2, 0xE3]);
        let raw_txn2 = Bytes::from(vec![0xF1, 0xF2, 0xF3]);

        // Set transactions for same signer/nonce on different chains
        conn.set_waiting_txn(chain_id1, signer, nonce, raw_txn1.clone(), TTL).await.unwrap();
        conn.set_waiting_txn(chain_id2, signer, nonce, raw_txn2.clone(), TTL).await.unwrap();

        // Verify each chain has independent storage
        let get1 = conn.get_waiting_txn(chain_id1, signer, nonce).await.unwrap();
        let get2 = conn.get_waiting_txn(chain_id2, signer, nonce).await.unwrap();

        assert_eq!(get1, Some(hex::encode(raw_txn1.clone())), "Transaction on chain 1 incorrect");
        assert_eq!(get2, Some(hex::encode(raw_txn2.clone())), "Transaction on chain 2 incorrect");
    }

    async fn test_delete_nonexistent_key(mut conn: MultiplexedConnection) {
        let chain_id = 12u64;
        let wallet_address = Address::from_slice(&[0x9F; 20]);
        let nonce = 999u64;

        // Delete a key that doesn't exist
        let txn_ids = vec![WaitingTransactionId { chain_id, wallet_address, nonce }];
        let del_result = conn.del_waiting_txn_keys(&txn_ids).await.unwrap();

        // Redis returns 0 when no keys were deleted
        assert_eq!(del_result, 0, "Should return 0 when deleting non-existent key");

        let get_result = conn.get_waiting_txn(chain_id, wallet_address, nonce).await.unwrap();
        assert!(get_result.is_none(), "Should return none after deleting non-existent key");
    }
}
