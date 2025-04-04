//! The JSON-RPC server module for the Maestro sequencer.

use crate::{
    errors::{Error, Error::Internal},
    layers::HeadersLayer,
    redis_manager::{StreamManager, TransactionRequest},
};
use alloy::primitives::Bytes;
use http::Extensions;
use jsonrpsee::{
    core::RpcResult,
    server::{middleware::http::ProxyGetRequestLayer, Server, ServerHandle},
    types::{ErrorCode, Params},
    RpcModule,
};
use redis::aio::MultiplexedConnection;
use serde_json::Value as JsonValue;
use shared::json_rpc::parse_send_raw_transaction_params;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tower::ServiceBuilder;
use tracing::{error, info};

/// Runs the base server for the sequencer
pub async fn run(
    port: i32,
    redis_conn: Option<MultiplexedConnection>,
) -> eyre::Result<(SocketAddr, ServerHandle)> {
    let optional_headers = vec!["x-synd-chain-id".to_string()];
    let http_middleware = ServiceBuilder::new()
        .layer(HeadersLayer::new(optional_headers)?)
        .layer(ProxyGetRequestLayer::new("/health", "health")?)
        .layer(ProxyGetRequestLayer::new("/test_redis", "test_redis")?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", port))
        .await?;

    let service = MaestroService::new(redis_conn);
    let mut module = RpcModule::new(service);

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler)?;
    module.register_async_method(
        "test_redis",
        |_, service: Arc<MaestroService>, _| async move {
            service.test_redis().await.map_err(|e| {
                error!("Test Redis error: {:?}", e);
                shared::json_rpc::Error::Internal("Failed to test Redis connection".to_string())
                    .to_json_rpc_error()
            })
        },
    )?;

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

    let _tx_data = parse_send_raw_transaction_params(params)?;

    // TODO spam plane

    // TODO control plane

    // TODO return real hash
    // let tx_hash = service.process_transaction(tx_data).await?;
    let tx_hash = Bytes::from("1234");
    Ok(format!("0x{}", alloy::hex::encode(tx_hash)))

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
pub struct MaestroService {
    redis_conn: Option<MultiplexedConnection>,
}

impl MaestroService {
    /// Create a new instance of the Maestro service
    pub const fn new(redis_conn: Option<MultiplexedConnection>) -> Self {
        Self { redis_conn }
    }

    /// TODO delete me
    /// func to test the redis connection
    pub async fn test_redis(&self) -> Result<JsonValue, Error> {
        // Check if we have a Redis connection
        let redis_conn = match &self.redis_conn {
            Some(conn) => conn.clone(),
            None => return Err(Internal("Redis not provided".to_string())),
        };

        // Initialize stream manager
        let mut stream_manager = StreamManager::new(redis_conn.clone());

        // Initialize the consumer group
        let consumer_group_name = stream_manager.init_consumer_group().await?;

        // Example transaction enqueue (for testing)
        let example_tx = TransactionRequest {
            tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            sender: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
            raw_tx: "0x02f8b303808459682f0085059ba5a4dc82520894a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880de0b6b3a764000080c080a0a7a56c53a21c1f0b636f222b5bf148c15ecec50e7a9425c0ebb681b61a5c9b7a0144d7a2d62f771de4225cce7c8463f35848ff6c10f4f791b8ed0337a06f0fd65".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs(),
        };
        let mut producer = stream_manager.create_producer()?;
        let tx_id = producer.enqueue_transaction(example_tx).await?;

        info!("Enqueued example transaction with ID: {}", tx_id);

        // Start processing in a separate task
        let mut consumer =
            stream_manager.create_consumer_with_group(consumer_group_name.as_str(), 1)?;
        tokio::spawn(async move {
            match consumer.start_processing_loop().await {
                Ok(_) => unreachable!("Processing loop should run indefinitely"),
                Err(e) => error!("Stream processing loop failed: {}", e),
            }
        });
        // Return a success JSON response
        Ok(serde_json::json!({
            "success": true,
            "message": "Redis test completed successfully",
            "transaction_id": tx_id
        }))
    }
}
