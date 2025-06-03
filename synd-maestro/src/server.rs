//! The JSON-RPC server module for the Maestro service.

use crate::{
    config::Config, layers::HeadersLayer, maestro::MaestroService, metrics::MaestroMetrics,
};
use alloy::{consensus::Transaction, primitives::ChainId};
use http::Extensions;
use jsonrpsee::{
    core::RpcResult,
    server::{middleware::http::ProxyGetRequestLayer, Server},
    types::{ErrorCode, Params},
    RpcModule,
};
use serde_json::Value as JsonValue;
use shared::{
    json_rpc::{
        parse_send_raw_transaction_params,
        InvalidInputError::ChainIdMismatched,
        RpcError::{self, InvalidInput},
    },
    tracing::{extract_tracing_context, SpanKind},
    tx_validation::validate_transaction,
};
use std::{collections::HashMap, future::Future, net::SocketAddr, pin::Pin, sync::Arc};
use tokio::time::Instant;
use tower::ServiceBuilder;
use tracing::{error, info, instrument};

/// Request header for `eth_sendRawTransaction` calls that holds the intended `chain_id`
pub const HEADER_CHAIN_ID: &str = "x-synd-chain-id";

/// shutdown function type
pub type ShutdownFn =
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<(), eyre::Error>> + Send>> + Send>;

/// Runs the base server for the sequencer
pub async fn run(
    config: Config,
    metrics: MaestroMetrics,
) -> eyre::Result<(SocketAddr, ShutdownFn)> {
    info!("Starting Maestro server:run");

    let optional_headers =
        vec![HEADER_CHAIN_ID.to_string(), "traceparent".to_string(), "tracestate".to_string()];
    // let optional_headers = vec![HEADER_CHAIN_ID.to_string()];
    let http_middleware = ServiceBuilder::new()
        .layer(HeadersLayer::new(optional_headers)?)
        .layer(ProxyGetRequestLayer::new("/health", "health")?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", config.port))
        .await?;

    // Create the service internally again
    let client = redis::Client::open(config.valkey_url.as_str())?;
    let valkey_conn = client.get_multiplexed_async_connection().await?;
    let service = Arc::new(MaestroService::new(valkey_conn, config.clone(), metrics).await?);
    info!("MaestroService created and connected to Valkey successfully!");

    let mut module = RpcModule::new(service.clone()); // Pass Arc for RpcModule state

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler)?;

    // Register health method (this will be hit by the health check middleware)
    module.register_async_method(
        "health",
        |_, service: Arc<Arc<MaestroService>>, _| async move {
            let health = service.health().await;
            match health {
                Ok(true) => Ok::<JsonValue, ErrorCode>(serde_json::json!({"health": true})),
                _ => Err(ErrorCode::InternalError),
            }
        },
    )?;

    // TODO (SEQ-908): Background job to unstick `waiting txn` cache

    info!("Registered RPC methods: {:#?}", module.method_names().collect::<Vec<_>>());

    let addr = server.local_addr()?;
    let handle = server.start(module);

    // Define the shutdown closure
    let shutdown_fn: ShutdownFn = Box::new(move || {
        let service_clone = Arc::clone(&service);
        Box::pin(async move {
            info!("Stopping RPC server...");
            if let Err(e) = handle.stop() {
                error!("Error stopping RPC server: {:?}", e);
            }
            handle.stopped().await;
            info!("RPC server stopped.");
            info!("Stopping MaestroService...");
            service_clone.shutdown().await;
            info!("MaestroService stopped.");
            Ok(())
        }) as Pin<Box<dyn Future<Output = Result<(), eyre::Error>> + Send>>
    });

    Ok((addr, shutdown_fn))
}

/// The handler for the `eth_sendRawTransaction` JSON-RPC method
#[instrument(
    skip_all,
    err,
    fields(
        otel.kind = ?SpanKind::Server,
    )
)]
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service_arc_arc: Arc<Arc<MaestroService>>,
    extensions: Extensions,
) -> RpcResult<String> {
    let map : HashMap<String, String> = HashMap::new();
    #[allow(clippy::expect_used)]
    let headers = extensions.get::<HashMap<String, String>>().unwrap_or(&map);
    extract_tracing_context(headers);

    let service = service_arc_arc.as_ref();
    let req_start = Instant::now();
    service.metrics.increment_maestro_requests_total(1);

    let raw_tx = parse_send_raw_transaction_params(params)?;
    let (tx, signer) = validate_transaction(&raw_tx)?;

    let chain_id = validate_chain_id(get_request_chain_id(&extensions), tx.chain_id())?;

    // fail fast if rpc provider for chain isn't configured
    service.get_rpc_provider(&chain_id)?;

    let tx_nonce = tx.nonce();
    let tx_hash = format!("0x{}", alloy::hex::encode(tx.hash()));

    info!(
        %tx_hash,
        %chain_id,
        "Submitting validated transaction",
    );

    service.handle_transaction(raw_tx, &tx, signer, chain_id, tx_nonce).await?;

    info!(%tx_hash, %chain_id, "Submitted forwarded transaction");

    service.metrics.record_maestro_requests_duration_ms(req_start.elapsed());
    service.metrics.increment_maestro_successful_transactions_total(1);
    Ok(tx_hash)
}

fn get_request_chain_id(extensions: &Extensions) -> Option<ChainId> {
    extensions
        .get::<HashMap<String, String>>()
        .and_then(|map| map.get(HEADER_CHAIN_ID))
        .and_then(|chain_id| chain_id.parse::<ChainId>().ok())
}

/// Returns `txn_chain_id` unwrapped if valid
fn validate_chain_id(
    request_chain_id: Option<ChainId>,
    txn_chain_id: Option<ChainId>,
) -> Result<ChainId, RpcError> {
    match (request_chain_id, txn_chain_id) {
        (None, Some(txn_id)) => Ok(txn_id),
        (Some(req_id), Some(txn_id)) if req_id == txn_id => Ok(txn_id),
        (req_id, txn_id) => Err(InvalidInput(ChainIdMismatched(
            req_id.map_or("none".to_string(), |id| id.to_string()),
            txn_id.map_or("none".to_string(), |id| id.to_string()),
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Extensions;
    use reqwest;
    use shared::{
        json_rpc::InvalidInputError::ChainIdMismatched, service_start_utils::MetricsState,
    };
    use std::{collections::HashMap, time::Duration};

    #[test]
    fn test_get_request_chain_id_valid_chain_id() {
        let mut extensions = Extensions::new();

        // Create a valid HashMap with the expected chain ID key and value
        let mut headers = HashMap::new();
        headers.insert(HEADER_CHAIN_ID.to_string(), "12345".to_string());
        extensions.insert::<HashMap<String, String>>(headers);

        // Ensure the function correctly parses the chain ID
        let result = get_request_chain_id(&extensions);
        assert_eq!(result, Some(12345u64));
    }

    #[test]
    fn test_get_request_chain_id_missing_key() {
        let mut extensions = Extensions::new();

        // Create a HashMap without the expected chain ID key
        let mut headers = HashMap::new();
        headers.insert("some-other-header".to_string(), "12345".to_string());
        extensions.insert::<HashMap<String, String>>(headers);

        // Ensure the function returns None when the chain ID key is missing
        let result = get_request_chain_id(&extensions);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_request_chain_id_invalid_value() {
        let mut extensions = Extensions::new();

        // Create a HashMap with an invalid value for the chain ID key
        let mut headers = HashMap::new();
        headers.insert(HEADER_CHAIN_ID.to_string(), "not-a-number".to_string());
        extensions.insert::<HashMap<String, String>>(headers);

        // Ensure the function returns None for an invalid chain ID value
        let result = get_request_chain_id(&extensions);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_request_chain_id_empty_extensions() {
        let extensions = Extensions::new();

        // Ensure the function returns None when Extensions is empty
        let result = get_request_chain_id(&extensions);
        assert_eq!(result, None);
    }

    #[test]
    fn test_validate_chain_id_no_request_id() {
        let result = validate_chain_id(None, Some(12345u64));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12345u64);
    }

    #[test]
    fn test_validate_chain_id_matching_ids() {
        let result = validate_chain_id(Some(12345u64), Some(12345u64));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12345u64);
    }

    #[test]
    fn test_validate_chain_id_mismatched_ids() {
        let result = validate_chain_id(Some(12345u64), Some(67890u64));
        assert!(matches!(result, Err(InvalidInput(ChainIdMismatched(_, _)))));
    }

    #[test]
    fn test_validate_chain_id_no_txn_id() {
        let result = validate_chain_id(Some(12345u64), None);
        assert!(matches!(result, Err(InvalidInput(ChainIdMismatched(_, _)))));
    }

    #[test]
    fn test_validate_chain_id_both_none() {
        let result = validate_chain_id(None, None);
        assert!(matches!(result, Err(InvalidInput(ChainIdMismatched(_, _)))));
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let mut metrics_state = MetricsState::default();
        let metrics = MaestroMetrics::new(&mut metrics_state.registry);
        let mut config = Config::default();

        // Start Valkey container for testing
        let (valkey, valkey_url) = test_utils::docker::start_valkey().await.unwrap();
        config.valkey_url = valkey_url;

        // Create and run server
        let (addr, shutdown_fn) = run(config, metrics).await.unwrap();

        // Create HTTP client
        let client = reqwest::Client::new();

        // Test health endpoint
        let response = client.get(format!("http://{}/health", addr)).send().await.unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(
            response.json::<JsonValue>().await.unwrap(),
            serde_json::json!({"health": true})
        );

        // Stop Valkey container
        drop(valkey);
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Test unhealthy health endpoint
        let response = client.get(format!("http://{}/health", addr)).send().await.unwrap();
        assert_eq!(response.status(), 500);
        // Cleanup
        shutdown_fn().await.unwrap();
    }
}
