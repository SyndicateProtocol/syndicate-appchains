//! The `test_utils` module contains test utilities used across the repo.

#![allow(clippy::unwrap_used)] // These functions are used in tests only

use eyre::{eyre, Result};
use std::{
    fs,
    future::Future,
    hash::{DefaultHasher, Hash, Hasher},
    panic, thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

/// Returns a unique temporary path for tests.
///
/// The path is constructed by:
/// 1. Getting the caller's source location (file and line)
/// 2. Appending the current timestamp in nanoseconds and thread ID
/// 3. Hashing the combined string
/// 4. Creating a path in the system temp directory with format `"{prefix}_{hash}"`
///
/// This ensures unique paths for concurrent tests by including both the test location
/// and thread ID for debugging.
pub fn test_path(prefix: &str) -> String {
    let location = panic::Location::caller();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let thread_id = thread::current().id();

    let mut hasher = DefaultHasher::new();
    format!("{}:{}:{:?}", location, timestamp, thread_id).hash(&mut hasher);
    let hash = hasher.finish();

    let dir =
        std::env::temp_dir().join(format!("{}_{:x}", prefix, hash)).to_str().unwrap().to_string();
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Repeatedly checks if a condition becomes true within a timeout period.
///
/// This function is useful for tests that need to wait for asynchronous operations
/// to complete, replacing explicit sleep calls with a more robust mechanism.
///
/// Uses a default check interval of 50ms.
///
/// # Arguments
///
/// * `check` - A closure that returns a future resolving to Result<bool, E>. When it returns
///   Ok(true), the assertion is considered successful.
/// * `timeout` - Maximum duration to wait for the condition to become true.
///
/// # Returns
///
/// * `Result<(), String>` - Ok(()) if the condition becomes true within the timeout period.
///
/// # Example
///
/// ```
/// async fn example() -> eyre::Result<()> {
///     use std::time::Duration;
///     use test_utils::utils::assert_eventually;
///
///     let counter = tokio::sync::Mutex::new(0);
///     assert_eventually(
///         || async {
///             *counter.lock().await += 1;
///             Ok(*counter.lock().await >= 3)
///         },
///         Duration::from_secs(1),
///     )
///     .await
/// }
/// ```
pub async fn assert_eventually<F, Fut>(mut check: F, timeout: Duration) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<bool>>,
{
    const DEFAULT_CHECK_INTERVAL: Duration = Duration::from_millis(50);
    let start = Instant::now();

    while start.elapsed() < timeout {
        match check().await {
            Ok(true) => return Ok(()),
            Ok(false) => (),
            Err(e) => return Err(eyre!("Check function failed with error: {:?}", e)),
        }

        // Sleep before the next check
        tokio::time::sleep(DEFAULT_CHECK_INTERVAL).await;
    }

    Err(eyre!("Condition not satisfied within {:?}", timeout))
}

/// A macro for waiting until a condition becomes true or times out.
///
/// This is a more ergonomic wrapper around the `assert_eventually` function
/// that preserves the original call site in the stack trace.
///
/// # Arguments
///
/// * An expression to evaluate repeatedly - must evaluate to a boolean
/// * A timeout duration
///
/// # Example
///
/// ```
/// async fn example() {
///     use std::time::Duration;
///     use test_utils::{utils::assert_eventually, wait_until};
///
///     let counter = tokio::sync::Mutex::new(0);
///
///     // Basic usage
///     wait_until!(*counter.lock().await += 1; *counter.lock().await >= 3, Duration::from_secs(1));
/// }
/// ```
#[macro_export]
macro_rules! wait_until {    // With setup code
    ($setup:stmt; $condition:expr, $timeout:expr) => {{
        let wait_result = assert_eventually(
            || async {
                $setup;
                Ok($condition)
            },
            $timeout
        ).await;

        if wait_result.is_err() {
            panic!("Timed out waiting for condition");
        }
    }};

    // Without setup, just condition
    ($condition:expr, $timeout:expr) => {
        $crate::wait_until!({}; $condition, $timeout)
    };
}
