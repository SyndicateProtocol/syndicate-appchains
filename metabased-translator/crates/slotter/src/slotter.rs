//! Slotter module for metabased-translator
use crate::metrics::SlotterMetrics;
use common::types::{Chain, PartialBlock, Slot, SlotProcessor};
use thiserror::Error;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, trace};

/// Ingests blocks from the sequencing and settlement chains, slots them into slots, and sends the
/// slots to the slot processor to generate mchain blocks.
#[allow(clippy::expect_used)]
pub async fn run(
    settlement_delay: u64,
    mut sequencing_rx: Receiver<PartialBlock>,
    mut settlement_rx: Receiver<PartialBlock>,
    slot_processor: impl SlotProcessor,
    metrics: SlotterMetrics,
) -> Result<(), SlotterError> {
    info!("Starting Slotter");
    let mut set_block = get_next_block(
        settlement_rx.recv().await.expect("settlement channel closed"),
        Chain::Settlement,
        &metrics,
    )?;

    loop {
        trace!("Waiting for sequencing block");
        let mut slot = Slot {
            sequencing: get_next_block(
                sequencing_rx.recv().await.expect("sequencing channel closed"),
                Chain::Sequencing,
                &metrics,
            )?,
            settlement: Default::default(),
        };

        trace!("Waiting for settlement blocks");
        while set_block.timestamp + settlement_delay <= slot.sequencing.timestamp {
            slot.settlement.push(set_block);
            set_block = get_next_block(
                settlement_rx.recv().await.expect("settlement channel closed"),
                Chain::Settlement,
                &metrics,
            )?;
        }

        trace!("Processing slot");
        metrics.record_blocks_per_slot(slot.settlement.len() as u64 + 1);
        metrics.record_last_slot_created(slot.sequencing.number);
        slot_processor
            .process_slot(&slot)
            .await
            .map_err(|e| SlotterError::SlotProcessorError(e.to_string()))?;
    }
}

#[allow(clippy::result_large_err)]
fn get_next_block(
    block: PartialBlock,
    chain: Chain,
    metrics: &SlotterMetrics,
) -> Result<PartialBlock, SlotterError> {
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
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SlotterError {
    #[error("Slot processor error: {0}")]
    SlotProcessorError(String),
}
