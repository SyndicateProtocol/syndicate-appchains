//! The `test_utils` module contains test utilities used across the repo.

#![allow(clippy::unwrap_used)] // These functions are used in tests only

use alloy::primitives::keccak256;
use std::{
    fs, panic, thread,
    time::{SystemTime, UNIX_EPOCH},
};

/// Returns a unique temporary path for tests.
///
/// The path is constructed by:
/// 1. Getting the caller's source location (file and line)
/// 2. Appending the current timestamp in nanoseconds, process ID, and thread ID
/// 3. Hashing the combined string
/// 4. Creating a path in the system temp directory with format `"{prefix}_{hash}"`
///
/// This ensures unique paths for concurrent tests by including both the test location,
/// process ID, and thread ID for debugging.
pub fn test_path(prefix: &str) -> String {
    let location = panic::Location::caller();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let thread_id = thread::current().id();
    let process_id = std::process::id();

    let input = format!("{location}:{timestamp}:{process_id}:{thread_id:?}");
    let hash = keccak256(input.as_bytes());
    let hash_hex = alloy::hex::encode(hash);

    let dir =
        std::env::temp_dir().join(format!("{prefix}_{hash_hex}")).to_str().unwrap().to_string();
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A macro for waiting until a condition becomes true or times out.
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
///     use test_utils::wait_until;
///
///     let mut counter = 0;
///
///     // Basic usage
///     wait_until!(counter += 1; counter >= 3, Duration::from_secs(1));
/// }
/// ```
#[macro_export]
macro_rules! wait_until {    // With setup code
    ($setup:stmt; $condition:expr, $timeout:expr) => {{
        match tokio::time::timeout($timeout,
            async {
                while {$setup !$condition} {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
                eyre::Ok(())
            }
        ).await {
            core::result::Result::Err(e) => panic!("Condition not satisfied within: {:?} ({})", $timeout, e),
            core::result::Result::Ok(inner) => match inner {
                core::result::Result::Err(e) => panic!("Check function failed with error: {:?}", e),
                core::result::Result::Ok(()) => (),
            }
        };
    }};

    // Without setup, just condition
    ($condition:expr, $timeout:expr) => {
        $crate::wait_until!({}; $condition, $timeout)
    };
}
