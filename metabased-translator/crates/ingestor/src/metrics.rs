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

/// Labels used for a specific method & chain.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct MethodLabel {
    /// Sequencing or Settlement
    pub chain: &'static str,
    /// Metric method
    pub method: &'static str,
}
/// Labels used for a specific chain.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ChainLabel {
    /// Sequencing or Settlement
    pub chain: &'static str,
}

/// Structure holding metrics related to blockchain data ingestion.
#[derive(Debug, Clone)]
pub struct IngestorMetrics {
    /// Records rpc calls
    pub ingestor_rpc_calls: Family<MethodLabel, Counter>,
    /// Records rpc calls duration
    pub ingestor_rpc_calls_duration: Family<MethodLabel, Histogram>,
    /// Records last block fetched
    pub ingestor_last_block_fetched: Family<MethodLabel, Gauge>,
    /// Tracks the channel capacity
    pub ingestor_channel_capacity: Family<ChainLabel, Gauge>,
}

impl IngestorMetrics {
    /// Creates a new `IngestorMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let rpc_calls = Family::<MethodLabel, Counter>::default();
        registry.register(
            "ingestor_rpc_calls",
            "Number of RPC method calls done",
            rpc_calls.clone(),
        );

        let rpc_calls_duration = Family::<MethodLabel, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "ingestor_rpc_calls_latency",
            "Latency of RPC method call responses",
            rpc_calls_duration.clone(),
        );

        let last_block_fetched = Family::<MethodLabel, Gauge>::default();
        registry.register(
            "ingestor_last_block_fetched",
            "Tracks the last block number fetched for a specific RPC URL",
            last_block_fetched.clone(),
        );

        let ingestor_channel_capacity = Family::<ChainLabel, Gauge>::default();
        registry.register(
            "ingestor_channel_capacity",
            "Tracks the capacity of the sequencing chain channel",
            ingestor_channel_capacity.clone(),
        );
        Self {
            ingestor_rpc_calls: rpc_calls,
            ingestor_rpc_calls_duration: rpc_calls_duration,
            ingestor_last_block_fetched: last_block_fetched,
            ingestor_channel_capacity,
        }
    }

    /// Records the last block number fetched for a given label.
    pub fn record_last_block_fetched(&self, chain: Chain, block_number: u64) {
        self.ingestor_last_block_fetched
            .get_or_create(&MethodLabel { chain: chain.into(), method: "last_block_fetched" })
            .set(block_number as i64);
    }

    /// Records an RPC call event, incrementing counters and measuring duration.
    pub fn record_rpc_call(&self, chain: Chain, method: &'static str, duration: Duration) {
        // Increment the counter for the RPC method call
        self.ingestor_rpc_calls.get_or_create(&MethodLabel { chain: chain.into(), method }).inc();

        // Observe the latency of the RPC method call
        self.ingestor_rpc_calls_duration
            .get_or_create(&MethodLabel { chain: chain.into(), method })
            .observe(duration.as_secs_f64());
    }

    /// Updates the channel capacity for a given chain.
    pub fn update_channel_capacity(&self, chain: Chain, capacity: usize) {
        self.ingestor_channel_capacity
            .get_or_create(&ChainLabel { chain: chain.into() })
            .set(capacity as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;
    use std::time::Duration;

    #[test]
    fn test_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = IngestorMetrics::new(&mut registry);

        assert_eq!(
            metrics
                .ingestor_rpc_calls
                .get_or_create(&MethodLabel { chain: "sequencing", method: "eth_getBlockByNumber" })
                .get(),
            0
        );
    }

    #[test]
    fn test_record_last_block_fetched() {
        let mut registry = Registry::default();
        let metrics = IngestorMetrics::new(&mut registry);

        metrics.record_last_block_fetched(Chain::Sequencing, 100);

        let value = metrics
            .ingestor_last_block_fetched
            .get_or_create(&MethodLabel { chain: "sequencing", method: "last_block_fetched" })
            .get();

        assert_eq!(value, 100);
    }

    #[test]
    fn test_record_rpc_call() {
        let mut registry = Registry::default();
        let metrics = IngestorMetrics::new(&mut registry);
        let duration = Duration::from_millis(500);

        metrics.record_rpc_call(Chain::Settlement, "get_balance", duration);

        let counter_value = metrics
            .ingestor_rpc_calls
            .get_or_create(&MethodLabel { chain: "settlement", method: "get_balance" })
            .get();

        assert_eq!(counter_value, 1);
    }

    #[test]
    fn test_update_channel_capacity() {
        let mut registry = Registry::default();
        let metrics = IngestorMetrics::new(&mut registry);

        metrics.update_channel_capacity(Chain::Sequencing, 75);

        let capacity_value = metrics
            .ingestor_channel_capacity
            .get_or_create(&ChainLabel { chain: "sequencing" })
            .get();

        assert_eq!(capacity_value, 75);
    }
}
