//! The `metrics` module for the `Poster`
use alloy::primitives::Address;
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
    metrics::{family::Family, gauge::Gauge},
    registry::Registry,
};
use std::sync::Arc;
use tokio::{sync::RwLock, task::JoinHandle};
/// Labels for the wallet balance metric
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct WalletBalanceLabels {
    /// The wallet address being monitored
    wallet_address: String,
    /// The status of the balance check
    status: &'static str,
}

/// Structure holding metrics related to the `Poster`.
#[derive(Debug, Clone)]
pub struct PosterMetrics {
    /// Last posted block number
    pub last_posted_block_number: Gauge,
    /// Records wallet balance
    pub wallet_balance: Family<WalletBalanceLabels, Gauge>,
}

impl PosterMetrics {
    /// Creates a new `PosterMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let last_posted_block_number = Gauge::default();
        let wallet_balance = Family::<WalletBalanceLabels, Gauge>::default();

        registry.register(
            "last_posted_block_number",
            "Tracks the last posted block number",
            last_posted_block_number.clone(),
        );

        registry.register(
            "wallet_balance",
            "Current wallet balance in wei",
            wallet_balance.clone(),
        );

        Self { last_posted_block_number, wallet_balance }
    }

    /// Updates the last posted block number.
    pub fn record_last_block_posted(&self, number: u64) {
        self.last_posted_block_number.set(number as i64);
    }

    /// Records the current wallet balance
    pub fn record_wallet_balance(&self, balance: u128, wallet_address: Address) {
        self.wallet_balance
            .get_or_create(&WalletBalanceLabels {
                wallet_address: format!("{:#x}", wallet_address),
                status: "success",
            })
            .set(balance as i64);
    }

    /// Records a failed wallet balance check
    pub fn record_wallet_balance_error(&self, wallet_address: Address) {
        self.wallet_balance
            .get_or_create(&WalletBalanceLabels {
                wallet_address: format!("{:#x}", wallet_address),
                status: "error",
            })
            .set(0);
    }
}

/// Structure holding the global metrics state, including the Prometheus registry.
#[derive(Debug)]
pub struct MetricsState {
    /// Prometheus registry
    pub registry: Registry,
}

/// Starts a metrics server on the specified port, serving Prometheus-compatible metrics.
pub async fn start_metrics(metrics_state: MetricsState, port: u16) -> JoinHandle<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;

    #[test]
    fn test_new_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = PosterMetrics::new(&mut registry);

        assert_eq!(metrics.last_posted_block_number.get(), 0);
    }

    #[test]
    fn test_record_last_block_posted() {
        let mut registry = Registry::default();
        let metrics = PosterMetrics::new(&mut registry);

        metrics.record_last_block_posted(10);
        assert_eq!(metrics.last_posted_block_number.get(), 10);
    }
}
