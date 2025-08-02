//! The `metrics` module for the Slotter

use common::types::Chain;
use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        family::Family,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;

/// Labels used for Prometheus metric categorization.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    /// Sequencing or Settlement
    pub chain: &'static str,
}

/// Structure holding metrics related to the Slotter.
#[derive(Debug, Clone)]
pub struct SlotterMetrics {
    /// Records the last block number processed by the Slotter
    last_processed_block: Family<Labels, Gauge>,
    /// Tracks the timestamp lag (ms)
    timestamp_lag_ms: Family<Labels, Gauge>,
    /// Tracks blocks processed per slot
    blocks_per_slot: Histogram,
    /// Tracks the last slot created
    last_slot: Gauge,
}

impl SlotterMetrics {
    /// Creates a new `SlotterMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let last_processed_block = Family::<Labels, Gauge>::default();
        let timestamp_lag_ms = Family::<Labels, Gauge>::default();
        let blocks_per_slot = Histogram::new(exponential_buckets(1.0, 2.0, 8));
        let last_slot = Gauge::default();

        registry.register(
            "last_processed_block",
            "Tracks the last sequencing block number processed by the Slotter",
            last_processed_block.clone(),
        );

        registry.register(
            "timestamp_lag_ms",
            "Tracks the timestamp lag (ms) for the sequencing chain",
            timestamp_lag_ms.clone(),
        );

        registry.register(
            "blocks_per_slot",
            "Histogram tracking blocks processed per slot",
            blocks_per_slot.clone(),
        );

        registry.register(
            "last_slot",
            "Tracks the last slot created",
            last_slot.clone(),
        );

        Self {
            last_processed_block,
            timestamp_lag_ms,
            blocks_per_slot,
            last_slot,
        }
    }

    /// Records the last block processed by the Slotter.
    pub fn record_last_processed_block(&self, block_number: u64, chain: Chain) {
        self.last_processed_block
            .get_or_create(&Labels { chain: chain.into() })
            .set(block_number as i64);
    }

    /// Updates the timestamp lag metric (current time - latest block timestamp)
    pub fn update_chain_timestamp_lag(&self, block_timestamp: u64, chain: Chain) {
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis() as u64,
            Err(_) => {
                error!("System time before Unix epoch");
                return;
            }
        };

        let block_timestamp_ms = block_timestamp * 1000; // Convert seconds to milliseconds

        let lag = now.saturating_sub(block_timestamp_ms); // Avoid negative values
        self.timestamp_lag_ms
            .get_or_create(&Labels { chain: chain.into() })
            .set(lag as i64);
    }

    /// Records the number of blocks processed per slot.
    pub fn record_blocks_per_slot(&self, blocks: u64) {
        self.blocks_per_slot.observe(blocks as f64);
    }

    /// Records the last slot number created
    pub fn record_last_slot(&self, slot_number: u64) {
        self.last_slot.set(slot_number as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = SlotterMetrics::new(&mut registry);

        assert_eq!(
            metrics
                .last_processed_block
                .get_or_create(&Labels { chain: "sequencing" })
                .get(),
            0
        );
    }

    #[test]
    fn test_record_last_processed_block() {
        let mut registry = Registry::default();
        let metrics = SlotterMetrics::new(&mut registry);

        metrics.record_last_processed_block(42, Chain::Sequencing);
        assert_eq!(
            metrics
                .last_processed_block
                .get_or_create(&Labels { chain: "sequencing" })
                .get(),
            42
        );

        metrics.record_last_processed_block(84, Chain::Settlement);
        assert_eq!(
            metrics
                .last_processed_block
                .get_or_create(&Labels { chain: "settlement" })
                .get(),
            84
        );
    }

    #[test]
    fn test_update_chain_timestamp_lag() {
        let mut registry = Registry::default();
        let metrics = SlotterMetrics::new(&mut registry);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let past_timestamp = now - 5; // 5 seconds ago
        metrics.update_chain_timestamp_lag(past_timestamp, Chain::Sequencing);
        assert!(
            metrics.timestamp_lag_ms.get_or_create(&Labels { chain: "sequencing" }).get() >=
                5000
        );

        let past_timestamp = now - 10000; // 10 seconds ago
        metrics.update_chain_timestamp_lag(past_timestamp, Chain::Settlement);
        assert!(
            metrics.timestamp_lag_ms.get_or_create(&Labels { chain: "settlement" }).get() >=
                10000
        );
    }

    #[test]
    fn test_record_blocks_per_slot() {
        let mut registry = Registry::default();
        let metrics = SlotterMetrics::new(&mut registry);

        metrics.record_blocks_per_slot(5);
        metrics.record_blocks_per_slot(10);
        metrics.record_blocks_per_slot(20);
        // Histogram records values but cannot assert specific values directly
    }

    #[test]
    fn test_record_last_slot() {
        let mut registry = Registry::default();
        let metrics = SlotterMetrics::new(&mut registry);

        metrics.record_last_slot(42);
        assert_eq!(metrics.last_slot.get(), 42);

        metrics.record_last_slot(100);
        assert_eq!(metrics.last_slot.get(), 100);
    }
}
