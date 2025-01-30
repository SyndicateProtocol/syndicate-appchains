//! The `metrics` module for the Slotting

use common::types::Chain;
use prometheus_client::{
    metrics::{
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;

/// Structure holding metrics related to the Slotting.
#[derive(Debug)]
pub struct SlottingMetrics {
    /// Records the last sequencing block number processed by the Slotting
    pub last_sequencing_block: Gauge,
    /// Records the last settlement block number processed by the Slotting
    pub last_settlement_block: Gauge,
    /// Tracks the current number of active slots
    pub active_slots: Gauge,
    /// Tracks the timestamp lag (ms) for the sequencing chain
    pub sequencing_timestamp_lag_ms: Gauge,
    /// Tracks the timestamp lag (ms) for the settlement chain
    pub settlement_timestamp_lag_ms: Gauge,
    /// Tracks blocks processed per slot
    pub blocks_per_slot: Histogram,
    /// Tracks the channel capacity for sequencing chain
    pub sequencing_channel_capacity: Gauge,
    /// Tracks the channel capacity for settlement chain
    pub settlement_channel_capacity: Gauge,
}

impl SlottingMetrics {
    /// Creates a new `SlottingMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let slotting_last_sequencing_block = Gauge::default();
        let slotting_last_settlement_block = Gauge::default();
        let active_slots = Gauge::default();
        let sequencing_timestamp_lag_ms = Gauge::default();
        let settlement_timestamp_lag_ms = Gauge::default();
        let blocks_per_slot = Histogram::new(exponential_buckets(1.0, 2.0, 8));
        let sequencing_channel_capacity = Gauge::default();
        let settlement_channel_capacity = Gauge::default();

        registry.register(
            "slotting_last_sequencing_block",
            "Tracks the last sequencing block number processed by the Slotting",
            slotting_last_sequencing_block.clone(),
        );

        registry.register(
            "slotting_last_settlement_block",
            "Tracks the last settlement block number processed by the Slotting",
            slotting_last_settlement_block.clone(),
        );

        registry.register(
            "slotting_active_slots",
            "Tracks the number of active slots being processed",
            active_slots.clone(),
        );

        registry.register(
            "slotting_chain_timestamp_lag_ms_sequencing",
            "Tracks the timestamp lag (ms) for the sequencing chain",
            sequencing_timestamp_lag_ms.clone(),
        );

        registry.register(
            "slotting_chain_timestamp_lag_ms_settlement",
            "Tracks the timestamp lag (ms) for the settlement chain",
            settlement_timestamp_lag_ms.clone(),
        );

        registry.register(
            "slotting_blocks_per_slot",
            "Histogram tracking blocks processed per slot",
            blocks_per_slot.clone(),
        );

        registry.register(
            "slotting_sequencing_channel_capacity",
            "Tracks the capacity of the sequencing chain channel",
            sequencing_channel_capacity.clone(),
        );

        registry.register(
            "slotting_settlement_channel_capacity",
            "Tracks the capacity of the settlement chain channel",
            settlement_channel_capacity.clone(),
        );

        Self {
            last_sequencing_block: slotting_last_sequencing_block,
            last_settlement_block: slotting_last_settlement_block,
            active_slots,
            sequencing_timestamp_lag_ms,
            settlement_timestamp_lag_ms,
            blocks_per_slot,
            sequencing_channel_capacity,
            settlement_channel_capacity,
        }
    }

    /// Records the last block processed by the Slotting.
    pub fn record_last_block_processed(&self, block_number: u64, chain: Chain) {
        let metric = match chain {
            Chain::Sequencing => &self.last_sequencing_block,
            Chain::Settlement => &self.last_settlement_block,
        };

        metric.set(block_number as i64);
    }

    /// Updates the number of active slots.
    pub fn update_active_slots(&self, slots: usize) {
        self.active_slots.set(slots as i64);
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

        let lag = now.saturating_sub(block_timestamp); // Avoid negative values
        let metric = match chain {
            Chain::Sequencing => &self.sequencing_timestamp_lag_ms,
            Chain::Settlement => &self.settlement_timestamp_lag_ms,
        };

        metric.set(lag as i64);
    }

    /// Records the number of blocks processed per slot.
    pub fn record_blocks_per_slot(&self, blocks: u64) {
        self.blocks_per_slot.observe(blocks as f64);
    }

    /// Updates the channel capacity for a given chain.
    pub fn update_channel_capacity(&self, capacity: usize, chain: Chain) {
        let metric = match chain {
            Chain::Sequencing => &self.sequencing_channel_capacity,
            Chain::Settlement => &self.settlement_channel_capacity,
        };
        metric.set(capacity as i64);
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

        assert_eq!(metrics.last_sequencing_block.get(), 0);
        assert_eq!(metrics.last_settlement_block.get(), 0);
        assert_eq!(metrics.active_slots.get(), 0);
    }

    #[test]
    fn test_record_last_block_processed() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.record_last_block_processed(42, Chain::Sequencing);
        assert_eq!(metrics.last_sequencing_block.get(), 42);

        metrics.record_last_block_processed(84, Chain::Settlement);
        assert_eq!(metrics.last_settlement_block.get(), 84);
    }

    #[test]
    fn test_update_active_slots() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.update_active_slots(10);
        assert_eq!(metrics.active_slots.get(), 10);

        metrics.update_active_slots(0);
        assert_eq!(metrics.active_slots.get(), 0);
    }

    #[test]
    fn test_update_chain_timestamp_lag() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let past_timestamp = now - 5000; // 5 seconds ago
        metrics.update_chain_timestamp_lag(past_timestamp, Chain::Sequencing);
        assert!(metrics.sequencing_timestamp_lag_ms.get() >= 5000);

        let past_timestamp = now - 10000; // 10 seconds ago
        metrics.update_chain_timestamp_lag(past_timestamp, Chain::Settlement);
        assert!(metrics.settlement_timestamp_lag_ms.get() >= 10000);
    }

    #[test]
    fn test_record_blocks_per_slot() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.record_blocks_per_slot(5);
        metrics.record_blocks_per_slot(10);

        // Indirect validation - Histogram should accept observations without errors
        metrics.record_blocks_per_slot(20);
    }

    #[test]
    fn test_update_channel_capacity() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.update_channel_capacity(50, Chain::Sequencing);
        assert_eq!(metrics.sequencing_channel_capacity.get(), 50);

        metrics.update_channel_capacity(100, Chain::Settlement);
        assert_eq!(metrics.settlement_channel_capacity.get(), 100);
    }
}
