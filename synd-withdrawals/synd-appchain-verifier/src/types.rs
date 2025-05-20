//! Types for the synd-appchain-verifier

use alloy::{
    consensus::{Receipt as AlloyReceipt, RlpEncodableReceipt, TxReceipt},
    eips::{Encodable2718, Typed2718},
    primitives::{Address, Bytes, B256, U256},
    rpc::types::Block,
};
use serde::{Deserialize, Serialize};
use shared::types::Receipt;
use synd_mchain::db::DelayedMessage;

/// A typed receipt for RLP and EIP-2718 encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedReceipt {
    /// The type of the receipt (0 = legacy).
    pub ty: u8,
    /// The underlying receipt data.
    pub receipt: AlloyReceipt,
}

impl Typed2718 for TypedReceipt {
    fn ty(&self) -> u8 {
        self.ty
    }
}

impl Encodable2718 for TypedReceipt {
    fn encode_2718_len(&self) -> usize {
        (if self.ty != 0 { 1 } else { 0 }) +
            self.receipt.rlp_encoded_length_with_bloom(&self.receipt.bloom())
    }

    fn encode_2718(&self, out: &mut dyn alloy::rlp::BufMut) {
        if self.ty != 0 {
            out.put_u8(self.ty);
        }
        self.receipt.rlp_encode_with_bloom(&self.receipt.bloom(), out)
    }
}

/// Settlement chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementChainInput {
    /// The blocks to verify
    pub blocks: Vec<Block>,

    /// The receipts to verify
    pub receipts: Vec<Vec<Receipt>>,

    /// Start block number
    pub start_block_number: u64,

    /// End block hash
    pub end_block_hash: B256,

    /// Unused delayed messages
    pub unused_delayed_messages: Vec<DelayedMessageBlock>,

    /// Unused delayed messages hash
    pub unused_delayed_messages_hash: B256,
}

/// Sequencing chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequencingChainInput {
    /// The blocks to verify
    pub blocks: Vec<Block>,

    /// The receipts to verify
    pub receipts: Vec<Vec<Receipt>>,

    /// Start block number
    pub start_block_number: u64,

    /// End block hash
    pub end_block_hash: B256,
}

/// Output for the verifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifierOutput {
    /// Appchain block hash
    pub block_hash: B256,
    /// Minimum timestamp
    pub min_timestamp: u64,
    /// Maximum timestamp
    pub max_timestamp: u64,
    /// Minimum block number
    pub min_block_number: u64,
    /// Maximum block number    
    pub max_block_number: u64,
    /// Messages
    pub messages: Vec<L1IncomingMessage>,
    /// Batch
    pub batch: Bytes,
}

/// L1 incoming message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessage {
    /// Header
    pub header: L1IncomingMessageHeader,
    /// L2 message
    pub l2msg: Bytes,
}

/// L1 incoming message header
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessageHeader {
    /// Kind
    pub kind: u8,
    /// Poster
    pub poster: Address,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Request ID
    pub request_id: B256,
    /// L1 base fee
    pub l1_base_fee: U256,
}

/// Delayed message block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayedMessageBlock {
    /// Timestamp
    pub timestamp: u64,
    /// Delayed messages
    pub delayed_messages: Vec<DelayedMessage>,
}
