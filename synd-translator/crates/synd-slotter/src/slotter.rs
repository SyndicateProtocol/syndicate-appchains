//! Slotter module for `synd-translator`

use crate::{batch::build_batch, metrics::SlotterMetrics};
use alloy::primitives::FixedBytes;
use common::types::{Chain, SequencingBlock, SettlementBlock};
use shared::tracing::SpanKind;
use synd_block_builder::appchains::shared::RollupAdapter;
use synd_chain_ingestor::client::BlockStreamT;
use synd_mchain::{
    client::MchainProvider,
    db::{ArbitrumBatch, MBlock, Slot, L1_BLOCK_NUM_HARDFORK_TS},
};
use thiserror::Error;
use tracing::{info, instrument, trace};

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
    rollup_adapter: impl RollupAdapter,
    mchain: &impl MchainProvider,
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
        let timestamp = seq_block.block_ref.timestamp;

        let mut mblock = MBlock {
            timestamp,
            slot: Slot {
                seq_block_number: seq_block.block_ref.number,
                seq_block_hash: seq_block.block_ref.hash,
                set_block_hash: FixedBytes::ZERO,
                set_block_number: 0,
            },
            payload: None,
        };

        let mut delayed_msgs = vec![];

        let mut blocks_per_slot: u64 = 1;
        let slot_end_ts = seq_block.block_ref.timestamp.saturating_sub(settlement_delay);

        while set_block.block_ref.timestamp <= slot_end_ts {
            blocks_per_slot += 1;
            delayed_msgs.append(&mut set_block.messages);
            set_block = settlement
                .recv(slot_end_ts)
                .await
                .map_err(|e| SlotterError::IngestorError(Chain::Settlement, e.to_string()))?;
            metrics.record_last_processed_block(set_block.block_ref.number, Chain::Settlement);
            metrics.update_chain_timestamp_lag(set_block.block_ref.timestamp, Chain::Settlement);
        }

        let l1_block_number = if timestamp < L1_BLOCK_NUM_HARDFORK_TS {
            seq_block.block_ref.number
        } else if delayed_msgs.is_empty() {
            0
        } else {
            set_block.block_ref.number - 1
        };

        let (tx_count, sequenced_batch) =
            build_batch(&seq_block, &rollup_adapter, l1_block_number, timestamp)?;

        if tx_count > 0 || !delayed_msgs.is_empty() {
            mblock.payload = Some(ArbitrumBatch::new(sequenced_batch, delayed_msgs));
        }
        mblock.slot.set_block_hash = set_block.block_ref.hash;
        mblock.slot.set_block_number = set_block.block_ref.number;

        trace!("Processing slot {:?}", mblock.slot);
        let time = std::time::Instant::now();
        mchain
            .add_batch(&mblock)
            .await
            .map_err(|e| SlotterError::SlotProcessorError(e.to_string()))?;
        if let Some(payload) = mblock.payload {
            info!(
                "Sent slot {} ({} seq, {} set) with timestamp {} in {:?}",
                mblock.slot.seq_block_number,
                tx_count,
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

    /// Error happened when building a batch from a list of txs - SHOULD NOT HAPPEN
    #[error("error when building a batch: {0}")]
    BuildBatchError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{metrics::SlotterMetrics, slotter::run};
    use alloy::{
        eips::Encodable2718,
        network::{EthereumWallet, TransactionBuilder},
        primitives::{address, Address, Bytes, FixedBytes, Log, U256},
        rpc::types::TransactionRequest,
        signers::local::PrivateKeySigner,
        sol_types::SolEvent,
    };
    use async_trait::async_trait;
    use common::types::{SequencingBlock, SettlementBlock};
    use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::TransactionProcessed;
    use prometheus_client::registry::Registry;
    use shared::types::BlockRef;
    use std::sync::{Arc, Mutex};
    use synd_block_builder::appchains::{
        arbitrum::arbitrum_adapter::ArbitrumAdapter,
        shared::sequencing_transaction_parser::L2MessageKind,
    };
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

    const SEQUENCING_CONTRACT_ADDRESS: Address =
        address!("0x0000000000000000000000000000000000000123");

    fn create_rollup_adapter() -> ArbitrumAdapter {
        ArbitrumAdapter::new(&synd_block_builder::config::BlockBuilderConfig {
            mchain_ws_url: String::new(),
            sequencing_contract_address: Some(SEQUENCING_CONTRACT_ADDRESS),
            arbitrum_bridge_address: Some(Address::ZERO),
            arbitrum_inbox_address: Some(Address::ZERO),
        })
    }

    fn create_seq_block(number: u64, timestamp: u64, txs: Vec<Bytes>) -> SequencingBlock {
        SequencingBlock {
            block_ref: BlockRef { number, timestamp, hash: FixedBytes::from([number as u8; 32]) },
            parent_hash: FixedBytes::from([(number - 1) as u8; 32]),
            logs: txs
                .iter()
                .map(|tx| Log {
                    address: SEQUENCING_CONTRACT_ADDRESS,
                    data: TransactionProcessed { sender: Default::default(), data: tx.clone() }
                        .encode_log_data(),
                })
                .collect(),
            log_tx_hashes: vec![],
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

    async fn dummy_tx() -> Bytes {
        let tx = TransactionRequest::default()
            .with_to(Address::ZERO)
            .with_nonce(0)
            .with_gas_limit(0)
            .with_max_fee_per_gas(0)
            .with_max_priority_fee_per_gas(0)
            .build(&EthereumWallet::from(PrivateKeySigner::random()))
            .await
            .unwrap();
        let mut encoded_tx = tx.encoded_2718();
        encoded_tx.splice(0..0, vec![L2MessageKind::SignedTx as u8]);
        encoded_tx.into()
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
        let seq_blocks = vec![create_seq_block(1, 100, vec![dummy_tx().await])];

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
            let _ = run(
                settlement_delay,
                sequencing,
                settlement,
                create_rollup_adapter(),
                &mchain_provider_clone,
                &metrics,
            )
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
        let seq_blocks = vec![create_seq_block(1, 200, vec![dummy_tx().await])];

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
            let _ = run(
                settlement_delay,
                sequencing,
                settlement,
                create_rollup_adapter(),
                &mchain_provider_clone,
                &metrics,
            )
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
        let seq_blocks = vec![create_seq_block(1, 100, vec![])];

        // Settlement blocks with no messages
        let set_blocks = vec![create_set_block(1, 89, vec![]), create_set_block(2, 91, vec![])];

        let sequencing = TestBlockStream::new(seq_blocks);
        let settlement = TestBlockStream::new(set_blocks);

        let mchain_provider_clone = mchain_provider.clone();
        let handle = tokio::spawn(async move {
            let _ = run(
                settlement_delay,
                sequencing,
                settlement,
                create_rollup_adapter(),
                &mchain_provider_clone,
                &metrics,
            )
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
        let seq_blocks = vec![create_seq_block(1, 100, vec![])];

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
            let _ = run(
                settlement_delay,
                sequencing,
                settlement,
                create_rollup_adapter(),
                &mchain_provider_clone,
                &metrics,
            )
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

    /// Helper to extract L1 block number and timestamp from batch headers
    fn extract_batch_headers(batch_data: &Bytes) -> eyre::Result<(u64, u64)> {
        use alloy::rlp::{Decodable, Header as RlpHeader};

        let mut decompressed = Vec::new();
        brotli::BrotliDecompress(&mut &batch_data[1..], &mut decompressed)?;

        let mut buf = &decompressed[..];
        let mut header_block_number = 0;
        let mut header_timestamp = 0;

        while !buf.is_empty() {
            let header = RlpHeader::decode(&mut buf)?;
            let segment_data = &buf[..header.payload_length];
            buf = &buf[header.payload_length..];

            if !segment_data.is_empty() {
                match segment_data[0] {
                    4 => {
                        // BatchSegmentKind::AdvanceL1BlockNumber
                        let mut segment_buf = &segment_data[1..];
                        header_block_number += u64::decode(&mut segment_buf)?;
                    }
                    3 => {
                        // BatchSegmentKind::AdvanceTimestamp
                        let mut segment_buf = &segment_data[1..];
                        header_timestamp += u64::decode(&mut segment_buf)?;
                    }
                    _ => {}
                }
            }
        }
        Ok((header_block_number, header_timestamp))
    }

    #[tokio::test]
    async fn test_hardfork_timestamp_changes_batch_header_source() -> eyre::Result<()> {
        let settlement_delay = 10;
        let dummy = dummy_tx().await;

        // Test before and after hardfork
        for (test_name, timestamp, expected_is_settlement) in [
            ("before_hardfork", L1_BLOCK_NUM_HARDFORK_TS - 1, false),
            ("after_hardfork", L1_BLOCK_NUM_HARDFORK_TS, true),
        ] {
            let mchain_provider = MockMchainProvider::new(1);
            let metrics = SlotterMetrics::new(&mut Registry::default());

            let seq_block_num = 1000;
            let set_block_num = 2000;
            let set_timestamp = timestamp - settlement_delay + 1;

            // Create a delayed message for settlement blocks to ensure l1_block_number uses
            // set_block.block_ref.number after hardfork (not 0).
            // Must be in a settlement block with timestamp <= slot_end_ts to be processed.
            let delayed_msg = DelayedMessage {
                kind: 0,
                sender: Address::ZERO,
                data: Bytes::from("test_message"),
                base_fee_l1: U256::from(1000),
            };

            let seq_blocks = vec![create_seq_block(seq_block_num, timestamp, vec![dummy.clone()])];
            let set_blocks = vec![
                create_set_block(set_block_num, timestamp - settlement_delay, vec![delayed_msg]),
                create_set_block(set_block_num + 1, set_timestamp, vec![]),
            ];

            let mchain_clone = mchain_provider.clone();
            let handle = tokio::spawn(async move {
                let _ = run(
                    settlement_delay,
                    TestBlockStream::new(seq_blocks),
                    TestBlockStream::new(set_blocks),
                    create_rollup_adapter(),
                    &mchain_clone,
                    &metrics,
                )
                .await;
            });

            mchain_provider.wait_for_blocks().await;
            drop(handle);

            let blocks = mchain_provider.get_blocks();
            let (block_num, ts) =
                extract_batch_headers(&blocks[0].payload.as_ref().unwrap().batch_data)?;

            // After hardfork, block number comes from settlement chain (when delayed_msgs present),
            // but timestamp always comes from sequencing chain
            let expected_block = if expected_is_settlement { set_block_num } else { seq_block_num };
            let expected_ts = timestamp;

            assert_eq!(block_num, expected_block, "{}: wrong block number", test_name);
            assert_eq!(ts, expected_ts, "{}: wrong timestamp", test_name);
        }

        Ok(())
    }
}
