//! The `server` module sets up and runs the base server for the sequencer
use crate::config::TCSequencerConfig;
use eyre::Result;
use jsonrpsee::{
    server::{middleware::http::ProxyGetRequestLayer, Server, ServerHandle},
    types::error::ErrorCode,
    RpcModule,
};
use serde_json::Value as JsonValue;
use std::net::SocketAddr;
use tc_client::tc_client::{send_raw_transaction_handler, TCClient};
use tower::ServiceBuilder;
use tracing::info;

/// Runs the base server for the sequencer
pub async fn run_server(
    config: &TCSequencerConfig,
    tc_client: TCClient,
) -> Result<(SocketAddr, ServerHandle)> {
    // Middleware to proxy "/health" GET requests to "health" RPC method
    let http_middleware =
        ServiceBuilder::new().layer(ProxyGetRequestLayer::new("/health", "health")?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", config.port))
        .await?;

    let mut module = RpcModule::new(tc_client);

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler)?;

    // Register health method (this will be hit by the health check middleware)
    module.register_method("health", |_, _, _| {
        Ok::<JsonValue, ErrorCode>(serde_json::json!({"health": true}))
    })?;

    info!("Registered RPC methods: {:#?}", module.method_names().collect::<Vec<_>>());

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use jsonrpsee::{core::client::ClientT, http_client::HttpClient};
    use mockito::{self, Matcher};
    use reqwest::{Client, StatusCode};
    use serde_json::Value as JsonValue;
    use std::str::FromStr;
    use tc_client::config::{TCConfig, TCEndpoint};
    use url::Url;

    #[tokio::test]
    async fn test_run_server() {
        // Setup mock TC server
        let mut server = mockito::Server::new_async().await;
        let mock_tc_server = server
            .mock("POST", Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"transactionId": "561b77ed-6e35-4248-87ef-c6af93f3bad9"}"#)
            .create_async()
            .await;

        let url = Url::from_str(server.url().as_str()).unwrap();

        let config = TCSequencerConfig {
            batcher: Default::default(),
            tc: TCConfig {
                tc_endpoint: TCEndpoint::Raw(url),
                sequencing_address: Address::ZERO,
                ..Default::default()
            },
            port: 8080,
        };

        // Start server
        let (_addr, _handle) =
            run_server(&config, TCClient::new(&config.tc).unwrap()).await.unwrap();

        // Check health endpoint
        let health_response = Client::new()
            .get(format!("http://localhost:{}/health", config.port))
            .send()
            .await
            .unwrap();
        assert_eq!(health_response.status(), StatusCode::OK);

        // Setup RPC client
        let client =
            HttpClient::builder().build(format!("http://localhost:{}", config.port)).unwrap();

        // Call the eth_sendRawTransaction endpoint
        let tx_hash: JsonValue = client
            .request("eth_sendRawTransaction", [
                "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772",
            ])
            .await
            .unwrap();
        assert_eq!(
            tx_hash,
            serde_json::json!("0xc429e5f128387d224ba8bed6885e86525e14bfdc2eb24b5e9c3351a1176fd81f")
        );

        mock_tc_server.assert();
        drop(server);
    }
}
