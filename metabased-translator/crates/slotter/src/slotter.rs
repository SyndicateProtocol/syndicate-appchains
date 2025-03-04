//! Slotter module for metabased-translator

use crate::{config::SlotterConfig, metrics::SlotterMetrics};
use alloy::primitives::B256;
use common::types::{Block, BlockAndReceipts, BlockRef, Chain, KnownState, Slot};
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
#[derive(Debug)]
pub struct Slotter {
    settlement_delay: u64,

    latest_sequencing_block: Option<BlockRef>,
    latest_settlement_block: Option<BlockRef>,
    min_chain_head_timestamp: u64,

    /// Stores all open and unsafe slots
    slots: VecDeque<Slot>,

    /// Unassigned settlement blocks
    unassigned_settlement_blocks: VecDeque<BlockAndReceipts>,

    /// Sender for sending slots to the consumer
    sender: Sender<Slot>,

    /// Metrics
    metrics: SlotterMetrics,
}

impl Slotter {
    /// Creates a new [`Slotter`] that receives blocks from two chains and organizes them into
    /// slots.
    pub fn new(
        config: &SlotterConfig,
        known_state: Option<KnownState>,
        metrics: SlotterMetrics,
    ) -> (Self, Receiver<Slot>) {
        let (slot_tx, slot_rx) = channel(100);
        let (latest_sequencing_block, latest_settlement_block) = match known_state {
            Some(known_state) => {
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
            slots: VecDeque::new(),
            unassigned_settlement_blocks: VecDeque::new(),
            settlement_delay: config.settlement_delay,
            sender: slot_tx,
            metrics,
            min_chain_head_timestamp,
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
        mut sequencing_rx: Receiver<Arc<BlockAndReceipts>>,
        mut settlement_rx: Receiver<Arc<BlockAndReceipts>>,
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
        sequencing_rx: &'a mut Receiver<Arc<BlockAndReceipts>>,
        settlement_rx: &'a mut Receiver<Arc<BlockAndReceipts>>,
    ) -> (
        &'a mut Receiver<Arc<BlockAndReceipts>>,
        Chain,
        &'a mut Receiver<Arc<BlockAndReceipts>>,
        Chain,
    ) {
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
        block_info: Arc<BlockAndReceipts>,
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

        self.metrics.update_active_slots(self.slots.len());
        Ok(())
    }

    fn update_latest_block(&mut self, block: &Block, chain: Chain) -> Result<(), SlotterError> {
        if let Some(latest) = match chain {
            Chain::Sequencing => self.latest_sequencing_block.as_ref(),
            Chain::Settlement => self.latest_settlement_block.as_ref(),
        } {
            if block.number != latest.number + 1 {
                return Err(SlotterError::BlockNumberSkipped {
                    chain,
                    current_block_number: latest.number,
                    received_block_number: block.number,
                });
            }

            if block.number <= latest.number || !block.parent_hash.eq(&latest.hash) {
                return Err(SlotterError::ReorgDetected {
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
        block_info: Arc<BlockAndReceipts>,
    ) -> Result<(), SlotterError> {
        let latest_slot = self.slots.back_mut();
        trace!("latest_slot: {:?}", latest_slot);

        // Each sequencer block creates a new slot
        let slot_number = match latest_slot {
            Some(slot) => slot.number + 1,
            None => START_SLOT,
        };

        // Create a new slot for this sequencer block
        let mut new_slot = Slot::new(slot_number, block_info);

        // check if any unassigned settlement blocks belong in this slot
        // if so, add them to the slot and remove them from the unassigned list
        // otherwise, add the slot to the front of the list
        while let Some(set_block) = self.unassigned_settlement_blocks.front() {
            if self.settlement_timestamp(set_block) > new_slot.timestamp() {
                // we have seen a settlement block that belongs in a later slot, this one
                // can be closed
                self.sender.send(new_slot.clone()).await?;
                debug!(%new_slot, "new slot created and closed");
                self.metrics.record_last_slot_created(slot_number);
                return Ok(());
            }
            let block = self.unassigned_settlement_blocks.pop_front().ok_or_else(|| {
                SlotterError::NoSlotsAvailable(
                    "Slot disappeared between front() and pop_front()".to_string(),
                )
            })?;
            new_slot.push_settlement_block(Arc::new(block));
        }
        debug!(%new_slot, "new opened slot created");

        // Add the new slot
        self.slots.push_back(new_slot);
        self.metrics.record_last_slot_created(slot_number);
        Ok(())
    }

    async fn process_settlement_block(
        &mut self,
        set_block: Arc<BlockAndReceipts>,
    ) -> Result<(), SlotterError> {
        let set_ts = self.settlement_timestamp(&set_block);

        while let Some(mut slot) = self.slots.pop_front() {
            if set_ts > slot.timestamp() {
                debug!(slot_number = slot.number, "slot closed");
                trace!("slot: {:?}", slot);
                self.sender.send(slot.clone()).await?;
                continue;
            }
            slot.push_settlement_block(set_block.clone());
            debug!(
                slot_number = slot.number,
                block_number = set_block.block.number,
                "block added to the slot"
            );
            trace!("settlement block added to slot: {:?}", slot);
            //add the slot back to the front of the list
            self.slots.push_front(slot);
            return Ok(());
        }

        // if we get here, the block doesn't fit in any slot
        self.unassigned_settlement_blocks
            .push_back(Arc::try_unwrap(set_block).unwrap_or_else(|arc| (*arc).clone()));
        debug!("block added to the unassigned list");
        Ok(())
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
    use common::types::BlockAndReceipts;
    use prometheus_client::registry::Registry;
    use std::{str::FromStr, time::Duration};
    use tokio::time::timeout;
    use tracing_test::traced_test;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    struct TestSetup {
        slot_rx: Receiver<Slot>,
        sequencing_tx: Sender<Arc<BlockAndReceipts>>,
        settlement_tx: Sender<Arc<BlockAndReceipts>>,
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
        let (slotter, slot_rx) = Slotter::new(config, None, metrics);

        (slotter, slot_rx)
    }

    fn create_test_block(number: u64, timestamp: u64) -> Arc<BlockAndReceipts> {
        Arc::new(BlockAndReceipts {
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
        })
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
        assert_eq!(slot.timestamp(), 10);
        assert_eq!(slot.number, START_SLOT);
        assert_eq!(slot.settlement.len(), 1);
        assert!(slot_rx.is_empty());

        // send  few bock for the sequencing chain with timestamps lower than the settlement
        // timestamp
        sequencing_tx.send(create_test_block(2, 11)).await.unwrap();
        sequencing_tx.send(create_test_block(3, 12)).await.unwrap();
        sequencing_tx.send(create_test_block(4, 13)).await.unwrap();

        for i in 1..4 {
            let slot = recv(&mut slot_rx).await.unwrap();
            assert_eq!(slot.timestamp(), 10 + i);
            assert_eq!(slot.number, START_SLOT + i);
            assert_eq!(slot.sequencing.block.number, 1 + i);
            assert_eq!(slot.sequencing.block.timestamp, 10 + i);
            assert_eq!(slot.settlement.len(), 0);
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
        assert_eq!(slot.timestamp(), 40);
        assert_eq!(slot.number, START_SLOT + 4);
        assert_eq!(slot.sequencing.block.timestamp, 40);
        assert_eq!(slot.settlement.len(), 1);
        assert_eq!(slot.settlement[0].block.timestamp, 20);
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
        assert_eq!(slot.timestamp(), 10);
        assert_eq!(slot.sequencing.block.number, 1);
        assert_eq!(slot.settlement.len(), 0);

        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.number, START_SLOT + 1);
        assert_eq!(slot.timestamp(), 12);
        assert_eq!(slot.sequencing.block.number, 2);
        assert_eq!(slot.settlement.len(), 1);
        assert_eq!(slot.settlement[0].block.number, 1);
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
        assert_eq!(slot.timestamp(), 100);
        assert_eq!(slot.sequencing.block.number, 1);
        // Settlement block should not be in this slot since it was delayed to timestamp 160
        assert_eq!(slot.settlement.len(), 0);

        // Send sequencing block with timestamps 150
        sequencing_tx.send(create_test_block(3, 150)).await.unwrap();

        // there should be a slot at ts=110 and empty slot at ts=150 in the channel
        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp(), 110);
        assert_eq!(slot.sequencing.block.number, 2);
        assert_eq!(slot.settlement.len(), 0);

        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp(), 150);
        assert_eq!(slot.sequencing.block.number, 3);
        assert_eq!(slot.settlement.len(), 0);

        // Send settlement and sequencing blocks with timestamp 170
        settlement_tx.send(create_test_block(2, 170)).await.unwrap();
        sequencing_tx.send(create_test_block(4, 170)).await.unwrap();

        // The slot with ts 170 should be marked as Closed, containing the settlement block with
        // ts=160
        let slot = recv(&mut slot_rx).await.unwrap();
        assert_eq!(slot.timestamp(), 170);
        assert_eq!(slot.settlement.len(), 1);
        assert_eq!(slot.settlement[0].block.number, 1);
        assert_eq!(slot.sequencing.block.number, 4);

        // Verify no more slots were marked as Closed
        assert!(slot_rx.try_recv().is_err());
    }
}
