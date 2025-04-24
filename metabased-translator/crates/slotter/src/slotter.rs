//! Slotter module for metabased-translator
use crate::metrics::SlotterMetrics;
use alloy::primitives::{FixedBytes, B256};
use common::types::{BlockRef, Chain, KnownState, SeqBlock, SetBlock};
use mchain::{
    db::{MBlock, Slot},
    mchain::MProvider,
};
use thiserror::Error;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, trace};

/// Ingests blocks from the sequencing and settlement chains, slots them into slots, and sends the
/// slots to the slot processor to generate mchain blocks.
#[allow(clippy::expect_used)]
pub async fn run(
    settlement_delay: u64,
    known_state: Option<KnownState>,
    mut sequencing_rx: Receiver<SeqBlock>,
    mut settlement_rx: Receiver<SetBlock>,
    provider: &MProvider,
    metrics: SlotterMetrics,
) -> Result<(), SlotterError> {
    let (mut latest_sequencing_block, mut latest_settlement_block) = match known_state {
        Some(known_state) => {
            (Some(known_state.sequencing_block), Some(known_state.settlement_block))
        }
        None => (None, None),
    };

    info!("Starting Slotter");

    trace!("Waiting for settlement block");
    let mut set_block = settlement_rx.recv().await.expect("settlement channel closed");
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
        let seq_block = sequencing_rx.recv().await.expect("sequencing channel closed");
        validate_block(
            &mut latest_sequencing_block,
            &seq_block.block_ref,
            seq_block.parent_hash,
            Chain::Sequencing,
            &metrics,
        )?;
        let mut mblock = MBlock {
            timestamp: seq_block.block_ref.timestamp,
            slot: Slot {
                seq_block_number: seq_block.block_ref.number,
                seq_block_hash: seq_block.block_ref.hash,
                set_block_hash: FixedBytes::ZERO,
                set_block_number: 0,
            },
            batch: seq_block.batch,
            messages: Default::default(),
        };

        trace!("Waiting for settlement blocks");
        let mut blocks_per_slot: u64 = 1;
        while set_block.block_ref.timestamp + settlement_delay <= seq_block.block_ref.timestamp {
            blocks_per_slot += 1;
            mblock.messages.append(&mut set_block.messages);
            set_block = settlement_rx.recv().await.expect("settlement channel closed");
            validate_block(
                &mut latest_settlement_block,
                &set_block.block_ref,
                set_block.parent_hash,
                Chain::Settlement,
                &metrics,
            )?;
        }

        mblock.slot.set_block_hash = set_block.block_ref.hash;
        mblock.slot.set_block_number = set_block.block_ref.number;

        if seq_block.tx_count == 0 && mblock.messages.is_empty() {
            trace!("Skipping empty slot {:?}", mblock.slot);
            provider
                .update_slot(&mblock.slot)
                .await
                .map_err(|e| SlotterError::SlotProcessorError(e.to_string()))?;
            metrics.record_last_slot_created(mblock.slot.seq_block_number);
            continue;
        }

        trace!("Processing slot {:?}", mblock.slot);
        info!("Sending slot {} with timestamp {}", mblock.slot.seq_block_number, mblock.timestamp);
        provider
            .add_batch(&mblock)
            .await
            .map_err(|e| SlotterError::SlotProcessorError(e.to_string()))?;
        info!("Sent slot {}", mblock.slot.seq_block_number);
        metrics.record_blocks_per_slot(blocks_per_slot);
        metrics.record_last_slot_created(mblock.slot.seq_block_number);
        // metrics.record_transactions_per_slot(seq_block.tx_count + mblock.messages.len() as u64);
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
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{
        metrics::SlotterMetrics,
        slotter::{
            validate_block, SetBlock,
            SlotterError::{BlockNumberSkipped, EarlierTimestamp, ReorgDetected},
        },
    };
    use alloy::primitives::{FixedBytes, U256};
    use common::types::{BlockRef, Chain, KnownState};
    use mchain::mchain::MProvider;
    use prometheus_client::registry::Registry;
    use tokio::sync::mpsc;

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    #[tokio::test]
    async fn known_state_reorg() -> eyre::Result<()> {
        let (_seq_tx, seq_rx) = mpsc::channel(1);
        let (set_tx, set_rx) = mpsc::channel(1);
        let latest: BlockRef = Default::default();
        let set_block = SetBlock {
            block_ref: BlockRef { hash: U256::from(1).into(), ..Default::default() },
            ..Default::default()
        };
        set_tx.send(set_block.clone()).await?;
        let result = run(
            0,
            Some(KnownState {
                mchain_block_number: 0,
                sequencing_block: Default::default(),
                settlement_block: latest.clone(),
            }),
            seq_rx,
            set_rx,
            &MProvider::new("http://localhost:80".parse()?),
            SlotterMetrics::new(&mut Registry::default()),
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
