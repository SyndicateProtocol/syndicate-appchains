//! The `metrics` module  handles metrics recording for the metabased translator

use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use prometheus_client::{encoding::text::encode, registry::Registry};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

impl MetricsState {
    pub fn new() -> Self {
        let registry = Registry::default();
        Self { registry }
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
