//! The `metrics` module  handles metrics recording for the metabased translator

use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        counter::Counter,
        family::Family,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use shared::json_rpc::RpcError;
use std::time::Duration;

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
        error_category: Option<&RpcError>,
    ) {
        let error_category = error_to_metric_category(error_category);
        self.relayer_rpc_calls.get_or_create(&Labels { rpc_method: method, error_category }).inc();
        self.relayer_rpc_calls_latency
            .get_or_create(&Labels { rpc_method: method, error_category })
            .observe(duration.as_secs_f64());
    }
}

/// Grouping errors into categories to reduce Prometheus metric cardinality
pub fn error_to_metric_category(error: Option<&RpcError>) -> &'static str {
    error.map_or("none", |error| match error {
        RpcError::InvalidRequest | RpcError::Parse | RpcError::InvalidInput(_) => {
            "validation_error"
        }
        RpcError::MethodNotFound(_) | RpcError::MethodNotSupported => "method_error",
        RpcError::ResourceNotFound | RpcError::ResourceUnavailable => "resource_error",
        RpcError::Internal(_) | RpcError::Server => "server_error",
        RpcError::Contract(_) => "contract_error",
        RpcError::InvalidParams(_) => "params_error",
        RpcError::TransactionRejected(_) => "tx_error",
        RpcError::LimitExceeded => "limit_error",
    })
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use reqwest::Client;
    use shared::metrics::{start_metrics, MetricsState};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_start_metrics() {
        let metrics_state = MetricsState::default();
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
