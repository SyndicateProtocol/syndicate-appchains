//! The Batcher service for the Maestro.

use crate::config::BatcherConfig;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use common::types::Transaction;
use eyre::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json;
use shared::json_rpc::{
    Error::{Internal, InvalidInput},
    InvalidInputError::UnsupportedChainId,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tracing::{debug, error, info};

// Implemented in Daniils PR
#[allow(missing_docs)]
#[derive(Debug, Clone)]

pub struct StreamManager {}
impl StreamManager {
    async fn read_next_batch(
        &self,
        _key: String,
        _batch_size: usize,
    ) -> Result<Option<Vec<Transaction>>> {
        Ok(Some(vec![])) // dummy for now
    }
}

/// Batcher service
#[derive(Debug, Clone)]
pub struct Batcher {
    /// The RPC URLs for the chains
    pub chain_rpc_urls: HashMap<String, String>,
    /// The batch size for the batcher
    pub batch_size: usize,
    /// The Redis client for the batcher
    pub redis_client: StreamManager,
    /// The stream key for the batcher
    pub stream_key: String,
    /// The sequencer client for the batcher
    pub sequencer_client: Client,
}

/// The request for batching transactions
#[derive(Deserialize, Debug)]
pub struct BatchRequest {
    /// The chain ID for the batch
    pub chain_id: u64,
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: BatcherConfig) -> Result<()> {
    let batcher = Arc::new(Batcher::new(config.clone()));

    let app = Router::new()
        .route(
            "/batch",
            post(|State(batcher): State<Arc<Batcher>>, Json(req): Json<BatchRequest>| async move {
                handle_batch_request(State(batcher), Json(req)).await
            }),
        )
        .with_state(batcher);
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;

    info!("Starting batcher server on port {}", config.port);
    axum_server::bind(addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn handle_batch_request(
    State(batcher): State<Arc<Batcher>>,
    Json(req): Json<BatchRequest>,
) -> impl IntoResponse {
    match batcher.read_transactions_and_send_batch(req).await {
        Ok(json) => json.into_response(),
        Err(e) => {
            error!("handler error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to process batch request").into_response()
        }
    }
}

impl Batcher {
    /// Create a new instance of the Maestro service
    pub fn new(config: BatcherConfig) -> Self {
        info!("Initialized Batcher with RPC urls: {:?}", config.chain_rpc_urls);
        let redis_client = StreamManager {};
        let sequencer_client = Client::new();
        Self {
            chain_rpc_urls: config.chain_rpc_urls,
            batch_size: config.batch_size,
            redis_client,
            stream_key: "txs:".to_string(),
            sequencer_client,
        }
    }
    async fn read_transactions_and_send_batch(
        &self,
        req: BatchRequest,
    ) -> Result<Json<Vec<Transaction>>, shared::json_rpc::Error> {
        info!("Received batch request for chain ID: {}", req.chain_id);

        let stream_group = format!("{}{}", self.stream_key, req.chain_id);

        match self.redis_client.read_next_batch(stream_group, self.batch_size).await {
            Ok(Some(transactions)) => {
                info!("Batch read successfully");
                let batch = self.batch_transactions(transactions);
                if let Err(e) = self.send_batch(batch.clone(), req.chain_id).await {
                    error!("Error sending batch: {}", e);
                    return Err(e);
                }
                Ok(Json(batch))
            }
            Ok(None) => {
                info!("No batch found");
                Ok(Json(vec![]))
            }
            Err(e) => {
                error!("Error reading batch from Redis: {}", e);
                Err(Internal("failed to read batch".into()))
            }
        }
    }

    fn batch_transactions(&self, transactions: Vec<Transaction>) -> Vec<Transaction> {
        transactions.into_iter().take(self.batch_size).collect()
    }

    async fn send_batch(
        &self,
        batch: Vec<Transaction>,
        chain_id: u64,
    ) -> Result<(), shared::json_rpc::Error> {
        // JSON-RPC request payload for eth_sendBatch
        let batch_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendBatch",
            "params": [batch],
            "id": 1
        });

        let rpc_url = self.chain_rpc_urls.get(&chain_id.to_string()).ok_or_else(|| {
            error!(%chain_id, "batch attempted to unsupported RPC");
            InvalidInput(UnsupportedChainId(chain_id))
        })?;

        // Fire and forget
        let response = self
            .sequencer_client
            .post(rpc_url)
            .header("Content-Type", "application/json")
            .json(&batch_payload)
            .send()
            .await
            .map_err(|e| {
                error!(%rpc_url, %chain_id, %e, "reqwest client error forwarding txn to RPC");
                Internal("internal error sending batch".to_string())
            })?;

        if !response.status().is_success() {
            error!(
                %rpc_url,
                %chain_id,
                status = %response.status(),
                "RPC returned non-200 status for forwarded batch"
            );
            return Err(Internal(format!("RPC endpoint returned status {}", response.status())));
        }

        let response_body = response.text().await.map_err(|e| {
            error!(%rpc_url, %chain_id, %e, "Failed to read response body from RPC");
            Internal("Failed to read response from RPC endpoint".to_string())
        })?;

        let json_response: serde_json::Value =
            serde_json::from_str(&response_body).map_err(|e| {
                error!(%rpc_url, %chain_id, %e, "Failed to parse JSON response from RPC");
                Internal("Invalid JSON response from RPC endpoint".to_string())
            })?;

        if let Some(error) = json_response.get("error") {
            error!(%rpc_url, %chain_id, error = ?error, "RPC returned JSON-RPC error");
            return Err(Internal(format!("RPC endpoint returned error: {}", error)));
        }

        debug!(%rpc_url, %chain_id, "Successfully forwarded batch to RPC");
        Ok(())
    }
}
