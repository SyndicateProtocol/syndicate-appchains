//! This crate is for testing the `Maestro` server with maximum realism

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
    use maestro::config::Config;
    use std::collections::HashMap;
    use test_utils::transaction::{get_eip1559_transaction_hex, get_legacy_transaction_hex};
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    fn dummy_config_with_url(mock_url: String) -> Config {
        let mut chain_id_nitro_urls = HashMap::new();
        // Add URLs for the chain IDs used in tests
        chain_id_nitro_urls.insert("4".to_string(), mock_url.clone());
        chain_id_nitro_urls.insert("5".to_string(), mock_url);

        Config {
            port: 0,
            redis_address: None,
            chain_rpc_urls: chain_id_nitro_urls,
            validation_timeout: Duration::from_secs(1),
            skip_validation: false,
        }
    }

    // Initialize the server for this test function
    async fn setup_server_with_mock_server() -> (SocketAddr, ServerHandle, String, MockServer) {
        // Start the mock HTTP server first
        let mock_server = MockServer::start().await;

        // Set up mock response for eth_sendRawTransaction
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
            })))
            .mount(&mock_server)
            .await;

        // Create config with mock server URL
        let config = dummy_config_with_url(mock_server.uri());

        // Start the actual Maestro server with our mocked config
        let (addr, handle) = server::run(config).await.expect("Failed to start server");
        let base_url = format!("http://{}", addr);

        // Give the server time to initialize
        sleep(Duration::from_millis(100)).await;

        (addr, handle, base_url, mock_server)
    }

    async fn with_test_server<Fut>(test_fn: impl FnOnce(Client, String) -> Fut + Send) -> Result<()>
    where
        Fut: Future<Output = Result<()>> + Send,
    {
        let (_addr, handle, base_url, _mock_server) = setup_server_with_mock_server().await;
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
        let tx_hex = get_legacy_transaction_hex(4, 690);

        with_test_server(|client, base_url| async move {
            // Test with valid transaction input
            let tx_response = client
                .post(base_url)
                .header("x-synd-chain-id", "4")
                .json(&json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_sendRawTransaction",
                    "params": [tx_hex]
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
    async fn test_chain_id_mismatch() -> Result<()> {
        with_test_server(|client, base_url| async move {
            // Create transaction with chain ID that doesn't match the header
            let tx_hex = get_legacy_transaction_hex(5, 690);

            let mismatch_response = client
                .post(&base_url)
                .header("x-synd-chain-id", "4") // Different from tx chain ID (5)
                .json(&json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_sendRawTransaction",
                    "params": [tx_hex]
                }))
                .send()
                .await?;

            let mismatch_json: Value = mismatch_response.json().await?;
            assert!(
                mismatch_json.get("error").is_some(),
                "Chain ID mismatch should return an error"
            );

            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_eip1559_transaction() -> Result<()> {
        with_test_server(|client, base_url| async move {
            // Test EIP-1559 transaction
            let eip1559_tx = get_eip1559_transaction_hex(4, 123);

            let response = client
                .post(&base_url)
                .header("x-synd-chain-id", "4")
                .json(&json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_sendRawTransaction",
                    "params": [eip1559_tx]
                }))
                .send()
                .await?;

            assert!(response.status().is_success(), "EIP-1559 transaction request failed");
            let json_resp: Value = response.json().await?;
            assert!(
                json_resp.get("result").is_some(),
                "Transaction response missing 'result' field"
            );

            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_headers_missing() -> Result<()> {
        let tx_hex = get_legacy_transaction_hex(4, 690);
        with_test_server(|client, base_url| async move {
            // Test request with missing optional headers
            let no_headers_response = client
                .post(base_url)
                .json(&json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_sendRawTransaction",
                    "params": [tx_hex]
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
        let tx_hex = get_legacy_transaction_hex(4, 690);
        with_test_server(|client, base_url| async move {
            // Test with malformed headers
            let malformed_headers_response = client
                .post(base_url)
                .header("x-synd-chain-id-Wrong", "invalid")
                .json(&json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_sendRawTransaction",
                    "params": [tx_hex]
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
        let tx_hex = get_legacy_transaction_hex(4, 690);
        with_test_server(|client, base_url| async move {
            // Test with malformed headers
            let malformed_headers_response = client
                .post(base_url)
                .header("x-synd-chain-id", b"\xff\xff\xff".as_ref()) // Invalid ASCII that we can't parse
                .json(&json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_sendRawTransaction",
                    "params": [tx_hex]
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
        let tx_hex = get_legacy_transaction_hex(4, 690);
        let tx_hex2 = get_legacy_transaction_hex(4, 691);
        with_test_server(|client, base_url| async move {
            // Test with wrong param count
            let wrong_count_response = send_rpc_request(
                &client,
                &base_url,
                "eth_sendRawTransaction",
                json!([tx_hex, tx_hex2]),
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
