//! The `metrics` module for the `Mchain`

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Structure holding metrics related to the Ingestor.
#[derive(Debug, Default, Clone)]
pub struct ChainIngestorMetrics {
    block_number: Gauge,
    block_timestamp: Gauge,
    block_delay: Gauge,
    reorg_blocks: Gauge,
}

#[allow(missing_docs)]
impl ChainIngestorMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self::default();
        registry.register(
            "block_number",
            "Tracks the latest block number ingested",
            metrics.block_number.clone(),
        );

        registry.register(
            "block_timestamp",
            "Tracks the latest block timestamp",
            metrics.block_timestamp.clone(),
        );

        registry.register(
            "block_delay",
            "Tracks the latest block delay",
            metrics.block_delay.clone(),
        );

        registry.register(
            "reorg_blocks",
            "Tracks the number of reorged blocks",
            metrics.reorg_blocks.clone(),
        );

        metrics
    }

    pub fn record_block(&self, number: u64, timestamp: u64) {
        let now = get_epoch_time().as_secs() as i64;
        self.block_delay.set(now - timestamp as i64);
        self.block_number.set(number as i64);
        self.block_timestamp.set(timestamp as i64);
    }

    pub fn record_reorg(&self, depth: u64) {
        self.reorg_blocks.inc_by(depth as i64);
    }
}

fn get_epoch_time() -> Duration {
    #[allow(clippy::unwrap_used)]
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}
