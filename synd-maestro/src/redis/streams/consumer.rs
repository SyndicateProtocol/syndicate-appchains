//!
//! This module provides the consumer implementation for Redis streams used to queue
//! and process transactions across different chains.

use crate::redis::streams::producer::tx_stream_key;
use alloy::primitives::ChainId;
use redis::{
    aio::MultiplexedConnection,
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands,
};
use std::time::Duration;

/// A consumer for Redis streams that reads transaction data.
///
/// This consumer reads from a Redis stream using the XREAD command, maintaining its position
/// in the stream using the last processed message ID. It's designed to work with transaction
/// data that has been enqueued by a `StreamProducer`.
///
/// # Example
/// ```no_compile
/// use redis::aio::MultiplexedConnection;
/// use synd-maestro::redis::consumer::StreamConsumer;
///
/// let conn: MultiplexedConnection = // ... get Redis connection
/// let chain_id = 1;
/// let start_from_id = "0-0";
/// let mut consumer = StreamConsumer::new(conn, chain_id, start_from_id);
///
/// // Read next transaction
/// if let Some((tx_data, msg_id)) = consumer.recv().await? {
///     // Process transaction data
/// }
/// ```
#[derive(Debug)]
pub struct StreamConsumer {
    conn: MultiplexedConnection,
    stream_key: String,
    last_id: String,
}

impl StreamConsumer {
    /// Creates a new `StreamConsumer` for a specific chain.
    ///
    /// # Arguments
    /// * `conn` - A Redis multiplexed connection to use for reading from the stream
    /// * `chain_id` - The ID of the chain to consume transactions for
    /// * `start_from_id` - The ID of the message to start reading from ("0-0" to start from the
    ///   beginning)
    /// # Returns
    /// A new `StreamConsumer` instance configured to read from the stream for the given chain.
    pub fn new(conn: MultiplexedConnection, chain_id: ChainId, start_from_id: String) -> Self {
        // NOTE maybe we don't need MultiplexedConnection here (unless we want to have multiple
        // consumers per service)
        Self { conn, stream_key: tx_stream_key(chain_id), last_id: start_from_id }
    }

    /// Receives the next transaction from the stream.
    ///
    /// This method reads one message at a time and updates the internal `last_id` to track the
    /// position in the stream.
    ///
    /// # Arguments
    /// * `number_of_messages` - The number of messages to read from the stream
    ///
    /// # Returns
    /// * `Ok(Some((Vec<u8>, String)))` - The raw transaction data and the last ID if a new message
    ///   was received
    /// * `Ok([])` - If no new messages are available
    /// * `Err(_)` - If there was an error reading from the stream or parsing the message
    ///
    /// # Notes
    /// * Currently reads one message at a time for simplicity. For high-throughput scenarios,
    ///   consider implementing batch reading with an internal buffer.
    /// * Blocks for `block_duration` waiting for a new message to arrive
    pub async fn recv(
        &mut self,
        max_msg_count: usize,
        block_duration: Duration,
    ) -> eyre::Result<Vec<(Vec<u8>, String)>> {
        let opts = StreamReadOptions::default()
            .block(block_duration.as_millis() as usize)
            .count(max_msg_count);

        let reply: Option<StreamReadReply> = self
            .conn
            .xread_options(&[self.stream_key.as_str()], &[self.last_id.as_str()], &opts)
            .await?;

        let reply = match reply {
            Some(r) => r,
            None => return Ok(vec![]),
        };

        let key_data = reply
            .keys
            .first()
            .ok_or_else(|| eyre::eyre!("Expected at least one stream key in reply"))?;

        if key_data.ids.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::with_capacity(key_data.ids.len());

        for id in &key_data.ids {
            let raw_tx =
                id.map.get("data").ok_or_else(|| eyre::eyre!("No data found in message"))?;

            match raw_tx {
                redis::Value::BulkString(data) => {
                    results.push((data.clone(), id.id.clone()));
                }
                _ => return Err(eyre::eyre!("Expected binary data, got different type")),
            }
        }

        if let Some(last_id) = reply.keys[0].ids.last() {
            self.last_id = last_id.id.clone();
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::redis::streams::producer::{CheckFinalizationResult, StreamProducer};
    use std::time::Duration;
    use test_utils::docker::start_redis;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_produce_consume_transaction() {
        // Start Redis container
        let (_redis, redis_url) = start_redis().await.unwrap();

        // Connect to Redis
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        // Setup test data
        let chain_id = 1;
        let test_data = b"test transaction data".to_vec();

        // Create producer and consumer
        let producer = StreamProducer::new(
            conn.clone(),
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
        )
        .await;
        let mut consumer = StreamConsumer::new(conn, chain_id, "0-0".to_string());

        // Send transaction
        producer.enqueue_transaction(&test_data).await.unwrap();

        // Receive and verify
        let received = consumer.recv(1, Duration::from_secs(1)).await.unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].0, test_data);
        assert!(!received[0].1.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_produce_consume_multiple_transactions() {
        // Start Redis container
        let (_redis, redis_url) = start_redis().await.unwrap();

        // Connect to Redis
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        // Setup test data
        let chain_id = 1;
        let test_data1 = b"test transaction data 1".to_vec();
        let test_data2 = b"test transaction data 2".to_vec();

        // Create producer and consumer
        let producer = StreamProducer::new(
            conn.clone(),
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
        )
        .await;
        let mut consumer = StreamConsumer::new(conn, chain_id, "0-0".to_string());

        // Send transactions
        producer.enqueue_transaction(&test_data1).await.unwrap();
        producer.enqueue_transaction(&test_data2).await.unwrap();

        // Receive and verify
        let received = consumer.recv(2, Duration::from_secs(1)).await.unwrap();
        assert_eq!(received.len(), 2);
        assert_eq!(received[0].0, test_data1);
        assert_eq!(received[1].0, test_data2);
        assert!(!received[0].1.is_empty());
        assert!(!received[1].1.is_empty());
    }
}
