//! This crate is for testing the `Maestro` server

use alloy::transports::http::{reqwest::Response, Client};
use jsonrpsee::server::ServerHandle;
use serde_json::{json, Value};
use std::{net::SocketAddr, time::Duration};
use tokio::{sync::OnceCell, time::sleep};

// Global static server setup
static SERVER_SETUP: OnceCell<(SocketAddr, ServerHandle, String)> = OnceCell::const_new();

// Initialize the server once for all tests
async fn setup_server() -> &'static (SocketAddr, ServerHandle, String) {
    SERVER_SETUP
        .get_or_init(|| async {
            // Start the server
            #[allow(clippy::expect_used)]
            let (addr, handle) = maestro::server::run().await.expect("Failed to start server");
            let base_url = format!("http://{}", addr);

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
        .header("X-Synd-Chain-Id", "1")
        .header("X-Synd-Method-Name", method)
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
    assert_eq!(health_json, json!({"health": true}), "Health check returned unexpected response");

    Ok(())
}

#[tokio::test]
async fn test_valid_transaction() -> eyre::Result<()> {
    let (_, _, base_url) = setup_server().await;
    let client = Client::new();

    // Test with valid transaction input
    let tx_response = client
        .post(base_url)
        .header("X-Synd-Chain-Id", "1")
        .header("X-Synd-Method-Name", "eth_sendRawTransaction")
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
async fn test_malformed_headers() -> eyre::Result<()> {
    let (_, _, base_url) = setup_server().await;
    let client = Client::new();

    // Test request with malformed required headers
    let no_headers_response = client
        .post(base_url)
        .header("X-Synd-Chain-Id", "1")
        .header("X-Synd-Wrong-Header", "eth_sendRawTransaction")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_sendRawTransaction",
            "params": ["0x1234"]
        }))
        .send()
        .await?;

    assert!(
        !no_headers_response.status().is_success(),
        "Request without malformed headers should fail"
    );

    Ok(())
}

#[tokio::test]
async fn test_missing_headers() -> eyre::Result<()> {
    let (_, _, base_url) = setup_server().await;
    let client = Client::new();

    // Test request with missing required headers
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
        !no_headers_response.status().is_success(),
        "Request without required headers should fail"
    );

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
    let wrong_count_response =
        send_rpc_request(&client, base_url, "eth_sendRawTransaction", json!(["0x1234", "0x5678"]))
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
        send_rpc_request(&client, base_url, "eth_sendRawTransaction", json!(["not_hex"])).await?;

    let json: Value = invalid_hex_response.json().await?;
    assert!(json.get("error").is_some(), "Invalid hex should return an error");

    Ok(())
}

// This module contains test teardown logic, running after all tests are complete
mod teardown {
    use super::*;

    #[tokio::test]
    // Make this the last test to run using the 'z' prefix
    async fn z_shutdown_server() -> eyre::Result<()> {
        // Only try to shut down if the server was actually started
        if let Some((_, handle, _)) = SERVER_SETUP.get() {
            // Gracefully shut down the server
            handle.stop()?;
        }

        Ok(())
    }
}
