//! Types module for metabased-translator

use alloy::primitives::{Address, Bytes, FixedBytes, B256};
use mchain::db::DelayedMessage;
use serde::{Deserialize, Serialize};
use shared::types::BlockRef;
use strum_macros::Display;

#[allow(missing_docs)]
pub const EMPTY_BATCH: Bytes = Bytes::from_static(&alloy::hex!("003b"));

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

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialLogWithTxdata {
    pub address: Address,
    pub topics: Vec<B256>,
    pub data: Bytes,
    pub tx_calldata: Bytes,
}

#[allow(missing_docs)]
impl PartialLogWithTxdata {
    pub fn new(log: alloy::rpc::types::Log, tx_data: Bytes) -> Self {
        Self {
            address: log.address(),
            topics: log.topics().into(),
            data: log.data().data.clone(),
            tx_calldata: tx_data,
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialBlock {
    pub block_ref: BlockRef,
    pub parent_hash: B256,
    pub logs: Vec<PartialLogWithTxdata>,
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

#[allow(missing_docs)]
pub trait GetBlockRef {
    fn block_ref(&self) -> &BlockRef;
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

impl GetBlockRef for PartialBlock {
    fn block_ref(&self) -> &BlockRef {
        &self.block_ref
    }
}
