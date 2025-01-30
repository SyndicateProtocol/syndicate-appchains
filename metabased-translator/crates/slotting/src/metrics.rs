//! The `metrics` module for the Slotting

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
/// Structure holding metrics related to the Slotting.
#[derive(Debug)]
pub struct SlottingMetrics {
    /// Records the last sequencing block number processed by the Slotting
    pub last_sequencing_block: Gauge,
    /// Records the last settlement block number processed by the Slotting
    pub last_settlement_block: Gauge,
    /// Tracks the current number of active slots
    pub active_slots: Gauge,
    /// Tracks the Slotting service status
    pub slotting_status: Gauge,
}

impl SlottingMetrics {
    /// Creates a new `SlottingMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let slotting_last_sequencing_block = Gauge::default();
        let slotting_last_settlement_block = Gauge::default();
        let active_slots = Gauge::default();
        let slotting_status = Gauge::default();

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
            "slotting_status",
            "Indicates the current status of the Slotting (0 = NotStarted, 1 = Started, 2 = Stopped)",
            slotting_status.clone(),
        );

        Self {
            last_sequencing_block: slotting_last_sequencing_block,
            last_settlement_block: slotting_last_settlement_block,
            active_slots,
            slotting_status,
        }
    }

    /// Records the last block processed by the Slotting.
    pub fn record_last_block_processed(&self, block_number: u64, chain: String) {
        let metric = match chain.as_str() {
            "sequencing" => &self.last_sequencing_block,
            "settlement" => &self.last_settlement_block,
            _ => return,
        };

        metric.set(block_number as i64);
    }

    /// Updates the number of active slots.
    pub fn update_active_slots(&self, slots: usize) {
        self.active_slots.set(slots as i64);
    }

    /// Updates the Slotting status.
    pub fn update_slotting_status(&self, status: i32) {
        self.slotting_status.set(status as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;

    #[test]
    fn test_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        assert_eq!(metrics.last_sequencing_block.get(), 0);
        assert_eq!(metrics.last_settlement_block.get(), 0);
        assert_eq!(metrics.active_slots.get(), 0);
        assert_eq!(metrics.slotting_status.get(), 0);
    }

    #[test]
    fn test_record_last_block_processed() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.record_last_block_processed(42, "sequencing".to_string());
        assert_eq!(metrics.last_sequencing_block.get(), 42);

        metrics.record_last_block_processed(84, "settlement".to_string());
        assert_eq!(metrics.last_settlement_block.get(), 84);

        metrics.record_last_block_processed(100, "invalid".to_string()); // Should not change anything
        assert_eq!(metrics.last_sequencing_block.get(), 42);
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
    fn test_update_slotting_status() {
        let mut registry = Registry::default();
        let metrics = SlottingMetrics::new(&mut registry);

        metrics.update_slotting_status(1);
        assert_eq!(metrics.slotting_status.get(), 1);

        metrics.update_slotting_status(0);
        assert_eq!(metrics.slotting_status.get(), 0);
    }
}
