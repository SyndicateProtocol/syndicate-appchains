//! This crate is for testing the `Maestro` server

use alloy::transports::http::{reqwest::Response, Client};
use maestro::server;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_maestro_server() -> eyre::Result<()> {
    // Start the server in a separate task
    let server_task = tokio::spawn(async {
        let (addr, handle) = server::run().await.expect("Failed to start server");

        // Return both the address and handle so we can use them later
        (addr, handle)
    });

    // Give the server a moment to start up
    sleep(Duration::from_millis(100)).await;

    // Get the server address
    let (addr, handle) = server_task.await?;
    let base_url = format!("http://{}", addr);

    // Create HTTP client
    let client = Client::new();

    // Test 1: Health check
    let health_response = client.get(format!("{}/health", base_url)).send().await?;

    assert!(health_response.status().is_success());
    let health_json: Value = health_response.json().await?;
    assert_eq!(health_json, json!({"health": true}));

    // Test 2: Check the eth_sendRawTransaction method with valid input
    let tx_response = client
        .post(&base_url)
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

    assert!(tx_response.status().is_success());
    let tx_json: Value = tx_response.json().await?;
    // Checking that we get a valid response with the expected structure
    assert!(tx_json.get("result").is_some());

    // Test 3: Check for missing required headers
    let no_headers_response = client
        .post(&base_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_sendRawTransaction",
            "params": ["0x1234"]
        }))
        .send()
        .await?;

    assert!(!no_headers_response.status().is_success());

    // Gracefully shut down the server
    handle.stop()?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_params() -> eyre::Result<()> {
    // Start the server
    let server_task = tokio::spawn(async {
        let (addr, handle) = server::run().await.expect("Failed to start server");
        (addr, handle)
    });

    sleep(Duration::from_millis(100)).await;
    let (addr, handle) = server_task.await?;
    let base_url = format!("http://{}", addr);

    let client = Client::new();

    // Test with empty params array
    let empty_params_response =
        send_rpc_request(&client, &base_url, "eth_sendRawTransaction", json!([])).await?;

    let json: Value = empty_params_response.json().await?;
    assert!(json.get("error").is_some());

    // Test with wrong param count
    let wrong_count_response =
        send_rpc_request(&client, &base_url, "eth_sendRawTransaction", json!(["0x1234", "0x5678"]))
            .await?;

    let json: Value = wrong_count_response.json().await?;
    assert!(json.get("error").is_some());

    // Test with invalid hex
    let invalid_hex_response =
        send_rpc_request(&client, &base_url, "eth_sendRawTransaction", json!(["not_hex"])).await?;

    let json: Value = invalid_hex_response.json().await?;
    assert!(json.get("error").is_some());

    // Gracefully shut down the server
    handle.stop()?;

    Ok(())
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
