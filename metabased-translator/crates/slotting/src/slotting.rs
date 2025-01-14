use alloy::rpc::types::Block;
use eyre::Error;
use std::{
    collections::LinkedList,
    sync::{
        atomic::{
            AtomicU8,
            Ordering::{Acquire, Release},
        },
        mpsc::{self, Receiver, Sender},
    },
    thread::JoinHandle,
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chain {
    Sequencing,
    Settlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotState {
    /// A slot that is considered final and cannot rollback (we don't expect any underlying chains to reorg this far)
    Finalized,
    /// A slot that is considered final according to the source L2s finality guarantees (it can only be rolled back if a L1 reorg happens)
    Safe,
    /// A slot that we don't expect to fit more blocks into. It should be considered cannonical unless a reorg happens
    Unsafe,
    /// A slot to which incoming blocks might still be added
    Open,
}

/// A `Slot` is a collection of source chain blocks  to be sent to the block builder
#[derive(Debug, Clone)]
pub struct Slot {
    /// the number of the slot - `slot_number` == `MetaChain`'s block number
    pub slot_number: u64,
    /// the timestamp of the slot
    pub timestamp: u64,
    /// the blocks from the sequencing chain to be included in the slot
    pub sequencing_chain_blocks: Vec<Block>,
    /// the blocks from the settlement chain to be included in the slot
    pub settlement_chain_blocks: Vec<Block>,

    state: SlotState,
}

impl Slot {
    const fn new(number: u64, timestamp: u64) -> Self {
        Self {
            slot_number: number,
            timestamp,
            sequencing_chain_blocks: Vec::new(),
            settlement_chain_blocks: Vec::new(),
            state: SlotState::Open,
        }
    }
}

/// Maximum time to wait for blocks before considering a slot final (24 hours in milliseconds)
const MAX_WAIT_MS: u64 = 24 * 60 * 60 * 1000;

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Slotter {
    config: SlotterConfig,

    sequencing_chain_receiver: Receiver<Block>,
    settlement_chain_receiver: Receiver<Block>,

    /// 0: not started
    /// 1: started
    /// 2: stopped
    status: AtomicU8,
    thread: Option<JoinHandle<()>>,

    /// Stores the last N slots
    slots: LinkedList<Slot>,

    /// Maximum number of slots to keep
    max_slots: usize,
}

/// Configuration for the slotter
#[derive(Debug)]
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
        sequencing_chain_receiver: Receiver<Block>,
        settlement_chain_receiver: Receiver<Block>,
        config: SlotterConfig,
    ) -> Result<Self, Error> {
        let max_slots = (MAX_WAIT_MS / config.slot_duration_ms) as usize;

        Ok(Self {
            sequencing_chain_receiver,
            settlement_chain_receiver,
            config,
            thread: None,
            status: AtomicU8::new(0),
            slots: LinkedList::new(),
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
        let (sender, receiver) = mpsc::channel();

        let join_handle = std::thread::spawn(move || {
            let mut current_slot = Slot::new(self.config.start_slot, self.config.start_timestamp);
            let mut pending_blocks: Vec<(Block, Chain)> = Vec::new();

            /// Determines if a block belongs in a slot based on its timestamp.
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
            /// This means each slot processes blocks from the previous time window,
            /// ensuring blocks are only processed after their slot window is complete.
            const fn block_belongs_in_slot(
                block_timestamp_ms: u64,
                slot_timestamp_ms: u64,
                slot_duration_ms: u64,
            ) -> bool {
                block_timestamp_ms > slot_timestamp_ms.saturating_sub(slot_duration_ms)
                    && block_timestamp_ms <= slot_timestamp_ms
            }

            loop {
                if self.status.load(Acquire) == 2 {
                    return;
                }

                // Process blocks from both chains
                for (receiver, chain) in [
                    (&self.sequencing_chain_receiver, Chain::Sequencing),
                    (&self.settlement_chain_receiver, Chain::Settlement),
                ] {
                    while let Ok(block) = receiver.try_recv() {
                        if block.header.timestamp
                            < current_slot
                                .timestamp
                                .saturating_sub(self.config.slot_duration_ms)
                        {
                            // Block is too old, skip it
                            // TODO reorg handling
                            continue;
                        }

                        if block_belongs_in_slot(
                            block.header.timestamp,
                            current_slot.timestamp,
                            self.config.slot_duration_ms,
                        ) {
                            Self::process_block(
                                &mut current_slot,
                                block,
                                chain,
                                &sender,
                                &mut self.slots,
                                self.max_slots,
                            );
                        } else {
                            // Block belongs in a future slot
                            current_slot = Self::advance_slot(
                                current_slot,
                                &sender,
                                &mut self.slots,
                                self.max_slots,
                                self.config.slot_duration_ms,
                            );
                            pending_blocks.push((block, chain));
                        }
                    }
                }

                // Process pending blocks
                pending_blocks.retain(|(block, chain)| {
                    if block_belongs_in_slot(
                        block.header.timestamp,
                        current_slot.timestamp,
                        self.config.slot_duration_ms,
                    ) {
                        Self::process_block(
                            &mut current_slot,
                            block.clone(),
                            *chain,
                            &sender,
                            &mut self.slots,
                            self.max_slots,
                        );
                        false
                    } else {
                        true
                    }
                });

                std::thread::sleep(Duration::from_millis(100)); // TODO make configurable?, maybe there is a better way (like select in go)
            }
        });

        self.thread = Some(join_handle);
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

    #[allow(clippy::expect_used)] // TODO revisit
    fn try_mark_slots_unsafe(
        slots: &mut LinkedList<Slot>,
        current_slot: &Slot,
        sender: &Sender<Slot>,
    ) {
        // If we have blocks from both chains in current slot, mark previous slots as unsafe
        if !current_slot.sequencing_chain_blocks.is_empty()
            && !current_slot.settlement_chain_blocks.is_empty()
        {
            for slot in slots.iter_mut() {
                if slot.state == SlotState::Open {
                    slot.state = SlotState::Unsafe;
                    sender.send(slot.clone()).expect("Failed to send slot");
                }
            }
        }
    }

    fn process_block(
        slot: &mut Slot,
        block: Block,
        chain: Chain,
        sender: &Sender<Slot>,
        slots: &mut LinkedList<Slot>,
        max_slots: usize,
    ) {
        match chain {
            Chain::Sequencing => slot.sequencing_chain_blocks.push(block),
            Chain::Settlement => slot.settlement_chain_blocks.push(block),
        }

        Self::try_mark_slots_unsafe(slots, slot, sender);
    }

    fn update_slots(slots: &mut LinkedList<Slot>, slot: Slot, max_slots: usize) {
        slots.push_back(slot);
        if slots.len() > max_slots {
            slots.pop_front(); // TODO this means MAX_WAIT has passed, this block should be marked as unsafe and sent to the consumer
        }
    }

    fn advance_slot(
        current: Slot,
        sender: &Sender<Slot>,
        slots: &mut LinkedList<Slot>,
        max_slots: usize,
        slot_duration_ms: u64,
    ) -> Slot {
        Self::try_mark_slots_unsafe(slots, &current, sender);

        let next_slot = Slot::new(
            current.slot_number + 1,
            current.timestamp + slot_duration_ms,
        );
        Self::update_slots(slots, current, max_slots);
        next_slot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_block(timestamp: u64) -> Block {
        Block {
            header: alloy::rpc::types::Header {
                inner: alloy::consensus::Header {
                    timestamp,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_slotter() {
        let (seq_tx, seq_rx) = mpsc::channel();
        let (settle_tx, settle_rx) = mpsc::channel();

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

        let slot_rx = slotter.start().unwrap();

        // First block should trigger slot advance since 3010 > 1000+2000
        seq_tx.send(create_test_block(3010)).unwrap();

        // This should go into the current slot
        settle_tx.send(create_test_block(3011)).unwrap();

        // These should trigger marking the slot 1 as unsafe
        seq_tx.send(create_test_block(13005)).unwrap();
        settle_tx.send(create_test_block(13006)).unwrap();

        let slot = slot_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(slot.timestamp, start_timestamp_ms + slot_duration_ms);
        assert_eq!(slot.slot_number, 1);
        assert_eq!(slot.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot.settlement_chain_blocks.len(), 1);
        assert_eq!(slot.state, SlotState::Unsafe);
    }
}
