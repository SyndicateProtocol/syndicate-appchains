//! Slotter module for metabased-translator
use crate::metrics::SlotterMetrics;
use alloy::primitives::B256;
use common::types::{BlockRef, Chain, KnownState, PartialBlock, Slot, SlotProcessor};
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
    let mut set_block = get_next_block(
        &mut latest_settlement_block,
        settlement_rx.recv().await.expect("settlement channel closed"),
        Chain::Settlement,
        &metrics,
    )?;

    loop {
        trace!("Waiting for sequencing block");
        let mut slot = Slot {
            sequencing: get_next_block(
                &mut latest_sequencing_block,
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
                &mut latest_settlement_block,
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
    latest: &mut Option<BlockRef>,
    block: PartialBlock,
    chain: Chain,
    metrics: &SlotterMetrics,
) -> Result<PartialBlock, SlotterError> {
    if let Some(latest) = latest {
        if block.number > latest.number + 1 {
            return Err(SlotterError::BlockNumberSkipped {
                chain,
                current_block: latest.clone(),
                received_block: BlockRef::new(&block),
            });
        }

        if !block.parent_hash.eq(&latest.hash) {
            return Err(SlotterError::ReorgDetected {
                chain,
                current_block: latest.clone(),
                received_block: BlockRef::new(&block),
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

    *latest = Some(BlockRef::new(&block));

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
    use crate::{
        metrics::SlotterMetrics,
        slotter::{
            get_next_block,
            SlotterError::{BlockNumberSkipped, EarlierTimestamp, ReorgDetected},
        },
    };
    use alloy::primitives::{FixedBytes, U256};
    use common::types::{BlockRef, Chain, PartialBlock};
    use prometheus_client::registry::Registry;

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    #[test]
    fn valid_block() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(BlockRef { number: 0, timestamp: 0, hash: FixedBytes::ZERO });
            let block = PartialBlock { number: 1, ..Default::default() };
            assert_eq!(
                get_next_block(
                    &mut latest,
                    block.clone(),
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Ok(block.clone())
            );
            assert_eq!(latest, Some(BlockRef::new(&block)));
            let mut latest = None;
            let block = PartialBlock { number: 10, ..Default::default() };
            assert_eq!(
                get_next_block(
                    &mut latest,
                    block.clone(),
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Ok(block.clone())
            );
            assert_eq!(latest, Some(BlockRef::new(&block)));
        }
    }

    #[test]
    fn block_number_skipped() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(BlockRef { number: 0, timestamp: 0, hash: FixedBytes::ZERO });
            let latest_copy = latest.clone();
            let block = PartialBlock { number: 2, ..Default::default() };
            assert_eq!(
                get_next_block(
                    &mut latest,
                    block.clone(),
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Err(BlockNumberSkipped {
                    chain,
                    current_block: latest_copy.clone().unwrap(),
                    received_block: BlockRef::new(&block)
                })
            );
            assert_eq!(latest, latest_copy);
        }
    }

    #[test]
    fn reorg_detected() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(BlockRef { number: 0, timestamp: 0, hash: FixedBytes::ZERO });
            let latest_copy = latest.clone();
            let block =
                PartialBlock { number: 1, parent_hash: U256::from(1).into(), ..Default::default() };
            assert_eq!(
                get_next_block(
                    &mut latest,
                    block.clone(),
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Err(ReorgDetected {
                    chain,
                    current_block: latest_copy.clone().unwrap(),
                    received_block: BlockRef::new(&block),
                    received_parent_hash: block.parent_hash,
                })
            );
            assert_eq!(latest, latest_copy);
        }
    }

    #[test]
    fn earlier_timestamp() {
        for chain in [Chain::Sequencing, Chain::Settlement] {
            let mut latest = Some(BlockRef { number: 0, timestamp: 1, hash: FixedBytes::ZERO });
            let latest_copy = latest.clone();
            let block = PartialBlock { number: 1, ..Default::default() };
            assert_eq!(
                get_next_block(
                    &mut latest,
                    block.clone(),
                    chain,
                    &SlotterMetrics::new(&mut Registry::default())
                ),
                Err(EarlierTimestamp { chain, current: 1, received: 0 })
            );
            assert_eq!(latest, latest_copy);
        }
    }
}
