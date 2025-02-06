//! Slotting module for metabased-translator

use crate::{config::SlotterConfig, metrics::SlotterMetrics};
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

/// Maximum time to wait for blocks before considering a slot final (24 hours in seconds)
/// A slot becomes safe if both chains have progressed `MAX_WAIT_SEC` seconds past the slot's
/// timestamp
const MAX_WAIT_SEC: u64 = 24 * 60 * 60;

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
/// - max_slots is the number of slots to keep and is determined by `MAX_WAIT_SEC` / `slot_duration_ms`, thus if a slot is to be dropped, it must be marked as Safe
/// - Slots older than max_slots are dropped
/// - A slot becomes Unsafe when both chains progress past it
/// ```
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Slotter {
    slot_duration: u64,

    latest_sequencing_block: Option<BlockRef>,
    latest_settlement_block: Option<BlockRef>,

    /// Stores all open and unsafe slots
    slots: LinkedList<Slot>,

    /// Sender for sending slots to the consumer
    sender: Sender<Slot>,

    #[derivative(Debug = "ignore")]
    store: Box<dyn TranslatorStore + Send + Sync>,

    /// Metrics
    metrics: SlotterMetrics,
}

const START_SLOT: u64 = 0;

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
    /// The [`Slotter`] maintains a window of slots spanning the last 24 hours ([`MAX_WAIT`]),
    /// with the number of slots determined by [`MAX_WAIT`] / `slot_duration`.
    ///
    /// Each slot contains blocks from both chains which timestamps fall within the slot's window:
    /// (`slot_timestamp` - `slot_duration`, `slot_timestamp`]
    ///
    /// # Returns
    /// A new [`Slotter`] instance that can be started with `start()`
    pub fn new(
        config: &SlotterConfig,
        safe_state: Option<SafeState>,
        store: Box<dyn TranslatorStore + Send + Sync>,
        metrics: SlotterMetrics,
    ) -> (Self, Receiver<Slot>) {
        let (slot_tx, slot_rx) = channel(100);
        let mut slots = LinkedList::new();
        let (latest_sequencing_block, latest_settlement_block) = match safe_state {
            Some(safe_state) => {
                slots.push_front(safe_state.slot);
                (Some(safe_state.sequencing_block), Some(safe_state.settlement_block))
            }
            None => {
                slots.push_front(Slot::new(START_SLOT, config.start_slot_timestamp));
                (None, None)
            }
        };

        let slotter = Self {
            latest_sequencing_block,
            latest_settlement_block,
            slots,
            slot_duration: config.slot_duration,
            sender: slot_tx,
            store,
            metrics,
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
    pub async fn start(
        mut self,
        mut sequencing_rx: Receiver<BlockAndReceipts>,
        mut settlement_rx: Receiver<BlockAndReceipts>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) {
        info!("Starting Slotter");

        loop {
            let seq_ts = self.latest_sequencing_block.as_ref().map_or(0, |b| b.timestamp);
            let set_ts = self.latest_settlement_block.as_ref().map_or(0, |b| b.timestamp);

            // prefer to consume from the chain that is lagging behind
            let (first_rx, first_chain, second_rx, second_chain) = if seq_ts <= set_ts {
                (&mut sequencing_rx, Chain::Sequencing, &mut settlement_rx, Chain::Settlement)
            } else {
                (&mut settlement_rx, Chain::Settlement, &mut sequencing_rx, Chain::Sequencing)
            };

            let process_result = select! {
                biased;
                Some(block) = first_rx.recv() => {
                    self.process_block(block, first_chain, self.slot_duration).await
                }
                Some(block) = second_rx.recv() => {
                    self.process_block(block, second_chain, self.slot_duration).await
                }
                _ = &mut shutdown_rx => {
                    info!("Slotter stopped: {}", self);
                    drop(self.sender);
                    break;
                }
            };

            // TODO SEQ-570 - fix (channel metrics should be tracked on the sender side)
            // self.metrics.update_channel_capacity(sequencing_rx.capacity(), Chain::Sequencing);
            // self.metrics.update_channel_capacity(settlement_rx.capacity(), Chain::Settlement);

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
                    SlotterError::NoSlotsAvailable(_) => {
                        panic!("No slots available. This should never happen - if it does, it's an implementation error. {e}");
                    }
                    SlotterError::SlotSendError(_) => {
                        // unrecoverable. on shutdown slotter should stop before block-builder
                        panic!("Failed to send slot through channel: {e}");
                    }
                    SlotterError::EarlierTimestamp { .. } => {
                        panic!("Earlier timestamp - this should never happen (where a block is received with the expected block number, but a lower timestamp) {e}");
                    }
                    SlotterError::DbError(_) => {
                        panic!("Database error: {e}"); // TODO SEQ-489 - decide what to do here
                    }
                    SlotterError::SlotNumberOverflow => {
                        panic!("Slot number overflow - this should never happen unless timestamps are extremely far apart, or a block was received for a slot in the past (before START_SLOT)");
                    }
                    SlotterError::TimestampOverflow => {
                        panic!("Timestamp overflow - this should never happen unless timestamps are extremely far apart");
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

            // Mark slots as unsafe if both chains have progressed past them.
            // buffer is used to reverse the list of unsafe slots
            // so that they are sent from oldest to newest over the channel.
            let mut buffer = vec![];
            for slot in &mut self.slots {
                match slot.state {
                    SlotState::Unsafe | SlotState::Safe => {
                        debug!(%slot, "Found non-open slot, stopping iteration");
                        break; // assume all blocks past this point are already marked as unsafe
                    }
                    SlotState::Open => {
                        if slot.timestamp < min_timestamp {
                            debug!(%slot, "Marking slot as unsafe");
                            slot.state = SlotState::Unsafe;
                            buffer.push(slot.clone());
                        }
                    }
                }
            }
            for slot in buffer.into_iter().rev() {
                debug!(%slot, "Sending slot");
                self.sender.send(slot).await?;
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
        let block_number = block_info.block.number;
        self.update_latest_block(&block_info.block, chain)?;
        let latest_slot = self
            .slots
            .front_mut()
            .ok_or(SlotterError::NoSlotsAvailable("slot collection is empty".to_string()))?;
        self.metrics.record_blocks_per_slot(latest_slot.get_total_blocks() as u64);
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
                debug!("Block belongs to an earlier slot");
                let mut inserted = false;

                let (target_slot_number, target_timestamp) = Self::calculate_slot_position(
                    latest_slot.number,
                    latest_slot.timestamp,
                    block_timestamp,
                    slot_duration_ms,
                )?;

                // Find the right position to insert the new slot
                for (idx, slot) in self.slots.iter_mut().enumerate() {
                    match block_slot_ordering(block_timestamp, slot.timestamp, slot_duration_ms) {
                        Ordering::Equal => {
                            debug!(%slot, "Found matching slot, adding block");
                            slot.push_block(block_info, chain);
                            inserted = true;
                            break;
                        }
                        Ordering::Less => {
                            // Keep looking for the right slot
                        }
                        Ordering::Greater => {
                            // We've gone too far, insert new slot before this one
                            let mut new_slot = Slot::new(target_slot_number, target_timestamp);
                            debug!(%new_slot, "Creating new slot between existing slots");
                            new_slot.push_block(block_info, chain);

                            // Split list at idx, insert new slot, then reattach tail
                            let mut tail = self.slots.split_off(idx);
                            self.slots.push_back(new_slot);
                            self.slots.append(&mut tail);

                            inserted = true;
                            break;
                        }
                    }
                }

                if !inserted {
                    // TODO remove this error - this shouldn't happen anymore... just create the
                    // slot at the end of the list...
                    return Err(SlotterError::BlockTooOld { chain, block_number, block_timestamp });
                }
            }
            Ordering::Equal => {
                debug!("Block fits in current slot");
                latest_slot.push_block(block_info, chain)
            }
            Ordering::Greater => {
                debug!("Creating new slot for block");

                let (target_slot_number, target_timestamp) = Self::calculate_slot_position(
                    latest_slot.number,
                    latest_slot.timestamp,
                    block_timestamp,
                    slot_duration_ms,
                )?;

                let mut slot = Slot::new(target_slot_number, target_timestamp);
                self.metrics.record_last_slot(slot.number);
                slot.push_block(block_info, chain);
                debug!(%slot, "Created new slot");
                self.slots.push_front(slot);
            }
        }
        self.metrics.update_active_slots(self.slots.len());
        self.mark_unsafe_slots(block_timestamp, chain).await?;
        self.cleanup_slots().await?;
        Ok(())
    }

    /// `cleanup_slots` will mark slots as safe if both chains have progressed `MAX_WAIT` seconds
    /// past them and we have seen blocks from both chains after that point.
    async fn cleanup_slots(&mut self) -> Result<(), SlotterError> {
        debug!("Checking for slots to mark as safe");

        // Get latest blocks from both chains
        let (seq_block, set_block) =
            match (&self.latest_sequencing_block, &self.latest_settlement_block) {
                (Some(seq), Some(set)) => (seq, set),
                _ => {
                    debug!("No blocks seen for both chains yet, skipping cleanup");
                    return Ok(());
                }
            };

        // Get the minimum timestamp between the latest blocks from both chains
        let min_chain_head_timestamp = seq_block.timestamp.min(set_block.timestamp);

        // Check slots from oldest to newest (back to front)
        while let Some(slot) = self.slots.back() {
            // Has 24h passed since this slot's timestamp?
            // If we can't mark this slot as safe, we can't mark newer ones either
            if slot.timestamp + MAX_WAIT_SEC > min_chain_head_timestamp ||
                slot.state == SlotState::Open
            {
                break;
            }

            // Remove the slot from the list
            let mut slot = self.slots.pop_back().ok_or_else(|| {
                SlotterError::NoSlotsAvailable(
                    "Slot disappeared between back() and pop_back()".to_string(),
                )
            })?;

            debug!(%slot, "Saving safe slot to DB");
            slot.state = SlotState::Safe;
            self.store.save_slot(&slot).await?;
        }

        self.metrics.update_active_slots(self.slots.len());
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
        self.metrics.record_last_processed_block(block.number, chain);
        self.metrics.update_chain_timestamp_lag(block.timestamp, chain);
        Ok(())
    }

    /// Calculate slot number and timestamp for a given block timestamp
    ///
    /// # Arguments
    /// * `latest_slot_number` - Number of the latest slot
    /// * `latest_timestamp` - Timestamp of the latest slot
    /// * `block_timestamp` - Timestamp of the block
    /// * `slot_duration` - Duration of each slot in seconds
    ///
    /// # Returns
    /// * `(slot_number, slot_timestamp)` - Calculated slot number and timestamp
    fn calculate_slot_position(
        latest_slot_number: u64,
        latest_timestamp: u64,
        block_timestamp: u64,
        slot_duration: u64,
    ) -> Result<(u64, u64), SlotterError> {
        let slots_diff: i64 = if block_timestamp > latest_timestamp {
            ((block_timestamp - latest_timestamp).div_ceil(slot_duration)) as i64
        } else {
            -(((latest_timestamp - block_timestamp) / slot_duration) as i64)
        };

        let target_slot_number = latest_slot_number
            .checked_add_signed(slots_diff)
            .ok_or(SlotterError::SlotNumberOverflow)?;
        let target_timestamp = latest_timestamp
            .checked_add_signed(slots_diff * (slot_duration as i64))
            .ok_or(SlotterError::TimestampOverflow)?;

        Ok((target_slot_number, target_timestamp))
    }
}

impl std::fmt::Display for Slotter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Slotter[slots_len:{}, slot_duration:{}ms, latest_seq:{}, latest_set:{}]",
            self.slots.len(),
            self.slot_duration,
            self.latest_sequencing_block.as_ref().map_or("none".to_string(), |b| b.to_string()),
            self.latest_settlement_block.as_ref().map_or("none".to_string(), |b| b.to_string())
        )
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
    #[error("No slots available {0}")]
    NoSlotsAvailable(String),

    #[error("Failed to send slot through channel: {0}")]
    SlotSendError(#[from] SendError<Slot>),

    #[error("Block timestamp {block_timestamp}, number {block_number} for {chain} chain is in the past ")]
    BlockTooOld { chain: Chain, block_number: u64, block_timestamp: u64 },

    #[error("{chain} chain reorg detected. Current: #{current_block_number}, Received: #{received_block_number}")]
    ReorgDetected { chain: Chain, current_block_number: u64, received_block_number: u64 },

    #[error("{chain} chain block number skipped. Current: #{current_block_number}, Received: #{received_block_number}")]
    BlockNumberSkipped { chain: Chain, current_block_number: u64, received_block_number: u64 },

    #[error("{chain} chain timestamp must not decrease. Current: {current}, Received: {received}")]
    EarlierTimestamp { chain: Chain, current: u64, received: u64 },

    #[error("Database error: {0}")]
    DbError(#[from] DbError),

    #[error("Slot number overflow")]
    SlotNumberOverflow,

    #[error("Timestamp overflow")]
    TimestampOverflow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;
    use assert_matches::assert_matches;
    use prometheus_client::registry::Registry;
    use std::str::FromStr;
    use tracing_test::traced_test;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }
    use common::db::RocksDbStore;
    use test_utils::test_path;

    struct TestSetup {
        slot_rx: Receiver<Slot>,
        sequencing_tx: Sender<BlockAndReceipts>,
        settlement_tx: Sender<BlockAndReceipts>,
        shutdown_tx: oneshot::Sender<()>,
    }

    async fn create_slotter_and_spawn(slot_start_timestamp: u64, slot_duration: u64) -> TestSetup {
        let (slotter, slot_rx) = create_slotter(slot_start_timestamp, slot_duration);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (seq_tx, seq_rx) = channel(100);
        let (settle_tx, settle_rx) = channel(100);
        tokio::spawn(async move { slotter.start(seq_rx, settle_rx, shutdown_rx).await });

        TestSetup { slot_rx, sequencing_tx: seq_tx, settlement_tx: settle_tx, shutdown_tx }
    }

    fn create_slotter(
        slot_start_timestamp_ms: u64,
        slot_duration: u64,
    ) -> (Slotter, Receiver<Slot>) {
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);
        let store = Box::new(RocksDbStore::new(test_path("slotter_db").as_str()).unwrap());

        let (slotter, slot_rx) = Slotter::new(
            &SlotterConfig { slot_duration, start_slot_timestamp: slot_start_timestamp_ms },
            None,
            store,
            metrics,
        );

        (slotter, slot_rx)
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
    /// Slot 0 [9]:
    /// ┌───────────────────┐
    /// │ empty             │
    /// └───────────────────┘
    ///
    /// Slot 1 [10]:
    /// ┌───────────────────┐
    /// │ seq    @ 11 #1    │
    /// │ seq    @ 11 #2    │
    /// │ settle @ 11 #1    │ -> Only marked as Unsafe once the blocks for next slot are received
    /// └───────────────────┘
    ///
    /// Slot 2 [11]:
    /// ┌───────────────────┐
    /// │ seq    @ 12 #3    │
    /// │ settle @ 12 #2    │ -> Shouldn't be received (never marked as unsafe)
    /// └───────────────────┘
    ///
    /// Legend:
    /// @ timestamp
    /// # block number
    /// ```
    #[tokio::test]
    #[traced_test]
    async fn test_slotter() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        // NOTE: IMPORTANT - keep _shutdown in scope, otherwise `slotter` will be terminated
        // immediatelly
        let TestSetup { mut slot_rx, sequencing_tx, settlement_tx, shutdown_tx: _shutdown_tx } =
            create_slotter_and_spawn(slot_start_timestamp, slot_duration).await;
        assert!(slot_rx.is_empty());

        // send initial blocks, these should fit in slot 1 and make slot 0 be marked as unsafe
        sequencing_tx.send(create_test_block(1, 11)).await.unwrap();

        settlement_tx.send(create_test_block(1, 11)).await.unwrap();

        let slot1 = slot_rx.recv().await.unwrap();
        assert_eq!(slot1.timestamp, slot_start_timestamp);
        assert_eq!(slot1.number, START_SLOT);
        assert_eq!(slot1.sequencing_chain_blocks.len(), 0);
        assert_eq!(slot1.settlement_chain_blocks.len(), 0);
        assert_eq!(slot1.state, SlotState::Unsafe);

        assert!(slot_rx.is_empty());

        // send a block for the settlement chain that should fit in slot 2
        settlement_tx.send(create_test_block(2, 12)).await.unwrap(); // this block should be fit in slot 1

        // slot 1 should still be opened (we haven't received any blocks for the sequencing chain
        // ahead of the slot)
        assert!(slot_rx.is_empty());

        // send a bock for the sequencing chain that still fits in slot 1
        sequencing_tx.send(create_test_block(2, 11)).await.unwrap();

        // slot 1 should still be opened (we haven't received any blocks for the sequencing chain
        // ahead of the slot)
        assert!(slot_rx.is_empty());

        // send a block for the sequencing chain that should fit in slot 2
        // this should mark slot 1 as unsafe
        sequencing_tx.send(create_test_block(3, 12)).await.unwrap();

        let slot2 = slot_rx.recv().await.unwrap();
        assert_eq!(slot2.timestamp, slot_start_timestamp + slot_duration);
        assert_eq!(slot2.number, START_SLOT + 1);
        assert_eq!(slot2.sequencing_chain_blocks.len(), 2);
        assert_eq!(slot2.settlement_chain_blocks.len(), 1);
        assert_eq!(slot2.state, SlotState::Unsafe);

        // slot 2 should still be opened
        assert!(slot_rx.is_empty());
    }

    #[tokio::test]
    async fn test_update_latest_block_success_sequencing() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let (mut slotter, _) = create_slotter(slot_start_timestamp, slot_duration);

        let block = create_test_block(2, 200);

        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));

        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_sequencing_block.unwrap().number, 2);
    }

    #[tokio::test]
    async fn test_update_latest_block_success_settlement() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let (mut slotter, _) = create_slotter(slot_start_timestamp, slot_duration);

        let block = create_test_block(3, 300);

        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));

        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_settlement_block.unwrap().number, 3);
    }

    #[tokio::test]
    async fn test_reorg_detected() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let (mut slotter, _) = create_slotter(slot_start_timestamp, slot_duration);

        let block = create_test_block(1, 50);

        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(2, 10_001).block));

        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert_matches!(result, Err(SlotterError::ReorgDetected { .. }));
    }

    #[tokio::test]
    async fn test_block_number_skipped() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let (mut slotter, _) = create_slotter(slot_start_timestamp, slot_duration);

        let block = create_test_block(4, 400);

        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));

        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert_matches!(result, Err(SlotterError::BlockNumberSkipped { .. }));
    }

    #[tokio::test]
    async fn test_earlier_timestamp() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let (mut slotter, _) = create_slotter(slot_start_timestamp, slot_duration);

        let block = create_test_block(2, 50);

        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));

        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert_matches!(result, Err(SlotterError::EarlierTimestamp { .. }));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_slotter_db_shutdown_and_resume() {
        const CHAN_CAPACITY: usize = 100;
        let (seq_tx, seq_rx) = channel(CHAN_CAPACITY);
        let (set_tx, set_rx) = channel(CHAN_CAPACITY);

        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let config = SlotterConfig { slot_duration, start_slot_timestamp: slot_start_timestamp };

        // Create a fresh DB for this test
        let db_path = test_path("slotter_db");
        let store = Box::new(RocksDbStore::new(db_path.as_str()).unwrap());

        // Start first slotter instance
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);
        let (slotter, mut slot_rx) = Slotter::new(&config, None, store, metrics);
        let handle = tokio::spawn(async move { slotter.start(seq_rx, set_rx, shutdown_rx).await });

        // Send some blocks
        // slot 2
        seq_tx.send(create_test_block(2, 11)).await.unwrap();
        set_tx.send(create_test_block(2, 11)).await.unwrap();

        // slot 3
        seq_tx.send(create_test_block(3, 12)).await.unwrap();
        set_tx.send(create_test_block(3, 12)).await.unwrap();

        // This should make slot 1 and 2 unsafe
        let slot1 = slot_rx.recv().await.unwrap();
        assert_eq!(slot1.number, START_SLOT);
        assert_eq!(slot1.sequencing_chain_blocks.len(), 0);
        assert_eq!(slot1.settlement_chain_blocks.len(), 0);
        assert_eq!(slot1.state, SlotState::Unsafe);

        let slot2 = slot_rx.recv().await.unwrap();
        assert_eq!(slot2.number, START_SLOT + 1);
        assert_eq!(slot2.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot2.settlement_chain_blocks.len(), 1);
        assert_eq!(slot2.sequencing_chain_blocks[0].block.number, 2);
        assert_eq!(slot2.settlement_chain_blocks[0].block.number, 2);
        assert_eq!(slot2.state, SlotState::Unsafe);

        // Send blocks that are MAX_WAIT_SEC (24 hours) ahead, this should make slots 1, 2 and 3
        // safe
        set_tx.send(create_test_block(4, 12 + MAX_WAIT_SEC)).await.unwrap();
        seq_tx.send(create_test_block(4, 12 + MAX_WAIT_SEC)).await.unwrap();

        // shutdown slotter
        let _ = shutdown_tx.send(());
        handle.await.unwrap();

        // Create new channels for the resumed slotter
        let (new_seq_tx, new_seq_rx) = channel(CHAN_CAPACITY);
        let (new_set_tx, new_settle_rx) = channel(CHAN_CAPACITY);

        // Create new store instance pointing to same DB, and get the latest state from the DB
        let resumed_store = Box::new(RocksDbStore::new(db_path.as_str()).unwrap());
        let resumed_state = resumed_store.get_latest().await.unwrap().unwrap();
        assert_eq!(resumed_state.slot.number, START_SLOT + 2);
        assert_eq!(resumed_state.sequencing_block.number, 3);
        assert_eq!(resumed_state.settlement_block.number, 3);

        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);

        // Create new slotter that should resume from DB
        let (resumed_slotter, mut resumed_slot_rx) =
            Slotter::new(&config, Some(resumed_state), resumed_store, metrics);

        let (_shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::spawn(
            async move { resumed_slotter.start(new_seq_rx, new_settle_rx, shutdown_rx).await },
        );

        // Send new blocks to resumed slotter (since only slot 0,1,2 have been saved to the DB, we
        // should send blocks #4 (for slot 5))
        new_seq_tx.send(create_test_block(4, 13)).await.unwrap();
        new_set_tx.send(create_test_block(4, 13)).await.unwrap();

        // sending blocks for slot 5 should mark slot 4 as unsafe
        new_seq_tx.send(create_test_block(5, 14)).await.unwrap();
        new_set_tx.send(create_test_block(5, 14)).await.unwrap();

        // Should get slot 3 marked as unsafe
        let slot4 = resumed_slot_rx.recv().await.unwrap();
        assert_eq!(slot4.number, START_SLOT + 3);
        assert_eq!(slot4.state, SlotState::Unsafe);
        assert_eq!(slot4.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot4.settlement_chain_blocks.len(), 1);
        assert_eq!(slot4.sequencing_chain_blocks[0].block.number, 4);
        assert_eq!(slot4.settlement_chain_blocks[0].block.number, 4);
    }

    #[test]
    fn test_calculate_slot_position() {
        // Test with newer block
        let (slot_num, ts) = Slotter::calculate_slot_position(100, 1000, 1005, 2).unwrap();
        assert_eq!(slot_num, 103); // 3 slots ahead
        assert_eq!(ts, 1006); // 1000 + (3 * 2)

        // Test with older block
        let (slot_num, ts) = Slotter::calculate_slot_position(100, 1000, 995, 2).unwrap();
        assert_eq!(slot_num, 98); // 2 slots behind
        assert_eq!(ts, 996); // 1000 - (2 * 2)

        // Test with same timestamp
        let (slot_num, ts) = Slotter::calculate_slot_position(100, 1000, 1000, 2).unwrap();
        assert_eq!(slot_num, 100); // Same slot
        assert_eq!(ts, 1000); // Same timestamp

        // Test with exact slot boundary
        let (slot_num, ts) = Slotter::calculate_slot_position(100, 1000, 1006, 2).unwrap();
        assert_eq!(slot_num, 103); // 3 slots ahead
        assert_eq!(ts, 1006); // 1000 + (3 * 2)

        let (slot_num, ts) =
            Slotter::calculate_slot_position(12212095, 1736924186, 1736924187, 2).unwrap();
        assert_eq!(slot_num, 12212096);
        assert_eq!(ts, 1736924188);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_insert_block_between_slots() {
        let slot_start_timestamp = 10;
        let slot_duration = 1;
        let TestSetup { mut slot_rx, sequencing_tx, settlement_tx, shutdown_tx: _shutdown } =
            create_slotter_and_spawn(slot_start_timestamp, slot_duration).await;

        // Create initial slots by sending blocks
        // Slot ts=10
        sequencing_tx.send(create_test_block(1, 10)).await.unwrap();

        // Slot ts=12
        sequencing_tx.send(create_test_block(2, 12)).await.unwrap();

        // Now send a settlement block that should create a new slot between the previous slots -
        // ts=11
        settlement_tx.send(create_test_block(1, 11)).await.unwrap();

        // send a settlement block for ts = 15 , meaning slots ts=10,11 should be marked as unsafe
        settlement_tx.send(create_test_block(2, 15)).await.unwrap();

        // This should trigger slot  to be marked as unsafe
        let slot10 = slot_rx.recv().await.unwrap();
        assert_eq!(slot10.number, START_SLOT);
        assert_eq!(slot10.timestamp, 10);
        assert_eq!(slot10.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot10.settlement_chain_blocks.len(), 0);
        assert_eq!(slot10.sequencing_chain_blocks[0].block.number, 1);
        assert_eq!(slot10.state, SlotState::Unsafe);

        let slot11 = slot_rx.recv().await.unwrap();
        assert_eq!(slot11.number, START_SLOT + 1);
        assert_eq!(slot11.timestamp, 11);
        assert_eq!(slot11.sequencing_chain_blocks.len(), 0);
        assert_eq!(slot11.settlement_chain_blocks.len(), 1);
        assert_eq!(slot11.settlement_chain_blocks[0].block.number, 1);
        assert_eq!(slot11.state, SlotState::Unsafe);
    }
}
