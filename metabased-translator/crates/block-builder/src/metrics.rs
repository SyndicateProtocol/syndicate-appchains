//! The `metrics` module for the `BlockBuilder`

use prometheus_client::{encoding::EncodeLabelSet, metrics::gauge::Gauge, registry::Registry};
/// Labels used for Prometheus metric categorization.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    /// Sequencing or Settlement
    pub chain: &'static str,
}

/// Structure holding metrics related to the `BlockBuilder`.
#[derive(Debug)]
pub struct BlockBuilderMetrics {
    /// Tracks the channel capacity
    pub block_builder_channel_capacity: Gauge,
    /// Tracks the number of built transactions per slot
    pub block_builder_transactions_per_slot: Gauge,
    /// Records the last slot number processed by the `BlockBuilder`s
    pub block_builder_last_processed_slot: Gauge,
}

impl BlockBuilderMetrics {
    /// Creates a new `BlockBuilderMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let block_builder_channel_capacity = Gauge::default();
        let block_builder_transactions_per_slot = Gauge::default();
        let block_builder_last_processed_slot = Gauge::default();

        registry.register(
            "block_builder_channel_capacity",
            "Tracks the capacity of the slotting channel",
            block_builder_channel_capacity.clone(),
        );
        registry.register(
            "block_builder_transactions_per_slot",
            "Tracks the number of built transactions per slot",
            block_builder_transactions_per_slot.clone(),
        );
        registry.register(
            "block_builder_last_processed_slot",
            "Records the last slot number processed by the BlockBuilder",
            block_builder_last_processed_slot.clone(),
        );

        Self {
            block_builder_channel_capacity,
            block_builder_transactions_per_slot,
            block_builder_last_processed_slot,
        }
    }

    /// Updates the channel capacity
    pub fn update_channel_capacity(&self, capacity: usize) {
        self.block_builder_channel_capacity.set(capacity as i64);
    }

    /// Records the number of transactions per slot
    pub fn record_transactions_per_slot(&self, transactions_len: usize) {
        self.block_builder_transactions_per_slot.set(transactions_len as i64);
    }

    /// Records the last slot number processed
    pub fn record_last_slot(&self, slot_number: u64) {
        self.block_builder_last_processed_slot.set(slot_number as i64);
    }
}
