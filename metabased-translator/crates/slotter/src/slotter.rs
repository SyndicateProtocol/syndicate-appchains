//! Slotter module for metabased-translator
use crate::metrics::SlotterMetrics;
use alloy::primitives::B256;
use common::types::{BlockRef, Chain, KnownState, PartialBlock, Slot, SlotProcessor};
use eyre::Report;
use thiserror::Error;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, trace};

/// Ingests blocks from the sequencing and settlement chains, slots them into slots, and sends the
/// slots to the slot processor to generate mchain blocks.
#[allow(clippy::expect_used)]
pub async fn run(
    settlement_delay: u64,
    known_state: Option<KnownState>,
    mut sequencing_rx: Receiver<PartialBlock>,
    mut settlement_rx: Receiver<PartialBlock>,
    slot_processor: impl SlotProcessor,
    metrics: SlotterMetrics,
) -> Result<(), SlotterError> {
    let (mut latest_sequencing_block, mut latest_settlement_block) = match known_state {
        Some(known_state) => {
            (Some(known_state.sequencing_block), Some(known_state.settlement_block))
        }
        None => (None, None),
    };

    info!("Starting Slotter");
    let mut slot: Slot = Default::default();

    loop {
        info!("Waiting for sequencing block");
        slot.sequencing = validate_block(
            &mut latest_sequencing_block,
            sequencing_rx.recv().await.expect("sequencing channel closed"),
            Chain::Sequencing,
            &metrics,
        )?;

        info!("Waiting for settlement blocks");
        let mut block = match slot.settlement.pop() {
            Some(block) => block,
            None => validate_block(
                &mut latest_settlement_block,
                settlement_rx.recv().await.expect("settlement channel closed"),
                Chain::Settlement,
                &metrics,
            )?,
        };

        while block.timestamp + settlement_delay <= slot.sequencing.timestamp {
            slot.settlement.push(block);
            block = validate_block(
                &mut latest_settlement_block,
                settlement_rx.recv().await.expect("settlement channel closed"),
                Chain::Settlement,
                &metrics,
            )?;
        }

        info!("Processing slot");
        metrics.record_blocks_per_slot(slot.settlement.len() as u64 + 1);
        metrics.record_last_slot_created(slot.sequencing.number);
        slot_processor.process_slot(&slot).await?;
        slot.settlement = vec![block];
    }
}

fn validate_block(
    latest: &mut Option<BlockRef>,
    block: PartialBlock,
    chain: Chain,
    metrics: &SlotterMetrics,
) -> Result<PartialBlock, SlotterError> {
    if let Some(latest) = latest {
        if block.number > latest.number + 1 {
            return Err(SlotterError::BlockNumberSkipped {
                chain,
                current_block: Box::new(latest.clone()),
                received_block: Box::new(BlockRef::new(&block)),
            });
        }

        if !block.parent_hash.eq(&latest.hash) {
            return Err(SlotterError::ReorgDetected {
                chain,
                current_block: Box::new(latest.clone()),
                received_block: Box::new(BlockRef::new(&block)),
                received_parent_hash: block.parent_hash,
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

    *latest = Some(BlockRef { number: block.number, timestamp: block.timestamp, hash: block.hash });

    trace!(
        chain = ?chain,
        block_number = block.number,
        block_timestamp = block.timestamp,
        block_hash = %block.hash,
        "Processing block"
    );

    metrics.record_last_processed_block(block.number, chain);
    metrics.update_chain_timestamp_lag(block.timestamp, chain);
    Ok(block)
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum SlotterError {
    #[error(
        "{chain} chain reorg detected. Current: #{current_block}, Received: #{received_block}, Received parent hash: #{received_parent_hash}"
    )]
    ReorgDetected {
        chain: Chain,
        current_block: Box<BlockRef>,
        received_block: Box<BlockRef>,
        received_parent_hash: B256,
    },

    #[error("{chain} chain block number skipped. Current: #{current_block}, Received: #{received_block}")]
    BlockNumberSkipped { chain: Chain, current_block: Box<BlockRef>, received_block: Box<BlockRef> },

    #[error("{chain} chain timestamp must not decrease. Current: {current}, Received: {received}")]
    EarlierTimestamp { chain: Chain, current: u64, received: u64 },

    #[error("Slot processor error: {0}")]
    SlotProcessorError(#[from] Report),
}
