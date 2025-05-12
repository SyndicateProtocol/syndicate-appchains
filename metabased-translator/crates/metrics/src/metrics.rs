//! The `metrics` module  handles metrics recording for the metabased translator
use ingestor::metrics::IngestorMetrics;
use prometheus_client::registry::Registry;
use slotter::metrics::SlotterMetrics;

/// Structure holding all metrics related to the translator.
#[derive(Debug, Clone)]
pub struct TranslatorMetrics {
    /// Metrics for the sequencing ingestor
    pub ingestor_sequencing: IngestorMetrics,
    /// Metrics for the settlement ingestor
    pub ingestor_settlement: IngestorMetrics,
    /// Metrics for the slotter
    pub slotter: SlotterMetrics,
}

impl TranslatorMetrics {
    /// Creates a new `TranslatorMetrics` instance and registers it in the provided Prometheus
    /// registry.
    pub fn new(registry: &mut Registry) -> Self {
        let ingestor_sequencing = IngestorMetrics::new(registry);
        let ingestor_settlement = IngestorMetrics::new(registry);
        let slotter = SlotterMetrics::new(registry);
        Self { ingestor_sequencing, ingestor_settlement, slotter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use common::types::Chain;
    use ingestor::metrics::MethodLabel;
    use reqwest::Client;
    use shared::service_start_utils::{start_metrics_and_health, MetricsState};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_record_last_block_fetched() {
        let mut registry = Registry::default();
        let ingestor_metrics = IngestorMetrics::new(&mut registry);
        ingestor_metrics.record_last_block_fetched(Chain::Settlement, 100);

        let gauge = ingestor_metrics
            .ingestor_last_block_fetched
            .get_or_create(&MethodLabel {
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
            .get_or_create(&MethodLabel { chain: Chain::Settlement.into(), method: "test_method" })
            .clone();
        assert_eq!(counter.get(), 1);
    }

    #[tokio::test]
    async fn test_start_metrics() {
        let metrics_state = MetricsState::default();
        let port = 9001;

        let handle = start_metrics_and_health(metrics_state, port).await;

        sleep(Duration::from_secs(1)).await;
        let client = Client::new();
        let response = client.get(format!("http://localhost:{}/metrics", port)).send().await;

        assert!(response.is_ok());
        let status = response.unwrap().status();
        assert_eq!(status, StatusCode::OK, "Unexpected status code: {:?}", status);

        handle.abort();
    }
}
