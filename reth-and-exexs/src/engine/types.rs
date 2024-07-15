//! This module was heavily inspired by [magi](https://github.com/a16z/magi).

use alloy::primitives::{Address, Bytes, FixedBytes, U64};
use serde::{Deserialize, Serialize};

use crate::l3_block::L3Block;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PayloadAttributes {
    /// 64 bit value for the timestamp field of the new payload.
    pub timestamp: U64,
    /// 32 byte value for the prevRandao field of the new payload.
    pub prev_randao: FixedBytes<32>,
    ///  20 bytes suggested value for the feeRecipient field of the new payload.
    pub suggested_fee_recipient: Address,
    /// Array of transactions to be included in the new payload.
    pub transactions: Option<Vec<Bytes>>,
    /// Boolean value indicating whether or not the payload should be built without including transactions from the txpool.
    pub no_tx_pool: bool,
    /// 64 bit value for the gasLimit field of the new payload.
    /// The gasLimit is optional w.r.t. compatibility with L1, but required when used as rollup.
    /// This field overrides the gas limit used during block-building.
    /// If not specified as rollup, a STATUS_INVALID is returned.
    pub gas_limit: U64,
    /// Beaconchain withdrawals. This exists only for compatibility with L1, and is not used. Prior
    /// to Canyon, this value is always None. After Canyon it is an empty array. Note that we use
    /// the () type here since we never have a non empty array.
    pub withdrawals: Option<Vec<()>>,
    // /// The batch epoch number from derivation. This value is not expected by the engine is skipped
    // /// during serialization and deserialization.
    // #[serde(skip)]
    // pub epoch: Option<Epoch>,
    // /// The L1 block number when this batch was first fully derived. This value is not expected by
    // /// the engine and is skipped during serialization and deserialization.
    // #[serde(skip)]
    // pub l1_inclusion_block: Option<u64>,
    // /// The L2 sequence number of the block. This value is not expected by the engine and is
    // /// skipped during serialization and deserialization.
    // #[serde(skip)]
    // pub seq_number: Option<u64>,
}

impl From<L3Block> for PayloadAttributes {
    fn from(b: L3Block) -> Self {
        PayloadAttributes {
            timestamp: b.timestamp,
            prev_randao: FixedBytes::random(), // Get from L2 block: https://specs.optimism.io/protocol/derivation.html#building-individual-payload-attributes
            suggested_fee_recipient: Address::ZERO,
            transactions: Some(b.transaction_list), // TODO: Make sure this is in the right structure
            no_tx_pool: true,
            gas_limit: U64::from(0), // System config value: https://specs.optimism.io/glossary.html#system-configuration
            withdrawals: None,
        }
    }
}

/// ## PayloadId
pub type PayloadId = U64;

// TODO: Implement:

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExecutionPayload {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForkChoiceUpdate {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub struct ForkchoiceState {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadStatus {}
