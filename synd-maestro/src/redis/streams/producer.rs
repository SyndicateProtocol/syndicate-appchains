//! This module provides the producer implementation for Redis streams used to queue
//! and process transactions across different chains.

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use std::{self, time::Duration};
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tracing::{debug, error};

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
    _finalization_checker_handle: JoinHandle<()>,
}

impl StreamProducer {
    /// Creates a new producer instance for a specific chain
    ///
    /// # Arguments
    /// * `conn` - An established Redis connection
    /// * `chain_id` - The chain identifier this producer will write to
    /// * `finalization_checker_interval` - How often to check for finalized transactions
    /// * `finalization_duration` - Duration after which a transaction is considered finalized
    ///
    /// # Returns
    /// A new `StreamProducer` instance configured for the specified chain
    pub fn new<F, Fut>(
        conn: MultiplexedConnection,
        chain_id: u64,
        finalization_checker_interval: Duration,
        finalization_duration: Duration,
        handle_finalized_tx: F,
    ) -> Self
    where
        F: Fn(&[u8]) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = CheckFinalizationResult> + Send + 'static,
    {
        let stream_key = tx_stream_key(chain_id);
        let _finalization_checker_handle = start_finalization_checker(
            conn.clone(),
            stream_key.clone(),
            finalization_checker_interval,
            finalization_duration,
            handle_finalized_tx,
        );
        Self { conn, stream_key, _finalization_checker_handle }
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
    pub async fn enqueue_transaction(&self, raw_tx: &[u8]) -> Result<String, RedisError> {
        let id: String = self.conn.clone().xadd(&self.stream_key, "*", &[("data", raw_tx)]).await?;
        debug!(%self.stream_key, %id, "Enqueued transaction");
        Ok(id)
    }
}

/// Result of the finalization check
#[derive(Debug, PartialEq, Eq)]
pub enum CheckFinalizationResult {
    /// Re-submit the transaction to the stream
    ReSubmit,
    /// Done processing the transaction, will not be re-submitted
    Done,
}

/// Starts a background task to check for finalized transactions from the Redis stream
///
/// # Arguments
/// * `conn` - Redis multiplexed connection
/// * `stream_key` - Key of the Redis stream to check
/// * `finalization_checker_interval` - How often to check for old entries
/// * `finalization_duration` - Maximum age of entries to keep
/// * `check_finalization` - Function to call with each entry's raw data, this will decide whether
///   to resubmit the transaction or not
///
/// # Returns
/// A `JoinHandle` for the spawned finalization checker task
fn start_finalization_checker<F, Fut>(
    mut conn: MultiplexedConnection,
    stream_key: String,
    finalization_checker_interval: Duration,
    finalization_duration: Duration,
    check_finalization: F,
) -> JoinHandle<()>
where
    F: Fn(&[u8]) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = CheckFinalizationResult> + Send + 'static,
{
    let finalization_ms = finalization_duration.as_millis();
    tokio::spawn(async move {
        let mut interval_timer = tokio::time::interval(finalization_checker_interval);
        interval_timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            interval_timer.tick().await;

            #[allow(clippy::unwrap_used)] // safe to unwrap
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let min_time = current_time - finalization_ms;
            let max_id = format!("{}-0", min_time);

            // Use XRANGE to get all entries that are older than finalization_duration
            #[allow(clippy::type_complexity)]
            let result: Result<Vec<(String, Vec<(String, Vec<u8>)>)>, RedisError> =
                conn.xrange(&stream_key, "-", &max_id).await;

            if let Err(e) = result {
                debug!(%stream_key, %e, "Failed to fetch old entries");
                continue;
            }

            let entries = match result {
                Ok(entries) => entries,
                Err(e) => {
                    error!(%stream_key, %e, "Failed to fetch old entries");
                    continue;
                }
            };
            if entries.is_empty() {
                continue;
            }

            // Collect IDs for deletion and call XDEL on all of them
            let ids: Vec<String> = entries.iter().map(|(id, _)| id.clone()).collect();
            match conn.xdel::<_, _, usize>(&stream_key, &ids).await {
                Ok(count) => debug!(%stream_key, count, "Deleted processed entries"),
                Err(e) => {
                    debug!(%stream_key, %e, "Failed to delete processed entries");
                    continue
                }
            }

            // Process each entry with the callback function
            for (id, fields) in &entries {
                // Find and process the "data" field
                if let Some(field_value) =
                    fields.iter().find_map(|(name, value)| (name == "data").then_some(value))
                {
                    debug!(%stream_key, %id, "Processing old entry before deletion");
                    match check_finalization(field_value).await {
                        CheckFinalizationResult::Done => {
                            debug!(%stream_key, %id, "Transaction finalized")
                        }
                        CheckFinalizationResult::ReSubmit => {
                            let resubmission_id = conn
                                .clone()
                                .xadd(&stream_key, "*", &[("data", field_value)])
                                .await
                                .unwrap_or_else(|e| {
                                    error!(%stream_key, %id, %e, "Failed to resubmit transaction");
                                    "failed-to-resubmit".to_string()
                                });
                            debug!(%stream_key, %resubmission_id, "Transaction not finalized, re-submitting with id {}", resubmission_id);
                        }
                    };
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use test_utils::docker::start_redis;
    use tokio::{
        sync::Mutex,
        time::{sleep, Duration},
    };

    #[tokio::test]
    async fn test_stream_finalization_checker() {
        // This test verifies the behavior of the `start_finalization_checker` with conditional
        // logic in the callback and ensures items with future-dated IDs are not processed.
        // 1. It populates a Redis stream with 5 initial entries (intended to be "old") and 1
        //    "extra" entry with a far-future ID (intended to remain unprocessed by this test run).
        // 2. It starts the finalization checker with a short interval (100ms) and finalization
        //    duration (50ms).
        // 3. The test waits for 150ms, allowing the checker to run effectively once.
        // 4. In this run, the checker should: a. Identify only the 5 "old" entries (those with IDs
        //    older than `now - 50ms`). b. The "extra" entry with the future ID should NOT be
        //    selected for processing. c. Delete the 5 selected "old" entries from the stream. d.
        //    Invoke the provided callback for each of these 5 "old" entries.
        // 5. The callback logic for the 5 processed entries is conditional:
        //    - For the first 2 invocations, it returns `CheckFinalizationResult::ReSubmit`.
        //    - For the next 3 invocations, it returns `CheckFinalizationResult::Done`.
        // 6. `start_finalization_checker` implementation details for callback results:
        //    - `Done` arm: Does NOT re-submit the item. Item remains deleted from its original ID.
        //    - `ReSubmit` arm: DOES re-submit the item via XADD (it gets a new ID).
        // 7. Consequently: a. The `callback_counter` should be incremented 5 times (only the "old"
        //    items processed). b. The `callback_data` should store the data of these 5 invocations.
        //    c. The stream should contain 3 entries at the end:
        //       - The 2 items for which `ReSubmit` was returned (re-added with new IDs).
        //       - The 1 "extra" item (which was never processed or deleted).
        //       - The 3 items for which `Done` was returned will not be in the stream from their
        //         original IDs.

        // Setup Redis connection
        let (_redis, redis_url) = start_redis().await.unwrap();

        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let conn = client.get_multiplexed_async_connection().await.unwrap();

        // Create test stream
        let stream_key = "test:finalization:stream";
        let mut conn_clone = conn.clone();

        // Add some test data
        for i in 0..5 {
            let _: String =
                conn_clone.xadd(stream_key, "*", &[("data", format!("test-{}", i))]).await.unwrap();
        }
        // add an extra entry that shouldn't be included in the finalization check
        let _: String = conn_clone
            .xadd(stream_key, "9999999999999-0", &[("data", "test-extra".to_string())])
            .await
            .unwrap();

        // Create a shared counter to track callback invocations
        let callback_counter = Arc::new(AtomicUsize::new(0));
        let callback_data = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));

        // Clone Arcs for the test to retain ownership for assertions
        let assert_callback_counter = Arc::clone(&callback_counter);
        let assert_callback_data = Arc::clone(&callback_data);

        // Start finalization checker with short intervals for testing
        let finalization_checker_interval = Duration::from_millis(100);
        let finalization_duration = Duration::from_millis(50);

        let _finalization_checker_handle = start_finalization_checker(
            conn.clone(),
            stream_key.to_string(),
            finalization_checker_interval,
            finalization_duration,
            move |data: &[u8]| {
                // Create static reference to avoid lifetime issues
                let data = data.to_vec();
                let callback_counter = Arc::clone(&callback_counter);
                let data_store = Arc::clone(&callback_data);

                async move {
                    // Important: Read the counter *before* incrementing for the current decision
                    let current_invocation_index = callback_counter.load(Ordering::SeqCst);
                    callback_counter.fetch_add(1, Ordering::SeqCst);

                    // Lock the mutex asynchronously
                    let mut callback_storage = data_store.lock().await;
                    callback_storage.push(data.clone());

                    // Assuming items are processed in order of addition (test-0, test-1, ...)
                    // First 2 calls return ReSubmit, next 3 return Done
                    if current_invocation_index < 2 {
                        CheckFinalizationResult::ReSubmit
                    } else {
                        CheckFinalizationResult::Done
                    }
                }
            },
        );

        // Wait for finalization checking to occur
        sleep(Duration::from_millis(150)).await;

        // Assert that the callback was invoked 5 times (5 items * 1 run)
        assert_eq!(
            assert_callback_counter.load(Ordering::SeqCst),
            5,
            "Callback should be invoked 5 times"
        );

        // Assert that the callback received the correct data
        let data_guard = assert_callback_data.lock().await;
        assert_eq!(data_guard.len(), 5, "Callback should have collected 5 data items");
        for i in 0..5 {
            let expected_data = format!("test-{}", i).into_bytes();
            assert_eq!(data_guard[i], expected_data, "Mismatch in collected data for item {}", i);
        }
        drop(data_guard);

        // Verify stream length
        let stream_len: usize = conn_clone.xlen(stream_key).await.unwrap();
        // After one run:
        // - 5 items processed (test-0 to test-4).
        // - 2 items ('ReSubmit' from callback for test-0, test-1) are re-added.
        // - 3 items ('Done' from callback for test-2, test-3, test-4) are deleted and not re-added.
        // - 1 "extra" item (test-extra) was never processed because its ID is in the future.
        // Stream length should be 2 (re-submitted) + 1 (extra) = 3.
        assert_eq!(
            stream_len, 3,
            "Stream should contain 2 re-submitted entries and 1 extra, unprocessed entry"
        );
    }
}
