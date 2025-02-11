//! The `metrics` module for the `BlockBuilder`

use crate::connectors::metrics::MChainMetrics;
use prometheus_client::{encoding::EncodeLabelSet, metrics::gauge::Gauge, registry::Registry};
/// Labels used for Prometheus metric categorization.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    /// Sequencing or Settlement
    pub chain: &'static str,
}

/// Structure holding metrics related to the `BlockBuilder`.
#[derive(Debug, Clone)]
pub struct BlockBuilderMetrics {
    /// Tracks the number of built transactions
    pub block_builder_transactions_per_slot: Gauge,
    /// Records the last slot number processed by the `BlockBuilder`s
    pub block_builder_last_processed_slot: Gauge,
    /// `MChain` metrics
    pub mchain_metrics: MChainMetrics,
}

impl BlockBuilderMetrics {
    /// Creates a new `BlockBuilderMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let block_builder_transactions_per_slot = Gauge::default();
        let block_builder_last_processed_slot = Gauge::default();

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

        let mchain_metrics = MChainMetrics::new(registry);

        Self {
            block_builder_transactions_per_slot,
            block_builder_last_processed_slot,
            mchain_metrics,
        }
    }
    /// Records the number of built transactions
    pub fn record_transactions_per_slot(&self, transactions_len: usize) {
        self.block_builder_transactions_per_slot.set(transactions_len as i64);
    }

    /// Records the last slot number processed
    pub fn record_last_slot(&self, slot_number: u64) {
        self.block_builder_last_processed_slot.set(slot_number as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;

    #[test]
    fn test_new_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = BlockBuilderMetrics::new(&mut registry);

        assert_eq!(metrics.block_builder_transactions_per_slot.get(), 0);
        assert_eq!(metrics.block_builder_last_processed_slot.get(), 0);
    }

    #[test]
    fn test_record_transactions_per_slot() {
        let mut registry = Registry::default();
        let metrics = BlockBuilderMetrics::new(&mut registry);

        metrics.record_transactions_per_slot(5);
        assert_eq!(metrics.block_builder_transactions_per_slot.get(), 5);

        metrics.record_transactions_per_slot(15);
        assert_eq!(metrics.block_builder_transactions_per_slot.get(), 15);
    }

    #[test]
    fn test_record_last_slot() {
        let mut registry = Registry::default();
        let metrics = BlockBuilderMetrics::new(&mut registry);

        metrics.record_last_slot(42);
        assert_eq!(metrics.block_builder_last_processed_slot.get(), 42);

        metrics.record_last_slot(100);
        assert_eq!(metrics.block_builder_last_processed_slot.get(), 100);
    }
}
