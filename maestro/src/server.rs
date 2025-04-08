//! The JSON-RPC server module for the Maestro sequencer.

use crate::{constants::HEADER_CHAIN_ID, layers::HeadersLayer};
use alloy::{
    consensus::{Transaction, TxEnvelope},
    hex,
    primitives::{Bytes, ChainId, TxHash},
};
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
        parse_send_raw_transaction_params, Error, InvalidInputError,
        InvalidInputError::ChainIDMismatched,
    },
    tx_validation::validate_transaction,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tracing::{debug, info};

/// Runs the base server for the sequencer
pub async fn run(port: i32) -> eyre::Result<(SocketAddr, ServerHandle)> {
    let optional_headers = vec!["x-synd-chain-id".to_string()];
    let http_middleware = ServiceBuilder::new()
        .layer(HeadersLayer::new(optional_headers)?)
        .layer(ProxyGetRequestLayer::new("/health", "health")?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", port))
        .await?;

    let service = MaestroService::new();
    let mut module = RpcModule::new(service);

    // Register RPC methods
    module.register_async_method("eth_sendRawTransaction", send_raw_transaction_handler_v1)?;

    // Register health method (this will be hit by the health check middleware)
    module.register_method("health", |_, _, _| {
        Ok::<JsonValue, ErrorCode>(serde_json::json!({"health": true}))
    })?;

    info!("Registered RPC methods: {:#?}", module.method_names().collect::<Vec<_>>());

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

/// The `v0` handler for the `eth_sendRawTransaction` JSON-RPC method. This forwards transactions
/// to Arbitrum Nitro instances
pub async fn send_raw_transaction_handler_v1(
    params: Params<'static>,
    service: Arc<MaestroService>,
    extensions: Extensions,
) -> RpcResult<String> {
    // let start = Instant::now();

    let tx_data = parse_send_raw_transaction_params(params)?;
    let request_chain_id = get_request_chain_id(extensions);

    let result = async {
        let tx_hash = service.process_raw_transaction(tx_data, request_chain_id).await?;
        Ok::<_, Error>(format!("0x{}", hex::encode(tx_hash)))
    }
    .await;

    result.map_err(|e| e.to_json_rpc_error())

    // TODO spam plane

    // TODO control plane

    // TODO return real hash
    // let tx_hash = service.process_transaction(tx_data).await?;
    // Ok(format!("0x{}", alloy::hex::encode(tx_hash)))

    // let duration = start.elapsed();
    //
    // result
    //     .inspect(|_| service.metrics.record_rpc_call("eth_sendRawTransaction", duration, None))
    //     .map_err(|e| {
    //         service.metrics.record_rpc_call("eth_sendRawTransaction", duration, Some(&e));
    //         e.to_json_rpc_error()
    //     })
}

fn get_request_chain_id(extensions: Extensions) -> Option<ChainId> {
    extensions
        .get::<HashMap<String, String>>()
        .and_then(|map| map.get(HEADER_CHAIN_ID))
        .and_then(|chain_id| chain_id.parse::<ChainId>().ok())
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

    async fn process_raw_transaction(
        &self,
        raw_tx: Bytes,
        request_chain_id: Option<ChainId>,
    ) -> Result<TxHash, Error> {
        info!("Processing raw transaction: {}", hex::encode(&raw_tx));
        let original_tx = validate_transaction(&raw_tx)?;
        Self::check_chain_ids_from_req_and_txn(request_chain_id, original_tx.clone())?;

        debug!("Submitting validated transaction to Nitro");

        // TODO (SEQ-782): send to Nitro
        Ok(*original_tx.tx_hash())
    }

    fn check_chain_ids_from_req_and_txn(
        request_chain_id: Option<ChainId>,
        original_tx: TxEnvelope,
    ) -> Result<(), Error> {
        #[allow(clippy::expect_used)]
        let txn_chain_id = original_tx.chain_id().expect("validated txn should have chain id");
        if let Some(req_chain_id) = request_chain_id {
            if req_chain_id != txn_chain_id {
                return Err(Error::InvalidInput(ChainIDMismatched(
                    req_chain_id.to_string(),
                    txn_chain_id.to_string(),
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Extensions;
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
}
