//! This crate is for testing the `synd-maestro` service with maximum realism

use alloy::transports::http::{reqwest::Response, Client};
use eyre::Result;
use jsonrpsee::server::ServerHandle;
use serde_json::{json, Value};
use std::{future::Future, net::SocketAddr, time::Duration};
use synd_maestro::server;
use test_utils::wait_until;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use synd_maestro::{config::Config, server::HEADER_CHAIN_ID};
    use test_utils::{
        docker::{start_redis, Docker},
        transaction::{get_eip1559_transaction_hex, get_legacy_transaction_hex},
    };
    use wiremock::{
        matchers::{body_partial_json, method},
        Mock, MockServer, ResponseTemplate,
    };

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    // Initialize the server for this test function
    async fn setup_server(
        mock_rpc_server_4: Option<MockServer>,
        mock_rpc_server_5: Option<MockServer>,
    ) -> (SocketAddr, ServerHandle, String, Docker, MockServer, MockServer) {
        // Create new mock servers if not provided, otherwise use the ones passed in
        let mock_rpc_server_4 = match mock_rpc_server_4 {
            Some(server) => server,
            None => {
                let server = MockServer::start().await;
                set_default_mock_responses_for_server_4(&server).await;
                server
            }
        };

        let mock_rpc_server_5 = match mock_rpc_server_5 {
            Some(server) => server,
            None => {
                let server = MockServer::start().await;
                set_default_mock_responses_for_server_5(&server).await;
                server
            }
        };

        let mut chain_rpc_urls = HashMap::new();
        chain_rpc_urls.insert("4".to_string(), mock_rpc_server_4.uri());
        chain_rpc_urls.insert("5".to_string(), mock_rpc_server_5.uri());

        let (redis, redis_url) = start_redis().await.unwrap();
        // Create config with mock server URL
        let config = Config {
            port: 0,
            metrics_port: 8081,
            redis_url,
            chain_rpc_urls,
            validation_timeout: Duration::from_secs(1),
            skip_validation: false,
            waiting_txn_ttl: Duration::from_secs(20), // shorter than default
            wallet_nonce_ttl: Duration::from_secs(3),
            finalization_duration: Duration::from_secs(5 * 60),
            finalization_checker_interval: Duration::from_secs(5 * 60),
        };

        // Start the actual Maestro server with our mocked config
        let (addr, handle) = server::run(config).await.expect("Failed to start server");
        let base_url = format!("http://{}", addr);

        // Wait for server to be ready by checking health endpoint
        let client = Client::new();
        wait_until!(
            client
                .get(format!("{}/health", base_url))
                .send()
                .await
                .is_ok_and(|x| x.status().is_success()),
            Duration::from_secs(5)
        );

        (addr, handle, base_url, redis, mock_rpc_server_4, mock_rpc_server_5)
    }

    async fn set_default_mock_responses_for_server_4(mock_rpc_server_4: &MockServer) {
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x1"  // Realistic nonce result format
            })))
            .mount(mock_rpc_server_4)
            .await;

        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_chainId"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x4"  // Chain ID 4 in hex
            })))
            .mount(mock_rpc_server_4)
            .await;
    }

    async fn set_default_mock_responses_for_server_5(mock_rpc_server_5: &MockServer) {
        // Set up default mocks for server 5
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_chainId"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x5"  // Chain ID 5 in hex
            })))
            .mount(mock_rpc_server_5)
            .await;

        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x1"  // Realistic nonce result format
            })))
            .mount(mock_rpc_server_5)
            .await;
    }

    async fn with_test_server<Fut>(
        mock_rpc_server_4: Option<MockServer>,
        mock_rpc_server_5: Option<MockServer>,
        test_fn: impl FnOnce(Client, String) -> Fut + Send,
    ) -> Result<()>
    where
        Fut: Future<Output = Result<()>> + Send,
    {
        let (_addr, handle, base_url, _redis, _mock_rpc_server4, _mock_rpc_server5) =
            setup_server(mock_rpc_server_4, mock_rpc_server_5).await;
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
            .header(HEADER_CHAIN_ID, "1")
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
        with_test_server(None, None, |client, base_url| async move {
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
        let tx_hex = get_legacy_transaction_hex(4, 1);

        with_test_server(None, None, |client, base_url| async move {
            // Test with valid transaction input
            let tx_response = client
                .post(base_url)
                .header(HEADER_CHAIN_ID, "4")
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
            assert!(
                tx_json.get("result").is_some(),
                "Transaction response missing 'result' field: {}",
                tx_json
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_chain_id_mismatch() -> Result<()> {
        with_test_server(None, None, |client, base_url| async move {
            // Create transaction with chain ID that doesn't match the header
            let tx_hex = get_legacy_transaction_hex(5, 690);

            let mismatch_response = client
                .post(&base_url)
                .header(HEADER_CHAIN_ID, "4") // Different from tx chain ID (5)
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
        with_test_server(None, None, |client, base_url| async move {
            // Test EIP-1559 transaction
            let eip1559_tx = get_eip1559_transaction_hex(4, 1);

            let response = client
                .post(&base_url)
                .header(HEADER_CHAIN_ID, "4")
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
                "Transaction response missing 'result' field: {}",
                json_resp
            );

            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_headers_missing() -> Result<()> {
        let tx_hex = get_legacy_transaction_hex(4, 690);
        with_test_server(None, None, |client, base_url| async move {
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
        with_test_server(None, None, |client, base_url| async move {
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
        with_test_server(None, None, |client, base_url| async move {
            // Test with malformed headers
            let malformed_headers_response = client
                .post(base_url)
                .header(HEADER_CHAIN_ID, b"\xff\xff\xff".as_ref()) // Invalid ASCII that we can't parse
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
        with_test_server(None, None, |client, base_url| async move {
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
        with_test_server(None, None, |client, base_url| async move {
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
    async fn test_unsupported_chain_id() -> Result<()> {
        let tx_hex = get_legacy_transaction_hex(9999, 2); // Unsupported in dummy config
        with_test_server(None, None, |client, base_url| async move {
            // Test with wrong param count
            let wrong_chain_id_response =
                send_rpc_request(&client, &base_url, "eth_sendRawTransaction", json!([tx_hex]))
                    .await?;

            let json: Value = wrong_chain_id_response.json().await?;
            assert!(json.get("error").is_some(), "Wrong parameter count should return an error");
            // Access the error message and check if it contains our substring
            let error_message = json["error"]["message"].as_str().unwrap_or("");
            assert!(
                error_message.contains("chain ID mismatch"),
                "Error message should mention 'chain ID mismatch', but got: '{}'",
                error_message
            );
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_invalid_hex() -> Result<()> {
        with_test_server(None, None, |client, base_url| async move {
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

    #[tokio::test]
    async fn test_transaction_nonce_too_high() -> Result<()> {
        let tx_hex = get_legacy_transaction_hex(4, 690);

        // Create and set up a custom mock server 4
        let mock_server_4 = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_chainId"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x4"  // Chain ID 4 in hex
            })))
            .mount(&mock_server_4)
            .await;

        // Txn nonce too high - 690 vs 2
        //
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount",
                "params": ["0x96216849c49358b10257cb55b28ea603c874b05e", "latest"] // wallet used by test
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x2" // nonce 2
            })))
            .mount(&mock_server_4)
            .await;

        with_test_server(
            Some(mock_server_4), // Use our custom server for chain 4
            None,                // Use the default server for chain 5
            |client, base_url| async move {
                // Test logic using the custom-mocked server
                let tx_response = client
                    .post(base_url)
                    .header(HEADER_CHAIN_ID, "4")
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
                assert!(
                    tx_json.get("result").is_some(),
                    "Transaction response should succeed: {}",
                    tx_json
                );

                Ok(())
            },
        )
        .await
    }

    #[tokio::test]
    async fn test_transaction_nonce_too_low() -> Result<()> {
        let tx_hex = get_legacy_transaction_hex(4, 690);

        // Create and set up a custom mock server 4
        let mock_server_4 = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_chainId"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x4"  // Chain ID 4 in hex
            })))
            .mount(&mock_server_4)
            .await;

        // Txn nonce too low - 690 vs 691
        //
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount",
                "params": ["0x96216849c49358b10257cb55b28ea603c874b05e", "latest"] // wallet used by test
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x2B3" // nonce 691
            })))
            .mount(&mock_server_4)
            .await;

        with_test_server(
            Some(mock_server_4), // Use our custom server for chain 4
            None,                // Use the default server for chain 5
            |client, base_url| async move {
                // Test logic using the custom-mocked server
                let tx_response = client
                    .post(base_url)
                    .header(HEADER_CHAIN_ID, "4")
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
                assert!(
                    tx_json.get("result").is_none(),
                    "Transaction response should not succeed: {}",
                    tx_json
                );
                assert_eq!(
                    tx_json.get("error"),
                    Some(&json!({
                        "code": -32003,
                        "message": "transaction rejected: transaction nonce too low: expected 691, got 690"
                    }))
                );
                Ok(())
            },
        )
            .await
    }

    #[tokio::test]
    async fn test_transaction_nonce_equal_to_rpc() -> Result<()> {
        let tx_hex = get_legacy_transaction_hex(4, 690);

        // Create and set up a custom mock server 4
        let mock_server_4 = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_chainId"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x4"  // Chain ID 4 in hex
            })))
            .mount(&mock_server_4)
            .await;

        // Txn nonce equal - 690 vs 690
        //
        Mock::given(method("POST"))
            .and(body_partial_json(json!({
                "method": "eth_getTransactionCount",
                "params": ["0x96216849c49358b10257cb55b28ea603c874b05e", "latest"] // wallet used by test
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x2B2" // nonce 690
            })))
            .mount(&mock_server_4)
            .await;

        with_test_server(
            Some(mock_server_4), // Use our custom server for chain 4
            None,                // Use the default server for chain 5
            |client, base_url| async move {
                // Test logic using the custom-mocked server
                let tx_response = client
                    .post(base_url)
                    .header(HEADER_CHAIN_ID, "4")
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
                assert!(
                    tx_json.get("result").is_some(),
                    "Transaction response should succeed: {}",
                    tx_json
                );
                Ok(())
            },
        )
        .await
    }
}
