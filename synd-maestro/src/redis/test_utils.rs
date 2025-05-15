//! Util functions for writing unit tests that share a Redis connection
use redis::aio::MultiplexedConnection;
use test_utils::docker::{start_redis, Docker};

/// Helper to get Redis resources for tests
pub async fn init_redis_and_get_connection() -> (MultiplexedConnection, String, Docker) {
    let (container, redis_url) = start_redis().await.unwrap();
    let client = redis::Client::open(redis_url.as_str()).unwrap();
    (client.get_multiplexed_async_connection().await.unwrap(), redis_url, container)
}
