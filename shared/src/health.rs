//! Shared `health` module
use axum::response::{IntoResponse, Json};

/// Handler for the `/health` endpoint, returning a simple 200 OK and JSON response.
pub async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "health": true }))
}
