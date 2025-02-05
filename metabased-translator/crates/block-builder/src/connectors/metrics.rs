//! The `metrics` module for the `MChain`

use alloy::rpc::types::Block;
use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
/// Structure holding metrics related to the `MChain`.
#[derive(Debug, Clone)]
pub struct MChainMetrics {
    /// Last mined block number
    pub mchain_last_mined_block_number: Gauge,

    /// Last mined block timestamp
    pub mchain_last_mined_block_timestamp_seconds: Gauge,
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

        Self {
            mchain_last_mined_block_number,
            mchain_last_mined_block_timestamp_seconds: mchain_last_mined_block_timestamp,
        }
    }

    /// Updates the last mined block number and timestamp metrics.
    pub fn record_last_block_mined(&self, block: &Block) {
        self.mchain_last_mined_block_number.set(block.header.number as i64);
        self.mchain_last_mined_block_timestamp_seconds.set(block.header.timestamp as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        consensus::Header as CHeader,
        rpc::types::{Block, BlockTransactions, Header, Transaction},
    };
    use prometheus_client::registry::Registry;

    #[test]
    fn test_new_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = MChainMetrics::new(&mut registry);

        assert_eq!(metrics.mchain_last_mined_block_number.get(), 0);
        assert_eq!(metrics.mchain_last_mined_block_timestamp_seconds.get(), 0);
    }

    #[test]
    fn test_record_last_block_mined() {
        let mut registry = Registry::default();
        let metrics = MChainMetrics::new(&mut registry);

        let mut block = Block::new(
            Header::new(CHeader::default()),
            BlockTransactions::<Transaction>::Full(vec![]),
        );
        block.header.number = 10;
        block.header.timestamp = 100000;
        metrics.record_last_block_mined(&block);
        assert_eq!(metrics.mchain_last_mined_block_number.get(), 10);
        assert_eq!(metrics.mchain_last_mined_block_timestamp_seconds.get(), 100000);
    }
}
