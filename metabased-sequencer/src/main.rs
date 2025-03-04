//! Metabased Sequencer is a service that processes and validates transactions
//! before submitting them to the Metabased chain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use eyre::Result;
use jsonrpsee::{
    server::{Server, ServerHandle},
    types::error::ErrorCode,
    RpcModule,
};
use serde_json::Value as JsonValue;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod contract;
mod metrics;
mod service;

use config::Config;
use metrics::{start_metrics, MetricsState};
use service::{send_raw_transaction_handler, MetabasedService};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    // Parse config
    let config = Config::parse()?;

    // Initialize metrics
    let metrics = MetricsState::new();
    let metrics_handler = start_metrics(metrics, 7777).await;

    // Start server
    let (addr, handle) = run_server(&config).await?;

    info!(
        addr = %addr,
        "Metabased Sequencer v2 server running"
    );

    // Keep the server running
    tokio::select! {
        _ = handle.stopped() => {}
        _ = metrics_handler => {}
    }

    Ok(())
}

async fn run_server(config: &Config) -> Result<(SocketAddr, ServerHandle)> {
    let server = Server::builder().build(format!("0.0.0.0:{}", config.port)).await?;

    let service = MetabasedService::new(config)?;

    let mut module = RpcModule::new(service);

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler)?;
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
    use alloy::{
        node_bindings::Anvil,
        primitives::{Address, B256},
        providers::{Provider, ProviderBuilder},
    };
    use jsonrpsee::{core::client::ClientT, http_client::HttpClient};
    use std::str::FromStr;
    use url::Url;

    #[tokio::test]
    async fn test_run_server() {
        // Setup sequencing chain
        let anvil = Anvil::new().port(8181_u16).chain_id(12345678).try_spawn().unwrap();

        // Setup provider
        let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

        let private_key =
            B256::from_str("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
                .unwrap();
        let sequencer_address =
            Address::from_str("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap();

        let config = Config {
            chain_contract_address: Address::from_str("0x0000000000000000000000000000000000000000")
                .unwrap(),
            chain_rpc_url: Url::parse("http://localhost:8181").unwrap(),
            private_key,
            port: 8282,
        };

        // Start server
        let (_addr, _handle) = run_server(&config).await.unwrap();

        // Setup RPC client
        let client =
            HttpClient::builder().build(format!("http://localhost:{}", config.port)).unwrap();

        // Call the health endpoint
        let health: JsonValue = client.request("health", [()]).await.unwrap();
        assert_eq!(health, serde_json::json!({"health": true}));

        // Check Nonce Before
        let nonce = provider.get_transaction_count(sequencer_address).await.unwrap();
        assert_eq!(nonce, 0);

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

        // Check Nonce
        let nonce = provider.get_transaction_count(sequencer_address).await.unwrap();
        assert_eq!(nonce, 1);

        // Send 3 more transactions
        for _ in 0..3 {
            let _: JsonValue = client
                .request("eth_sendRawTransaction", [
                    "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772",
                ])
                .await
                .unwrap();
        }

        // Check Nonce
        let nonce = provider.get_transaction_count(sequencer_address).await.unwrap();
        assert_eq!(nonce, 4);
    }
}
