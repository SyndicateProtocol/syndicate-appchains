//! The JSON-RPC server module for the Maestro sequencer.

use crate::{config::Config, constants::HEADER_CHAIN_ID, layers::HeadersLayer};
use alloy::{
    consensus::Transaction,
    hex,
    primitives::{Bytes, ChainId, TxHash},
    transports::http::Client,
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
        parse_send_raw_transaction_params, Error,
        Error::{Internal, InvalidInput},
        InvalidInputError::{ChainIdMismatched, UnsupportedChainId},
    },
    tx_validation::validate_transaction,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tracing::{debug, error, info, warn};

/// Runs the base server for the sequencer
pub async fn run(config: Config) -> eyre::Result<(SocketAddr, ServerHandle)> {
    info!("Starting Maestro server:run");

    let optional_headers = vec!["x-synd-chain-id".to_string()];
    let http_middleware = ServiceBuilder::new()
        .layer(HeadersLayer::new(optional_headers)?)
        .layer(ProxyGetRequestLayer::new("/health", "health")?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", config.port))
        .await?;

    let client = Client::builder().timeout(config.validation_timeout).build()?;

    let service = MaestroService::new(config, client);
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
) -> RpcResult<JsonValue> {
    // let start = Instant::now();

    info!("Entered send_raw_transaction_handler_v1");

    let tx_data = parse_send_raw_transaction_params(params)?;
    info!("parse params succeeded");
    let request_chain_id = get_request_chain_id(extensions);
    info!("got request chain id");
    let tx_hash = service.process_raw_transaction_v1(tx_data, request_chain_id).await?;
    info!(%tx_hash, "got txhash");

    Ok(serde_json::json!({
        "jsonrpc": "2.0",
        "result": format!("0x{}", hex::encode(tx_hash)),
        "id": 1
    }))

    // TODO spam plane

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
pub struct MaestroService {
    chain_id_nitro_urls: HashMap<String, String>,
    client: Client,
}

impl MaestroService {
    /// Create a new instance of the Maestro service
    pub fn new(config: Config, client: Client) -> Self {
        Self { chain_id_nitro_urls: config.chain_rpc_urls, client }
    }

    async fn process_raw_transaction_v1(
        &self,
        raw_tx: Bytes,
        request_chain_id: Option<ChainId>,
    ) -> Result<TxHash, Error> {
        let hex_tx = format!("0x{}", hex::encode(&raw_tx));
        info!("Processing raw transaction: {}", hex_tx);
        let original_tx = validate_transaction(&raw_tx)?;
        let txn_chain_id = Self::validate_chain_id(request_chain_id, original_tx.chain_id())?;
        let tx_hash = original_tx.tx_hash().to_string();
        debug!(
            %tx_hash,
            %txn_chain_id,
            "Submitting validated transaction to Nitro",
        );

        // JSON-RPC request payload for eth_sendRawTransaction
        let raw_txn_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendRawTransaction",
            "params": [hex_tx],
            "id": 1
        });

        let nitro_url =
            self.chain_id_nitro_urls.get(&txn_chain_id.to_string()).ok_or_else(|| {
                error!(%txn_chain_id, %tx_hash, "txn attempted to unsupported Nitro RPC");
                InvalidInput(UnsupportedChainId(txn_chain_id))
            })?;

        // Fire and forget
        let response = self
            .client
            .post(nitro_url)
            .header("Content-Type", "application/json")
            .json(&raw_txn_payload)
            .send()
            .await
            .map_err(|e| {
                error!(%nitro_url, %tx_hash, %txn_chain_id, %e, "reqwest client error forwarding txn to Nitro");
                Internal("internal error sending transaction".to_string())
            })?;

        if !response.status().is_success() {
            warn!(%nitro_url, %tx_hash, %txn_chain_id, "Nitro returned non-200 status for forwarded txn");
        }

        Ok(*original_tx.tx_hash())
    }

    /// Returns `txn_chain_id` unwrapped if valid
    fn validate_chain_id(
        request_chain_id: Option<ChainId>,
        txn_chain_id: Option<ChainId>,
    ) -> Result<ChainId, Error> {
        match (request_chain_id, txn_chain_id) {
            (None, Some(txn_id)) => Ok(txn_id),
            (Some(req_id), Some(txn_id)) if req_id == txn_id => Ok(txn_id),
            (req_id, txn_id) => Err(InvalidInput(ChainIdMismatched(
                req_id.map_or("none".to_string(), |id| id.to_string()),
                txn_id.map_or("none".to_string(), |id| id.to_string()),
            ))),
        }
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
