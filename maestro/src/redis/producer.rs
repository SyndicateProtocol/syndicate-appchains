//!
//! This module provides the producer implementation for Redis streams used to queue
//! and process transactions across different chains.

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use tracing::debug;

/// Base key for Redis transaction streams
/// Format: `maestro:transactions:{chain_id}`
const TX_STREAM_KEY: &str = "maestro:transactions";

/// Generates a Redis stream key for a specific chain
///
/// # Arguments
/// * `chain_id` - The chain identifier to create the stream key for
///
/// # Returns
/// A string in the format `maestro:transactions:{chain_id}`
pub fn tx_stream_key(chain_id: u64) -> String {
    format!("{}:{}", TX_STREAM_KEY, chain_id)
}

/// Redis Stream producer for enqueueing transactions to a specific chain's stream
///
/// This producer handles writing raw transaction data to Redis streams, which are used
/// for reliable message queuing. Each chain has its own dedicated stream identified
/// by the chain ID.
///
/// # Examples
/// ```rust
/// use maestro::redis::producer::StreamProducer;
/// use redis::aio::MultiplexedConnection;
///
/// async fn example(redis_conn: MultiplexedConnection) {
///     let mut producer = StreamProducer::new(redis_conn, 1);
///     let tx_data = vec![0x01, 0x02, 0x03];
///     let id = producer.enqueue_transaction(tx_data).await.unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct StreamProducer {
    conn: MultiplexedConnection,
    stream_key: String,
}

impl StreamProducer {
    /// Creates a new producer instance for a specific chain
    ///
    /// # Arguments
    /// * `conn` - An established Redis connection
    /// * `chain_id` - The chain identifier this producer will write to
    ///
    /// # Returns
    /// A new `StreamProducer` instance configured for the specified chain
    pub fn new(conn: MultiplexedConnection, chain_id: u64) -> Self {
        Self { conn, stream_key: tx_stream_key(chain_id) }
    }

    /// Enqueues a raw transaction to the Redis stream
    ///
    /// This method writes the transaction data to the Redis stream and returns
    /// the generated stream entry ID. The ID is in the format `{timestamp}-{sequence}`.
    ///
    /// # Arguments
    /// * `raw_tx` - The raw transaction bytes to enqueue
    ///
    /// # Returns
    /// * `Ok(String)` - The stream entry ID if successful
    /// * `Err(Error)` - If the Redis operation fails
    ///
    /// # Errors
    /// Returns an error if:
    /// * Redis connection fails
    /// * Stream write operation fails
    /// * Connection is dropped
    pub async fn enqueue_transaction(&mut self, raw_tx: Vec<u8>) -> Result<String, RedisError> {
        let id: String = self.conn.xadd(&self.stream_key, "*", &[("data", raw_tx)]).await?;
        debug!("Enqueued transaction with ID: {}", id);
        Ok(id)
    }
}
