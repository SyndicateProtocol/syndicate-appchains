//! Slotting module for metabased-translator

use crate::config::SlottingConfig;
use common::{
    db::{DbError, SafeState, TranslatorStore},
    types::{Block, BlockAndReceipts, BlockRef, Chain, Slot, SlotState},
};
use derivative::Derivative;
use std::{cmp::Ordering, collections::LinkedList};
use thiserror::Error;
use tokio::{
    select,
    sync::{
        mpsc::{channel, error::SendError, Receiver, Sender},
        oneshot,
    },
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
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Slotter {
    config: SlottingConfig,

    sequencing_chain_rx: Receiver<BlockAndReceipts>,
    settlement_chain_rx: Receiver<BlockAndReceipts>,

    latest_sequencing_block: Option<BlockRef>,
    latest_settlement_block: Option<BlockRef>,

    /// Stores the last N slots (both open and closed)
    slots: LinkedList<Slot>,

    /// Maximum number of slots to keep
    max_slots: usize,

    /// Sender for sending slots to the consumer
    sender: Sender<Slot>,

    #[derivative(Debug = "ignore")]
    store: Box<dyn TranslatorStore + Send + Sync>,
}

const fn calculate_max_slots(slot_duration_ms: u64) -> usize {
    (MAX_WAIT_MS / slot_duration_ms) as usize
}

impl Slotter {
    /// Creates a new [`Slotter`] that receives blocks from two chains and organizes them into
    /// slots.
    ///
    /// # Arguments
    /// * `sequencing_chain_receiver` - Channel receiving blocks from the sequencing chain
    /// * `settlement_chain_receiver` - Channel receiving blocks from the settlement chain
    /// * `config` - Configuration for slot timing and initial state
    /// * `store` - Storage backend for saving slots
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
    pub fn new(
        sequencing_chain_receiver: Receiver<BlockAndReceipts>,
        settlement_chain_receiver: Receiver<BlockAndReceipts>,
        config: SlottingConfig,
        safe_state: Option<SafeState>,
        store: Box<dyn TranslatorStore + Send + Sync>,
    ) -> (Self, Receiver<Slot>) {
        let (slot_tx, slot_rx) = channel(100);
        let mut slots = LinkedList::new();
        let (latest_sequencing_block, latest_settlement_block) = match safe_state {
            Some(safe_state) => {
                slots.push_front(safe_state.slot);
                (Some(safe_state.sequencing_block), Some(safe_state.settlement_block))
            }
            None => {
                slots.push_front(Slot::new(1, config.start_slot_timestamp));
                (None, None)
            }
        };

        let slotter = Self {
            sequencing_chain_rx: sequencing_chain_receiver,
            settlement_chain_rx: settlement_chain_receiver,
            latest_sequencing_block,
            latest_settlement_block,
            slots,
            max_slots: calculate_max_slots(config.slot_duration_ms),
            config,
            sender: slot_tx,
            store,
        };
        (slotter, slot_rx)
    }

    /// Starts the [`Slotter`] loop.
    ///
    /// The [`Slotter`] will:
    /// 1. Receive blocks from both sequencing and settlement chains
    /// 2. Place blocks into slots based on their timestamps
    /// 3. Mark slots as unsafe when both chains have progressed past them (or max wait time has
    ///    passed)
    /// 4. Send completed slots through the returned channel
    ///
    /// # Returns
    /// - A tuple containing:
    ///   - A receiver that will get slots as they are processed
    ///   - A shutdown function that can be called to stop the slotter
    pub async fn start(mut self, mut shutdown_rx: oneshot::Receiver<()>) {
        info!("Starting Slotter");
        loop {
            let process_result = select! {
                _ = &mut shutdown_rx => {
                    info!("Slotter stopped");
                    drop(self.sender);
                    break;
                }
                Some(block) = self.sequencing_chain_rx.recv() => {
                    self.process_block(
                        block,
                        Chain::Sequencing,
                        self.config.slot_duration_ms,
                    )
                    .await
                }
                Some(block) = self.settlement_chain_rx.recv() => {
                    self.process_block(
                        block,
                        Chain::Settlement,
                        self.config.slot_duration_ms,
                    )
                    .await
                }
            };

            match process_result {
                Ok(_) => (),
                Err(e) => match e {
                    SlotterError::ReorgDetected { .. } => {
                        panic!("Reorgs not yet implemented {e}"); // TODO SEQ-429 - implement reorg
                                                                  // handing
                    }
                    SlotterError::BlockNumberSkipped { .. } => {
                        panic!("Block number skipped {e}"); // TODO SEQ-489 - decide what to do if a
                                                            // block is skipped
                    }
                    SlotterError::BlockTooOld { .. } => {
                        panic!("Block too old {e}"); // TODO SEQ-489 - decide what to do if a block
                                                     // is too old
                    }
                    SlotterError::NoSlotsAvailable => {
                        panic!("No slots available. This should never happen - if it does, it's an implementation error. {e}");
                    }
                    SlotterError::SlotSendError(_) => {
                        panic!("Failed to send slot through channel: {e}"); // TODO SEQ-489 - decide
                                                                            // what to do here
                                                                            // (likely to occur
                                                                            // during shutdown, or
                                                                            // the received is
                                                                            // blocked)
                    }
                    SlotterError::EarlierTimestamp { .. } => {
                        panic!("Earlier timestamp - this should never happen (where a block is received with the expected block number, but a lower timestamp) {e}");
                    }
                    SlotterError::DbError(_) => {
                        panic!("Database error: {e}"); // TODO SEQ-489 - decide what to do here
                    }
                },
            }
        }
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
    ) -> Result<(), SlotterError> {
        // Get the other chain's latest block timestamp
        let other_chain_timestamp = match chain {
            Chain::Sequencing => self.latest_settlement_block.as_ref(),
            Chain::Settlement => self.latest_sequencing_block.as_ref(),
        }
        .map(|b| b.timestamp);

        // Only mark slots as unsafe if we have blocks from both chains
        if let Some(other_timestamp) = other_chain_timestamp {
            let min_timestamp = other_timestamp.min(block_timestamp);
            debug!(min_timestamp, "Marking slots as unsafe");

            // Mark slots as unsafe if both chains have progressed past them
            for slot in &mut self.slots {
                match slot.state {
                    SlotState::Unsafe | SlotState::Safe => {
                        debug!(%slot, "Found non-open slot, stopping iteration");
                        return Ok(()); // assume all blocks past this point are already marked as
                                       // unsafe
                    }
                    SlotState::Open => {
                        if slot.timestamp < min_timestamp {
                            debug!(%slot, "Marking slot as unsafe");
                            slot.state = SlotState::Unsafe;
                            self.sender.send(slot.clone()).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_block(
        &mut self,
        block_info: BlockAndReceipts,
        chain: Chain,
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
                let mut latest_slot_number = latest_slot.number;

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

        self.mark_unsafe_slots(block_timestamp, chain).await?;
        self.cleanup_slots().await?;
        Ok(())
    }

    /// `cleanup_slots` will remove slots that are older than the `MAX_WAIT_MS` and mark them as
    /// safe. (any slots not marked as unsafe until this point will be sent through the channel)
    async fn cleanup_slots(&mut self) -> Result<(), SlotterError> {
        debug!("Cleaning up slots");
        while self.slots.len() > self.max_slots {
            if let Some(mut slot) = self.slots.pop_back() {
                debug!(%slot, "Cleaning up old slot, marking as safe and sending if open");
                let prev_state = slot.state;
                slot.state = SlotState::Safe;

                // Save to DB first
                self.store.save_slot(&slot).await?;

                // Send slot if it was open
                if prev_state == SlotState::Open {
                    self.sender.send(slot.clone()).await?
                }
            }
        }
        Ok(())
    }

    fn update_latest_block(&mut self, block: &Block, chain: Chain) -> Result<(), SlotterError> {
        if let Some(latest) = match chain {
            Chain::Sequencing => self.latest_sequencing_block.as_ref(),
            Chain::Settlement => self.latest_settlement_block.as_ref(),
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

            if block.timestamp < latest.timestamp {
                return Err(SlotterError::EarlierTimestamp {
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
            "Updated latest block seen"
        );

        match chain {
            Chain::Sequencing => self.latest_sequencing_block = Some(BlockRef::new(block)),
            Chain::Settlement => self.latest_settlement_block = Some(BlockRef::new(block)),
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

    #[error("{chain} chain timestamp must not decrease. Current: {current}, Received: {received}")]
    EarlierTimestamp { chain: Chain, current: u64, received: u64 },

    #[error("Database error: {0}")]
    DbError(#[from] DbError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;
    use common::db::RocksDbStore;
    use std::str::FromStr;
    use test_utils::test_path;

    struct TestSetup {
        slot_rx: Receiver<Slot>,
        sequencing_tx: Sender<BlockAndReceipts>,
        settlement_tx: Sender<BlockAndReceipts>,
        shutdown_tx: oneshot::Sender<()>,
    }

    async fn create_slotter_and_spawn(
        slot_start_timestamp_ms: u64,
        slot_duration_ms: u64,
    ) -> TestSetup {
        let (slotter, slot_rx, seq_tx, settle_tx) =
            create_slotter(slot_start_timestamp_ms, slot_duration_ms);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::spawn(async move { slotter.start(shutdown_rx).await });

        TestSetup { slot_rx, sequencing_tx: seq_tx, settlement_tx: settle_tx, shutdown_tx }
    }

    fn create_slotter(
        slot_start_timestamp_ms: u64,
        slot_duration_ms: u64,
    ) -> (Slotter, Receiver<Slot>, Sender<BlockAndReceipts>, Sender<BlockAndReceipts>) {
        let (seq_tx, seq_rx) = channel(100);
        let (settle_tx, settle_rx) = channel(100);
        let store = Box::new(RocksDbStore::new(test_path("slotting_db").as_str()).unwrap());

        let (slotter, slot_rx) = Slotter::new(
            seq_rx,
            settle_rx,
            SlottingConfig { slot_duration_ms, start_slot_timestamp: slot_start_timestamp_ms },
            None,
            store,
        );

        (slotter, slot_rx, seq_tx, settle_tx)
    }

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
    #[tracing_test::traced_test]
    async fn test_slotter() {
        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        // NOTE: IMPORTANT - keep _shutdown in scope, otherwise `slotter` will be terminated
        // immediatelly
        let TestSetup { mut slot_rx, sequencing_tx, settlement_tx, shutdown_tx: _shutdown_tx } =
            create_slotter_and_spawn(slot_start_timestamp_ms, slot_duration_ms).await;
        assert!(slot_rx.is_empty());

        // send initial blocks, these should fit in slot 1 and make slot 0 be marked as unsafe
        sequencing_tx.send(create_test_block(1, 10_001)).await.unwrap();

        settlement_tx.send(create_test_block(1, 10_002)).await.unwrap();

        let slot1 = slot_rx.recv().await.unwrap();
        assert_eq!(slot1.timestamp, slot_start_timestamp_ms);
        assert_eq!(slot1.number, 1);
        assert_eq!(slot1.sequencing_chain_blocks.len(), 0);
        assert_eq!(slot1.settlement_chain_blocks.len(), 0);
        assert_eq!(slot1.state, SlotState::Unsafe);

        assert!(slot_rx.is_empty());

        // send a block for the settlement chain that should fit in slot 2
        settlement_tx.send(create_test_block(2, 11_001)).await.unwrap(); // this block should be fit in slot 1

        // slot 1 should still be opened (we haven't received any blocks for the sequencing chain
        // ahead of the slot)
        assert!(slot_rx.is_empty());

        // send a bock for the sequencing chain that still fits in slot 1
        sequencing_tx.send(create_test_block(2, 11_000)).await.unwrap();

        // slot 1 should still be opened (we haven't received any blocks for the sequencing chain
        // ahead of the slot)
        assert!(slot_rx.is_empty());

        // send a block for the sequencing chain that should fit in slot 2
        // this should mark slot 1 as unsafe
        sequencing_tx.send(create_test_block(3, 11_001)).await.unwrap();

        let slot2 = slot_rx.recv().await.unwrap();
        assert_eq!(slot2.timestamp, slot_start_timestamp_ms + slot_duration_ms);
        assert_eq!(slot2.number, 2);
        assert_eq!(slot2.sequencing_chain_blocks.len(), 2);
        assert_eq!(slot2.settlement_chain_blocks.len(), 1);
        assert_eq!(slot2.state, SlotState::Unsafe);

        // slot 2 should still be opened
        assert!(slot_rx.is_empty());
    }

    #[tokio::test]
    async fn test_update_latest_block_success_sequencing() {
        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        let (mut slotter, _, _, _) = create_slotter(slot_start_timestamp_ms, slot_duration_ms);

        let block = create_test_block(2, 200);

        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));

        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_sequencing_block.unwrap().number, 2);
    }

    #[tokio::test]
    async fn test_update_latest_block_success_settlement() {
        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        let (mut slotter, _, _, _) = create_slotter(slot_start_timestamp_ms, slot_duration_ms);

        let block = create_test_block(3, 300);

        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));

        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_settlement_block.unwrap().number, 3);
    }

    #[tokio::test]
    async fn test_reorg_detected() {
        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        let (mut slotter, _, _, _) = create_slotter(slot_start_timestamp_ms, slot_duration_ms);

        let block = create_test_block(1, 50);

        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(2, 10_001).block));

        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert!(matches!(result, Err(SlotterError::ReorgDetected { .. })));
    }

    #[tokio::test]
    async fn test_block_number_skipped() {
        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        let (mut slotter, _, _, _) = create_slotter(slot_start_timestamp_ms, slot_duration_ms);

        let block = create_test_block(4, 400);

        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));

        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert!(matches!(result, Err(SlotterError::BlockNumberSkipped { .. })));
    }

    #[tokio::test]
    async fn test_earlier_timestamp() {
        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        let (mut slotter, _, _, _) = create_slotter(slot_start_timestamp_ms, slot_duration_ms);

        let block = create_test_block(2, 50);

        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));

        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert!(matches!(result, Err(SlotterError::EarlierTimestamp { .. })));
    }

    #[tokio::test]
    async fn test_slotter_db_shutdown_and_resume() {
        const CHAN_CAPACITY: usize = 100;
        let (seq_tx, seq_rx) = channel(CHAN_CAPACITY);
        let (set_tx, set_rx) = channel(CHAN_CAPACITY);

        let slot_start_timestamp_ms = 10_000;
        let slot_duration_ms = 1_000;
        let config =
            SlottingConfig { slot_duration_ms, start_slot_timestamp: slot_start_timestamp_ms };

        // Create a fresh DB for this test
        let db_path = test_path("slotting_db");
        let store = Box::new(RocksDbStore::new(db_path.as_str()).unwrap());

        // Start first slotter instance
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (slotter, mut slot_rx) = Slotter::new(seq_rx, set_rx, config.clone(), None, store);
        let handle = tokio::spawn(async move { slotter.start(shutdown_rx).await });

        // Send some blocks
        // slot 2
        seq_tx.send(create_test_block(2, 10_001)).await.unwrap();
        set_tx.send(create_test_block(2, 10_002)).await.unwrap();

        // slot 3
        seq_tx.send(create_test_block(3, 11_001)).await.unwrap();
        set_tx.send(create_test_block(3, 11_002)).await.unwrap();

        // This should make slot 1 and 2 unsafe
        let slot1 = slot_rx.recv().await.unwrap();
        assert_eq!(slot1.number, 1);
        assert_eq!(slot1.sequencing_chain_blocks.len(), 0);
        assert_eq!(slot1.settlement_chain_blocks.len(), 0);
        assert_eq!(slot1.state, SlotState::Unsafe);

        let slot2 = slot_rx.recv().await.unwrap();
        assert_eq!(slot2.number, 2);
        assert_eq!(slot2.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot2.settlement_chain_blocks.len(), 1);
        assert_eq!(slot2.sequencing_chain_blocks[0].block.number, 2);
        assert_eq!(slot2.settlement_chain_blocks[0].block.number, 2);
        assert_eq!(slot2.state, SlotState::Unsafe);

        // Send blocks that are MAX_WAIT_MS (24 hours) ahead, this should make slots 1, 2 and 3 safe
        set_tx.send(create_test_block(4, 11_001 + MAX_WAIT_MS)).await.unwrap();
        // NOTE: don't send a block for the sequencing chain, that would mark all the empty slots as
        // unsafe and atempt to send them over the channel (which would get filled up and stuck)

        let slot3 = slot_rx.recv().await.unwrap();
        assert_eq!(slot3.number, 3);
        assert_eq!(slot3.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot3.settlement_chain_blocks.len(), 1);
        assert_eq!(slot3.state, SlotState::Safe); // slot 3 should be received as safe because slotter thinks MAX_WAIT_MS has passed

        // shutdown slotter
        let _ = shutdown_tx.send(());
        handle.await.unwrap();

        // Create new channels for the resumed slotter
        let (new_seq_tx, new_seq_rx) = channel(CHAN_CAPACITY);
        let (new_set_tx, new_settle_rx) = channel(CHAN_CAPACITY);

        // Create new store instance pointing to same DB, and get the latest state from the DB
        let resumed_store = Box::new(RocksDbStore::new(db_path.as_str()).unwrap());
        let resumed_state = resumed_store.get_latest().await.unwrap();

        // Create new slotter that should resume from DB
        let (resumed_slotter, mut resumed_slot_rx) =
            Slotter::new(new_seq_rx, new_settle_rx, config, resumed_state, resumed_store);

        let (_shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::spawn(async move { resumed_slotter.start(shutdown_rx).await });

        // Send new blocks to resumed slotter (since only slot 0,1,2 have been saved to the DB, we
        // should send blocks #4 (for slot 5))
        new_seq_tx.send(create_test_block(4, 12_001)).await.unwrap();
        new_set_tx.send(create_test_block(4, 12_002)).await.unwrap();

        // sending blocks for slot 5 should mark slot 4 as unsafe
        new_seq_tx.send(create_test_block(5, 13_001)).await.unwrap();
        new_set_tx.send(create_test_block(5, 13_002)).await.unwrap();

        // Should get slot 3 marked as unsafe
        let slot4 = resumed_slot_rx.recv().await.unwrap();
        assert_eq!(slot4.number, 4);
        assert_eq!(slot4.state, SlotState::Unsafe);
        assert_eq!(slot4.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot4.settlement_chain_blocks.len(), 1);
        assert_eq!(slot4.sequencing_chain_blocks[0].block.number, 4);
        assert_eq!(slot4.settlement_chain_blocks[0].block.number, 4);
    }

    #[test]
    fn test_calculate_max_slots() {
        // MAX_WAIT_MS is 24 hours = 86_400_000 ms

        // Test with 1 second slots
        assert_eq!(calculate_max_slots(1_000), 86_400);

        // Test with 1 minute slots
        assert_eq!(calculate_max_slots(60_000), 1_440);

        // Test with 1 hour slots
        assert_eq!(calculate_max_slots(3_600_000), 24);

        // Test with 12 hour slots
        assert_eq!(calculate_max_slots(43_200_000), 2);

        // Test with slot duration equal to MAX_WAIT_MS
        assert_eq!(calculate_max_slots(MAX_WAIT_MS), 1);

        // Test with slot duration larger than MAX_WAIT_MS
        assert_eq!(calculate_max_slots(MAX_WAIT_MS + 1), 0);
    }
}
