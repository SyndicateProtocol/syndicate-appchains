//! The JSON-RPC server module for the Maestro service.

use crate::{config::Config, layers::HeadersLayer, maestro::MaestroService};
use alloy::{consensus::Transaction, primitives::ChainId};
use http::Extensions;
use jsonrpsee::{
    core::RpcResult,
    server::{middleware::http::ProxyGetRequestLayer, Server, ServerHandle},
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
    tx_validation::validate_transaction,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tracing::info;

/// Request header for `eth_sendRawTransaction` calls that holds the intended `chain_id`
pub const HEADER_CHAIN_ID: &str = "x-synd-chain-id";

/// Runs the base server for the sequencer
pub async fn run(config: Config) -> eyre::Result<(SocketAddr, ServerHandle)> {
    info!("Starting Maestro server:run");

    let optional_headers = vec![HEADER_CHAIN_ID.to_string()];
    let http_middleware = ServiceBuilder::new()
        .layer(HeadersLayer::new(optional_headers)?)
        .layer(ProxyGetRequestLayer::new("/health", "health")?)
        .layer(ProxyGetRequestLayer::new("/test_redis", "test_redis")?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", config.port))
        .await?;

    let client = redis::Client::open(config.redis_url.as_str())?;
    let redis_conn = client.get_multiplexed_async_connection().await?;
    let service = MaestroService::new(redis_conn, config).await?;
    info!("Connected to Redis successfully!");

    let mut module = RpcModule::new(service);

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler)?;

    // Register health method (this will be hit by the health check middleware)
    module.register_method("health", |_, _, _| {
        Ok::<JsonValue, ErrorCode>(serde_json::json!({"health": true}))
    })?;

    // TODO (SEQ-908): Background job to unstick `waiting txn` cache

    info!("Registered RPC methods: {:#?}", module.method_names().collect::<Vec<_>>());

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

/// The handler for the `eth_sendRawTransaction` JSON-RPC method
pub async fn send_raw_transaction_handler(
    params: Params<'static>,
    service: Arc<MaestroService>,
    extensions: Extensions,
) -> RpcResult<String> {
    let raw_tx = parse_send_raw_transaction_params(params)?;
    let (tx, signer) = validate_transaction(&raw_tx)?;
    let chain_id = validate_chain_id(get_request_chain_id(extensions), tx.chain_id())?;
    let tx_nonce = tx.nonce();
    let tx_hash = format!("0x{}", alloy::hex::encode(tx.hash()));

    info!(
        %tx_hash,
        %chain_id,
        "Submitting validated transaction",
    );

    service.handle_transaction_and_manage_nonces(raw_tx, signer, chain_id, tx_nonce).await?;

    info!(%tx_hash, %chain_id, "Submitted forwarded transaction");
    Ok(tx_hash)
}

fn get_request_chain_id(extensions: Extensions) -> Option<ChainId> {
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
    use shared::json_rpc::InvalidInputError::ChainIdMismatched;
    use std::collections::HashMap;

    #[test]
    fn test_get_request_chain_id_valid_chain_id() {
        let mut extensions = Extensions::new();

        // Create a valid HashMap with the expected chain ID key and value
        let mut headers = HashMap::new();
        headers.insert(HEADER_CHAIN_ID.to_string(), "12345".to_string());
        extensions.insert::<HashMap<String, String>>(headers);

        // Ensure the function correctly parses the chain ID
        let result = get_request_chain_id(extensions);
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
        let result = get_request_chain_id(extensions);
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
        let result = get_request_chain_id(extensions);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_request_chain_id_empty_extensions() {
        let extensions = Extensions::new();

        // Ensure the function returns None when Extensions is empty
        let result = get_request_chain_id(extensions);
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
}
