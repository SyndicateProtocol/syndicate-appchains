//! The `metrics` module for the `MChain`

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};

/// Structure holding metrics related to the `MChain`.
#[derive(Debug, Clone)]
pub struct MChainMetrics {
    /// Last mined block number
    pub mchain_last_mined_block_number: Gauge,

    /// Last mined block timestamp
    pub mchain_last_mined_block_timestamp: Gauge,
}

impl MChainMetrics {
    /// Creates a new `MChainMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let mchain_last_mined_block_number = Gauge::default();
        let mchain_last_mined_block_timestamp = Gauge::default();

        registry.register(
            "mchain_last_mined_block_number",
            "Tracks the last mined block number in the m-chain ",
            mchain_last_mined_block_number.clone(),
        );

        registry.register(
            "mchain_last_mined_block_timestamp",
            "Tracks the last mined block timestamp in the m-chain ",
            mchain_last_mined_block_timestamp.clone(),
        );

        Self { mchain_last_mined_block_number, mchain_last_mined_block_timestamp }
    }

    /// Records the last block number
    pub fn record_last_block_mined(&self, block: alloy::rpc::types::Block) {
        self.mchain_last_mined_block_number.set(block.header.number as i64);
        self.mchain_last_mined_block_timestamp.set(block.header.timestamp as i64);
    }
}
