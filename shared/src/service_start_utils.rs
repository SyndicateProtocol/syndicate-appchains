//! Shared utilities for starting services
use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, MethodRouter},
    Router,
};
use prometheus_client::{encoding::text::encode, registry::Registry};
use std::sync::Arc;
use tracing::info;
/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug, Default)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

/// Starts a server on the specified port, serving
/// Prometheus-compatible metrics and a health check endpoint.
pub async fn start_metrics_and_health(
    metrics_state: MetricsState,
    port: u16,
    health_handler: Option<MethodRouter<Arc<MetricsState>>>,
) -> tokio::task::JoinHandle<()> {
    info!("starting metrics on port {}", port);
    let state = Arc::new(metrics_state);
    let health_route =
        health_handler.map_or_else(|| get(default_health_handler), |handler| handler);
    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", health_route)
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
    State(state): State<Arc<MetricsState>>,
) -> Result<Response<Body>, StatusCode> {
    let buffer = {
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

/// Handler for the `/health` endpoint, returning a simple 200 OK and JSON response.
pub async fn default_health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "health": true }))
}
