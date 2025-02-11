use std::{
    fs,
    hash::{DefaultHasher, Hash, Hasher},
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
    use std::{
        panic, thread,
        time::{SystemTime, UNIX_EPOCH},
    };

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
