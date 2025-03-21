//! This crate is for testing the `Maestro` server

use alloy::transports::http::{reqwest::Response, Client};
use jsonrpsee::server::ServerHandle;
use maestro::server;
use serde_json::{json, Value};
use std::{env, net::SocketAddr, time::Duration};
use tokio::{sync::OnceCell, time::sleep};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    // Static server setup - one per test function
    static SERVER_SETUP: OnceCell<(SocketAddr, ServerHandle, String)> = OnceCell::const_new();

    // Get port for testing - always use port 0 (dynamic) when not in a test group
    const fn get_test_port() -> i32 {
        // Always use 0 for dynamic port allocation
        // This lets the OS assign an available port automatically
        0
    }

    // Initialize the server for this test function
    async fn setup_server() -> &'static (SocketAddr, ServerHandle, String) {
        SERVER_SETUP
            .get_or_init(|| async {
                // Log which test group we're in
                let test_group =
                    env::var("NEXTEST_TEST_GROUP").unwrap_or_else(|_| "@global".to_string());
                let group_slot =
                    env::var("NEXTEST_TEST_GROUP_SLOT").unwrap_or_else(|_| "none".to_string());

                println!("Running with test group: {}, slot: {}", test_group, group_slot);

                // Get dynamic port (0)
                let port = get_test_port();
                println!("Using dynamic port (0)");

                // Start the server
                let (addr, handle) = server::run(port).await.expect("Failed to start server");
                let base_url = format!("http://{}", addr);
                println!("Server started at {} (OS assigned port {})", base_url, addr.port());

                // Give the server a moment to fully initialize
                sleep(Duration::from_millis(100)).await;

                (addr, handle, base_url)
            })
            .await
    }

    // Helper function to send JSON-RPC requests with required headers
    async fn send_rpc_request(
        client: &Client,
        url: &str,
        method: &str,
        params: Value,
    ) -> eyre::Result<Response> {
        let response = client
            .post(url)
            .header("x-synd-chain-id", "1")
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params
            }))
            .send()
            .await?;

        Ok(response)
    }

    #[tokio::test]
    async fn test_health_check() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test health check endpoint
        let health_response = client.get(format!("{}/health", base_url)).send().await?;

        assert!(health_response.status().is_success(), "Health check failed");
        let health_json: Value = health_response.json().await?;
        assert_eq!(
            health_json,
            json!({"health": true}),
            "Health check returned unexpected response"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_valid_transaction() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test with valid transaction input
        let tx_response = client
            .post(base_url)
            .header("x-synd-chain-id", "1")
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": ["0x1234"]
            }))
            .send()
            .await?;

        assert!(tx_response.status().is_success(), "Valid transaction request failed");
        let tx_json: Value = tx_response.json().await?;
        assert!(tx_json.get("result").is_some(), "Transaction response missing 'result' field");

        Ok(())
    }

    #[tokio::test]
    async fn test_headers_missing() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test request with missing optional headers
        let no_headers_response = client
            .post(base_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": ["0x1234"]
            }))
            .send()
            .await?;

        assert!(
            no_headers_response.status().is_success(),
            "Request without required headers should pass"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_headers_malformed_key() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test with malformed headers
        let malformed_headers_response = client
            .post(base_url)
            .header("x-synd-chain-id-Wrong", "invalid")
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": ["0x1234"]
            }))
            .send()
            .await?;

        assert!(
            malformed_headers_response.status().is_success(),
            "Request should still succeed despite weird header"
        );
        let header_map = malformed_headers_response.extensions().get::<HashMap<String, String>>();
        assert!(header_map.is_none(), "Header map should not exist");

        Ok(())
    }

    #[tokio::test]
    async fn test_headers_valid_key_invalid_value() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test with malformed headers
        let malformed_headers_response = client
            .post(base_url)
            .header("x-synd-chain-id", b"\xff\xff\xff".as_ref()) // Invalid ASCII that we can't parse
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": ["0x1234"]
            }))
            .send()
            .await?;

        assert!(malformed_headers_response.status().is_client_error(), "Request should fail");

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_params() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test with empty params array
        let empty_params_response =
            send_rpc_request(&client, base_url, "eth_sendRawTransaction", json!([])).await?;

        let json: Value = empty_params_response.json().await?;
        assert!(json.get("error").is_some(), "Empty params should return an error");

        Ok(())
    }

    #[tokio::test]
    async fn test_wrong_param_count() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test with wrong param count
        let wrong_count_response = send_rpc_request(
            &client,
            base_url,
            "eth_sendRawTransaction",
            json!(["0x1234", "0x5678"]),
        )
        .await?;

        let json: Value = wrong_count_response.json().await?;
        assert!(json.get("error").is_some(), "Wrong parameter count should return an error");

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_hex() -> eyre::Result<()> {
        let (_, _, base_url) = setup_server().await;
        let client = Client::new();

        // Test with invalid hex
        let invalid_hex_response =
            send_rpc_request(&client, base_url, "eth_sendRawTransaction", json!(["not_hex"]))
                .await?;

        let json: Value = invalid_hex_response.json().await?;
        assert!(json.get("error").is_some(), "Invalid hex should return an error");

        Ok(())
    }

    // Test to ensure server is properly stopped
    // This will run last since nextest runs in alphabetical order
    #[tokio::test]
    async fn zzzz_test_teardown_shutdown_server() -> eyre::Result<()> {
        if let Some((_, handle, _)) = SERVER_SETUP.get() {
            handle.stop()?;
            sleep(Duration::from_millis(100)).await;
            println!("Server successfully stopped");
        } else {
            println!("No server instance to stop");
        }

        Ok(())
    }
}
