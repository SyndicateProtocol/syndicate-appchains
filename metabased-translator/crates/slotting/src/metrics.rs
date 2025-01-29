//! The `metrics` module for the ingestor

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
/// Structure holding metrics related to slotter.
#[derive(Debug)]
pub struct SlottingMetrics {
    /// Records last block processed by the sequencing chain
    pub last_sequencing_block: Gauge,
    /// Records last block processed by the settlement chain
    pub last_settlement_block: Gauge,
    /// Tracks the current number of active slots
    pub active_slots: Gauge,
    /// Tracks the slotter service status
    pub slotter_status: Gauge,
}

impl SlottingMetrics {
    /// Creates a new `SlottingMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let last_sequencing_block = Gauge::default();
        let last_settlement_block = Gauge::default();
        let active_slots = Gauge::default();
        let slotter_status = Gauge::default();

        registry.register(
            "last_sequencing_block",
            "Tracks the last block number fetched from the sequencing chain",
            last_sequencing_block.clone(),
        );

        registry.register(
            "last_settlement_block",
            "Tracks the last block number fetched from the settlement chain",
            last_settlement_block.clone(),
        );

        registry.register(
            "active_slots",
            "Tracks the number of active slots being processed",
            active_slots.clone(),
        );

        registry.register(
            "slotter_status",
            "Indicates the current status of the Slotter (0 = NotStarted, 1 = Started, 2 = Stopped)",
            slotter_status.clone(),
        );

        Self { last_sequencing_block, last_settlement_block, active_slots, slotter_status }
    }

    /// Records the last block number fetched for a given chain.
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

    /// Updates the status of the Slotter (1 = running, 0 = stopped).
    pub fn update_slotter_status(&self, status: i32) {
        self.slotter_status.set(status as i64);
    }
}
