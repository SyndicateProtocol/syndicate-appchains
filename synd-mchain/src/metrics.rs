//! The `metrics` module for the `Mchain`

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone)]
pub struct BlockMetrics {
    pub number: Gauge,
    pub timestamp: Gauge,
    pub delay: Gauge,
}

#[derive(Debug, Default, Clone)]
pub struct ReorgMetrics {
    pub blocks: Gauge,
    pub duration: Gauge,
}

/// Structure holding metrics related to the `Poster`.
#[derive(Debug, Default, Clone)]
pub struct MchainMetrics {
    pub sequencing_block: BlockMetrics,
    pub last_block: BlockMetrics,
    pub finalized_block: BlockMetrics,
    pub last_reorg: ReorgMetrics,
    pub total_reorg: ReorgMetrics,
    pub last_reorg_timestamp: Gauge,
    pub total_reorg_count: Gauge,
}

impl MchainMetrics {
    /// Creates a new `PosterMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self::default();
        registry.register(
            "sequencing_block_number",
            "Tracks the latest sequencing chain block number",
            metrics.sequencing_block.number.clone(),
        );
        registry.register(
            "sequencing_block_timestamp",
            "Tracks the latest sequencing chain block timestamp",
            metrics.sequencing_block.timestamp.clone(),
        );
        registry.register(
            "sequencing_block_delay",
            "Tracks the latest sequencing chain block delay",
            metrics.sequencing_block.delay.clone(),
        );

        registry.register(
            "last_block_number",
            "Tracks the latest synd-mchain block number",
            metrics.last_block.number.clone(),
        );
        registry.register(
            "last_block_timestamp",
            "Tracks the latest synd-mchain block timestamp",
            metrics.last_block.timestamp.clone(),
        );
        registry.register(
            "last_block_delay",
            "Tracks the latest synd-mchain block delay",
            metrics.last_block.delay.clone(),
        );

        registry.register(
            "finalized_block_number",
            "Tracks the finalized synd-mchain block number",
            metrics.finalized_block.number.clone(),
        );
        registry.register(
            "finalized_block_timestamp",
            "Tracks the finalized synd-mchain block timestamp",
            metrics.finalized_block.timestamp.clone(),
        );
        registry.register(
            "finalized_block_delay",
            "Tracks the finalization delay",
            metrics.finalized_block.delay.clone(),
        );

        registry.register(
            "last_reorg_blocks",
            "Tracks the lastest reorg depth",
            metrics.last_reorg.blocks.clone(),
        );
        registry.register(
            "last_reorg_duration",
            "Tracks the latest reorg duration",
            metrics.last_reorg.duration.clone(),
        );
        registry.register(
            "last_reorg_timestamp",
            "Tracks the lastest reorg timestamp",
            metrics.last_reorg_timestamp.clone(),
        );

        registry.register(
            "total_reorg_blocks",
            "Tracks the number of reorged blocks",
            metrics.total_reorg.blocks.clone(),
        );
        registry.register(
            "total_reorg_duration",
            "Tracks the time the latest block is inaccurate",
            metrics.total_reorg.duration.clone(),
        );
        registry.register(
            "total_reorg_count",
            "Tracks the number of reorgs",
            metrics.total_reorg_count.clone(),
        );

        metrics
    }

    pub fn record_sequencing_block(&self, number: u64, timestamp: u64) {
        Self::record_block(&self.sequencing_block, number, timestamp);
    }

    pub fn record_last_block(&self, number: u64, timestamp: u64) {
        Self::record_block(&self.last_block, number, timestamp);
    }

    pub fn record_finalized_block(&self, number: u64, timestamp: u64) {
        Self::record_block(&self.finalized_block, number, timestamp);
    }

    fn record_block(block: &BlockMetrics, number: u64, timestamp: u64) -> i64 {
        let now = get_epoch_time().as_secs() as i64;
        block.delay.set(now - timestamp as i64);
        block.number.set(number as i64);
        block.timestamp.set(timestamp as i64);
        now
    }

    pub fn record_reorg(&self, start_block: u64, end_block: u64, start_timestamp: u64) {
        let now = Self::record_block(&self.last_block, start_block, start_timestamp);
        let depth = end_block as i64 - start_block as i64;
        let duration = now - start_timestamp as i64;
        self.last_reorg_timestamp.set(now);
        self.last_reorg.blocks.set(depth);
        self.last_reorg.duration.set(duration);
        self.total_reorg_count.inc();
        self.total_reorg.blocks.inc_by(depth);
        self.total_reorg.duration.inc_by(duration);
    }
}

fn get_epoch_time() -> Duration {
    #[allow(clippy::unwrap_used)]
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;

    #[test]
    fn test_new_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = MchainMetrics::new(&mut registry);

        assert_eq!(metrics.last_block.number.get(), 0);
    }

    #[test]
    fn test_record_last_block_posted() {
        let mut registry = Registry::default();
        let metrics = MchainMetrics::new(&mut registry);

        metrics.record_last_block(10, 0);
        assert_eq!(metrics.last_block.number.get(), 10);
    }
}
