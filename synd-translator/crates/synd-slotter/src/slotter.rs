//! Slotter module for `synd-translator`
use crate::metrics::SlotterMetrics;
use alloy::primitives::{FixedBytes, B256};
use common::types::{Chain, SequencingBlock, SettlementBlock};
use shared::types::BlockRef;
use synd_chain_ingestor::client::BlockStreamT;
use synd_mchain::{
    client::{KnownState, Provider},
    db::{MBlock, Slot},
};
use thiserror::Error;
use tracing::{error, info, trace};

/// Ingests blocks from the sequencing and settlement chains, slots them into slots, and sends the
/// slots to the slot processor to generate `synd-mchain` blocks.
#[allow(clippy::expect_used)]
#[allow(clippy::cognitive_complexity)]
pub async fn run(
    settlement_delay: u64,
    known_state: Option<KnownState>,
    mut sequencing: impl BlockStreamT<SequencingBlock> + Send,
    mut settlement: impl BlockStreamT<SettlementBlock> + Send,
    provider: &impl Provider,
    metrics: &SlotterMetrics,
) -> Result<(), SlotterError> {
    let (mut latest_sequencing_block, mut latest_settlement_block) = match known_state {
        Some(known_state) => {
            (Some(known_state.sequencing_block), Some(known_state.settlement_block))
        }
        None => (None, None),
    };

    info!("Starting Slotter");

    trace!("Waiting for settlement block");
    let mut set_block = settlement
        .recv(0)
        .await
        .map_err(|e| SlotterError::IngestorError(Chain::Settlement, e.to_string()))?;

    if let Some(latest) = latest_settlement_block {
        if set_block.block_ref.hash != latest.hash {
            return Err(SlotterError::ReorgDetected {
                chain: Chain::Settlement,
                current_block: latest,
                received_block: set_block.block_ref,
                received_parent_hash: set_block.parent_hash,
            });
        }
    }
    latest_settlement_block = Some(set_block.block_ref.clone());

    loop {
        trace!("Waiting for sequencing block");
        let seq_block = sequencing
            .recv(0)
            .await
            .map_err(|e| SlotterError::IngestorError(Chain::Sequencing, e.to_string()))?;
        validate_block(
            &mut latest_sequencing_block,
            &seq_block.block_ref,
            seq_block.parent_hash,
            Chain::Sequencing,
            metrics,
        )?;
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
            validate_block(
                &mut latest_settlement_block,
                &set_block.block_ref,
                set_block.parent_hash,
                Chain::Settlement,
                metrics,
            )?;
        }

        if seq_block.tx_count > 0 || !messages.is_empty() {
            mblock.payload = Some((seq_block.batch, messages));
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
                payload.1.len(),
                mblock.timestamp,
                time.elapsed()
            );
        }
        metrics.record_blocks_per_slot(blocks_per_slot);
        metrics.record_last_slot_created(mblock.slot.seq_block_number);
    }
}

// TODO(SEQ-847): move this reorg checking logic to the ingestors instead.
#[allow(clippy::result_large_err)]
fn validate_block(
    latest: &mut Option<BlockRef>,
    block: &BlockRef,
    parent_hash: FixedBytes<32>,
    chain: Chain,
    metrics: &SlotterMetrics,
) -> Result<(), SlotterError> {
    if let Some(latest) = latest {
        if block.number > latest.number + 1 {
            return Err(SlotterError::BlockNumberSkipped {
                chain,
                current_block: latest.clone(),
                received_block: block.clone(),
            });
        }

        if !parent_hash.eq(&latest.hash) {
            return Err(SlotterError::ReorgDetected {
                chain,
                current_block: latest.clone(),
                received_block: block.clone(),
                received_parent_hash: parent_hash,
            });
        }

        if block.timestamp < latest.timestamp {
            return Err(SlotterError::EarlierTimestamp {
                chain,
                current: latest.timestamp,
                received: block.timestamp,
            });
        }
    }

    *latest = Some(block.clone());

    trace!(
        chain = ?chain,
        block_number = block.number,
        block_timestamp = block.timestamp,
        block_hash = %block.hash,
        "Processing block"
    );

    metrics.record_last_processed_block(block.number, chain);
    metrics.update_chain_timestamp_lag(block.timestamp, chain);
    Ok(())
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SlotterError {
    #[error(
        "{chain} chain reorg detected. Current: #{current_block}, Received: #{received_block}, Received parent hash: #{received_parent_hash}"
    )]
    ReorgDetected {
        chain: Chain,
        current_block: BlockRef,
        received_block: BlockRef,
        received_parent_hash: B256,
    },

    #[error("{chain} chain block number skipped. Current: #{current_block}, Received: #{received_block}")]
    BlockNumberSkipped { chain: Chain, current_block: BlockRef, received_block: BlockRef },

    #[error("{chain} chain timestamp must not decrease. Current: {current}, Received: {received}")]
    EarlierTimestamp { chain: Chain, current: u64, received: u64 },

    #[error("Slot processor error: {0}")]
    SlotProcessorError(String),

    #[error("{0} chain ingestor error: {1}")]
    IngestorError(Chain, String),
}

#[cfg(test)]
mod tests {
    use crate::{
        metrics::SlotterMetrics,
        slotter::{
            run, validate_block,
            SlotterError::{BlockNumberSkipped, EarlierTimestamp, ReorgDetected},
        },
    };
    use alloy::primitives::{FixedBytes, U256};
    use async_trait::async_trait;
    use common::types::{Chain, SettlementBlock};
    use prometheus_client::registry::Registry;
    use shared::types::BlockRef;
    use synd_chain_ingestor::client::BlockStreamT;
    use synd_mchain::client::{ClientError, DeserializeOwned, KnownState, Provider, ToRpcParams};

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    #[allow(dead_code)]
    struct PanicProvider {}

    #[async_trait]
    impl Provider for PanicProvider {
        async fn request<Params: ToRpcParams + Send, T: DeserializeOwned>(
            &self,
            _method: &'static str,
            _params: Params,
        ) -> Result<T, ClientError> {
            panic!("unexpected call to raw_request");
        }
    }

    struct MockBlockStream<Block>(pub Vec<Block>);

    #[async_trait]
    impl<Block: Send> BlockStreamT<Block> for MockBlockStream<Block> {
        async fn recv(&mut self, timestamp: u64) -> eyre::Result<Block> {
            assert_eq!(timestamp, 0);
            Ok(self.0.pop().unwrap())
        }
    }

    #[tokio::test]
    async fn known_state_reorg() -> eyre::Result<()> {
        let latest: BlockRef = Default::default();
        let set_block = SettlementBlock {
            block_ref: BlockRef { hash: U256::from(1).into(), ..Default::default() },
            ..Default::default()
        };
        let set_stream = MockBlockStream(vec![set_block.clone()]);
        let result = run(
            0,
            Some(KnownState {
                sequencing_block: Default::default(),
                settlement_block: latest.clone(),
            }),
            MockBlockStream(Default::default()),
            set_stream,
            &PanicProvider {},
            &SlotterMetrics::new(&mut Registry::default()),
        )
        .await;
        assert_eq!(
            result,
            Err(ReorgDetected {
                chain: Chain::Settlement,
                current_block: latest,
                received_block: set_block.block_ref,
                received_parent_hash: set_block.parent_hash,
            })
        );
        Ok(())
    }

    #[test]
    fn valid_block() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(Default::default());
            let block = BlockRef { number: 1, ..Default::default() };
            assert_eq!(
                validate_block(
                    &mut latest,
                    &block,
                    FixedBytes::ZERO,
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Ok(())
            );
            assert_eq!(latest, Some(block));
            let mut latest = None;
            let block = BlockRef { number: 10, ..Default::default() };
            assert_eq!(
                validate_block(
                    &mut latest,
                    &block,
                    FixedBytes::ZERO,
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Ok(())
            );
            assert_eq!(latest, Some(block));
        }
    }

    #[test]
    fn block_number_skipped() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(Default::default());
            let latest_copy = latest.clone();
            let block = BlockRef { number: 2, ..Default::default() };
            assert_eq!(
                validate_block(
                    &mut latest,
                    &block,
                    FixedBytes::ZERO,
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Err(BlockNumberSkipped {
                    chain,
                    current_block: latest_copy.clone().unwrap(),
                    received_block: block
                })
            );
            assert_eq!(latest, latest_copy);
        }
    }

    #[test]
    fn reorg_detected() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(Default::default());
            let latest_copy = latest.clone();
            let block = BlockRef { number: 1, ..Default::default() };
            let parent_hash = U256::from(1).into();
            assert_eq!(
                validate_block(
                    &mut latest,
                    &block,
                    parent_hash,
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Err(ReorgDetected {
                    chain,
                    current_block: latest_copy.clone().unwrap(),
                    received_block: block,
                    received_parent_hash: parent_hash,
                })
            );
            assert_eq!(latest, latest_copy);
        }
    }

    #[test]
    fn earlier_timestamp() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(BlockRef { timestamp: 1, ..Default::default() });
            let latest_copy = latest.clone();
            let block = BlockRef { number: 1, ..Default::default() };
            assert_eq!(
                validate_block(
                    &mut latest,
                    &block,
                    FixedBytes::ZERO,
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Err(EarlierTimestamp { chain, current: 1, received: 0 })
            );
            assert_eq!(latest, latest_copy);
        }
    }
}
