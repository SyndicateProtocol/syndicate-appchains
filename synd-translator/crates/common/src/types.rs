//! Types module for `synd-translator`

use alloy::primitives::FixedBytes;
use shared::types::{BlockRef, GetBlockRef, PartialBlock};
use strum_macros::Display;
use synd_mchain::db::DelayedMessage;

#[allow(missing_docs)]
pub type SequencingBlock = PartialBlock;

#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct SettlementBlock {
    pub block_ref: BlockRef,
    pub parent_hash: FixedBytes<32>,
    pub messages: Vec<DelayedMessage>,
}

#[allow(missing_docs)]
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

impl GetBlockRef for SettlementBlock {
    fn block_ref(&self) -> &BlockRef {
        &self.block_ref
    }
}
