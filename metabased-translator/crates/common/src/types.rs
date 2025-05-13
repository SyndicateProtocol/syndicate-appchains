//! Types module for metabased-translator

use alloy::primitives::{Bytes, FixedBytes};
use shared::types::{BlockRef, GetBlockRef};
use strum_macros::Display;
use synd_mchain::db::DelayedMessage;

#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct SequencingBlock {
    pub block_ref: BlockRef,
    pub parent_hash: FixedBytes<32>,
    pub batch: Bytes,
    pub tx_count: u64,
}

#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct SettlementBlock {
    pub block_ref: BlockRef,
    pub parent_hash: FixedBytes<32>,
    pub messages: Vec<DelayedMessage>,
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Sequencing,
    Settlement,
}

impl From<Chain> for &'static str {
    fn from(chain: Chain) -> &'static str {
        match chain {
            Chain::Sequencing => "sequencing",
            Chain::Settlement => "settlement",
        }
    }
}

impl GetBlockRef for SequencingBlock {
    fn block_ref(&self) -> &BlockRef {
        &self.block_ref
    }
}

impl GetBlockRef for SettlementBlock {
    fn block_ref(&self) -> &BlockRef {
        &self.block_ref
    }
}
