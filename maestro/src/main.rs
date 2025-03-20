//! Maestro is a service that filters and coordinates transaction requests to sequencers

use alloy::primitives::Bytes;
use axum::http::Extensions;
use eyre::Result;
use jsonrpsee::{
    core::{JsonValue, RpcResult},
    server::{middleware::http::ProxyGetRequestLayer, Server, ServerHandle},
    types::{ErrorCode, Params},
    RpcModule,
};
use maestro::{
    errors::{
        Error,
        Error::InvalidParams,
        InvalidParamsError::{MissingParam, NotAnArray, NotHexEncoded, WrongParamCount},
    },
    layers::HeadersLayer,
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder().with_max_level(Level::DEBUG).json().with_target(true).init();

    // TODO
    // let config = Config::initialize();
    // info!("Config: {:?}", config);

    // TODO metrics, if necessary

    let (addr, handle) = run_server().await?;

    info!(
        addr = %addr,
        "Maestro server running"
    );

    // Keep the server running
    tokio::select! {
        _ = handle.stopped() => {}
    }
    Ok(())
}

/// Runs the base server for the sequencer
pub async fn run_server(// config: &Config,
    // relayer_metrics: RelayerMetrics,
) -> Result<(SocketAddr, ServerHandle)> {
    // Middleware to proxy "/health" GET requests to "health" RPC method
    let required_headers = vec!["X-Synd-Chain-Id".to_string(), "X-Synd-Method-Name".to_string()];
    let http_middleware = ServiceBuilder::new()
        .layer(HeadersLayer::new(required_headers)?)
        .layer(ProxyGetRequestLayer::new("/health", "health")?);

    let port = 8111;
    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", port))
        .await?;

    let service = MaestroService::new();
    let mut module = RpcModule::new(service);

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

/// The handler for the `eth_sendRawTransaction` JSON-RPC method.
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    _service: Arc<MaestroService>,
    _extensions: Extensions,
) -> RpcResult<String> {
    // let start = Instant::now();
    let result = async {
        let _tx_data = parse_send_raw_transaction_params(params)?;

        // TODO spam plane

        // TODO control plane

        // TODO return real hash
        // let tx_hash = service.process_transaction(tx_data).await?;
        let tx_hash = Bytes::from("1234");
        Ok::<_, Error>(format!("0x{}", alloy::hex::encode(tx_hash)))
    }
    .await;

    result.map_err(|e| e.to_json_rpc_error())

    // let duration = start.elapsed();
    //
    // result
    //     .inspect(|_| service.metrics.record_rpc_call("eth_sendRawTransaction", duration, None))
    //     .map_err(|e| {
    //         service.metrics.record_rpc_call("eth_sendRawTransaction", duration, Some(&e));
    //         e.to_json_rpc_error()
    //     })
}

/// The service for filtering and directing transactions
#[derive(Debug)]
pub struct MaestroService {}

impl Default for MaestroService {
    fn default() -> Self {
        Self::new()
    }
}

impl MaestroService {
    /// Create a new instance of the Maestro service
    pub const fn new() -> Self {
        Self {}
    }
}

// TODO [SEQ-663]: Refactor this function
fn parse_send_raw_transaction_params(params: Params<'static>) -> Result<Bytes, Error> {
    let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap_or("[]"))?;
    let arr = json.as_array_mut().ok_or(InvalidParams(NotAnArray))?;
    if arr.len() != 1 {
        return Err(InvalidParams(WrongParamCount(arr.len())));
    }
    let item = arr.pop().ok_or(InvalidParams(MissingParam))?;
    let raw_tx = item.as_str().ok_or(InvalidParams(NotHexEncoded))?.to_string();
    let tx_data: Bytes = alloy::hex::decode(&raw_tx).map(Bytes::from)?;

    Ok(tx_data)
}
