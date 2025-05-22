//! This module provides the producer implementation for Redis streams used to queue
//! and process transactions across different chains.

use alloy::primitives::keccak256;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use std::{self, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle, time::MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, trace};

/// Base key for Redis transaction streams
/// Format: `synd-maestro:transactions:{chain_id}`
const TX_STREAM_KEY: &str = "synd-maestro:transactions";

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
/// use synd_maestro::redis::streams::producer::{CheckFinalizationResult, StreamProducer};
///
/// async fn example(redis_conn: MultiplexedConnection) {
///     let mut producer = StreamProducer::new(
///         redis_conn,
///         1,
///         Duration::from_secs(60 * 60 * 24),
///         Duration::from_secs(60 * 60 * 24),
///         5, // max_transaction_retries
///         |_raw_tx: &[u8]| async { CheckFinalizationResult::Done },
///     );
///     let tx_data = vec![0x01, 0x02, 0x03];
///     let id = producer.enqueue_transaction(&tx_data).await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct StreamProducer {
    conn: MultiplexedConnection,
    pub(crate) stream_key: String,
    shutdown_token: CancellationToken,
    finalization_task_handle: Mutex<Option<JoinHandle<()>>>,
}

impl StreamProducer {
    /// Creates a new producer instance for a specific chain
    ///
    /// # Arguments
    /// * `conn` - An established Redis connection
    /// * `chain_id` - The chain identifier this producer will write to
    /// * `finalization_checker_interval` - How often to check for finalized transactions
    /// * `finalization_duration` - Duration after which a transaction is considered finalized
    /// * `max_transaction_retries` - Maximum number of times a transaction can be re-submitted
    /// * `handle_finalized_tx` - Callback to check if a transaction is finalized
    ///
    /// # Returns
    /// A new `StreamProducer` instance configured for the specified chain
    pub fn new<F, Fut>(
        conn: MultiplexedConnection,
        chain_id: u64,
        finalization_checker_interval: Duration,
        finalization_duration: Duration,
        max_transaction_retries: u32,
        handle_finalized_tx: F,
    ) -> Self
    where
        F: Fn(&[u8]) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = CheckFinalizationResult> + Send + 'static,
    {
        let stream_key = tx_stream_key(chain_id);
        let shutdown_token = CancellationToken::new();

        let handle = start_finalization_checker(
            conn.clone(),
            stream_key.clone(),
            finalization_checker_interval,
            finalization_duration,
            max_transaction_retries,
            handle_finalized_tx,
            shutdown_token.clone(),
        );
        Self {
            conn,
            stream_key,
            shutdown_token,
            finalization_task_handle: Mutex::new(Some(handle)),
        }
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
        Self::add_to_stream(&self.conn, &self.stream_key, raw_tx, 0).await
    }

    async fn add_to_stream(
        conn: &MultiplexedConnection,
        stream_key: &str,
        raw_tx: &[u8],
        retries: u32,
    ) -> Result<String, RedisError> {
        let id: String = conn
            .clone()
            .xadd(stream_key, "*", &[("data", raw_tx), ("retries", retries.to_string().as_bytes())])
            .await?;
        debug!(%stream_key, %id, "Enqueued transaction");
        Ok(id)
    }

    /// Get all transactions that are older than the finalization duration, deletes them from the
    /// stream and returns them
    async fn get_finalized_transactions(
        conn: &mut MultiplexedConnection,
        stream_key: &str,
        finalization_ms: u128,
    ) -> Result<Vec<(String, Vec<(String, Vec<u8>)>)>, RedisError> {
        #[allow(clippy::unwrap_used)] // safe to unwrap
        let current_time =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let min_time = current_time - finalization_ms;
        let max_id = format!("{}-0", min_time);

        // Use XRANGE to get all entries that are older than finalization_duration
        #[allow(clippy::type_complexity)]
        let result: Result<Vec<(String, Vec<(String, Vec<u8>)>)>, RedisError> =
            conn.xrange(stream_key, "-", &max_id).await;

        let entries = match result {
            Ok(entries) => entries,
            Err(e) => {
                error!(%stream_key, %max_id, %e, "Failed to fetch old entries");
                return Err(e);
            }
        };

        if entries.is_empty() {
            return Ok(vec![]);
        }

        // Collect IDs for deletion and call XDEL on all of them
        let ids: Vec<String> = entries.iter().map(|(id, _)| id.clone()).collect();
        if let Err(e) = conn.xdel::<_, _, usize>(&stream_key, &ids).await {
            error!(%stream_key, %max_id, %e, "Failed to delete finalized transaction entries");
            return Err(e);
        }
        trace!(%stream_key, %max_id, count = ids.len(), "Deleted entries");
        Ok(entries)
    }

    /// Shuts down the finalization checker task gracefully.
    ///
    /// Sends a shutdown signal to the checker and waits for it to finish its current loop.
    pub async fn shutdown(&self) {
        debug!(stream_key = %self.stream_key, "Attempting to shut down StreamProducer's finalization checker...");
        self.shutdown_token.cancel();

        let mut handle_guard = self.finalization_task_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            debug!(stream_key = %self.stream_key, "Awaiting finalization checker task to join...");
            if let Err(e) = handle.await {
                error!(stream_key = %self.stream_key, error = %e, "Finalization checker task panicked or was cancelled during join.");
            }
        } else {
            debug!(stream_key = %self.stream_key, "Finalization checker task already awaited or shutdown process was previously completed.");
        }
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

/// Parses the fields from a Redis stream entry.
///
/// # Arguments
/// * `fields` - The fields from the stream entry.
/// * `stream_key` - The key of the Redis stream (for logging purposes).
/// * `id` - The ID of the stream entry (for logging purposes).
///
/// # Returns
/// An `Option` containing a tuple of `(&[u8], u32)` representing the data and retries
/// if both fields are present and valid. Otherwise, returns `None`.
fn parse_stream_entry<'a>(
    fields: &'a [(String, Vec<u8>)],
    stream_key: &str,
    id: &str,
) -> Option<(&'a [u8], u32)> {
    let mut data_field_value: Option<&'a [u8]> = None;
    let mut retries_value: Option<u32> = None;

    for (name, value) in fields {
        match name.as_str() {
            "data" => data_field_value = Some(value),
            "retries" => {
                retries_value =
                    String::from_utf8(value.clone()).ok().and_then(|s| s.parse::<u32>().ok());
            }
            _ => {
                error!(%stream_key, entry_id = %id, "Unexpected field: {}", name);
            }
        }
    }

    match (data_field_value, retries_value) {
        (Some(data), Some(retry_count)) => Some((data, retry_count)),
        _ => {
            error!(%stream_key, entry_id = %id, "Missing required fields: data or retries");
            None
        }
    }
}

/// Starts a background task to check for finalized transactions from the Redis stream
///
/// # Arguments
/// * `conn` - Redis multiplexed connection
/// * `stream_key` - Key of the Redis stream to check
/// * `finalization_checker_interval` - How often to check for old entries
/// * `finalization_duration` - Maximum age of entries to keep
/// * `max_transaction_retries` - Maximum number of times a transaction can be re-submitted
/// * `check_finalization` - Function to call with each entry's raw data, this will decide whether
///   to resubmit the transaction or not
/// * `shutdown_token` - `CancellationToken` for shutting down the task
///
/// # Returns
/// A `JoinHandle` for the spawned finalization checker task
fn start_finalization_checker<F, Fut>(
    mut conn: MultiplexedConnection,
    stream_key: String,
    finalization_checker_interval: Duration,
    finalization_duration: Duration,
    max_transaction_retries: u32,
    check_finalization: F,
    shutdown_token: CancellationToken,
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
            tokio::select! {
                biased;
                _ = shutdown_token.cancelled() => {
                    info!(%stream_key, "Finalization checker: cancellation token activated. Exiting task.");
                    return;
                }
                _ = interval_timer.tick() => {
                    trace!(%stream_key, "Finalization checker: interval tick.");
                }
            }

            let entries = match StreamProducer::get_finalized_transactions(
                &mut conn,
                &stream_key,
                finalization_ms,
            )
            .await
            {
                Ok(entries) => entries,
                Err(e) => {
                    error!(%stream_key, %e, "Failed to fetch old entries");
                    continue;
                }
            };

            // Process each entry with the `check_finalization` callback function
            for (id, fields) in entries {
                let (data, retries) = match parse_stream_entry(&fields, &stream_key, &id) {
                    Some((data, retries)) => (data, retries),
                    None => continue,
                };
                if retries >= max_transaction_retries {
                    let tx_hash = keccak256(data);
                    debug!(%stream_key, entry_id = %id, %tx_hash, %retries, "Max resubmission retries reached. Transaction will not be re-submitted.");
                    continue;
                }

                trace!(%stream_key, entry_id = %id, retries, "Checking tx for finalization");
                match check_finalization(data).await {
                    CheckFinalizationResult::Done => {} // do nothing
                    CheckFinalizationResult::ReSubmit => {
                        if let Err(e) =
                            StreamProducer::add_to_stream(&conn, &stream_key, data, retries + 1)
                                .await
                        {
                            error!(
                                %stream_key,
                                original_id = %id,
                                error = %e,
                                "Failed to resubmit transaction."
                            );
                        }
                    }
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
        // It also tests the max_transaction_retries logic.
        // 1. It populates a Redis stream with 5 initial entries (intended to be "old") and 1
        //    "extra" entry with a far-future ID (intended to remain unprocessed by this test run).
        //    All entries are added with "retries: 0".
        // 2. It starts the finalization checker with a short interval (100ms), finalization
        //    duration (50ms), and max_transaction_retries (e.g., 1).
        // 3. The test waits for 150ms, allowing the checker to run effectively once.
        // 4. In this run, the checker should: a. Identify only the 5 "old" entries. b. The "extra"
        //    entry with the future ID should NOT be selected. c. Delete the 5 selected "old"
        //    entries from the stream via XDEL. d. Invoke the provided callback for each of these 5
        //    "old" entries.
        // 5. The callback logic for the 5 processed entries is conditional:
        //    - For the first 2 invocations, it returns `CheckFinalizationResult::ReSubmit`.
        //    - For the next 3 invocations, it returns `CheckFinalizationResult::Done`.
        // 6. `start_finalization_checker` behavior with `max_transaction_retries = 1`:
        //    - Item 0 (test-0, retries=0): Callback `ReSubmit`. `0 < 1` is true. Re-submitted with
        //      `retries = 1`.
        //    - Item 1 (test-1, retries=0): Callback `ReSubmit`. `0 < 1` is true. Re-submitted with
        //      `retries = 1`.
        //    - Item 2 (test-2, retries=0): Callback `Done`. Not re-submitted.
        //    - Item 3 (test-3, retries=0): Callback `Done`. Not re-submitted.
        //    - Item 4 (test-4, retries=0): Callback `Done`. Not re-submitted.
        // 7. Consequently: a. The `callback_counter` should be incremented 5 times. b. The
        //    `callback_data` should store the data of these 5 invocations. c. The stream should
        //    contain 3 entries at the end:
        //       - The 2 items for which `ReSubmit` was returned and allowed by retry limit
        //         (re-added with new IDs and retries=1).
        //       - The 1 "extra" item (which was never processed or deleted, retries=0).
        //       - The 3 items for which `Done` was returned are not re-added.

        // Setup Redis connection
        let (_redis, redis_url) = start_redis().await.unwrap();

        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let conn = client.get_multiplexed_async_connection().await.unwrap();

        // Create test stream
        let stream_key = "test:finalization:stream";
        let mut conn_clone = conn.clone();

        // Add some test data
        for i in 0..5 {
            let _: String = conn_clone
                .xadd(
                    stream_key,
                    "*",
                    &[("data", format!("test-{}", i).as_bytes()), ("retries", b"0" as &[u8])],
                )
                .await
                .unwrap();
        }
        // add an extra entry that shouldn't be included in the finalization check
        let _: String = conn_clone
            .xadd(
                stream_key,
                "9999999999999-0",
                &[("data", "test-extra".to_string().as_bytes()), ("retries", b"0" as &[u8])],
            )
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
        let max_transaction_retries = 1; // Test with 1 retry allowed

        let shutdown_token = CancellationToken::new();

        let _finalization_checker_handle = start_finalization_checker(
            conn.clone(),
            stream_key.to_string(),
            finalization_checker_interval,
            finalization_duration,
            max_transaction_retries,
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
            shutdown_token.clone(),
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
        // - 2 items ('ReSubmit' from callback for test-0, test-1) are re-added (with retries = 1).
        // - 3 items ('Done' from callback for test-2, test-3, test-4) are deleted and not re-added.
        // - 1 "extra" item (test-extra, with retries = 0) was never processed because its ID is in
        //   the future.
        // Stream length should be 2 (re-submitted) + 1 (extra) = 3.
        assert_eq!(
            stream_len, 3,
            "Stream should contain 2 re-submitted entries (retries=1) and 1 extra, unprocessed entry (retries=0)"
        );
    }

    #[tokio::test]
    async fn test_transaction_max_retries_behavior() {
        // This test verifies that a transaction is retried once when max_transaction_retries is 1,
        // and then dropped on the subsequent processing attempt.
        // 1. Set max_transaction_retries = 1.
        // 2. Add one entry with retries = 0.
        // 3. Callback always returns ReSubmit.
        // 4. After 1st cycle: entry is re-submitted with retries = 1. Callback invoked once.
        // 5. After 2nd cycle: entry with retries = 1 is dropped. Callback not invoked again. Stream
        //    empty.

        let (_redis, redis_url) = start_redis().await.unwrap();
        let client = redis::Client::open(redis_url.as_str()).unwrap();
        let conn = client.get_multiplexed_async_connection().await.unwrap();
        let mut conn_clone_for_setup = conn.clone();
        let mut conn_clone_for_assert = conn.clone();

        let stream_key = "test:max_retries_behavior:stream";
        let max_transaction_retries = 1;
        let item_data = b"single_retry_item_data";

        // Add initial entry
        let _: String = conn_clone_for_setup
            .xadd(stream_key, "*", &[("data", item_data as &[u8]), ("retries", b"0" as &[u8])])
            .await
            .unwrap();

        let callback_invocations = Arc::new(AtomicUsize::new(0));
        let finalization_callback = {
            let counter_clone = Arc::clone(&callback_invocations);
            move |_data: &[u8]| {
                let inner_counter_clone = Arc::clone(&counter_clone);
                async move {
                    inner_counter_clone.fetch_add(1, Ordering::SeqCst);
                    CheckFinalizationResult::ReSubmit
                }
            }
        };

        let finalization_checker_interval = Duration::from_millis(100);
        let finalization_duration = Duration::from_millis(50);

        let shutdown_token = CancellationToken::new();

        let _checker_handle = start_finalization_checker(
            conn.clone(),
            stream_key.to_string(),
            finalization_checker_interval,
            finalization_duration,
            max_transaction_retries,
            finalization_callback,
            shutdown_token.clone(),
        );

        // Wait for the first finalization cycle
        sleep(Duration::from_millis(150)).await; // interval (100) + duration (50)

        // Assertions after 1st cycle
        assert_eq!(
            callback_invocations.load(Ordering::SeqCst),
            1,
            "Callback should be invoked once after the first cycle"
        );

        #[allow(clippy::type_complexity)]
        let stream_entries_after_1st_cycle: Vec<(String, Vec<(String, Vec<u8>)>)> =
            conn_clone_for_assert.xrange(stream_key, "-", "+").await.unwrap();
        assert_eq!(
            stream_entries_after_1st_cycle.len(),
            1,
            "Stream should contain 1 entry after the first cycle"
        );

        let (_, fields) = &stream_entries_after_1st_cycle[0];
        let mut found_data: Option<&[u8]> = None;
        let mut found_retries: Option<String> = None;
        for (name, value) in fields {
            match name.as_str() {
                "data" => found_data = Some(value),
                "retries" => found_retries = Some(String::from_utf8(value.clone()).unwrap()),
                _ => {}
            }
        }
        assert_eq!(found_data.unwrap(), item_data, "Data mismatch after 1st cycle");
        assert_eq!(found_retries.unwrap(), "1", "Retries should be 1 after 1st cycle");

        // Wait for the second finalization cycle
        sleep(Duration::from_millis(150)).await; // interval (100) + duration (50)

        // Assertions after 2nd cycle
        assert_eq!(
            callback_invocations.load(Ordering::SeqCst),
            1,
            "Callback should still be invoked only once after the second cycle"
        );

        let stream_len_after_2nd_cycle: usize =
            conn_clone_for_assert.xlen(stream_key).await.unwrap();
        assert_eq!(stream_len_after_2nd_cycle, 0, "Stream should be empty after the second cycle");
    }
}
