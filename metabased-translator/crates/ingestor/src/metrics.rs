//! The `metrics` module for the ingestor

use common::types::Chain;
use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::time::Duration;

/// Labels used for Prometheus metric categorization.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    /// Sequencing or Settlement
    pub chain: &'static str,
    /// Metric method
    pub method: &'static str,
}

/// Structure holding metrics related to blockchain data ingestion.
#[derive(Debug)]
pub struct IngestorMetrics {
    /// Records rpc calls
    pub ingestor_rpc_calls: Family<Labels, Counter>,
    /// Records rpc calls duration
    pub ingestor_rpc_calls_duration: Family<Labels, Histogram>,
    /// Records last block fetched
    pub ingestor_last_block_fetched: Family<Labels, Gauge>,
}

impl IngestorMetrics {
    /// Creates a new `IngestorMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let rpc_calls = Family::<Labels, Counter>::default();
        registry.register(
            "ingestor_rpc_calls",
            "Number of RPC method calls done",
            rpc_calls.clone(),
        );

        let rpc_calls_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "ingestor_rpc_calls_latency",
            "Latency of RPC method call responses",
            rpc_calls_duration.clone(),
        );

        let last_block_fetched = Family::<Labels, Gauge>::default();
        registry.register(
            "ingestor_last_block_fetched",
            "Tracks the last block number fetched for a specific RPC URL",
            last_block_fetched.clone(),
        );

        Self {
            ingestor_rpc_calls: rpc_calls,
            ingestor_rpc_calls_duration: rpc_calls_duration,
            ingestor_last_block_fetched: last_block_fetched,
        }
    }

    /// Records the last block number fetched for a given label.
    pub fn record_last_block_fetched(&self, chain: Chain, block_number: u64) {
        self.ingestor_last_block_fetched
            .get_or_create(&Labels { chain: chain.into(), method: "last_block_fetched" })
            .set(block_number as i64);
    }

    /// Records an RPC call event, incrementing counters and measuring duration.
    pub fn record_rpc_call(&self, chain: Chain, method: &'static str, duration: Duration) {
        // Increment the counter for the RPC method call
        self.ingestor_rpc_calls.get_or_create(&Labels { chain: chain.into(), method }).inc();

        // Observe the latency of the RPC method call
        self.ingestor_rpc_calls_duration
            .get_or_create(&Labels { chain: chain.into(), method })
            .observe(duration.as_secs_f64());
    }
}
