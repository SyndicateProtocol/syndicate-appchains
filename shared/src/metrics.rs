//! Shared `metrics` module
use crate::health::health_handler;
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
#[derive(Debug, Default)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

/// Starts a metrics server on the specified port, serving Prometheus-compatible metrics.
pub async fn start_metrics(metrics_state: MetricsState, port: u16) -> tokio::task::JoinHandle<()> {
    let state = Arc::new(RwLock::new(metrics_state));
    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health_handler))
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind metrics server: {}", e);
        }
    };

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            eprintln!("Metrics server error: {}", e);
            std::process::exit(1); // stop the process if the metrics server fails (this should
                                   // never happen)
        }
    })
}

/// Handler for the `/metrics` endpoint, encoding and returning the Prometheus metrics.
async fn metrics_handler(
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
