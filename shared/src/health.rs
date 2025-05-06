use axum::response::{IntoResponse, Json};

pub(crate) async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "health": true }))
}
