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
mod service;

use config::Config;
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

    // Start server
    let (addr, handle) = run_server(&config).await?;

    info!(
        addr = %addr,
        "Metabased Sequencer v2 server running"
    );

    // Keep the server running
    handle.stopped().await;

    Ok(())
}

async fn run_server(config: &Config) -> Result<(SocketAddr, ServerHandle)> {
    let server = Server::builder().build(format!("0.0.0.0:{}", config.port)).await?;

    let service = MetabasedService::new(
        config.chain_contract_address,
        config.chain_rpc_url.clone(),
        config.private_key,
    )?;

    let mut module = RpcModule::new(service);

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler)?;
    module.register_method("health", |_, _| {
        Ok::<JsonValue, ErrorCode>(serde_json::json!({"health": true}))
    })?;

    info!("Registered RPC methods: {:#?}", module.method_names().collect::<Vec<_>>());

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}
