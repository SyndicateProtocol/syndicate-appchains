//! Slotter module for metabased-translator

use crate::{config::SlotterConfig, metrics::SlotterMetrics};
use alloy::primitives::B256;
use common::{
    db::{DbError, KnownState, TranslatorStore},
    types::{Block, BlockAndReceipts, BlockRef, Chain, Slot, SlotState},
};
use derivative::Derivative;
use eyre::{Error, Report};
use std::{collections::VecDeque, sync::Arc};
use thiserror::Error;
use tokio::{
    select,
    sync::{
        mpsc::{channel, error::SendError, Receiver, Sender},
        oneshot,
    },
};
use tracing::{debug, error, info, trace, warn};

/// Maximum time to wait for blocks before considering a slot final (24 hours in seconds)
/// A slot becomes safe if both chains have progressed `SLOT_SAFETY_WINDOW_SEC` seconds past the
/// slot's timestamp
const SLOT_SAFETY_WINDOW_SEC: u64 = 24 * 60 * 60;

/// The first slot number to use
const START_SLOT: u64 = 2;

/// Polls and ingests blocks from an Ethereum chain
///
/// Slots are stored in a linked list ordered by timestamp, with newer slots at the back.
///
/// ```text
/// LinkedList:
/// Front                                                           Back
/// [Oldest] <-> [Slot N-2] <-> [Slot N-1] <-> [Current Slot] <-> [Newest]
///
/// - A slot is created when a new sequencing block is received
/// - Settlement blocks are delayed by a configured time (settlement_delay)
/// - A slot becomes Closed when we see a settlement block that cannot fit into it (the block's timestamp (plus delay) is further in the future than the slot's timestamp)
///  ```
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Slotter {
    settlement_delay: u64,

    latest_sequencing_block: Option<BlockRef>,
    latest_settlement_block: Option<BlockRef>,
    min_chain_head_timestamp: u64,

    /// Timestamp of the last saved safe slot
    safe_timestamp: u64,

    /// Stores all open and unsafe slots
    slots: VecDeque<Slot>,

    /// Index of the oldest open slot
    oldest_open_slot_idx: usize,

    /// Unassigned settlement blocks
    unassigned_settlement_blocks: VecDeque<BlockAndReceipts>,

    /// Sender for sending slots to the consumer
    sender: Sender<Slot>,

    #[derivative(Debug = "ignore")]
    store: Arc<dyn TranslatorStore + Send + Sync>,

    /// Metrics
    metrics: SlotterMetrics,
}

impl Slotter {
    /// Creates a new [`Slotter`] that receives blocks from two chains and organizes them into
    /// slots.
    pub fn new(
        config: &SlotterConfig,
        known_state: Option<KnownState>,
        store: Arc<dyn TranslatorStore + Send + Sync>,
        metrics: SlotterMetrics,
    ) -> (Self, Receiver<Slot>) {
        let (slot_tx, slot_rx) = channel(100);
        let mut slots = VecDeque::new();
        let mut safe_timestamp = 0;
        let (latest_sequencing_block, latest_settlement_block) = match known_state {
            Some(known_state) => {
                if known_state.slot.state == SlotState::Safe {
                    safe_timestamp = known_state.slot.timestamp;
                }
                slots.push_back(known_state.slot);
                (Some(known_state.sequencing_block), Some(known_state.settlement_block))
            }
            None => (None, None),
        };

        // Calculate min_chain_head_timestamp from latest blocks
        let min_chain_head_timestamp = match (&latest_sequencing_block, &latest_settlement_block) {
            (Some(seq), Some(set)) => seq.timestamp.min(set.timestamp),
            _ => 0, // If we don't have both blocks yet, use 0
        };

        let slotter = Self {
            latest_sequencing_block,
            latest_settlement_block,
            safe_timestamp,
            slots,
            unassigned_settlement_blocks: VecDeque::new(),
            settlement_delay: config.settlement_delay,
            sender: slot_tx,
            store,
            metrics,
            min_chain_head_timestamp,
            oldest_open_slot_idx: 0,
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
    /// The receiver that was created during [`Slotter::new`] will get slots as they are processed
    pub async fn start(
        mut self,
        mut sequencing_rx: Receiver<BlockAndReceipts>,
        mut settlement_rx: Receiver<BlockAndReceipts>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), Error> {
        info!("Starting Slotter");

        loop {
            let (first_rx, first_chain, second_rx, second_chain) =
                self.prioritize_lagging_chain(&mut sequencing_rx, &mut settlement_rx);

            trace!("Prioritized lagging chain: {:?}", first_chain);

            let process_result = select! {
                biased;
                Some(block) = first_rx.recv() => {
                    self.process_block(block, first_chain).await
                }
                Some(block) = second_rx.recv() => {
                    self.process_block(block, second_chain).await
                }
                _ = &mut shutdown_rx => {
                    drop(self.sender);
                    info!("Slotter shut down");
                    return Err(Report::from(SlotterError::Shutdown));
                }
            };

            match process_result {
                Ok(_) => (),
                Err(e) => match e {
                    SlotterError::ReorgDetected { .. } => {
                        panic!("Reorgs not yet implemented {e}"); // TODO SEQ-429 - implement reorg
                                                                  // handing
                    }
                    SlotterError::Shutdown => {
                        warn!("Slotter shut down");
                        return Err(Report::from(e))
                    }
                    _ => panic!("Slotter error: {e}"),
                },
            }
        }
    }

    fn prioritize_lagging_chain<'a>(
        &self,
        sequencing_rx: &'a mut Receiver<BlockAndReceipts>,
        settlement_rx: &'a mut Receiver<BlockAndReceipts>,
    ) -> (&'a mut Receiver<BlockAndReceipts>, Chain, &'a mut Receiver<BlockAndReceipts>, Chain)
    {
        let seq_ts = self.latest_sequencing_block.as_ref().map_or(0, |b| b.timestamp);
        let set_ts = self.latest_settlement_block.as_ref().map_or(0, |b| b.timestamp);

        // prefer to consume from the chain that is lagging behind
        if seq_ts <= set_ts {
            (sequencing_rx, Chain::Sequencing, settlement_rx, Chain::Settlement)
        } else {
            (settlement_rx, Chain::Settlement, sequencing_rx, Chain::Sequencing)
        }
    }

    const fn settlement_timestamp(&self, block: &BlockAndReceipts) -> u64 {
        block.block.timestamp + self.settlement_delay
    }

    async fn process_block(
        &mut self,
        block_info: BlockAndReceipts,
        chain: Chain,
    ) -> Result<(), SlotterError> {
        self.update_latest_block(&block_info.block, chain)?;

        debug!(
            ?chain,
            block_number = block_info.block.number,
            block_timestamp = block_info.block.timestamp,
            block_hash = %block_info.block.hash,
            "Processing block"
        );

        match chain {
            Chain::Sequencing => {
                self.process_sequencing_block(block_info).await?;
            }
            Chain::Settlement => {
                self.process_settlement_block(block_info).await?;
            }
        }
        if self.min_chain_head_timestamp == 0 {
            debug!("No blocks seen for both chains yet, skipping cleanup");
            return Ok(());
        }

        self.cleanup_slots().await?;
        self.metrics.update_active_slots(self.slots.len());
        Ok(())
    }

    /// `cleanup_slots` will mark slots as safe if both chains have progressed
    /// `SLOT_SAFETY_WINDOW_SEC` seconds past them and we have seen blocks from both chains
    /// after that point.
    async fn cleanup_slots(&mut self) -> Result<(), SlotterError> {
        trace!("Checking for slots to mark as safe");
        // Check slots from oldest to newest (front to back)
        let mut slots_removed = 0;

        while let Some(slot) = self.slots.front() {
            // Has 24h passed since this slot's timestamp?
            // If we can't mark this slot as safe, we can't mark newer ones either
            if slot.timestamp + SLOT_SAFETY_WINDOW_SEC > self.min_chain_head_timestamp ||
                slot.state == SlotState::Open
            {
                break;
            }

            // Remove the slot from the list
            let mut slot = self.slots.pop_front().ok_or_else(|| {
                SlotterError::NoSlotsAvailable(
                    "Slot disappeared between front() and pop_front()".to_string(),
                )
            })?;
            slots_removed += 1;

            trace!(%slot, "Saving safe slot to DB");
            slot.state = SlotState::Safe;
            self.safe_timestamp = slot.timestamp;
            self.store.save_safe_slot(&slot).await?;
        }

        // Update index after removing slots
        if slots_removed > 0 {
            //TODO add a test for this
            self.oldest_open_slot_idx = self.oldest_open_slot_idx.saturating_sub(slots_removed);
        }

        self.metrics.update_active_slots(self.slots.len());
        Ok(())
    }

    fn update_latest_block(&mut self, block: &Block, chain: Chain) -> Result<(), SlotterError> {
        if block.timestamp < self.safe_timestamp {
            return Err(SlotterError::BlockBeforeSafeTimestamp {
                chain,
                safe_timestamp: self.safe_timestamp,
                block_timestamp: block.timestamp,
                block_number: block.number,
                block_hash: block.hash,
            });
        }

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

        trace!(
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

        // Update min_chain_head_timestamp if we have blocks from both chains
        if let (Some(seq), Some(set)) =
            (&self.latest_sequencing_block, &self.latest_settlement_block)
        {
            self.min_chain_head_timestamp =
                seq.timestamp.min(set.timestamp + self.settlement_delay);
        }

        self.metrics.record_last_processed_block(block.number, chain);
        self.metrics.update_chain_timestamp_lag(block.timestamp, chain);
        Ok(())
    }

    async fn process_sequencing_block(
        &mut self,
        block_info: BlockAndReceipts,
    ) -> Result<(), SlotterError> {
        let latest_slot = self.slots.back_mut();
        trace!("latest_slot: {:?}", latest_slot);

        // Each sequencer block creates a new slot
        let slot_number = match latest_slot {
            Some(slot) => slot.number + 1,
            None => START_SLOT,
        };
        let slot_timestamp = block_info.block.timestamp;

        // Create a new slot for this sequencer block
        let mut new_slot = Slot::new(slot_number, slot_timestamp, block_info);

        // check if any unassigned settlement blocks belong in this slot
        // if so, add them to the slot and remove them from the unassigned list
        // otherwise, add the slot to the front of the list
        while let Some(set_block) = self.unassigned_settlement_blocks.front() {
            if self.settlement_timestamp(set_block) > slot_timestamp {
                // we have seen a settlement block that belongs in a later slot, this one
                // can be closed
                new_slot.state = SlotState::Closed;
                self.sender.send(new_slot.clone()).await?;
                break;
            }
            let block = self.unassigned_settlement_blocks.pop_front().ok_or_else(|| {
                SlotterError::NoSlotsAvailable(
                    "Slot disappeared between front() and pop_front()".to_string(),
                )
            })?;
            new_slot.push_settlement_block(block);
        }
        debug!(%new_slot, "new slot created");

        // Add the new slot
        self.slots.push_back(new_slot);
        self.metrics.record_last_slot_created(slot_number);
        Ok(())
    }

    async fn process_settlement_block(
        &mut self,
        set_block: BlockAndReceipts,
    ) -> Result<(), SlotterError> {
        let ts = self.settlement_timestamp(&set_block);
        let mut closed_slots = 0;
        let mut inserted = false;
        let sender = self.sender.clone();

        for slot in self.iter_from_oldest_open() {
            if ts > slot.timestamp {
                if slot.state == SlotState::Open {
                    debug!(slot_number = slot.number, "slot closed");
                    trace!("slot: {:?}", slot);
                    slot.state = SlotState::Closed;
                    sender.send(slot.clone()).await?;
                    closed_slots += 1;
                }
                continue;
            }
            slot.push_settlement_block(set_block.clone());
            debug!(
                slot_number = slot.number,
                block_number = set_block.block.number,
                "block added to the slot"
            );
            trace!("slot: {:?}", slot);
            inserted = true;
            break;
        }

        if closed_slots > 0 {
            self.oldest_open_slot_idx += closed_slots;
        }

        if !inserted {
            self.unassigned_settlement_blocks.push_back(set_block);
            debug!("block added to the unassigned list");
        }
        Ok(())
    }

    fn iter_from_oldest_open(&mut self) -> impl Iterator<Item = &mut Slot> {
        self.slots.range_mut(self.oldest_open_slot_idx..)
    }
}

impl std::fmt::Display for Slotter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Slotter[slots_len:{}, latest_seq:{}, latest_set:{}]",
            self.slots.len(),
            self.latest_sequencing_block.as_ref().map_or("none".to_string(), |b| b.to_string()),
            self.latest_settlement_block.as_ref().map_or("none".to_string(), |b| b.to_string())
        )
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum SlotterError {
    #[error("No slots available {0}")]
    NoSlotsAvailable(String),

    #[error("Failed to send slot through channel: {0}")]
    SlotSendError(String),

    #[error("{chain} chain reorg detected. Current: #{current_block_number}, Received: #{received_block_number}")]
    ReorgDetected { chain: Chain, current_block_number: u64, received_block_number: u64 },

    #[error("{chain} chain block number skipped. Current: #{current_block_number}, Received: #{received_block_number}")]
    BlockNumberSkipped { chain: Chain, current_block_number: u64, received_block_number: u64 },

    #[error("{chain} chain timestamp must not decrease. Current: {current}, Received: {received}")]
    EarlierTimestamp { chain: Chain, current: u64, received: u64 },

    #[error("Block timestamp is before the safe timestamp. Safe timestamp: {safe_timestamp}, Block timestamp: {block_timestamp}, Block number: {block_number}, Block hash: {block_hash}")]
    BlockBeforeSafeTimestamp {
        chain: Chain,
        safe_timestamp: u64,
        block_timestamp: u64,
        block_number: u64,
        block_hash: B256,
    },

    #[error("Database error: {0}")]
    DbError(#[from] DbError),

    #[error("Slotter was shut down")]
    Shutdown,
}

impl From<SendError<Slot>> for SlotterError {
    fn from(e: SendError<Slot>) -> Self {
        Self::SlotSendError(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::B256;
    use assert_matches::assert_matches;
    use prometheus_client::registry::Registry;
    use std::{str::FromStr, time::Duration};
    use tokio::time::timeout;
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

    async fn create_slotter_and_spawn(config: &SlotterConfig) -> TestSetup {
        let (slotter, slot_rx) = create_slotter(config);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (seq_tx, seq_rx) = channel(100);
        let (settle_tx, settle_rx) = channel(100);
        tokio::spawn(async move { slotter.start(seq_rx, settle_rx, shutdown_rx).await });

        TestSetup { slot_rx, sequencing_tx: seq_tx, settlement_tx: settle_tx, shutdown_tx }
    }

    fn create_slotter(config: &SlotterConfig) -> (Slotter, Receiver<Slot>) {
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);
        let store = Arc::new(RocksDbStore::new(test_path("slotter_db").as_str()).unwrap());

        let (slotter, slot_rx) = Slotter::new(config, None, store, metrics);

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

    // receives with a timeout (avoids tests hanging)
    async fn recv(slot_rx: &mut Receiver<Slot>) -> Result<Slot, String> {
        match timeout(Duration::from_secs(1), slot_rx.recv()).await {
            Ok(Some(slot)) => Ok(slot),
            Ok(None) => Err("Channel closed".to_string()),
            Err(elapsed) => Err(format!("Timeout after 1s: {elapsed}")),
        }
    }

    /// Test scenario:
    /// Slots are only marked as Closed once a settlement block with a bigger timestamp is received
    /// ```text
    /// Slot [START_SLOT] [ts=10]:
    /// ┌───────────────────┐
    /// │ seq    ts=10 #1   │
    /// │ settle ts=10 #1   │
    /// └───────────────────┘
    ///
    /// Slots [START_SLOT+1, +2, +3] [ts=11, 12, 13]:
    /// ┌───────────────────┐ ┌───────────────────┐ ┌───────────────────┐
    /// │ seq    ts=11 #2   │ │ seq    ts=12 #3   │ │ seq    ts=13 #4   │
    /// └───────────────────┘ └───────────────────┘ └───────────────────┘
    ///
    /// Slot [START_SLOT+4] [ts=40]:
    /// ┌───────────────────┐
    /// │ seq    ts=20 #5   │
    /// │ settle ts=40 #2   │
    /// └───────────────────┘
    ///
    /// Legend:
    /// @ timestamp
    /// # block number
    /// ```
    #[tokio::test]
    #[traced_test]
    async fn test_slotter() {
        // NOTE: IMPORTANT - keep _shutdown_tx in scope, otherwise `slotter` will be terminated
        // immediatelly
        let TestSetup { mut slot_rx, sequencing_tx, settlement_tx, shutdown_tx: _shutdown_tx } =
            create_slotter_and_spawn(&SlotterConfig { settlement_delay: 0 }).await;

        // send initial blocks, these should fit in slot [START_SLOT], send channel should be empty
        sequencing_tx.send(create_test_block(1, 10)).await.unwrap();
        settlement_tx.send(create_test_block(1, 10)).await.unwrap();

        assert!(slot_rx.is_empty());

        // send a block for the settlement chain far in the future, that should this should mark
        // slot [START_SLOT] as Closed, and have any sequecing blocks with a lower timestamp form a
        // Closed empty slot
        settlement_tx.send(create_test_block(2, 20)).await.unwrap();

        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp, 10);
        assert_eq!(slot.number, START_SLOT);
        assert_eq!(slot.settlement_blocks.len(), 1);
        assert_eq!(slot.state, SlotState::Closed);
        assert!(slot_rx.is_empty());

        // send  few bock for the sequencing chain with timestamps lower than the settlement
        // timestamp
        sequencing_tx.send(create_test_block(2, 11)).await.unwrap();
        sequencing_tx.send(create_test_block(3, 12)).await.unwrap();
        sequencing_tx.send(create_test_block(4, 13)).await.unwrap();

        for i in 1..4 {
            let slot = recv(&mut slot_rx).await.unwrap();
            assert_eq!(slot.timestamp, 10 + i);
            assert_eq!(slot.number, START_SLOT + i);
            assert_eq!(slot.sequencing_block.block.number, 1 + i);
            assert_eq!(slot.sequencing_block.block.timestamp, 10 + i);
            assert_eq!(slot.settlement_blocks.len(), 0);
            assert_eq!(slot.state, SlotState::Closed);
        }
        assert!(slot_rx.is_empty());

        //send a sequencing block that should create a slot that includes settlement block ts=20
        // (should still be open, tho)
        sequencing_tx.send(create_test_block(5, 40)).await.unwrap();

        // sending a sequencing block with a higher timestamp shouldn't mark the slot ts=40 as
        // Closed
        sequencing_tx.send(create_test_block(6, 41)).await.unwrap();
        assert!(slot_rx.is_empty());

        // only when a settlement block with a higher timestamp than ts=40 is sent, the slot ts=40
        // should be marked as Closed
        settlement_tx.send(create_test_block(3, 42)).await.unwrap();
        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp, 40);
        assert_eq!(slot.number, START_SLOT + 4);
        assert_eq!(slot.sequencing_block.block.timestamp, 40);
        assert_eq!(slot.settlement_blocks.len(), 1);
        assert_eq!(slot.settlement_blocks[0].block.timestamp, 20);
        assert_eq!(slot.state, SlotState::Closed);
    }

    #[tokio::test]
    async fn test_update_latest_block_success_sequencing() {
        let (mut slotter, _) = create_slotter(&SlotterConfig::default());
        let block = create_test_block(2, 200);
        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));
        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_sequencing_block.unwrap().number, 2);
    }

    #[tokio::test]
    async fn test_update_latest_block_success_settlement() {
        let (mut slotter, _) = create_slotter(&SlotterConfig::default());
        let block = create_test_block(3, 300);
        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));
        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_settlement_block.unwrap().number, 3);
    }

    #[tokio::test]
    async fn test_reorg_detected() {
        let (mut slotter, _) = create_slotter(&SlotterConfig::default());
        let block = create_test_block(1, 50);
        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(2, 10_001).block));
        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert_matches!(result, Err(SlotterError::ReorgDetected { .. }));
    }

    #[tokio::test]
    async fn test_block_number_skipped() {
        let (mut slotter, _) = create_slotter(&SlotterConfig::default());
        let block = create_test_block(4, 400);
        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));
        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert_matches!(result, Err(SlotterError::BlockNumberSkipped { .. }));
    }

    #[tokio::test]
    async fn test_earlier_timestamp() {
        let (mut slotter, _) = create_slotter(&SlotterConfig::default());
        let block = create_test_block(2, 50);
        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));
        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert_matches!(result, Err(SlotterError::EarlierTimestamp { .. }));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_slotter_db_shutdown_and_resume_safe() {
        const CHAN_CAPACITY: usize = 100;
        let (seq_tx, seq_rx) = channel(CHAN_CAPACITY);
        let (set_tx, set_rx) = channel(CHAN_CAPACITY);

        let config = SlotterConfig { settlement_delay: 0 };

        // Create a fresh DB for this test
        let db_path = test_path("slotter_db");
        let store = Arc::new(RocksDbStore::new(db_path.as_str()).unwrap());

        // Start first slotter instance
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);
        let (slotter, mut slot_rx) = Slotter::new(&config, None, store, metrics);
        let handle = tokio::spawn(async move { slotter.start(seq_rx, set_rx, shutdown_rx).await });

        // Send some blocks
        // slot [START_SLOT]
        seq_tx.send(create_test_block(1, 20)).await.unwrap();
        set_tx.send(create_test_block(1, 20)).await.unwrap();

        // slot [START_SLOT+1]
        seq_tx.send(create_test_block(2, 30)).await.unwrap();
        set_tx.send(create_test_block(2, 30)).await.unwrap();

        // This should make slot [START_SLOT]  as Closed
        let slot1 = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot1.number, START_SLOT);
        assert_eq!(slot1.settlement_blocks.len(), 1);
        assert_eq!(slot1.sequencing_block.block.number, 1);
        assert_eq!(slot1.settlement_blocks[0].block.number, 1);
        assert_eq!(slot1.state, SlotState::Closed);

        // Send blocks that are SLOT_SAFETY_WINDOW_SEC (24 hours) ahead, this should make all
        // previous slots as Safe
        set_tx.send(create_test_block(3, 30 + SLOT_SAFETY_WINDOW_SEC)).await.unwrap();
        seq_tx.send(create_test_block(3, 30 + SLOT_SAFETY_WINDOW_SEC)).await.unwrap();

        // drain the channel
        let _handle = tokio::spawn(async move {
            loop {
                slot_rx.recv().await;
            }
        });

        // shutdown slotter
        let _ = shutdown_tx.send(());
        let _ = handle.await.unwrap();

        // Create new channels for the resumed slotter
        let (new_seq_tx, new_seq_rx) = channel(CHAN_CAPACITY);
        let (new_set_tx, new_settle_rx) = channel(CHAN_CAPACITY);

        // Create new store instance pointing to same DB, and get the latest state from the DB
        let resumed_store = Arc::new(RocksDbStore::new(db_path.as_str()).unwrap());
        let resumed_state = resumed_store.get_safe_state().await.unwrap().unwrap();
        assert_eq!(resumed_state.slot.number, START_SLOT + 1);
        assert_eq!(resumed_state.sequencing_block.number, 2);
        assert_eq!(resumed_state.settlement_block.number, 2);

        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);

        // Create new slotter that should resume from DB
        let (resumed_slotter, mut resumed_slot_rx) =
            Slotter::new(&config, Some(resumed_state), resumed_store, metrics);

        let (_shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::spawn(
            async move { resumed_slotter.start(new_seq_rx, new_settle_rx, shutdown_rx).await },
        );

        // Send new blocks to resumed slotter (since only slot [START_SLOT] and [START_SLOT+1] have
        // saved to the DB, we should send blocks #4 (for slot [START_SLOT+2]))
        new_seq_tx.send(create_test_block(3, 40)).await.unwrap();
        new_set_tx.send(create_test_block(3, 40)).await.unwrap();

        // sending blocks for slot [START_SLOT+2] should mark slot [START_SLOT+2] as Closed
        new_seq_tx.send(create_test_block(4, 50)).await.unwrap();
        new_set_tx.send(create_test_block(4, 50)).await.unwrap();

        // Should get slot [START_SLOT+2] marked as Closed
        let slot = recv(&mut resumed_slot_rx).await.unwrap();
        assert_eq!(slot.number, START_SLOT + 2);
        assert_eq!(slot.state, SlotState::Closed);
        assert_eq!(slot.settlement_blocks.len(), 1);
        assert_eq!(slot.sequencing_block.block.number, 3);
        assert_eq!(slot.settlement_blocks[0].block.number, 3);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_insert_block_between_slots() {
        let TestSetup { mut slot_rx, sequencing_tx, settlement_tx, shutdown_tx: _shutdown } =
            create_slotter_and_spawn(&SlotterConfig { settlement_delay: 0 }).await;

        // Create initial slots by sending blocks
        // Slot ts=10
        sequencing_tx.send(create_test_block(1, 10)).await.unwrap();

        // Slot ts=12
        sequencing_tx.send(create_test_block(2, 12)).await.unwrap();

        // NOTE: this is necessary to ensure that the slotter has time to process the previous
        // blocks, otherwise the biased select will ensure the slots will be processed in order of
        // timestamp
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Now send a settlement block that should fit between the previous slots (belongs to slots
        // ts=12)
        settlement_tx.send(create_test_block(1, 11)).await.unwrap();

        // send a settlement block for ts = 15 , meaning slots ts=10,12 should be marked as
        // Closed
        settlement_tx.send(create_test_block(2, 15)).await.unwrap();

        // This should trigger slot  to be marked as Closed
        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.number, START_SLOT);
        assert_eq!(slot.timestamp, 10);
        assert_eq!(slot.sequencing_block.block.number, 1);
        assert_eq!(slot.settlement_blocks.len(), 0);
        assert_eq!(slot.state, SlotState::Closed);

        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.number, START_SLOT + 1);
        assert_eq!(slot.timestamp, 12);
        assert_eq!(slot.sequencing_block.block.number, 2);
        assert_eq!(slot.settlement_blocks.len(), 1);
        assert_eq!(slot.settlement_blocks[0].block.number, 1);
        assert_eq!(slot.state, SlotState::Closed);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_settlement_delay() {
        let TestSetup { mut slot_rx, sequencing_tx, settlement_tx, shutdown_tx: _shutdown } =
            create_slotter_and_spawn(&SlotterConfig { settlement_delay: 60 }).await;

        // Send initial blocks with timestamp  100
        settlement_tx.send(create_test_block(1, 100)).await.unwrap(); // Will be placed in slot 160 due to delay
        sequencing_tx.send(create_test_block(1, 100)).await.unwrap();

        // Send sequencing block for slot 110
        sequencing_tx.send(create_test_block(2, 110)).await.unwrap();

        // At this point, slot 100 should be marked as Closed because:
        // - Sequencing chain has progressed to timestamp 110
        // - Settlement chain block at timestamp 100 is treated as timestamp 160 (100 + 60)
        // Both chains have effectively progressed past slot 100

        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.number, START_SLOT);
        assert_eq!(slot.timestamp, 100);
        assert_eq!(slot.state, SlotState::Closed);
        assert_eq!(slot.sequencing_block.block.number, 1);
        // Settlement block should not be in this slot since it was delayed to timestamp 160
        assert_eq!(slot.settlement_blocks.len(), 0);

        // Send sequencing block with timestamps 150
        sequencing_tx.send(create_test_block(3, 150)).await.unwrap();

        // there should be a slot at ts=110 and empty slot at ts=150 in the channel
        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp, 110);
        assert_eq!(slot.state, SlotState::Closed);
        assert_eq!(slot.sequencing_block.block.number, 2);
        assert_eq!(slot.settlement_blocks.len(), 0);

        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp, 150);
        assert_eq!(slot.state, SlotState::Closed);
        assert_eq!(slot.sequencing_block.block.number, 3);
        assert_eq!(slot.settlement_blocks.len(), 0);

        // Send settlement and sequencing blocks with timestamp 170
        settlement_tx.send(create_test_block(2, 170)).await.unwrap();
        sequencing_tx.send(create_test_block(4, 170)).await.unwrap();

        // The slot with ts 170 should be marked as Closed, containing the settlement block with
        // ts=160
        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp, 170);
        assert_eq!(slot.settlement_blocks.len(), 1);
        assert_eq!(slot.settlement_blocks[0].block.number, 1);
        assert_eq!(slot.sequencing_block.block.number, 4);

        // Verify no more slots were marked as Closed
        assert!(slot_rx.try_recv().is_err());
    }
}
