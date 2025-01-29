//! The `metrics` module  handles metrics recording for the metabased translator

use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use prometheus_client::{
    encoding::{text::encode, EncodeLabelSet},
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpListener, spawn, sync::RwLock, task::JoinHandle};

/// Structure holding all metrics related to the translator.
#[derive(Debug)]
pub struct TranslatorMetrics {
    /// Metrics for the sequencing ingestor
    pub ingestor_sequencing: IngestorMetrics,
    /// Metrics for the settlement ingestor
    pub ingestor_settlement: IngestorMetrics,
}

impl TranslatorMetrics {
    /// Creates a new `TranslatorMetrics` instance and registers it in the provided Prometheus
    /// registry.
    pub fn new(registry: &mut Registry) -> Self {
        let ingestor_sequencing = IngestorMetrics::new(registry);
        let ingestor_settlement = IngestorMetrics::new(registry);
        Self { ingestor_sequencing, ingestor_settlement }
    }
}

/// Labels used for Prometheus metric categorization.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    label_name: String,
    method: &'static str,
}

/// Structure holding metrics related to blockchain data ingestion.
#[derive(Debug)]
pub struct IngestorMetrics {
    rpc_calls: Family<Labels, Counter>,
    rpc_calls_duration: Family<Labels, Histogram>,
    last_block_fetched: Family<Labels, Gauge>,
}

/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

impl IngestorMetrics {
    /// Creates a new `IngestorMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let rpc_calls = Family::<Labels, Counter>::default();
        registry.register("rpc_calls", "Number of RPC method calls done", rpc_calls.clone());

        let rpc_calls_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "rpc_calls_latency",
            "Latency of RPC method call responses",
            rpc_calls_duration.clone(),
        );

        let last_block_fetched = Family::<Labels, Gauge>::default();
        registry.register(
            "last_block_fetched",
            "Tracks the last block number fetched for a specific RPC URL",
            last_block_fetched.clone(),
        );

        Self { rpc_calls, rpc_calls_duration, last_block_fetched }
    }

    /// Records the last block number fetched for a given label.
    pub fn record_last_block_fetched(&self, label_name: String, block_number: u64) {
        self.last_block_fetched
            .get_or_create(&Labels { label_name, method: "last_block_fetched" })
            .set(block_number as i64);
    }

    /// Records an RPC call event, incrementing counters and measuring duration.
    pub fn record_rpc_call(&self, label_name: String, method: &'static str, duration: Duration) {
        let name = label_name.clone();
        // Increment the counter for the RPC method call
        self.rpc_calls.get_or_create(&Labels { label_name: name, method }).inc();

        // Observe the latency of the RPC method call
        self.rpc_calls_duration
            .get_or_create(&Labels { label_name, method })
            .observe(duration.as_secs_f64());
    }
}

/// Handler for the `/metrics` endpoint, encoding and returning the Prometheus metrics.
pub async fn metrics_handler(State(state): State<Arc<RwLock<MetricsState>>>) -> impl IntoResponse {
    let mut buffer = String::new();

    let value = encode(&mut buffer, &state.read().await.registry);
    if let Err(err) = value {
        eprintln!("Failed to encode metrics: {}", err);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to encode metrics"))
            .unwrap_or_else(|_| Response::new(Body::from("Error encoding metrics state")))
    }

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(Body::from(buffer))
        .unwrap_or_else(|_| Response::new(Body::from("Error unwrapping metrics response")))
}

/// Starts a metrics server on the specified port, serving Prometheus-compatible metrics.
pub async fn start_metrics(
    metrics_state: MetricsState,
    port: u16,
) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
    let state = Arc::new(RwLock::new(metrics_state));
    let router = Router::new().route("/metrics", get(metrics_handler)).with_state(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.map_err(|e| {
        eprintln!("Failed to bind to port {}: {}", port, e);
        e
    })?;

    let metrics_task: JoinHandle<()> = spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            eprintln!("Metrics server failed: {}", e);
        }
    });

    Ok(metrics_task)
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use reqwest::Client;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_record_last_block_fetched() {
        let mut registry = Registry::default();
        let ingestor_metrics = IngestorMetrics::new(&mut registry);
        ingestor_metrics.record_last_block_fetched("test_label".to_string(), 100);

        let gauge = ingestor_metrics
            .last_block_fetched
            .get_or_create(&Labels {
                label_name: "test_label".to_string(),
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
            "test_label".to_string(),
            "test_method",
            Duration::from_millis(500),
        );

        let counter = ingestor_metrics
            .rpc_calls
            .get_or_create(&Labels { label_name: "test_label".to_string(), method: "test_method" })
            .clone();
        assert_eq!(counter.get(), 1);
    }

    #[tokio::test]
    async fn test_start_metrics() {
        let registry = Registry::default();
        let metrics_state = MetricsState { registry };
        let port = 9001;

        let handle =
            start_metrics(metrics_state, port).await.expect("Failed to start metrics server");

        sleep(Duration::from_secs(1)).await;
        let client = Client::new();
        let response = client.get(format!("http://localhost:{}/metrics", port)).send().await;

        assert!(response.is_ok());
        let status = response.unwrap().status();
        assert_eq!(status, StatusCode::OK, "Unexpected status code: {:?}", status);

        handle.abort();
    }
}
