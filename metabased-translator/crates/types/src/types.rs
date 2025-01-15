use alloy::rpc::types::{Block, Log, Transaction};
use strum::Display;

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Sequencing,
    Settlement,
}

/// The state of a slot describing its finality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotState {
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
    pub sequencing_chain_blocks: Vec<BlockInfo>,
    /// the blocks from the settlement chain to be included in the slot
    pub settlement_chain_blocks: Vec<BlockInfo>,

    /// the finality state of the slot
    pub state: SlotState,
}

impl Slot {
    /// Creates a new slot
    pub const fn new(number: u64, timestamp: u64) -> Self {
        Self {
            slot_number: number,
            timestamp,
            sequencing_chain_blocks: Vec::new(),
            settlement_chain_blocks: Vec::new(),
            state: SlotState::Open,
        }
    }

    /// Checks if the slot is empty (does not include any blocks)
    pub fn is_empty(&self) -> bool {
        self.sequencing_chain_blocks.is_empty() && self.settlement_chain_blocks.is_empty()
    }
}

/// Information about a block
#[derive(Debug, Clone)]
pub struct BlockInfo {
    /// the block
    pub block: Block,
    /// the events in the block
    pub events: Vec<Log>,
    /// the transactions in the block
    pub txs: Vec<Transaction>,
}
