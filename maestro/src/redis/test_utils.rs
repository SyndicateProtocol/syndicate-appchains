//! Util functions for writing unit tests that share a Redis connection

use ::redis::aio::MultiplexedConnection;
use once_cell::sync::Lazy;
use std::sync::Arc;
use test_utils::docker::{start_redis, Docker};
use tokio::sync::Mutex;

// Redis container and URL information
struct RedisSetup {
    // We don't use this directly, but keep it to prevent the container from being dropped
    _container: Box<Docker>,
    redis_url: String,
}

/// Shared Redis instance for all tests using [`Mutex`]
static REDIS_SETUP: Lazy<Mutex<Option<Arc<RedisSetup>>>> = Lazy::new(|| Mutex::new(None));

/// Checks if a Redis connection already exists, otherwise creates one behind a [`Mutex`]
pub async fn initialize_redis() -> String {
    let setup_guard = REDIS_SETUP.lock().await;
    if let Some(setup) = &*setup_guard {
        // We already have Redis running, just return the URL
        setup.redis_url.clone()
    } else {
        // We need to start Redis, but we must drop the lock first
        drop(setup_guard);

        // Start Redis container with the generated name
        let (container, redis_url) = start_redis().await.unwrap();

        // Now reacquire the lock to update our static variable
        let mut setup_guard = REDIS_SETUP.lock().await;
        let redis_url_clone = redis_url.clone();

        // Store container in a static variable to keep it alive
        *setup_guard = Some(Arc::new(RedisSetup { _container: Box::new(container), redis_url }));

        redis_url_clone
    }
}

/// Helper to get a shared Redis connection for tests
pub async fn get_redis_connection() -> MultiplexedConnection {
    // Get the Redis URL first - separates the container name generation
    // and Redis initialization from connection creation
    let redis_url = initialize_redis().await;

    // Create a new connection to the shared Redis instance
    let client = redis::Client::open(redis_url.as_str()).unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}
