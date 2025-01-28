use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use prometheus_client::{
    encoding::{text::encode, EncodeLabelSet},
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    label_name: String,
    method: &'static str,
}

#[derive(Debug)]
pub struct IngestorMetrics {
    rpc_calls: Family<Labels, Counter>,
    rpc_calls_duration: Family<Labels, Histogram>,
    last_block_fetched: Family<Labels, Gauge>,
}

#[derive(Debug)]
pub struct MetricsState {
    pub registry: Registry,
}

impl IngestorMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let rpc_calls = Family::<Labels, Counter>::default();
        registry.register("rpc_calls", "Number of RPC method calls done", rpc_calls.clone());

        let rpc_calls_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "rpc_calls_latency",
            "Latency of RPC method call responses",
            rpc_calls_duration.clone(),
        );

        let last_block_fetched = Family::<Labels, Gauge>::default();
        registry.register(
            "last_block_fetched",
            "Tracks the last block number fetched for a specific RPC URL",
            last_block_fetched.clone(),
        );

        Self { rpc_calls, rpc_calls_duration, last_block_fetched }
    }

    pub fn record_last_block_fetched(&self, label_name: String, block_number: u64) {
        self.last_block_fetched
            .get_or_create(&Labels { label_name, method: "last_block_fetched" })
            .set(block_number as i64);
    }

    pub fn record_rpc_call(&self, label_name: String, method: &'static str, duration: Duration) {
        let name = label_name.clone();
        // Increment the counter for the RPC method call
        self.rpc_calls.get_or_create(&Labels { label_name: name, method }).inc();

        // Observe the latency of the RPC method call
        self.rpc_calls_duration
            .get_or_create(&Labels { label_name, method })
            .observe(duration.as_secs_f64());
    }
}

pub async fn metrics_handler(State(state): State<Arc<Mutex<MetricsState>>>) -> impl IntoResponse {
    let state = state.lock().await;
    let mut buffer = String::new();
    encode(&mut buffer, &state.registry).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(Body::from(buffer))
        .unwrap()
}

pub async fn start_metrics(metrics_state: MetricsState, port: u16) -> tokio::task::JoinHandle<()> {
    let state = Arc::new(Mutex::new(metrics_state));
    let router = Router::new().route("/metrics", get(metrics_handler)).with_state(state); // Create new Arc<Mutex<Registry>> directly
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    let metrics_task: tokio::task::JoinHandle<()> =
        tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });

    metrics_task
}
