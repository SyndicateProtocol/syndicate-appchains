//! Slotting module for metabased-translator

use crate::config::SlottingConfig;
use common::{
    service_status::{ServiceStatus, Status},
    types::{Block, BlockAndReceipts, Chain, Slot, SlotState},
};
use std::{cmp::Ordering, collections::LinkedList};
use thiserror::Error;
use tokio::{
    select,
    sync::mpsc::{channel, error::SendError, Receiver, Sender},
};
use tracing::{debug, error, info};

/// Maximum time to wait for blocks before considering a slot final (24 hours in milliseconds)
const MAX_WAIT_MS: u64 = 24 * 60 * 60 * 1000;

/// Polls and ingests blocks from an Ethereum chain
///
/// Slots are stored in a linked list ordered by timestamp, with newer slots at the back.
/// Each slot has a fixed duration and contains blocks from both chains that fall within its window.
///
/// ```text
/// LinkedList:
/// Back                                                           Front
/// [Oldest] <-> [Slot N-2] <-> [Slot N-1] <-> [Current Slot] <-> [Newest]
///
/// Where:
/// - max_slots is the number of slots to keep and is determined by `MAX_WAIT_MS` / `slot_duration_ms`, thus if a slot is to be dropped, it must be marked as Safe
/// - Slots older than max_slots are dropped
/// - A slot becomes Unsafe when both chains progress past it
/// ```
#[derive(Debug)]
pub struct Slotter {
    config: SlottingConfig,

    sequencing_chain_rx: Receiver<BlockAndReceipts>,
    settlement_chain_rx: Receiver<BlockAndReceipts>,

    latest_sequencing_chain_block: Option<BlockRef>,
    latest_settlement_chain_block: Option<BlockRef>,

    status: ServiceStatus,

    /// Stores the last N slots (both open and closed)
    slots: LinkedList<Slot>,

    /// Maximum number of slots to keep
    max_slots: usize,
}

#[derive(Debug)]
struct BlockRef {
    number: u64,
    timestamp: u64,
}

impl BlockRef {
    const fn new(block: &Block) -> Self {
        Self { number: block.number, timestamp: block.timestamp }
    }
}

impl Slotter {
    /// Creates a new [`Slotter`] that receives blocks from two chains and organizes them into
    /// slots.
    ///
    /// # Arguments
    /// * `sequencing_chain_receiver` - Channel receiving blocks from the sequencing chain
    /// * `settlement_chain_receiver` - Channel receiving blocks from the settlement chain
    /// * `config` - Configuration for slot timing and initial state
    ///
    /// # Details
    /// The [`Slotter`] maintains a window of slots spanning the last 24 hours ([`MAX_WAIT_MS`]),
    /// with the number of slots determined by [`MAX_WAIT_MS`] / `slot_duration_ms`.
    ///
    /// Each slot contains blocks from both chains which timestamps fall within the slot's window:
    /// (`slot_timestamp` - `slot_duration`, `slot_timestamp`]
    ///
    /// # Returns
    /// A new [`Slotter`] instance that can be started with `start()`
    pub async fn new(
        sequencing_chain_receiver: Receiver<BlockAndReceipts>,
        settlement_chain_receiver: Receiver<BlockAndReceipts>,
        config: SlottingConfig,
    ) -> Self {
        let max_slots = (MAX_WAIT_MS / config.slot_duration_ms) as usize;

        Self {
            sequencing_chain_rx: sequencing_chain_receiver,
            settlement_chain_rx: settlement_chain_receiver,
            latest_sequencing_chain_block: None,
            latest_settlement_chain_block: None,
            config: config.clone(),
            status: ServiceStatus::new(),
            slots: {
                let mut slots = LinkedList::new();
                slots.push_front(Slot::new(config.start_slot_number, config.start_slot_timestamp));
                slots
            },
            max_slots,
        }
    }

    /// Starts the [`Slotter`] in a new thread.
    ///
    /// The [`Slotter`] will:
    /// 1. Receive blocks from both sequencing and settlement chains
    /// 2. Place blocks into slots based on their timestamps
    /// 3. Mark slots as unsafe when both chains have progressed past them (or max wait time has
    ///    passed)
    /// 4. Send completed slots through the returned channel
    ///
    /// # Returns a receiver that will get slots as they are processed.
    /// TODO SEQ-480 - implement restore from DB
    pub fn start(mut self) -> Receiver<Slot> {
        self.status.store(Status::Started);
        let (sender, receiver) = channel(100); // TODO SEQ-490 - channel size shouldn't be hardcoded here
        info!("Starting Slotter");
        tokio::spawn(async move {
            loop {
                if self.status.load() == Status::Stopped {
                    // TODO SEQ-479 - graceful shutdown triggered
                    // - stop processing new blocks
                    // - go through the slots and save all safe slots to the DB (timestamp <
                    //   current_time - MAX_WAIT_MS)
                    // - potentially save all unsafe/opened blocks to the DB, so they can be resumed
                    //   later
                    info!("Slotter stopped");
                    return;
                }

                let process_result = select! {
                    Some(block) = self.sequencing_chain_rx.recv() => {
                        self.process_block(
                            block,
                            Chain::Sequencing,
                            &sender,
                            self.config.slot_duration_ms,
                        )
                        .await
                    }
                    Some(block) = self.settlement_chain_rx.recv() => {
                        self.process_block(
                            block,
                            Chain::Settlement,
                            &sender,
                            self.config.slot_duration_ms,
                        )
                        .await
                    }
                };

                match process_result {
                    Ok(_) => (),
                    Err(e) => match e {
                        SlotterError::ReorgDetected { .. } => {
                            panic!("Reorgs not yet implemented {e}"); // TODO SEQ-429 - implement
                                                                      // reorg handing
                        }
                        SlotterError::BlockNumberSkipped { .. } => {
                            panic!("Block number skipped {e}"); // TODO SEQ-489 - decide what to do
                                                                // if a block is skipped
                        }
                        SlotterError::BlockTooOld { .. } => {
                            panic!("Block too old {e}"); // TODO SEQ-489 - decide what to do if a
                                                         // block is too old
                        }
                        SlotterError::NoSlotsAvailable => {
                            panic!("No slots available. This should never happen - if it does, it's an implementation error. {e}");
                        }
                        SlotterError::SlotSendError(_) => {
                            panic!("Failed to send slot through channel: {e}"); // TODO SEQ-489 -
                                                                                // decide what to do
                                                                                // here (likely to
                                                                                // occur during
                                                                                // shutdown, or the
                                                                                // received is
                                                                                // blocked)
                        }
                        SlotterError::NonIncreasingTimestamp { .. } => {
                            panic!("Non-increasing timestamp - this should never happen (where a block is received with the expected block number, but a lower timestamp) {e}");
                        }
                    },
                }
            }
        });

        receiver
    }

    /// Stops the [`Slotter`] thread.
    ///
    /// Note: Currently performs a hard stop. Future implementations will:
    /// - Wait for thread to complete
    /// - Write info to DB, so it can be resumed later
    pub fn stop(&mut self) {
        info!("Stopping Slotter");
        self.status.store(Status::Stopped);
    }

    /// Marks slots as unsafe when both chains have progressed past them.
    ///
    /// A slot becomes unsafe when both chains have blocks with timestamps greater than the slot's
    /// timestamp. This indicates that both chains have moved past this slot and no more blocks
    /// will be added to it.
    ///
    /// # Algorithm
    /// 1. Gets the latest block timestamp from both chains
    /// 2. Takes the minimum timestamp between them
    /// 3. Marks all slots with timestamps less than this minimum as unsafe
    /// 4. Sends unsafe slots
    ///
    /// # Early Returns
    /// - Returns early if it encounters a slot that's already marked as unsafe (assumes all older
    ///   slots are also unsafe)
    ///
    /// # Errors
    /// - Returns `SlotterError::SlotSendError` if sending a slot through the channel fails
    async fn mark_unsafe_slots(
        &mut self,
        block_timestamp: u64,
        chain: Chain,
        sender: &Sender<Slot>,
    ) -> Result<(), SlotterError> {
        // Get the other chain's latest block timestamp
        let other_chain_timestamp = match chain {
            Chain::Sequencing => self.latest_settlement_chain_block.as_ref(),
            Chain::Settlement => self.latest_sequencing_chain_block.as_ref(),
        }
        .map(|b| b.timestamp);

        // Only mark slots as unsafe if we have blocks from both chains
        if let Some(other_timestamp) = other_chain_timestamp {
            let min_timestamp = other_timestamp.min(block_timestamp);
            debug!(min_timestamp, "Marking slots as unsafe");

            // Mark slots as unsafe if both chains have progressed past them
            for slot in &mut self.slots {
                if slot.state == SlotState::Unsafe {
                    debug!(%slot, "Found unsafe slot, stopping iteration");
                    return Ok(()); // assume all blocks past this point are already marked as unsafe
                }
                if slot.timestamp < min_timestamp && slot.state == SlotState::Open {
                    info!(%slot, "Marking slot as unsafe");
                    slot.state = SlotState::Unsafe;
                    sender.send(slot.clone()).await?
                }
            }
        }
        Ok(())
    }

    async fn process_block(
        &mut self,
        block_info: BlockAndReceipts,
        chain: Chain,
        sender: &Sender<Slot>,
        slot_duration_ms: u64,
    ) -> Result<(), SlotterError> {
        let block_timestamp = block_info.block.timestamp;
        self.update_latest_block(&block_info.block, chain)?;
        let latest_slot = self.slots.front_mut().ok_or(SlotterError::NoSlotsAvailable)?;
        debug!(
            ?chain,
            block_number = block_info.block.number,
            block_timestamp = block_info.block.timestamp,
            block_hash = %block_info.block.hash,
            %latest_slot,
            "Processing block"
        );

        match block_slot_ordering(block_timestamp, latest_slot.timestamp, slot_duration_ms) {
            Ordering::Less => {
                debug!("Block belongs to earlier slot");
                // Find the slot this block belongs to
                let mut inserted = false;
                for slot in &mut self.slots {
                    if matches!(
                        block_slot_ordering(block_timestamp, slot.timestamp, slot_duration_ms,),
                        Ordering::Equal
                    ) {
                        debug!(%slot, "Block fits in slot");
                        slot.push_block(block_info, chain);
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    return Err(SlotterError::BlockTooOld { chain, block_timestamp });
                }
            }
            Ordering::Equal => {
                debug!("Block fits in current slot");
                latest_slot.push_block(block_info, chain)
            }
            Ordering::Greater => {
                debug!("Creating new slots to fit block");
                let mut latest_timestamp = latest_slot.timestamp;
                let mut latest_slot_number = latest_slot.slot_number;

                // Create empty slots until we reach or pass the block's timestamp
                // this accomplishes two things:
                // - it creates slots for which we might still receive blocks (from the other chain)
                // - keeps the list full, meaning the max_slots limit will always trigger on the
                //   correct max_wait window
                while latest_timestamp < block_timestamp {
                    let next_timestamp = latest_timestamp + slot_duration_ms;
                    let next_slot_number = latest_slot_number + 1;
                    let slot = Slot::new(next_slot_number, next_timestamp);
                    debug!(%slot, "Creating new slot");
                    self.slots.push_front(slot);
                    latest_timestamp = next_timestamp;
                    latest_slot_number = next_slot_number;
                }

                let latest_slot = self.slots.front_mut().ok_or(SlotterError::NoSlotsAvailable)?;
                debug!(%latest_slot, "Pushing block to the newly created latest slot");
                latest_slot.push_block(block_info, chain);
            }
        }

        self.mark_unsafe_slots(block_timestamp, chain, sender).await?;
        self.cleanup_slots(sender).await?;
        Ok(())
    }

    async fn cleanup_slots(&mut self, sender: &Sender<Slot>) -> Result<(), SlotterError> {
        // Clean up old slots
        debug!("Cleaning up slots");
        while self.slots.len() > self.max_slots {
            if let Some(mut slot) = self.slots.pop_back() {
                if slot.state == SlotState::Open {
                    debug!(%slot, "Cleaned up old slot, marking as unsafe and sending");
                    slot.state = SlotState::Unsafe;
                    sender.send(slot).await?
                } else {
                    debug!(%slot, "Cleaned up old slot");
                }
            }
        }
        Ok(())
    }

    fn update_latest_block(&mut self, block: &Block, chain: Chain) -> Result<(), SlotterError> {
        if let Some(latest) = match chain {
            Chain::Sequencing => self.latest_sequencing_chain_block.as_ref(),
            Chain::Settlement => self.latest_settlement_chain_block.as_ref(),
        } {
            if block.number <= latest.number {
                return Err(SlotterError::ReorgDetected {
                    chain,
                    current_block_number: latest.number,
                    received_block_number: block.number,
                });
            }

            if block.number != latest.number + 1 {
                return Err(SlotterError::BlockNumberSkipped {
                    chain,
                    current_block_number: latest.number,
                    received_block_number: block.number,
                });
            }

            if block.timestamp <= latest.timestamp {
                return Err(SlotterError::NonIncreasingTimestamp {
                    chain,
                    current: latest.timestamp,
                    received: block.timestamp,
                });
            }
        }

        debug!(
            ?chain,
            block_number = block.number,
            block_timestamp = block.timestamp,
            block_hash = %block.hash,
            "Updated latest block"
        );

        match chain {
            Chain::Sequencing => self.latest_sequencing_chain_block = Some(BlockRef::new(block)),
            Chain::Settlement => self.latest_settlement_chain_block = Some(BlockRef::new(block)),
        }
        Ok(())
    }
}

/// Automatically map `sender.send(slot).await?` to `SlotterError::SlotSendError`
impl From<SendError<Slot>> for SlotterError {
    fn from(e: SendError<Slot>) -> Self {
        Self::SlotSendError(e)
    }
}

/// Determines how a block's timestamp relates to a slot's window.
///
/// Returns:
/// - `Ordering::Less` if block belongs in an earlier slot
/// - `Ordering::Equal` if block belongs in this slot
/// - `Ordering::Greater` if block belongs in a later slot
///
/// A block belongs in a slot if its timestamp falls within the window:
/// (`slot_timestamp` - `slot_duration`, `slot_timestamp`]
///
/// For example, with:
/// - `slot_duration` = 12
/// - `slot_timestamp` = 1000
///
/// The slot will include blocks with timestamps:
/// - 988 < timestamp <= 1000
const fn block_slot_ordering(
    block_timestamp_ms: u64,
    slot_timestamp_ms: u64,
    slot_duration_ms: u64,
) -> Ordering {
    let slot_start = slot_timestamp_ms.saturating_sub(slot_duration_ms);

    if block_timestamp_ms <= slot_start {
        Ordering::Less
    } else if block_timestamp_ms <= slot_timestamp_ms {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum SlotterError {
    #[error("No slots available")]
    NoSlotsAvailable,

    #[error("Failed to send slot through channel: {0}")]
    SlotSendError(SendError<Slot>),

    #[error("Block timestamp {block_timestamp} is less than the latest slot and does not match any slot. is the {chain} chain more than MAX_WAIT(24 hours) behind?")]
    BlockTooOld { chain: Chain, block_timestamp: u64 },

    #[error("{chain} chain reorg detected. Current: #{current_block_number}, Received: #{received_block_number}")]
    ReorgDetected { chain: Chain, current_block_number: u64, received_block_number: u64 },

    #[error("{chain} chain block number skipped. Current: #{current_block_number}, Received: #{received_block_number}")]
    BlockNumberSkipped { chain: Chain, current_block_number: u64, received_block_number: u64 },

    #[error("{chain} chain timestamp must increase. Current: {current}, Received: {received}")]
    NonIncreasingTimestamp { chain: Chain, current: u64, received: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;
    use std::str::FromStr;

    fn create_test_block(number: u64, timestamp: u64) -> BlockAndReceipts {
        BlockAndReceipts {
            block: Block {
                hash: B256::from_str(
                    "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(),
                number,
                parent_hash: B256::from_str(
                    "0234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(),
                logs_bloom: "0x0".to_string(),
                transactions_root: "0x0".to_string(),
                state_root: "0x0".to_string(),
                receipts_root: "0x0".to_string(),
                timestamp,
                transactions: vec![],
            },
            receipts: vec![],
        }
    }

    /// Test scenario:
    /// ```text
    /// Slot 0 [9000-10000]:
    /// ┌───────────────────┐
    /// │ empty             │
    /// └───────────────────┘
    ///
    /// Slot 1 [10001-11000]:
    /// ┌───────────────────┐
    /// │ seq    @ 10001 #1 │
    /// │ seq    @ 11000 #2 │
    /// │ settle @ 10001 #1 │ -> Only marked as Unsafe once the blocks for next slot are received
    /// └───────────────────┘
    ///
    /// Slot 2 [11001-12000]:
    /// ┌───────────────────┐
    /// │ seq    @ 11001 #3 │
    /// │ settle @ 11001 #2 │ -> Shouldn't be received (never marked as unsafe)
    /// └───────────────────┘
    ///
    /// Legend:
    /// @ timestamp
    /// # block number
    /// ```
    #[tokio::test]
    async fn test_slotter() {
        let (seq_tx, seq_rx) = channel(100);
        let (settle_tx, settle_rx) = channel(100);

        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000; // 1 second

        let slotter = Slotter::new(
            seq_rx,
            settle_rx,
            SlottingConfig {
                slot_duration_ms,
                start_slot_number: 0,
                start_slot_timestamp: slot_start_timestamp_ms,
            },
        )
        .await;

        let mut slot_rx = slotter.start();
        assert!(slot_rx.is_empty());

        // send initial blocks, these should fit in slot 1 and make slot 0 be marked as unsafe
        seq_tx.send(create_test_block(1, 10_001)).await.unwrap();

        settle_tx.send(create_test_block(1, 10_002)).await.unwrap();

        let slot0 = slot_rx.recv().await.unwrap();
        assert_eq!(slot0.timestamp, slot_start_timestamp_ms);
        assert_eq!(slot0.slot_number, 0);
        assert_eq!(slot0.sequencing_chain_blocks.len(), 0);
        assert_eq!(slot0.settlement_chain_blocks.len(), 0);
        assert_eq!(slot0.state, SlotState::Unsafe);

        assert!(slot_rx.is_empty());

        // send a block for the settlement chain that should fit in slot 2
        settle_tx.send(create_test_block(2, 11_001)).await.unwrap(); // this block should be fit in slot 1

        // slot 1 should still be opened (we haven't received any blocks for the sequencing chain
        // ahead of the slot)
        assert!(slot_rx.is_empty());

        // send a bock for the sequencing chain that still fits in slot 1
        seq_tx.send(create_test_block(2, 11_000)).await.unwrap();

        // slot 1 should still be opened (we haven't received any blocks for the sequencing chain
        // ahead of the slot)
        assert!(slot_rx.is_empty());

        // send a block for the sequencing chain that should fit in slot 2
        // this should mark slot 1 as unsafe
        seq_tx.send(create_test_block(3, 11_001)).await.unwrap();

        let slot1 = slot_rx.recv().await.unwrap();
        assert_eq!(slot1.timestamp, slot_start_timestamp_ms + slot_duration_ms);
        assert_eq!(slot1.slot_number, 1);
        assert_eq!(slot1.sequencing_chain_blocks.len(), 2);
        assert_eq!(slot1.settlement_chain_blocks.len(), 1);
        assert_eq!(slot1.state, SlotState::Unsafe);

        // slot 2 should still be opened
        assert!(slot_rx.is_empty());
    }
}
