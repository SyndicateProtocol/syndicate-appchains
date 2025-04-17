//!
//! This module provides the consumer implementation for Redis streams used to queue
//! and process transactions across different chains.

use super::producer::tx_stream_key;
use redis::{
    aio::MultiplexedConnection,
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands,
};

/// A consumer for Redis streams that reads transaction data.
///
/// This consumer reads from a Redis stream using the XREAD command, maintaining its position
/// in the stream using the last processed message ID. It's designed to work with transaction
/// data that has been enqueued by a `StreamProducer`.
///
/// # Example
/// ```no_compile
/// use redis::aio::MultiplexedConnection;
/// use maestro::redis::consumer::StreamConsumer;
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
    pub fn new(conn: MultiplexedConnection, chain_id: u64, start_from_id: String) -> Self {
        // NOTE maybe we don't need MultiplexedConnection here (unless we want to have multiple
        // consumers per service)
        Self { conn, stream_key: tx_stream_key(chain_id), last_id: start_from_id }
    }

    /// Receives the next transaction from the stream.
    ///
    /// This method blocks until a new transaction is available in the stream. It reads one
    /// message at a time and updates the internal `last_id` to track the position in the stream.
    ///
    /// # Arguments
    /// * `number_of_messages` - The number of messages to read from the stream
    ///
    /// # Returns
    /// * `Ok(Some((Vec<u8>, String)))` - The raw transaction data and the last ID if a new message
    ///   was received
    /// * `Ok(None)` - If no new messages are available (should not happen due to blocking)
    /// * `Err(_)` - If there was an error reading from the stream or parsing the message
    ///
    /// # Notes
    /// * Currently reads one message at a time for simplicity. For high-throughput scenarios,
    ///   consider implementing batch reading with an internal buffer.
    /// * Blocks indefinitely until a new message arrives (block=0)
    pub async fn recv(&mut self, max_msg_count: usize) -> eyre::Result<Option<(Vec<u8>, String)>> {
        let opts = StreamReadOptions::default()
            .block(0) //block indefinitely, until we get a new msg
            .count(max_msg_count);

        let reply: Option<StreamReadReply> = self
            .conn
            .xread_options(&[self.stream_key.as_str()], &[self.last_id.as_str()], &opts)
            .await?;
        match reply {
            Some(reply) => {
                self.last_id = reply.keys[0].ids[0].id.clone();
                let raw_tx = reply.keys[0].ids[0].map.get("data");

                match raw_tx {
                    Some(redis::Value::BulkString(data)) => {
                        Ok(Some((data.clone(), self.last_id.clone())))
                    }
                    Some(_) => Err(eyre::eyre!("Expected binary data, got different type")),
                    None => Err(eyre::eyre!("No data found in message")),
                }
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::redis::producer::StreamProducer;
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
        let mut producer = StreamProducer::new(conn.clone(), chain_id);
        let mut consumer = StreamConsumer::new(conn, chain_id, "0-0".to_string());

        // Send transaction
        producer.enqueue_transaction(test_data.clone()).await.unwrap();

        // Receive and verify
        let received = consumer.recv(1).await.unwrap().unwrap();
        assert_eq!(received.0, test_data);
        assert!(!received.1.is_empty());
    }
}
