//! The `metrics` module  handles metrics recording for the metabased translator

use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use block_builder::metrics::BlockBuilderMetrics;
use ingestor::metrics::IngestorMetrics;
use prometheus_client::{encoding::text::encode, registry::Registry};
use slotting::metrics::SlottingMetrics;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Structure holding all metrics related to the translator.
#[derive(Debug)]
pub struct TranslatorMetrics {
    /// Metrics for the sequencing ingestor
    pub ingestor_sequencing: IngestorMetrics,
    /// Metrics for the settlement ingestor
    pub ingestor_settlement: IngestorMetrics,
    /// Metrics for the slotting
    pub slotting: SlottingMetrics,
    /// Metrics for the block builder
    pub block_builder: BlockBuilderMetrics,
}

impl TranslatorMetrics {
    /// Creates a new `TranslatorMetrics` instance and registers it in the provided Prometheus
    /// registry.
    pub fn new(registry: &mut Registry) -> Self {
        let ingestor_sequencing = IngestorMetrics::new(registry);
        let ingestor_settlement = IngestorMetrics::new(registry);
        let slotting = SlottingMetrics::new(registry);
        let block_builder = BlockBuilderMetrics::new(registry);
        Self { ingestor_sequencing, ingestor_settlement, slotting, block_builder }
    }
}

/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

/// Handler for the `/metrics` endpoint, encoding and returning the Prometheus metrics.
pub async fn metrics_handler(
    State(state): State<Arc<RwLock<MetricsState>>>,
) -> Result<Response<Body>, StatusCode> {
    let buffer = {
        let state = state.read().await;
        let mut buffer = String::new();
        encode(&mut buffer, &state.registry).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        buffer
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain; version=1.0.0; charset=utf-8")
        .body(Body::from(buffer))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Starts a metrics server on the specified port, serving Prometheus-compatible metrics.
pub async fn start_metrics(metrics_state: MetricsState, port: u16) -> tokio::task::JoinHandle<()> {
    let state = Arc::new(RwLock::new(metrics_state));
    let router = Router::new().route("/metrics", get(metrics_handler)).with_state(state);

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind metrics server: {}", e);
            return tokio::spawn(async {});
        }
    };

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            eprintln!("Metrics server error: {}", e);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use common::types::Chain;
    use ingestor::metrics::Labels;
    use reqwest::Client;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_record_last_block_fetched() {
        let mut registry = Registry::default();
        let ingestor_metrics = IngestorMetrics::new(&mut registry);
        ingestor_metrics.record_last_block_fetched(Chain::Settlement, 100);

        let gauge = ingestor_metrics
            .ingestor_last_block_fetched
            .get_or_create(&Labels {
                chain: Chain::Settlement.into(),
                method: "last_block_fetched",
            })
            .clone();
        assert_eq!(gauge.get(), 100);
    }

    #[tokio::test]
    async fn test_record_rpc_call() {
        let mut registry = Registry::default();
        let ingestor_metrics = IngestorMetrics::new(&mut registry);

        ingestor_metrics.record_rpc_call(
            Chain::Settlement,
            "test_method",
            Duration::from_millis(500),
        );

        let counter = ingestor_metrics
            .ingestor_rpc_calls
            .get_or_create(&Labels { chain: Chain::Settlement.into(), method: "test_method" })
            .clone();
        assert_eq!(counter.get(), 1);
    }

    #[tokio::test]
    async fn test_start_metrics() {
        let registry = Registry::default();
        let metrics_state = MetricsState { registry };
        let port = 9001;

        let handle = start_metrics(metrics_state, port).await;

        sleep(Duration::from_secs(1)).await;
        let client = Client::new();
        let response = client.get(format!("http://localhost:{}/metrics", port)).send().await;

        assert!(response.is_ok());
        let status = response.unwrap().status();
        assert_eq!(status, StatusCode::OK, "Unexpected status code: {:?}", status);

        handle.abort();
    }
}
