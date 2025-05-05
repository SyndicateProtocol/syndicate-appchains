//! The `metrics` module  handles metrics recording for the metabased translator
use prometheus_client::registry::Registry;
use slotter::metrics::SlotterMetrics;

/// Structure holding all metrics related to the translator.
#[derive(Debug, Clone)]
pub struct TranslatorMetrics {
    /// Metrics for the slotter
    pub slotter: SlotterMetrics,
}

impl TranslatorMetrics {
    /// Creates a new `TranslatorMetrics` instance and registers it in the provided Prometheus
    /// registry.
    pub fn new(registry: &mut Registry) -> Self {
        let slotter = SlotterMetrics::new(registry);
        Self { slotter }
    }
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
