//! Util functions for writing unit tests that share a Valkey connection
use redis::aio::ConnectionManager;
use test_utils::docker::{start_valkey, E2EProcess};

/// Helper to get Valkey resources for tests
pub async fn init_valkey_and_get_connection() -> (ConnectionManager, String, E2EProcess) {
    let (valkey_container, valkey_url) = start_valkey().await.unwrap();
    let valkey_client = redis::Client::open(valkey_url.as_str()).unwrap();
    let conn_manager = ConnectionManager::new(valkey_client).await.unwrap();
    (conn_manager, valkey_url, valkey_container)
}
