//! This module provides the producer implementation for Redis streams used to queue
//! and process transactions across different chains.

use redis::{
    aio::MultiplexedConnection,
    streams::{StreamTrimOptions, StreamTrimmingMode},
    AsyncCommands, RedisError,
};
use std::time::Duration;
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tracing::debug;

/// Base key for Redis transaction streams
/// Format: `synd-maestro:transactions:{chain_id}`
const TX_STREAM_KEY: &str = "maestro:transactions";

/// Generates a Redis stream key for a specific chain
///
/// # Arguments
/// * `chain_id` - The chain identifier to create the stream key for
///
/// # Returns
/// A string in the format `synd-maestro:transactions:{chain_id}`
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
/// use redis::aio::MultiplexedConnection;
/// use std::time::Duration;
/// use synd_maestro::redis::streams::producer::StreamProducer;
///
/// async fn example(redis_conn: MultiplexedConnection) {
///     let mut producer = StreamProducer::new(
///         redis_conn,
///         1,
///         Duration::from_secs(60 * 60 * 24),
///         Duration::from_secs(60 * 60 * 24),
///     );
///     let tx_data = vec![0x01, 0x02, 0x03];
///     let id = producer.enqueue_transaction(tx_data).await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct StreamProducer {
    conn: MultiplexedConnection,
    pub(crate) stream_key: String,
    _prune_handle: JoinHandle<()>, // handle to keep the pruning task running
}

impl StreamProducer {
    /// Creates a new producer instance for a specific chain
    ///
    /// # Arguments
    /// * `conn` - An established Redis connection
    /// * `chain_id` - The chain identifier this producer will write to
    /// * `prune_interval` - How often to prune old entries
    /// * `prune_max_age` - Maximum age of entries to keep
    ///
    /// # Returns
    /// A new `StreamProducer` instance configured for the specified chain
    pub fn new(
        conn: MultiplexedConnection,
        chain_id: u64,
        prune_interval: Duration,
        prune_max_age: Duration,
    ) -> Self {
        let stream_key = tx_stream_key(chain_id);
        let _prune_handle =
            start_pruning(conn.clone(), stream_key.clone(), prune_interval, prune_max_age);
        Self { conn, stream_key, _prune_handle }
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
    pub async fn enqueue_transaction(&self, raw_tx: Vec<u8>) -> Result<String, RedisError> {
        let id: String = self.conn.clone().xadd(&self.stream_key, "*", &[("data", raw_tx)]).await?;
        debug!(%self.stream_key, %id, "Enqueued transaction");
        Ok(id)
    }
}

/// Starts a background task to prune old entries from the Redis stream
/// every `interval` prunes the stream for entries older than `max_age`
fn start_pruning(
    mut conn: MultiplexedConnection,
    stream_key: String,
    interval: Duration,
    max_age: Duration,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval_timer = tokio::time::interval(interval);
        interval_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            interval_timer.tick().await;

            #[allow(clippy::unwrap_used)] // safe to unwrap
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let minid = format!("{}-0", current_time - max_age.as_millis());

            if let Err(e) = conn
                .xtrim_options::<&String, String>(
                    &stream_key,
                    &StreamTrimOptions::minid(StreamTrimmingMode::Exact, minid),
                )
                .await
            {
                panic!("Failed to prune stream: {}", e);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::docker::start_redis;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_stream_pruning() {
        // Setup Redis connection
        let (_redis, redis_url) = start_redis().await.unwrap();

        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let conn = client.get_multiplexed_async_connection().await.unwrap();

        // Create test stream
        let stream_key = "test:pruning:stream";
        let mut conn_clone = conn.clone();

        // Add some test data
        for i in 0..5 {
            let _: String =
                conn_clone.xadd(stream_key, "*", &[("data", format!("test-{}", i))]).await.unwrap();
        }

        // Start pruning with short intervals for testing
        let prune_interval = Duration::from_millis(100);
        let max_age = Duration::from_millis(50);
        let _prune_handle =
            start_pruning(conn.clone(), stream_key.to_string(), prune_interval, max_age);

        // Wait for pruning to occur
        sleep(Duration::from_millis(200)).await;

        // Verify stream length
        let stream_len: usize = conn_clone.xlen(stream_key).await.unwrap();
        assert!(stream_len < 5, "Stream was not pruned");
    }
}
