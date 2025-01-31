//! The `metrics` module for the Slotting

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

/// Structure holding metrics related to the Slotting.
#[derive(Debug)]
pub struct SlottingMetrics {
    /// Records the last block number processed by the Slotting
    pub slotting_last_processed_block: Family<Labels, Gauge>,
    /// Tracks the current number of active slots
    pub slotting_active_slots: Gauge,
    /// Tracks the timestamp lag (ms)
    pub slotting_timestamp_lag_ms: Family<Labels, Gauge>,
    /// Tracks blocks processed per slot
    pub slotting_blocks_per_slot: Histogram,
    /// Tracks the channel capacity
    pub slotting_channel_capacity: Family<Labels, Gauge>,
}

impl SlottingMetrics {
    /// Creates a new `SlottingMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let slotting_last_processed_block = Family::<Labels, Gauge>::default();
        let slotting_active_slots = Gauge::default();
        let slotting_timestamp_lag_ms = Family::<Labels, Gauge>::default();
        let slotting_blocks_per_slot = Histogram::new(exponential_buckets(1.0, 2.0, 8));
        let slotting_channel_capacity = Family::<Labels, Gauge>::default();

        registry.register(
            "slotting_last_processed_block",
            "Tracks the last sequencing block number processed by the Slotting",
            slotting_last_processed_block.clone(),
        );

        registry.register(
            "slotting_active_slots",
            "Tracks the number of active slots being processed",
            slotting_active_slots.clone(),
        );

        registry.register(
            "slotting_timestamp_lag_ms",
            "Tracks the timestamp lag (ms) for the sequencing chain",
            slotting_timestamp_lag_ms.clone(),
        );

        registry.register(
            "slotting_blocks_per_slot",
            "Histogram tracking blocks processed per slot",
            slotting_blocks_per_slot.clone(),
        );

        registry.register(
            "slotting_channel_capacity",
            "Tracks the capacity of the sequencing chain channel",
            slotting_channel_capacity.clone(),
        );

        Self {
            slotting_last_processed_block,
            slotting_active_slots,
            slotting_timestamp_lag_ms,
            slotting_blocks_per_slot,
            slotting_channel_capacity,
        }
    }

    /// Records the last block processed by the Slotting.
    pub fn record_last_processed_block(&self, block_number: u64, chain: Chain) {
        self.slotting_last_processed_block
            .get_or_create(&Labels { chain: chain.into() })
            .set(block_number as i64);
    }

    /// Updates the number of active slots.
    pub fn update_active_slots(&self, slots: usize) {
        self.slotting_active_slots.set(slots as i64);
    }

    /// Updates the timestamp lag metric (current time - latest block timestamp)
    pub fn update_chain_timestamp_lag(&self, block_timestamp: u64, chain: Chain) {
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis() as u64,
            Err(_) => {
                error!("System time went backwards.");
                return;
            }
        };

        let block_timestamp_ms = block_timestamp * 1000; // Convert seconds to milliseconds

        let lag = now.saturating_sub(block_timestamp_ms); // Avoid negative values
        self.slotting_timestamp_lag_ms
            .get_or_create(&Labels { chain: chain.into() })
            .set(lag as i64);
    }

    /// Records the number of blocks processed per slot.
    pub fn record_blocks_per_slot(&self, blocks: u64) {
        self.slotting_blocks_per_slot.observe(blocks as f64);
    }

    /// Updates the channel capacity for a given chain.
    pub fn update_channel_capacity(&self, capacity: usize, chain: Chain) {
        self.slotting_channel_capacity
            .get_or_create(&Labels { chain: chain.into() })
            .set(capacity as i64);
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
        let metrics = SlottingMetrics::new(&mut registry);

        assert_eq!(
            metrics
                .slotting_last_processed_block
                .get_or_create(&Labels { chain: "Sequencing" })
                .get(),
            0
        );
        assert_eq!(metrics.slotting_active_slots.get(), 0);
    }

    #[test]
    fn test_record_last_processed_block() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.record_last_processed_block(42, Chain::Sequencing);
        assert_eq!(
            metrics
                .slotting_last_processed_block
                .get_or_create(&Labels { chain: "Sequencing" })
                .get(),
            42
        );

        metrics.record_last_processed_block(84, Chain::Settlement);
        assert_eq!(
            metrics
                .slotting_last_processed_block
                .get_or_create(&Labels { chain: "Settlement" })
                .get(),
            84
        );
    }

    #[test]
    fn test_update_active_slots() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.update_active_slots(10);
        assert_eq!(metrics.slotting_active_slots.get(), 10);

        metrics.update_active_slots(0);
        assert_eq!(metrics.slotting_active_slots.get(), 0);
    }

    #[test]
    fn test_update_chain_timestamp_lag() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let past_timestamp = now - 5000; // 5 seconds ago
        metrics.update_chain_timestamp_lag(past_timestamp, Chain::Sequencing);
        assert!(
            metrics.slotting_timestamp_lag_ms.get_or_create(&Labels { chain: "Sequencing" }).get() >=
                5000
        );

        let past_timestamp = now - 10000; // 10 seconds ago
        metrics.update_chain_timestamp_lag(past_timestamp, Chain::Settlement);
        assert!(
            metrics.slotting_timestamp_lag_ms.get_or_create(&Labels { chain: "Settlement" }).get() >=
                10000
        );
    }

    #[test]
    fn test_record_blocks_per_slot() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.record_blocks_per_slot(5);
        metrics.record_blocks_per_slot(10);
        metrics.record_blocks_per_slot(20);
        // Histogram records values but cannot assert specific values directly
    }

    #[test]
    fn test_update_channel_capacity() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.update_channel_capacity(50, Chain::Sequencing);
        assert_eq!(
            metrics.slotting_channel_capacity.get_or_create(&Labels { chain: "Sequencing" }).get(),
            50
        );

        metrics.update_channel_capacity(100, Chain::Settlement);
        assert_eq!(
            metrics.slotting_channel_capacity.get_or_create(&Labels { chain: "Settlement" }).get(),
            100
        );
    }
}
