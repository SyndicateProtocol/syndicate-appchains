//! This crate is for testing the `Maestro` server

use alloy::transports::http::{reqwest::Response, Client};
use eyre::Result;
use jsonrpsee::server::ServerHandle;
use maestro::server;
use serde_json::{json, Value};
use std::{future::Future, net::SocketAddr, time::Duration};
use tokio::time::sleep;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    // Static server setup - one per test function

    // Initialize the server for this test function
    async fn setup_server() -> (SocketAddr, ServerHandle, String) {
        // Start the server
        // Always use 0 for dynamic port allocation
        // This lets the OS assign an available port automatically
        let (addr, handle) = server::run(0).await.expect("Failed to start server");
        let base_url = format!("http://{}", addr);
        println!("Server started at {} (OS assigned port {})", &base_url, addr.port());

        // Give the server a moment to fully initialize
        // TODO SEQ-576 - This should use wait_until! but it's not available because we don't have a
        // shared utility crate
        sleep(Duration::from_millis(100)).await;

        (addr, handle, base_url)
    }

    async fn with_test_server<Fut>(test_fn: impl FnOnce(Client, String) -> Fut) -> Result<()>
    where
        Fut: Future<Output = Result<()>>,
    {
        let (_addr, handle, base_url) = setup_server().await;
        let client = Client::new();

        test_fn(client, base_url).await?;
        handle.stop()?;
        Ok(())
    }

    // Helper function to send JSON-RPC requests with required headers
    async fn send_rpc_request(
        client: &Client,
        url: &str,
        method: &str,
        params: Value,
    ) -> Result<Response> {
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
    async fn test_health_check() -> Result<()> {
        with_test_server(|client, base_url| async move {
            let health_response = client.get(format!("{}/health", base_url)).send().await?;

            assert!(health_response.status().is_success(), "Health check failed");
            let health_json: Value = health_response.json().await?;
            assert_eq!(
                health_json,
                json!({"health": true}),
                "Health check returned unexpected response"
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_valid_transaction() -> Result<()> {
        with_test_server(|client, base_url| async move {
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
        })
        .await
    }

    #[tokio::test]
    async fn test_headers_missing() -> Result<()> {
        with_test_server(|client, base_url| async move {
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
        })
        .await
    }

    #[tokio::test]
    async fn test_headers_malformed_key() -> Result<()> {
        with_test_server(|client, base_url| async move {
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
            let header_map =
                malformed_headers_response.extensions().get::<HashMap<String, String>>();
            assert!(header_map.is_none(), "Header map should not exist");
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_headers_valid_key_invalid_value() -> Result<()> {
        with_test_server(|client, base_url| async move {
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

            assert!(malformed_headers_response.status().is_success(), "Request should succeed");

            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_empty_params() -> Result<()> {
        with_test_server(|client, base_url| async move {
            // Test with empty params array
            let empty_params_response =
                send_rpc_request(&client, &base_url, "eth_sendRawTransaction", json!([])).await?;

            let json: Value = empty_params_response.json().await?;
            assert!(json.get("error").is_some(), "Empty params should return an error");

            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_wrong_param_count() -> Result<()> {
        with_test_server(|client, base_url| async move {
            // Test with wrong param count
            let wrong_count_response = send_rpc_request(
                &client,
                &base_url,
                "eth_sendRawTransaction",
                json!(["0x1234", "0x5678"]),
            )
            .await?;

            let json: Value = wrong_count_response.json().await?;
            assert!(json.get("error").is_some(), "Wrong parameter count should return an error");
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_invalid_hex() -> Result<()> {
        with_test_server(|client, base_url| async move {
            // Test with invalid hex
            let invalid_hex_response =
                send_rpc_request(&client, &base_url, "eth_sendRawTransaction", json!(["not_hex"]))
                    .await?;

            let json: Value = invalid_hex_response.json().await?;
            assert!(json.get("error").is_some(), "Invalid hex should return an error");

            Ok(())
        })
        .await
    }
}
