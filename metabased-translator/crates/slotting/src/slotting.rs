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
    Finalized,
    Safe,
    Unsafe,
    Translating,
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
            state: SlotState::Translating,
        }
    }
}

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

    /// Stores the last 10 slots
    slots: LinkedList<Slot>,
}

/// Configuration for the slotter
#[derive(Debug)]
pub struct SlotterConfig {
    /// Maximum number of slots to keep
    pub max_slots: usize,
    /// the duration of each slot
    pub slot_duration: u64,
    pub start_slot: u64,
    pub start_timestamp: u64,
}

impl Default for SlotterConfig {
    fn default() -> Self {
        Self {
            max_slots: 10,
            slot_duration: 12,
            start_slot: 0,
            start_timestamp: 0,
        }
    }
}

impl Slotter {
    pub async fn new(
        sequencing_chain_receiver: Receiver<Block>,
        settlement_chain_receiver: Receiver<Block>,
        config: SlotterConfig,
    ) -> Result<Self, Error> {
        Ok(Self {
            sequencing_chain_receiver,
            settlement_chain_receiver,
            config,
            thread: None,
            status: AtomicU8::new(0),
            slots: LinkedList::new(),
        })
    }

    pub fn start(mut self) -> Result<Receiver<Slot>, Error> {
        self.status.store(1, Release);
        let (sender, receiver) = mpsc::channel();

        let join_handle = std::thread::spawn(move || {
            let mut current_slot = Slot::new(self.config.start_slot, self.config.start_timestamp);
            let mut pending_blocks: Vec<(Block, Chain)> = Vec::new();

            const fn block_belongs_in_slot(
                block_timestamp: u64,
                slot_timestamp: u64,
                slot_duration: u64,
            ) -> bool {
                block_timestamp >= slot_timestamp
                    && block_timestamp < slot_timestamp + slot_duration
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
                        if block_belongs_in_slot(
                            block.header.timestamp,
                            current_slot.timestamp,
                            self.config.slot_duration,
                        ) {
                            Self::process_block(
                                &mut current_slot,
                                block,
                                chain,
                                &sender,
                                &mut self.slots,
                                self.config.max_slots,
                            );
                        } else {
                            current_slot = Self::advance_slot(
                                current_slot,
                                &sender,
                                &mut self.slots,
                                self.config.max_slots,
                                self.config.slot_duration,
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
                        self.config.slot_duration,
                    ) {
                        Self::process_block(
                            &mut current_slot,
                            block.clone(),
                            *chain,
                            &sender,
                            &mut self.slots,
                            self.config.max_slots,
                        );
                        false
                    } else {
                        true
                    }
                });

                std::thread::sleep(Duration::from_millis(100));
            }
        });

        self.thread = Some(join_handle);
        Ok(receiver)
    }

    pub fn stop(&mut self) {
        // TODO graceful shutdown
        // self.thread.take().unwrap().join().unwrap();
        self.status.store(0, Release);
    }

    #[allow(clippy::expect_used)] // TODO revisit
    fn process_block(
        slot: &mut Slot,
        block: Block,
        chain: Chain,
        sender: &Sender<Slot>,
        slots: &mut LinkedList<Slot>,
        max_slots: usize,
    ) -> bool {
        match chain {
            Chain::Sequencing => slot.sequencing_chain_blocks.push(block),
            Chain::Settlement => slot.settlement_chain_blocks.push(block),
        }

        let is_complete =
            !slot.sequencing_chain_blocks.is_empty() && !slot.settlement_chain_blocks.is_empty();

        if is_complete {
            slot.state = SlotState::Unsafe;
            sender.send(slot.clone()).expect("Failed to send slot");
            Self::update_slots(slots, slot.clone(), max_slots);
        }
        is_complete
    }

    fn update_slots(slots: &mut LinkedList<Slot>, slot: Slot, max_slots: usize) {
        slots.push_back(slot);
        if slots.len() > max_slots {
            slots.pop_front();
        }
    }

    #[allow(clippy::expect_used)] // TODO revisit
    fn advance_slot(
        current: Slot,
        sender: &Sender<Slot>,
        slots: &mut LinkedList<Slot>,
        max_slots: usize,
        slot_duration: u64,
    ) -> Slot {
        let current_slot_number = current.slot_number;
        let current_timestamp = current.timestamp;
        let current_clone = current.clone();
        Self::update_slots(slots, current, max_slots);
        sender.send(current_clone).expect("Failed to send slot");

        Slot::new(current_slot_number + 1, current_timestamp + slot_duration)
    }
}

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

        let start_timestamp = 1000;
        let slot_duration = 12;

        let slotter = Slotter::new(
            seq_rx,
            settle_rx,
            SlotterConfig {
                max_slots: 10,
                slot_duration,
                start_slot: 0,
                start_timestamp,
            },
        )
        .await
        .unwrap();

        let slot_rx = slotter.start().unwrap();

        // Send blocks from both chains
        seq_tx.send(create_test_block(start_timestamp + 5)).unwrap();
        settle_tx
            .send(create_test_block(start_timestamp + 7))
            .unwrap();

        // Receive and verify slot
        let slot = slot_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(slot.timestamp, start_timestamp);
        assert_eq!(slot.slot_number, 0);
        assert_eq!(slot.sequencing_chain_blocks.len(), 1);
        assert_eq!(slot.settlement_chain_blocks.len(), 1);
        assert_eq!(slot.state, SlotState::Unsafe);
    }
}
