//! Slotting module for metabased-translator

use alloy::rpc::types::Block;
use eyre::{eyre, Error};
use std::{
    cmp::Ordering,
    collections::LinkedList,
    sync::atomic::{
        AtomicU8,
        Ordering::{Acquire, Release},
    },
};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::{select, sync::mpsc::channel};
use types::{BlockInfo, Chain, Slot, SlotState};

/// Maximum time to wait for blocks before considering a slot final (24 hours in milliseconds)
const MAX_WAIT_MS: u64 = 24 * 60 * 60 * 1000;

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Slotter {
    config: SlotterConfig,

    sequencing_chain_receiver: Receiver<BlockInfo>,
    settlement_chain_receiver: Receiver<BlockInfo>,

    latest_sequencing_chain_block: Option<Block>,
    latest_settlement_chain_block: Option<Block>,

    /// 0: not started
    /// 1: started
    /// 2: stopped
    status: AtomicU8,

    /// Stores the last N slots (both open and closed)
    slots: LinkedList<Slot>,

    /// Maximum number of slots to keep
    max_slots: usize,
}

/// Configuration for the slotter
#[derive(Debug, Clone)]
pub struct SlotterConfig {
    /// The duration of each slot in milliseconds
    pub slot_duration_ms: u64,
    /// The slot number to start at
    pub start_slot: u64,
    /// The timestamp to start at
    pub start_timestamp: u64,
}

impl Default for SlotterConfig {
    fn default() -> Self {
        Self {
            slot_duration_ms: 2_000, // 2 seconds
            start_slot: 0,
            start_timestamp: 0,
        }
    }
}

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
impl Slotter {
    /// Creates a new slotter that receives blocks from two chains and organizes them into slots.
    ///
    /// # Arguments
    /// * `sequencing_chain_receiver` - Channel receiving blocks from the sequencing chain
    /// * `settlement_chain_receiver` - Channel receiving blocks from the settlement chain
    /// * `config` - Configuration for slot timing and initial state
    ///
    /// # Details
    /// The slotter maintains a window of slots spanning the last 24 hours (`MAX_WAIT_MS`),
    /// with the number of slots determined by `MAX_WAIT_MS` / `slot_duration_ms`.
    ///
    /// Each slot contains blocks from both chains whose timestamps fall within the slot's window:
    /// (`slot_timestamp` `slot_duration`on, `slot_timestamp`]
    ///
    /// # Returns
    /// A new Slotter instance that can be started with `start()`
    pub async fn new(
        sequencing_chain_receiver: Receiver<BlockInfo>,
        settlement_chain_receiver: Receiver<BlockInfo>,
        config: SlotterConfig,
    ) -> Result<Self, Error> {
        let max_slots = (MAX_WAIT_MS / config.slot_duration_ms) as usize;

        Ok(Self {
            sequencing_chain_receiver,
            settlement_chain_receiver,
            latest_sequencing_chain_block: None,
            latest_settlement_chain_block: None,
            config: config.clone(),
            status: AtomicU8::new(0),
            slots: {
                let mut slots = LinkedList::new();
                slots.push_front(Slot::new(config.start_slot, config.start_timestamp));
                slots
            },
            max_slots,
        })
    }

    /// Starts the slotter in a new thread.
    ///
    /// The slotter will:
    /// 1. Receive blocks from both sequencing and settlement chains
    /// 2. Place blocks into slots based on their timestamps
    /// 3. Mark slots as unsafe when both chains have progressed past them (or max wait time has passed)
    /// 4. Send completed slots through the returned channel
    ///
    /// Returns a receiver that will get slots as they are processed.
    /// TODO implement restore from DB
    pub fn start(mut self) -> Result<Receiver<Slot>, Error> {
        self.status.store(1, Release);
        let (sender, receiver) = channel(100); // TODO: make this configurable?

        tokio::spawn(async move {
            loop {
                if self.status.load(Acquire) == 2 {
                    // TODO graceful shutdown triggered
                    // - stop processing new blocks
                    // - go through the slots and save all safe slots to the DB (timestamp < current_time - MAX_WAIT_MS)
                    // - potentially save all unsafe/opened blocks to the DB, so they can be resumed later
                    return;
                }

                let process_result = select! {
                    Some(block) = self.sequencing_chain_receiver.recv() => {
                        self.process_block(
                            block,
                            Chain::Sequencing,
                            &sender,
                            self.config.slot_duration_ms,
                        )
                        .await
                    }
                    Some(block) = self.settlement_chain_receiver.recv() => {
                        self.process_block(
                            block,
                            Chain::Settlement,
                            &sender,
                            self.config.slot_duration_ms,
                        )
                        .await
                    }
                };

                if let Err(e) = process_result {
                    eprintln!("Error processing block: {}", e);
                    // TODO: handle errors properly
                    continue;
                }
            }
        });

        Ok(receiver)
    }

    /// Stops the slotter thread.
    ///
    /// Note: Currently performs a hard stop. Future implementations will:
    /// - Wait for thread to complete
    /// - Write info to DB, so it can be resumed later
    pub fn stop(&mut self) {
        // TODO graceful shutdown
        // self.thread.take().unwrap().join().unwrap();
        self.status.store(0, Release);
    }

    // TODO use thiserror
    async fn mark_unsafe_slots(
        &mut self,
        block: Block,
        chain: Chain,
        sender: &Sender<Slot>,
    ) -> Result<(), Error> {
        // Get the other chain's latest block timestamp
        let other_chain_timestamp = match chain {
            Chain::Sequencing => self.latest_settlement_chain_block.as_ref(),
            Chain::Settlement => self.latest_sequencing_chain_block.as_ref(),
        }
        .map(|b| b.header.timestamp);

        // Only mark slots as unsafe if we have blocks from both chains
        if let Some(other_timestamp) = other_chain_timestamp {
            let min_timestamp = other_timestamp.min(block.header.timestamp);

            // Mark slots as unsafe if both chains have progressed past them
            for slot in &mut self.slots {
                if slot.state == SlotState::Unsafe {
                    break; // assume all blocks past this point are already marked as unsafe
                }
                if slot.timestamp < min_timestamp && slot.state == SlotState::Open {
                    slot.state = SlotState::Unsafe;
                    if !slot.is_empty() {
                        sender
                            .send(slot.clone())
                            .await
                            .map_err(|_| eyre!("Failed to send slot"))?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_block(
        &mut self,
        block_info: BlockInfo,
        chain: Chain,
        sender: &Sender<Slot>,
        slot_duration_ms: u64,
    ) -> Result<(), Error> {
        // TODO try to reduce the number of clone calls
        let block_clone = block_info.block.clone();
        self.update_latest_block(block_clone.clone(), chain);
        let latest_slot = self
            .slots
            .front_mut()
            .ok_or_else(|| eyre!("No slots available"))?;

        match block_slot_ordering(
            block_info.block.header.timestamp,
            latest_slot.timestamp,
            slot_duration_ms,
        ) {
            Ordering::Less => {
                // Find the slot this block belongs to
                let mut inserted = false;
                for slot in &mut self.slots {
                    if matches!(
                        block_slot_ordering(
                            block_info.block.header.timestamp,
                            slot.timestamp,
                            slot_duration_ms,
                        ),
                        Ordering::Equal
                    ) {
                        match chain {
                            Chain::Sequencing => slot.sequencing_chain_blocks.push(block_info),
                            Chain::Settlement => slot.settlement_chain_blocks.push(block_info),
                        }
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    return Err(eyre!("Block timestamp {} is less than the latest slot and does not match any slot", block_clone.header.timestamp));
                }
            }
            Ordering::Equal => match chain {
                Chain::Sequencing => latest_slot.sequencing_chain_blocks.push(block_info),
                Chain::Settlement => latest_slot.settlement_chain_blocks.push(block_info),
            },
            Ordering::Greater => {
                let mut latest_timestamp = latest_slot.timestamp;
                let mut latest_slot_number = latest_slot.slot_number;

                // Create empty slots until we reach or pass the block's timestamp
                // this accomplishes two things:
                // - it creates slots for which we might still receive blocks (from the other chain)
                // - keeps the list full, meaning the max_slots limit will always trigger on the correct max_wait window
                while latest_timestamp < block_info.block.header.timestamp {
                    let next_timestamp = latest_timestamp + slot_duration_ms;
                    let next_slot_number = latest_slot_number + 1;
                    let slot = Slot::new(next_slot_number, next_timestamp);
                    self.slots.push_front(slot);
                    latest_timestamp = next_timestamp;
                    latest_slot_number = next_slot_number;
                }

                let latest_slot = self
                    .slots
                    .front_mut()
                    .ok_or_else(|| eyre!("No slots available"))?;
                match chain {
                    Chain::Sequencing => latest_slot.sequencing_chain_blocks.push(block_info),
                    Chain::Settlement => latest_slot.settlement_chain_blocks.push(block_info),
                }
            }
        }

        self.mark_unsafe_slots(block_clone, chain, sender).await?;
        self.cleanup_slots(sender).await?;
        Ok(())
    }

    // TODO use thiserror
    async fn cleanup_slots(&mut self, sender: &Sender<Slot>) -> Result<(), Error> {
        // Clean up old slots
        while self.slots.len() > self.max_slots {
            if let Some(mut slot) = self.slots.pop_back() {
                if slot.state == SlotState::Open {
                    slot.state = SlotState::Unsafe;
                    if !slot.is_empty() {
                        sender
                            .send(slot)
                            .await
                            .map_err(|_| eyre!("Failed to send slot"))?;
                    }
                }
            }
        }
        Ok(())
    }

    // TODO implement reorgs instead of panic
    fn update_latest_block(&mut self, block: Block, chain: Chain) {
        if let Some(latest) = match chain {
            Chain::Sequencing => &self.latest_sequencing_chain_block,
            Chain::Settlement => &self.latest_settlement_chain_block,
        } {
            assert!(
                block.header.number > latest.header.number,
                "{} chain block number must increase. Current: #{}, New: #{}",
                chain,
                latest.header.number,
                block.header.number
            );
            assert!(
                block.header.timestamp > latest.header.timestamp,
                "{} chain timestamp must increase. Current: {}, New: {}",
                chain,
                latest.header.timestamp,
                block.header.timestamp
            );
        }

        match chain {
            Chain::Sequencing => self.latest_sequencing_chain_block = Some(block),
            Chain::Settlement => self.latest_settlement_chain_block = Some(block),
        }
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
///
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_block(number: u64, timestamp: u64) -> BlockInfo {
        BlockInfo {
            block: Block {
                header: alloy::rpc::types::Header {
                    inner: alloy::consensus::Header {
                        number,
                        timestamp,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            events: vec![],
            txs: vec![],
        }
    }

    #[tokio::test]
    async fn test_slotter() {
        let (seq_tx, seq_rx) = channel(100);
        let (settle_tx, settle_rx) = channel(100);

        let start_timestamp_ms = 1_000;
        let slot_duration_ms = 2_000; // 2 seconds

        let slotter = Slotter::new(
            seq_rx,
            settle_rx,
            SlotterConfig {
                slot_duration_ms,
                start_slot: 0,
                start_timestamp: start_timestamp_ms,
            },
        )
        .await
        .unwrap();

        let mut slot_rx = slotter.start().unwrap();

        // Send blocks with small delays to ensure proper processing
        seq_tx.send(create_test_block(1, 3010)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;

        settle_tx.send(create_test_block(1, 3011)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;

        seq_tx.send(create_test_block(2, 13005)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;

        settle_tx.send(create_test_block(2, 13006)).await.unwrap();

        // Wait for slot to be marked unsafe
        let slot = slot_rx.recv().await.unwrap();

        assert_eq!(slot.timestamp, 5000); // First slot after 3010/3011 blocks
        assert_eq!(slot.slot_number, 2);
        assert_eq!(slot.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot.settlement_chain_blocks.len(), 1);
        assert_eq!(slot.state, SlotState::Unsafe);
    }
}
