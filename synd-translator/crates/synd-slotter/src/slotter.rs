//! Slotter module for `synd-translator`

use crate::metrics::SlotterMetrics;
use alloy::primitives::FixedBytes;
use common::types::{Chain, SequencingBlock, SettlementBlock};
use shared::tracing::SpanKind;
use synd_chain_ingestor::client::BlockStreamT;
use synd_mchain::{
    client::MchainProvider,
    db::{ArbitrumBatch, MBlock, Slot},
};
use thiserror::Error;
use tracing::{error, info, instrument, trace};

/// Ingests blocks from the sequencing and settlement chains, slots them into slots, and sends the
/// slots to the slot processor to generate `synd-mchain` blocks.
#[allow(clippy::expect_used)]
#[allow(clippy::cognitive_complexity)]
#[instrument(
    skip_all,
    err,
    fields(otel.kind = ?SpanKind::Internal),
)]
pub async fn run(
    settlement_delay: u64,
    mut sequencing: impl BlockStreamT<SequencingBlock> + Send,
    mut settlement: impl BlockStreamT<SettlementBlock> + Send,
    provider: &impl MchainProvider,
    metrics: &SlotterMetrics,
) -> Result<(), SlotterError> {
    info!("Starting Slotter");

    trace!("Waiting for settlement block");
    let mut set_block = settlement
        .recv(0)
        .await
        .map_err(|e| SlotterError::IngestorError(Chain::Settlement, e.to_string()))?;

    loop {
        trace!("Waiting for sequencing block");
        let seq_block = sequencing
            .recv(0)
            .await
            .map_err(|e| SlotterError::IngestorError(Chain::Sequencing, e.to_string()))?;

        metrics.record_last_processed_block(seq_block.block_ref.number, Chain::Sequencing);
        metrics.update_chain_timestamp_lag(seq_block.block_ref.timestamp, Chain::Sequencing);

        let mut mblock = MBlock {
            timestamp: seq_block.block_ref.timestamp,
            slot: Slot {
                seq_block_number: seq_block.block_ref.number,
                seq_block_hash: seq_block.block_ref.hash,
                set_block_hash: FixedBytes::ZERO,
                set_block_number: 0,
            },
            payload: None,
        };

        let mut messages = vec![];

        let mut blocks_per_slot: u64 = 1;
        let slot_end_ts = if seq_block.block_ref.timestamp >= settlement_delay {
            seq_block.block_ref.timestamp - settlement_delay + 1
        } else {
            Default::default()
        };
        while set_block.block_ref.timestamp < slot_end_ts {
            blocks_per_slot += 1;
            messages.append(&mut set_block.messages);
            set_block = settlement
                .recv(slot_end_ts)
                .await
                .map_err(|e| SlotterError::IngestorError(Chain::Settlement, e.to_string()))?;
            metrics.record_last_processed_block(set_block.block_ref.number, Chain::Settlement);
            metrics.update_chain_timestamp_lag(set_block.block_ref.timestamp, Chain::Settlement);
        }

        if seq_block.tx_count > 0 || !messages.is_empty() {
            mblock.payload = Some(ArbitrumBatch::new(seq_block.batch, messages));
        }
        mblock.slot.set_block_hash = set_block.block_ref.hash;
        mblock.slot.set_block_number = set_block.block_ref.number;

        trace!("Processing slot {:?}", mblock.slot);
        let time = std::time::Instant::now();
        provider
            .add_batch(&mblock)
            .await
            .map_err(|e| SlotterError::SlotProcessorError(e.to_string()))?;
        if let Some(payload) = mblock.payload {
            info!(
                "Sent slot {} ({} seq, {} set) with timestamp {} in {:?}",
                mblock.slot.seq_block_number,
                seq_block.tx_count,
                payload.delayed_messages.len(),
                mblock.timestamp,
                time.elapsed()
            );
        }
        metrics.record_blocks_per_slot(blocks_per_slot);
        metrics.record_last_slot(mblock.slot.seq_block_number);
    }
}

/// Slotter Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SlotterError {
    /// Means something went wrong when trying to apply the batch to the mchain
    #[error("Slot processor error: {0}")]
    SlotProcessorError(String),

    /// An ingestion error, this essentially will be handled as a reorg - the slotter will be
    /// restarted and attempt to reconcile the mchain state with the real world
    #[error("{0} chain ingestor error: {1}")]
    IngestorError(Chain, String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{metrics::SlotterMetrics, slotter::run};
    use alloy::primitives::{Address, Bytes, FixedBytes, U256};
    use async_trait::async_trait;
    use common::types::{SequencingBlock, SettlementBlock};
    use prometheus_client::registry::Registry;
    use shared::types::BlockRef;
    use std::sync::{Arc, Mutex};
    use synd_chain_ingestor::client::BlockStreamT;
    use synd_mchain::{
        client::{ClientError, DeserializeOwned, MchainProvider, ToRpcParams},
        db::DelayedMessage,
    };

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    #[derive(Clone)]
    struct MockMchainProvider {
        batches: Arc<Mutex<Vec<MBlock>>>,
        notify: Arc<tokio::sync::Notify>,
        target_blocks: usize,
    }

    impl MockMchainProvider {
        fn new(target_blocks: usize) -> Self {
            Self {
                batches: Arc::new(Mutex::new(Vec::new())),
                notify: Arc::new(tokio::sync::Notify::new()),
                target_blocks,
            }
        }

        fn get_blocks(&self) -> Vec<MBlock> {
            self.batches.lock().unwrap().clone()
        }

        async fn wait_for_blocks(&self) {
            loop {
                {
                    let blocks = self.batches.lock().unwrap();
                    if blocks.len() >= self.target_blocks {
                        return;
                    }
                }
                self.notify.notified().await;
            }
        }
    }

    #[async_trait]
    impl MchainProvider for MockMchainProvider {
        async fn request<Params: ToRpcParams + Send, T: DeserializeOwned>(
            &self,
            method: &'static str,
            _params: Params,
        ) -> Result<T, ClientError> {
            panic!("unexpected call to request: {method}");
        }

        async fn add_batch(&self, batch: &MBlock) -> eyre::Result<Option<u64>> {
            let len = {
                let mut batches = self.batches.lock().unwrap();
                batches.push(batch.clone());
                batches.len()
            };
            self.notify.notify_waiters();
            Ok(Some(len as u64))
        }
    }

    fn create_seq_block(
        number: u64,
        timestamp: u64,
        tx_count: u64,
        batch_data: &[u8],
    ) -> SequencingBlock {
        SequencingBlock {
            block_ref: BlockRef { number, timestamp, hash: FixedBytes::from([number as u8; 32]) },
            parent_hash: FixedBytes::from([(number - 1) as u8; 32]),
            batch: Bytes::from(batch_data.to_vec()),
            tx_count,
        }
    }

    fn create_set_block(
        number: u64,
        timestamp: u64,
        messages: Vec<DelayedMessage>,
    ) -> SettlementBlock {
        SettlementBlock {
            block_ref: BlockRef { number, timestamp, hash: FixedBytes::from([number as u8; 32]) },
            parent_hash: FixedBytes::from([(number - 1) as u8; 32]),
            messages,
        }
    }

    struct TestBlockStream<Block> {
        blocks: Vec<Block>,
        index: usize,
    }

    impl<Block> TestBlockStream<Block> {
        fn new(blocks: Vec<Block>) -> Self {
            Self { blocks, index: 0 }
        }
    }

    #[async_trait]
    impl<Block: Send + Clone> BlockStreamT<Block> for TestBlockStream<Block> {
        async fn recv(&mut self, _timestamp: u64) -> eyre::Result<Block> {
            if self.index < self.blocks.len() {
                let block = self.blocks[self.index].clone();
                self.index += 1;
                Ok(block)
            } else {
                // Return a pending future to simulate blocking
                std::future::pending().await
            }
        }
    }

    #[tokio::test]
    async fn test_basic_slotter_operation() -> eyre::Result<()> {
        let settlement_delay = 10;
        let mchain_provider = MockMchainProvider::new(1); // Expect 1 batch
        let metrics = SlotterMetrics::new(&mut Registry::default());

        // Create one sequencing block at timestamp 100
        let seq_blocks = vec![create_seq_block(1, 100, 5, b"seq_batch_1")];

        // Create settlement blocks:
        // - Block at timestamp 89 (will be included in slot)
        // - Block at timestamp 91 (will seal the slot, not included)
        let set_blocks = vec![
            create_set_block(1, 89, vec![]),
            create_set_block(2, 91, vec![]), // This seals the slot
        ];

        let sequencing = TestBlockStream::new(seq_blocks);
        let settlement = TestBlockStream::new(set_blocks);

        let mchain_provider_clone = mchain_provider.clone();
        // Run slotter in a separate task
        let handle = tokio::spawn(async move {
            let _ = run(settlement_delay, sequencing, settlement, &mchain_provider_clone, &metrics)
                .await;
        });

        // Wait for the expected number of blocks to be processed
        mchain_provider.wait_for_blocks().await;
        handle.abort();

        // Verify the slot was created correctly
        let blocks = mchain_provider.get_blocks();
        assert_eq!(blocks.len(), 1);

        let mblock = &blocks[0];
        assert_eq!(mblock.timestamp, 100);
        assert_eq!(mblock.slot.seq_block_number, 1);
        assert_eq!(mblock.slot.set_block_number, 2);
        assert!(mblock.payload.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_settlement_blocks_per_slot() -> eyre::Result<()> {
        let settlement_delay = 100;
        let mchain_provider = MockMchainProvider::new(1); // Expect 1 batch
        let metrics = SlotterMetrics::new(&mut Registry::default());

        // Create one sequencing block at timestamp 200
        let seq_blocks = vec![create_seq_block(1, 200, 2, b"seq_batch")];

        // Create multiple settlement blocks that should fit in the slot
        // slot_end_ts = 200 - 100 + 1 = 101
        // So settlement blocks with timestamp < 101 should be included
        let set_blocks = vec![
            create_set_block(1, 80, vec![]),
            create_set_block(2, 90, vec![]),
            create_set_block(3, 100, vec![]),
            create_set_block(4, 105, vec![]), // This one seals the slot
        ];

        let sequencing = TestBlockStream::new(seq_blocks);
        let settlement = TestBlockStream::new(set_blocks);

        let mchain_provider_clone = mchain_provider.clone();
        let handle = tokio::spawn(async move {
            let _ = run(settlement_delay, sequencing, settlement, &mchain_provider_clone, &metrics)
                .await;
        });

        mchain_provider.wait_for_blocks().await;
        handle.abort();

        let blocks = mchain_provider.get_blocks();
        assert_eq!(blocks.len(), 1);

        let mblock = &blocks[0];
        assert_eq!(mblock.slot.seq_block_number, 1);
        assert_eq!(mblock.slot.set_block_number, 4); // The sealing block

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_slot_no_payload() -> eyre::Result<()> {
        let settlement_delay = 10;
        let mchain_provider = MockMchainProvider::new(1); // Expect 1 batch
        let metrics = SlotterMetrics::new(&mut Registry::default());

        // Sequencing block with no transactions
        let seq_blocks = vec![create_seq_block(1, 100, 0, b"")];

        // Settlement blocks with no messages
        let set_blocks = vec![create_set_block(1, 89, vec![]), create_set_block(2, 91, vec![])];

        let sequencing = TestBlockStream::new(seq_blocks);
        let settlement = TestBlockStream::new(set_blocks);

        let mchain_provider_clone = mchain_provider.clone();
        let handle = tokio::spawn(async move {
            let _ = run(settlement_delay, sequencing, settlement, &mchain_provider_clone, &metrics)
                .await;
        });

        mchain_provider.wait_for_blocks().await;
        handle.abort();

        let blocks = mchain_provider.get_blocks();
        assert_eq!(blocks.len(), 1);

        // Empty slot should have no payload
        assert!(blocks[0].payload.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_slot_with_delayed_messages() -> eyre::Result<()> {
        let settlement_delay = 10;
        let mchain_provider = MockMchainProvider::new(1); // Expect 1 batch
        let metrics = SlotterMetrics::new(&mut Registry::default());

        // Sequencing block with no transactions
        let seq_blocks = vec![create_seq_block(1, 100, 0, b"")];

        // Settlement blocks with delayed messages
        let message1 = DelayedMessage {
            kind: 0,
            sender: Address::ZERO,
            data: Bytes::from("message1"),
            base_fee_l1: U256::from(1000),
        };
        let message2 = DelayedMessage {
            kind: 1,
            sender: Address::ZERO,
            data: Bytes::from("message2"),
            base_fee_l1: U256::from(2000),
        };

        let set_blocks = vec![
            create_set_block(1, 89, vec![message1]),
            create_set_block(2, 90, vec![message2]),
            create_set_block(3, 91, vec![]), // Sealing block
        ];

        let sequencing = TestBlockStream::new(seq_blocks);
        let settlement = TestBlockStream::new(set_blocks);

        let mchain_provider_clone = mchain_provider.clone();
        let handle = tokio::spawn(async move {
            let _ = run(settlement_delay, sequencing, settlement, &mchain_provider_clone, &metrics)
                .await;
        });

        mchain_provider.wait_for_blocks().await;
        handle.abort();

        let blocks = mchain_provider.get_blocks();
        assert_eq!(blocks.len(), 1);

        // Slot should have payload because of delayed messages
        let payload = blocks[0].payload.as_ref().unwrap();
        assert_eq!(payload.delayed_messages.len(), 2);

        Ok(())
    }
}
