//! Shared utilities for starting services
use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, MethodRouter},
    Router,
};
use jsonrpsee::{core::JsonValue, types::ErrorCode};
use prometheus_client::{encoding::text::encode, registry::Registry};
use redis::{aio::ConnectionManager, AsyncCommands};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tracing::{error, info};

/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug, Default)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

/// Starts a server on the specified port, serving Prometheus-compatible metrics, health check, and
/// readiness endpoints.
pub async fn start_http_server_with_aux_handlers(
    metrics_state: MetricsState,
    port: u16,
    health_handler: Option<MethodRouter<Arc<MetricsState>>>,
    ready_handler: Option<MethodRouter<Arc<MetricsState>>>,
) -> tokio::task::JoinHandle<()> {
    info!("starting http server on port {port}");
    let state = Arc::new(metrics_state);

    let health_route = health_handler.unwrap_or_else(|| get(default_health_handler));
    let ready_route = ready_handler.unwrap_or_else(|| get(default_ready_handler));

    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", health_route)
        .route("/ready", ready_route)
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind service server: {e}");
        }
    };

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            error!("router server error: {e}");
            std::process::exit(1);
        }
    })
}

/// Starts a server on the specified port, serving Prometheus-compatible metrics. This is useful
/// for JSON-RPC services which serve their other handlers via RPC methods.
pub async fn start_http_server_with_metrics_only(
    metrics_state: MetricsState,
    port: u16,
) -> tokio::task::JoinHandle<()> {
    info!("starting http server on port {port}");
    let state = Arc::new(metrics_state);

    let router = Router::new().route("/metrics", get(metrics_handler)).with_state(state);

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind service server: {e}");
        }
    };

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            error!("router server error: {e}");
            std::process::exit(1);
        }
    })
}

/// Handler for the `/metrics` endpoint which encodes and returns the Prometheus metrics.
pub async fn metrics_handler(
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

/// Default handler for the `/health` endpoint, returning a simple 200 OK and JSON response.
pub async fn default_health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "health": true }))
}

/// Default handler for the `/ready` endpoint, returning a simple 200 OK and JSON response.
pub async fn default_ready_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "ready": true }))
}

/// Handler for the `/ready` endpoint which returns a `200 OK` if the provided `is_ready`
/// boolean is `true`, otherwise returns a `503 Service Unavailable` with the provided error
/// message.
pub fn ready_handler_http(
    is_ready: Arc<AtomicBool>,
    error_msg: String,
) -> MethodRouter<Arc<MetricsState>> {
    get(move || {
        let is_ready = is_ready.clone();
        async move {
            if is_ready.load(Ordering::SeqCst) {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "ready": true,
                    })),
                )
            } else {
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(serde_json::json!({
                        "ready": false,
                        "message": error_msg
                    })),
                )
            }
        }
    })
}

/// Handler for the `/ready` endpoint which returns a `200 OK` if the provided `is_ready`
/// boolean is `true`, otherwise returns a `503 Service Unavailable` with the provided error
/// message.
pub fn ready_handler_jsonrpc(
    is_ready: Arc<AtomicBool>,
    error_msg: String,
) -> impl Fn() -> Result<JsonValue, ErrorCode> + Clone + Send + Sync + 'static {
    move || {
        if is_ready.load(Ordering::SeqCst) {
            Ok(serde_json::json!({
                "ready": true,
            }))
        } else {
            // Return success with ready: false (more informative than error)
            Ok(serde_json::json!({
                "ready": false,
                "message": error_msg.clone()
            }))
        }
    }
}

/// Checks if the cache connection is healthy
/// This method attempts to ping the cache connection to check if it is healthy.
pub fn cache_health_check_handler(
    valkey_conn: ConnectionManager,
) -> MethodRouter<Arc<MetricsState>> {
    let mut conn = valkey_conn;
    get(move || async move {
        // Add timeout to prevent hanging when cache is down
        let health = tokio::time::timeout(Duration::from_secs(2), conn.ping::<String>()).await;

        match health {
            Ok(Ok(_)) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))),
            Ok(Err(e)) => {
                error!("Cache connection is not healthy: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "ok": false,
                        "message": "Cache connection is not healthy"
                    })),
                )
            }
            Err(_) => {
                error!("Cache connection ping timed out");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "ok": false,
                        "message": "Cache connection ping timed out"
                    })),
                )
            }
        }
    })
}
