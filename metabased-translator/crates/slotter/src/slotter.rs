//! Slotter module for metabased-translator

use crate::{config::SlotterConfig, metrics::SlotterMetrics};
use alloy::primitives::B256;
use common::types::{Block, BlockAndReceipts, BlockRef, Chain, KnownState, Slot, SlotProcessor};
use eyre::Report;
use std::{collections::VecDeque, sync::Arc};
use thiserror::Error;
use tokio::{
    select,
    sync::{
        mpsc::{error::SendError, Receiver},
        oneshot,
    },
};
use tracing::{debug, error, info, trace, warn};

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
struct Slotter<P: SlotProcessor> {
    settlement_delay: u64,
    max_source_chain_latency: u64,

    latest_sequencing_block: Option<BlockRef>,
    latest_settlement_block: Option<BlockRef>,
    min_chain_head_timestamp: u64,

    /// Stores all open and unsafe slots
    slots: VecDeque<Slot>,

    /// Unassigned settlement blocks
    unassigned_settlement_blocks: VecDeque<BlockAndReceipts>,

    slot_processor: P,

    metrics: SlotterMetrics,
}

/// Starts a new [`Slotter`] tasks that receives blocks from the source chains and organizes them
/// into slots.
pub async fn run(
    config: &SlotterConfig,
    known_state: Option<KnownState>,
    sequencing_rx: Receiver<Arc<BlockAndReceipts>>,
    settlement_rx: Receiver<Arc<BlockAndReceipts>>,
    slot_processor: impl SlotProcessor,
    metrics: SlotterMetrics,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), SlotterError> {
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

    let slotter = Slotter {
        settlement_delay: config.settlement_delay,
        max_source_chain_latency: config.max_source_chain_latency,
        latest_sequencing_block,
        latest_settlement_block,
        min_chain_head_timestamp,
        slots: VecDeque::new(),
        unassigned_settlement_blocks: VecDeque::new(),
        slot_processor,
        metrics,
    };
    slotter.main_loop(sequencing_rx, settlement_rx, shutdown_rx).await
}

struct PrioritizeLaggingChainResult<'a>(
    &'a mut Receiver<Arc<BlockAndReceipts>>,
    Chain,
    &'a mut Receiver<Arc<BlockAndReceipts>>,
    Chain,
    u64,
);

impl<P: SlotProcessor> Slotter<P> {
    /// Starts the [`Slotter`] main loop.
    ///
    /// The [`Slotter`] will:
    /// 1. Receive blocks from both sequencing and settlement chains
    /// 2. Place blocks into slots based on their timestamps
    /// 3. Mark slots as unsafe when both chains have progressed past them (or max wait time has
    ///    passed)
    /// 4. Send completed slots through the returned channel
    ///
    /// The receiver that was created during [`Slotter::new`] will get slots as they are processed
    async fn main_loop(
        mut self,
        mut sequencing_rx: Receiver<Arc<BlockAndReceipts>>,
        mut settlement_rx: Receiver<Arc<BlockAndReceipts>>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), SlotterError> {
        info!("Starting Slotter");

        loop {
            let PrioritizeLaggingChainResult(
                first_rx,
                first_chain,
                second_rx,
                second_chain,
                latency,
            ) = self.prioritize_lagging_chain(&mut sequencing_rx, &mut settlement_rx);

            trace!("Prioritized lagging chain: {:?}", first_chain);

            let process_result =
                if self.max_source_chain_latency > 0 && latency > self.max_source_chain_latency {
                    trace!("Latency between chains is too high: {} seconds", latency);
                    // stop receiving from the lagging chain entirely
                    select! {
                        biased;
                        Some(block) = first_rx.recv() => {
                            self.process_block(block, first_chain).await
                        }
                        _ = &mut shutdown_rx => {
                            info!("Slotter shut down");
                            return Err(SlotterError::Shutdown);
                        }
                    }
                } else {
                    select! {
                        biased;
                        Some(block) = first_rx.recv() => {
                            self.process_block(block, first_chain).await
                        }
                        Some(block) = second_rx.recv() => {
                            self.process_block(block, second_chain).await
                        }
                        _ = &mut shutdown_rx => {
                            info!("Slotter shut down");
                            return Err(SlotterError::Shutdown);
                        }
                    }
                };

            match process_result {
                Ok(_) => (),
                Err(e) => match e {
                    SlotterError::ReorgDetected { .. } => return Err(e),
                    SlotterError::Shutdown => {
                        warn!("Slotter shut down");
                        return Err(e);
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
    ) -> PrioritizeLaggingChainResult<'a> {
        let seq_ts = self.latest_sequencing_block.as_ref().map_or(0, |b| b.timestamp);
        let set_ts = self.latest_settlement_block.as_ref().map_or(0, |b| b.timestamp);

        // prefer to consume from the chain that is lagging behind
        if seq_ts <= set_ts {
            PrioritizeLaggingChainResult(
                sequencing_rx,
                Chain::Sequencing,
                settlement_rx,
                Chain::Settlement,
                set_ts - seq_ts,
            )
        } else {
            PrioritizeLaggingChainResult(
                settlement_rx,
                Chain::Settlement,
                sequencing_rx,
                Chain::Sequencing,
                seq_ts - set_ts,
            )
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

        self.metrics.update_active_slots(self.slots.len());
        Ok(())
    }

    fn update_latest_block(&mut self, block: &Block, chain: Chain) -> Result<(), SlotterError> {
        if let Some(latest) = match chain {
            Chain::Sequencing => self.latest_sequencing_block.as_ref(),
            Chain::Settlement => self.latest_settlement_block.as_ref(),
        } {
            if block.number > latest.number + 1 {
                return Err(SlotterError::BlockNumberSkipped {
                    chain,
                    current_block: Box::new(latest.clone()),
                    received_block: Box::new(BlockRef::new(block)),
                });
            }

            if !block.parent_hash.eq(&latest.hash) {
                return Err(SlotterError::ReorgDetected {
                    chain,
                    current_block: Box::new(latest.clone()),
                    received_block: Box::new(BlockRef::new(block)),
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

        // Create a new slot for this sequencer block
        let mut new_slot = Slot::new(block_info);

        // check if any unassigned settlement blocks belong in this slot
        // if so, add them to the slot and remove them from the unassigned list
        // otherwise, add the slot to the front of the list
        while let Some(set_block) = self.unassigned_settlement_blocks.front() {
            if self.settlement_timestamp(set_block) > new_slot.timestamp() {
                // we have seen a settlement block that belongs in a later slot, this one
                // can be closed
                self.slot_processor.process_slot(&new_slot).await?;
                debug!(%new_slot, "slot processed");
                self.metrics.record_last_slot_created(new_slot.sequencing.block.number);
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
        self.metrics.record_last_slot_created(new_slot.sequencing.block.number);
        self.metrics.update_unassigned_settlement_blocks(self.unassigned_settlement_blocks.len());
        self.slots.push_back(new_slot);
        Ok(())
    }

    async fn process_settlement_block(
        &mut self,
        set_block: Arc<BlockAndReceipts>,
    ) -> Result<(), SlotterError> {
        let set_ts = self.settlement_timestamp(&set_block);

        while let Some(mut slot) = self.slots.pop_front() {
            if set_ts > slot.timestamp() {
                self.slot_processor.process_slot(&slot).await?;
                debug!(%slot, "slot processed");
                continue;
            }
            slot.push_settlement_block(set_block.clone());
            debug!(
                block_number = set_block.block.number,
                slot_timestamp = slot.timestamp(),
                "block added to a slot"
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

impl<P: SlotProcessor> std::fmt::Display for Slotter<P> {
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

    #[error(
        "{chain} chain reorg detected. Current: #{current_block}, Received: #{received_block}, Received parent hash: #{received_parent_hash}"
    )]
    ReorgDetected {
        chain: Chain,
        current_block: Box<BlockRef>,
        received_block: Box<BlockRef>,
        received_parent_hash: B256,
    },

    #[error("{chain} chain block number skipped. Current: #{current_block}, Received: #{received_block}")]
    BlockNumberSkipped { chain: Chain, current_block: Box<BlockRef>, received_block: Box<BlockRef> },

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

    #[error("Slot processor error: {0}")]
    SlotProcessorError(#[from] Report),

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
    use async_trait::async_trait;
    use common::types::BlockAndReceipts;
    use prometheus_client::registry::Registry;
    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::sync::mpsc::{channel, Sender};
    use tracing_test::traced_test;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    // Mock implementation of SlotProcessor for testing
    #[derive(Debug, Clone)]
    struct MockSlotProcessor {
        processed_slots: Arc<Mutex<Vec<Slot>>>,
    }

    impl MockSlotProcessor {
        fn new() -> Self {
            Self { processed_slots: Arc::new(Mutex::new(Vec::new())) }
        }

        fn get_processed_slots(&self) -> Vec<Slot> {
            self.processed_slots.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl SlotProcessor for MockSlotProcessor {
        async fn process_slot(&self, slot: &Slot) -> Result<(), eyre::Error> {
            self.processed_slots.lock().unwrap().push(slot.clone());
            Ok(())
        }
    }

    struct TestSetup {
        processor: MockSlotProcessor,
        sequencing_tx: Sender<Arc<BlockAndReceipts>>,
        settlement_tx: Sender<Arc<BlockAndReceipts>>,
        shutdown_tx: oneshot::Sender<()>,
    }

    async fn create_slotter_and_spawn(config: &SlotterConfig) -> TestSetup {
        let processor = MockSlotProcessor::new();
        let slotter = create_slotter(config, processor.clone());

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (seq_tx, seq_rx) = channel::<Arc<BlockAndReceipts>>(100);
        let (settle_tx, settle_rx) = channel::<Arc<BlockAndReceipts>>(100);
        tokio::spawn(async move { slotter.main_loop(seq_rx, settle_rx, shutdown_rx).await });

        TestSetup { processor, sequencing_tx: seq_tx, settlement_tx: settle_tx, shutdown_tx }
    }

    fn create_slotter(
        config: &SlotterConfig,
        processor: MockSlotProcessor,
    ) -> Slotter<MockSlotProcessor> {
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = SlotterMetrics::new(&mut metrics_state.registry);

        Slotter {
            latest_sequencing_block: None,
            latest_settlement_block: None,
            slots: VecDeque::new(),
            unassigned_settlement_blocks: VecDeque::new(),
            settlement_delay: config.settlement_delay,
            max_source_chain_latency: config.max_source_chain_latency,
            metrics,
            min_chain_head_timestamp: 0,
            slot_processor: processor,
        }
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
                    "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
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

    // Helper function to wait for a specific number of slots to be processed
    async fn wait_for_processed_slots(
        processor: &MockSlotProcessor,
        count: usize,
        timeout_ms: u64,
    ) -> Result<Vec<Slot>, String> {
        let start = std::time::Instant::now();
        let timeout_duration = Duration::from_millis(timeout_ms);

        loop {
            let slots = processor.get_processed_slots();
            if slots.len() >= count {
                return Ok(slots);
            }

            if start.elapsed() > timeout_duration {
                return Err(format!(
                    "Timeout after {}ms waiting for {} slots (got {})",
                    timeout_ms,
                    count,
                    slots.len()
                ));
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
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
        let TestSetup { processor, sequencing_tx, settlement_tx, shutdown_tx: _shutdown_tx } =
            create_slotter_and_spawn(&SlotterConfig {
                settlement_delay: 0,
                max_source_chain_latency: 0,
            })
            .await;

        // send initial blocks, these should fit in slot [START_SLOT], send channel should be empty
        sequencing_tx.send(create_test_block(1, 10)).await.unwrap();
        settlement_tx.send(create_test_block(1, 10)).await.unwrap();

        // No slots should be processed yet
        assert_eq!(processor.get_processed_slots().len(), 0);

        // send a block for the settlement chain far in the future, that should this should mark
        // slot [START_SLOT] as Closed, and have any sequecing blocks with a lower timestamp form a
        // Closed empty slot
        settlement_tx.send(create_test_block(2, 20)).await.unwrap();

        // Wait for the slot to be processed
        let slots = wait_for_processed_slots(&processor, 1, 1000).await.unwrap();
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].timestamp(), 10);
        assert_eq!(slots[0].settlement.len(), 1);

        // send  few bock for the sequencing chain with timestamps lower than the settlement
        // timestamp
        sequencing_tx.send(create_test_block(2, 11)).await.unwrap();
        sequencing_tx.send(create_test_block(3, 12)).await.unwrap();
        sequencing_tx.send(create_test_block(4, 13)).await.unwrap();

        // Wait for 3 more slots to be processed
        let slots = wait_for_processed_slots(&processor, 4, 1000).await.unwrap();
        for i in 1..4 {
            let slot_idx = i;
            assert_eq!(slots[slot_idx].timestamp(), 10 + i as u64);
            assert_eq!(slots[slot_idx].sequencing.block.number, 1 + i as u64);
            assert_eq!(slots[slot_idx].sequencing.block.timestamp, 10 + i as u64);
            assert_eq!(slots[slot_idx].settlement.len(), 0);
        }

        //send a sequencing block that should create a slot that includes settlement block ts=20
        // (should still be open, tho)
        sequencing_tx.send(create_test_block(5, 40)).await.unwrap();

        // sending a sequencing block with a higher timestamp shouldn't mark the slot ts=40 as
        // Closed
        sequencing_tx.send(create_test_block(6, 41)).await.unwrap();

        // No new slots should be processed yet
        assert_eq!(processor.get_processed_slots().len(), 4);

        // only when a settlement block with a higher timestamp than ts=40 is sent, the slot ts=40
        // should be marked as Closed
        settlement_tx.send(create_test_block(3, 42)).await.unwrap();

        // Wait for one more slot to be processed
        let slots = wait_for_processed_slots(&processor, 5, 1000).await.unwrap();
        let last_slot = &slots[4];
        assert_eq!(last_slot.timestamp(), 40);
        assert_eq!(last_slot.sequencing.block.timestamp, 40);
        assert_eq!(last_slot.settlement.len(), 1);
        assert_eq!(last_slot.settlement[0].block.timestamp, 20);
    }

    #[tokio::test]
    async fn test_update_latest_block_success_sequencing() {
        let processor = MockSlotProcessor::new();
        let mut slotter = create_slotter(&SlotterConfig::default(), processor);
        let block = create_test_block(2, 200);
        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));
        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_sequencing_block.unwrap().number, 2);
    }

    #[tokio::test]
    async fn test_update_latest_block_success_settlement() {
        let processor = MockSlotProcessor::new();
        let mut slotter = create_slotter(&SlotterConfig::default(), processor);
        let block = create_test_block(3, 300);
        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));
        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert!(result.is_ok());
        assert_eq!(slotter.latest_settlement_block.unwrap().number, 3);
    }

    #[tokio::test]
    async fn test_block_number_skipped() {
        let processor = MockSlotProcessor::new();
        let mut slotter = create_slotter(&SlotterConfig::default(), processor);
        let block = create_test_block(4, 400);
        slotter.latest_settlement_block = Some(BlockRef::new(&create_test_block(2, 200).block));
        let result = slotter.update_latest_block(&block.block, Chain::Settlement);

        assert_matches!(result, Err(SlotterError::BlockNumberSkipped { .. }));
    }

    #[tokio::test]
    async fn test_earlier_timestamp() {
        let processor = MockSlotProcessor::new();
        let mut slotter = create_slotter(&SlotterConfig::default(), processor);
        let block = create_test_block(2, 50);
        slotter.latest_sequencing_block = Some(BlockRef::new(&create_test_block(1, 100).block));
        let result = slotter.update_latest_block(&block.block, Chain::Sequencing);

        assert_matches!(result, Err(SlotterError::EarlierTimestamp { .. }));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_insert_block_between_slots() {
        let TestSetup { processor, sequencing_tx, settlement_tx, shutdown_tx: _shutdown } =
            create_slotter_and_spawn(&SlotterConfig {
                settlement_delay: 0,
                max_source_chain_latency: 0,
            })
            .await;

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

        // Wait for slots to be processed
        let slots = wait_for_processed_slots(&processor, 2, 1000).await.unwrap();

        // First slot should be ts=10 with no settlement blocks
        assert_eq!(slots[0].timestamp(), 10);
        assert_eq!(slots[0].sequencing.block.number, 1);
        assert_eq!(slots[0].settlement.len(), 0);

        // Second slot should be ts=12 with one settlement block
        assert_eq!(slots[1].timestamp(), 12);
        assert_eq!(slots[1].sequencing.block.number, 2);
        assert_eq!(slots[1].settlement.len(), 1);
        assert_eq!(slots[1].settlement[0].block.number, 1);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_settlement_delay() {
        let TestSetup { processor, sequencing_tx, settlement_tx, shutdown_tx: _shutdown } =
            create_slotter_and_spawn(&SlotterConfig {
                settlement_delay: 60,
                max_source_chain_latency: 0,
            })
            .await;

        // Send initial blocks with timestamp  100
        settlement_tx.send(create_test_block(1, 100)).await.unwrap(); // Will be placed in slot 160 due to delay
        sequencing_tx.send(create_test_block(1, 100)).await.unwrap();

        // Send sequencing block for slot 110
        sequencing_tx.send(create_test_block(2, 110)).await.unwrap();

        // At this point, slot 100 should be marked as Closed because:
        // - Sequencing chain has progressed to timestamp 110
        // - Settlement chain block at timestamp 100 is treated as timestamp 160 (100 + 60)
        // Both chains have effectively progressed past slot 100

        // Wait for the first slot to be processed
        let slots = wait_for_processed_slots(&processor, 1, 1000).await.unwrap();
        assert_eq!(slots[0].timestamp(), 100);
        assert_eq!(slots[0].sequencing.block.number, 1);
        // Settlement block should not be in this slot since it was delayed to timestamp 160
        assert_eq!(slots[0].settlement.len(), 0);

        // Send sequencing block with timestamps 150
        sequencing_tx.send(create_test_block(3, 150)).await.unwrap();

        // Wait for two more slots to be processed
        let slots = wait_for_processed_slots(&processor, 3, 1000).await.unwrap();

        // There should be a slot at ts=110 and empty slot at ts=150
        assert_eq!(slots[1].timestamp(), 110);
        assert_eq!(slots[1].sequencing.block.number, 2);
        assert_eq!(slots[1].settlement.len(), 0);

        assert_eq!(slots[2].timestamp(), 150);
        assert_eq!(slots[2].sequencing.block.number, 3);
        assert_eq!(slots[2].settlement.len(), 0);

        // Send settlement and sequencing blocks with timestamp 170
        settlement_tx.send(create_test_block(2, 170)).await.unwrap();
        sequencing_tx.send(create_test_block(4, 170)).await.unwrap();

        // Wait for one more slot to be processed
        let slots = wait_for_processed_slots(&processor, 4, 1000).await.unwrap();

        // The slot with ts 170 should be marked as Closed, containing the settlement block with
        // ts=160
        assert_eq!(slots[3].timestamp(), 170);
        assert_eq!(slots[3].settlement.len(), 1);
        assert_eq!(slots[3].settlement[0].block.number, 1);
        assert_eq!(slots[3].sequencing.block.number, 4);
    }

    #[tokio::test]
    async fn test_last_settlement_block_has_latest_timestamp() {
        let TestSetup { processor, sequencing_tx, settlement_tx, shutdown_tx: _shutdown_tx } =
            create_slotter_and_spawn(&SlotterConfig {
                settlement_delay: 0,
                max_source_chain_latency: 0,
            })
            .await;

        // Send sequencing block to create a slot
        sequencing_tx.send(create_test_block(1, 100)).await.unwrap();

        // Send settlement blocks with increasing timestamps
        settlement_tx.send(create_test_block(1, 50)).await.unwrap();
        settlement_tx.send(create_test_block(2, 70)).await.unwrap();
        settlement_tx.send(create_test_block(3, 90)).await.unwrap();

        // Send another settlement block with higher timestamp to close the slot
        settlement_tx.send(create_test_block(4, 110)).await.unwrap();

        // Wait for the slot to be processed
        let slots = wait_for_processed_slots(&processor, 1, 1000).await.unwrap();

        // Verify the slot contains all settlement blocks
        assert_eq!(slots[0].timestamp(), 100);
        assert_eq!(slots[0].sequencing.block.number, 1);
        assert_eq!(slots[0].settlement.len(), 3);

        // Now verify that the last settlement block has the latest timestamp
        if let Some(last_settlement) = slots[0].settlement.last() {
            assert_eq!(last_settlement.block.timestamp, 90);

            // Also verify all settlement blocks are in ascending timestamp order
            for i in 0..slots[0].settlement.len() - 1 {
                assert!(
                    slots[0].settlement[i].block.timestamp <
                        slots[0].settlement[i + 1].block.timestamp,
                    "Settlement blocks are not in ascending timestamp order"
                );
            }
        } else {
            panic!("Expected settlement blocks but found none");
        }
    }

    #[tokio::test]
    async fn test_parent_hash_mismatch_reorg_sequencing() {
        test_parent_hash_mismatch_reorg_helper(Chain::Sequencing).await;
    }

    #[tokio::test]
    async fn test_parent_hash_mismatch_reorg_settlement() {
        test_parent_hash_mismatch_reorg_helper(Chain::Settlement).await;
    }

    async fn test_parent_hash_mismatch_reorg_helper(chain: Chain) {
        let processor = MockSlotProcessor::new();
        let mut slotter = create_slotter(&SlotterConfig::default(), processor);

        // Create and set the first block
        let first_block = create_test_block(1, 50);
        slotter.update_latest_block(&first_block.block, chain).unwrap();

        // Create a second block with correct number (2) but different parent hash
        let second_block = Arc::new(BlockAndReceipts {
            block: Block {
                hash: B256::from_str(
                    "2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(),
                number: 2, // Correct sequential number
                parent_hash: B256::from_str(
                    "0000000000abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(), // Different hash than first_block.hash
                logs_bloom: "0x0".to_string(),
                transactions_root: "0x0".to_string(),
                state_root: "0x0".to_string(),
                receipts_root: "0x0".to_string(),
                timestamp: 60,
                transactions: vec![],
            },
            receipts: vec![],
        });

        // Attempt to update with the second block - should fail due to parent hash mismatch
        let result = slotter.update_latest_block(&second_block.block, chain);

        // Verify that a reorg was detected
        assert_matches!(result, Err(SlotterError::ReorgDetected { .. }));
    }

    #[tokio::test]
    async fn test_reorg_detected_block_number_sequencing() {
        test_reorg_detected_block_number_helper(Chain::Sequencing).await;
    }

    #[tokio::test]
    async fn test_reorg_detected_block_number_settlement() {
        test_reorg_detected_block_number_helper(Chain::Settlement).await;
    }

    async fn test_reorg_detected_block_number_helper(chain: Chain) {
        let processor = MockSlotProcessor::new();
        let mut slotter = create_slotter(&SlotterConfig::default(), processor);

        // Set up first block at number 2
        let first_block = Arc::new(BlockAndReceipts {
            block: Block {
                hash: B256::from_str(
                    "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(),
                number: 2,
                parent_hash: B256::from_str(
                    "0000000000000000000000000000000000000000000000000000000000000000",
                )
                .unwrap(),
                logs_bloom: "0x0".to_string(),
                transactions_root: "0x0".to_string(),
                state_root: "0x0".to_string(),
                receipts_root: "0x0".to_string(),
                timestamp: 100,
                transactions: vec![],
            },
            receipts: vec![],
        });

        slotter.update_latest_block(&first_block.block, chain).unwrap();

        // Create block with block number 1 (lower than current)
        let reorg_block = Arc::new(BlockAndReceipts {
            block: Block {
                hash: B256::from_str(
                    "2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(),
                number: 1, // Lower block number should trigger reorg
                parent_hash: B256::from_str(
                    "0000000000abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                )
                .unwrap(),
                logs_bloom: "0x0".to_string(),
                transactions_root: "0x0".to_string(),
                state_root: "0x0".to_string(),
                receipts_root: "0x0".to_string(),
                timestamp: 50,
                transactions: vec![],
            },
            receipts: vec![],
        });

        let result = slotter.update_latest_block(&reorg_block.block, chain);
        assert_matches!(result, Err(SlotterError::ReorgDetected { .. }));
    }

    #[tokio::test]
    async fn test_max_source_chain_latency() {
        //NOTE: this tests assumes the input channels are created with a capacity of 100
        // Create a slotter with max_source_chain_latency = 10 seconds
        let config = SlotterConfig { settlement_delay: 0, max_source_chain_latency: 10 };

        let TestSetup { processor, sequencing_tx, settlement_tx, shutdown_tx: _shutdown_tx } =
            create_slotter_and_spawn(&config).await;

        // Send initial blocks for both chains with close timestamps
        settlement_tx.send(create_test_block(1, 100)).await.unwrap();
        sequencing_tx.send(create_test_block(1, 110)).await.unwrap();

        // No slots should be processed yet
        assert_eq!(processor.get_processed_slots().len(), 0);

        // Advance sequencing chain significantly ahead (beyond max_latency)
        sequencing_tx.send(create_test_block(2, 111)).await.unwrap();

        // Try to advance sequencing chain even further - this should not be consumed
        sequencing_tx.send(create_test_block(3, 112)).await.unwrap();

        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(sequencing_tx.capacity(), 100 - 1);

        // Now catch up the settlement chain
        settlement_tx.send(create_test_block(2, 125)).await.unwrap(); // This brings settlement chain close enough to consume sequencing again
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert_eq!(sequencing_tx.capacity(), 100);
        assert_eq!(settlement_tx.capacity(), 100);
    }
}
