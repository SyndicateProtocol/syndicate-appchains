//! The `metrics` module  handles metrics recording for the metabased translator

use crate::errors::Error;
use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use prometheus_client::{
    encoding::{text::encode, EncodeLabelSet},
    metrics::{
        counter::Counter,
        family::Family,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;

/// Labels for the metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    rpc_method: &'static str,
    error_category: &'static str,
}

/// Structure holding metrics related to blockchain data ingestion.
#[derive(Debug)]
pub struct RelayerMetrics {
    /// Records rpc calls
    pub relayer_rpc_calls: Family<Labels, Counter>,
    /// Records rpc calls latency
    pub relayer_rpc_calls_latency: Family<Labels, Histogram>,
}

impl RelayerMetrics {
    /// Create a new `RelayerMetrics` instance.
    pub fn new(registry: &mut Registry) -> Self {
        let relayer_rpc_calls = Family::<Labels, Counter>::default();
        registry.register(
            "relayer_rpc_calls",
            "Number of RPC method calls received",
            relayer_rpc_calls.clone(),
        );
        let relayer_rpc_calls_latency = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "relayer_rpc_calls_latency",
            "Latency of RPC method calls responses",
            relayer_rpc_calls_latency.clone(),
        );

        Self { relayer_rpc_calls, relayer_rpc_calls_latency }
    }

    /// Records an RPC call event, incrementing counters and measuring duration.
    pub fn record_rpc_call(
        &self,
        method: &'static str,
        duration: Duration,
        error_category: Option<&Error>,
    ) {
        let error_category = error_to_metric_category(error_category);
        self.relayer_rpc_calls.get_or_create(&Labels { rpc_method: method, error_category }).inc();
        self.relayer_rpc_calls_latency
            .get_or_create(&Labels { rpc_method: method, error_category })
            .observe(duration.as_secs_f64());
    }
}

/// Grouping errors into categories to reduce Prometheus metric cardinality
pub fn error_to_metric_category(error: Option<&Error>) -> &'static str {
    error.map_or("none", |error| match error {
        Error::InvalidRequest | Error::Parse | Error::InvalidInput(_) => "validation_error",
        Error::MethodNotFound(_) | Error::MethodNotSupported => "method_error",
        Error::ResourceNotFound | Error::ResourceUnavailable => "resource_error",
        Error::Internal | Error::Server => "server_error",
        Error::Contract(_) => "contract_error",
        Error::InvalidParams(_) => "params_error",
        Error::TransactionRejected(_) => "tx_error",
        Error::LimitExceeded => "limit_error",
    })
}

/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

impl MetricsState {
    /// Create a new `MetricsState` instance with a default Prometheus registry.
    pub fn new() -> Self {
        let registry = Registry::default();
        Self { registry }
    }
}

impl Default for MetricsState {
    fn default() -> Self {
        Self::new()
    }
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
            panic!("Failed to bind metrics server: {}", e);
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
    use reqwest::Client;
    use std::time::Duration;
    use tokio::time::sleep;

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
