use alloy::primitives::{Address, Bytes, FixedBytes, B256, U64};
use serde::{Deserialize, Serialize};

use crate::l3_block::L3Block;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PayloadAttributes {
    /// 64 bit value for the timestamp field of the new payload.
    pub timestamp: U64,
    /// 32 byte value for the prevRandao field of the new payload.
    pub prev_randao: B256,
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
            prev_randao: B256::random(), // Get from L2 block: https://specs.optimism.io/protocol/derivation.html#building-individual-payload-attributes
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

/// ## PayloadStatus
///
/// The status of a payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadStatus {
    /// The status of the payload.
    pub status: Status,
    /// 32 Bytes - the hash of the most recent valid block in the branch defined by payload and its ancestors
    pub latest_valid_hash: Option<B256>,
    /// A message providing additional details on the validation error if the payload is classified as INVALID or INVALID_BLOCK_HASH.
    #[serde(default)]
    pub validation_error: Option<String>,
}

/// ## Status
///
/// The status of the payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// Valid Payload
    Valid,
    /// Invalid Payload
    Invalid,
    /// Currently syncing
    Syncing,
    /// Payload is accepted
    Accepted,
    /// Payload contains an invalid block hash
    InvalidBlockHash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub struct ForkchoiceState {
    /// 32 byte block hash of the head of the canonical chain
    pub head_block_hash: B256,
    /// 32 byte "safe" block hash of the canonical chain under certain synchrony and honesty assumptions
    /// This value MUST be either equal to or an ancestor of headBlockHash
    pub safe_block_hash: B256,
    /// 32 byte block hash of the most recent finalized block
    pub finalized_block_hash: B256,
}

/// ## ExecutionPayload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayload {
    /// A 32 byte hash of the parent payload
    pub parent_hash: B256,
    /// A 20 byte hash (aka Address) for the feeRecipient field of the new payload
    pub fee_recipient: FixedBytes<20>,
    /// A 32 byte state root hash
    pub state_root: B256,
    /// A 32 byte receipt root hash
    pub receipts_root: B256,
    /// A 32 byte logs bloom filter
    pub logs_bloom: Bytes,
    /// A 32 byte beacon chain randomness value
    pub prev_randao: B256,
    /// A 64 bit number for the current block index
    pub block_number: U64,
    /// A 64 bit value for the gas limit
    pub gas_limit: U64,
    /// A 64 bit value for the gas used
    pub gas_used: U64,
    /// A 64 bit value for the timestamp field of the new payload
    pub timestamp: U64,
    /// 0 to 32 byte value for extra data
    pub extra_data: Bytes,
    /// 256 bits for the base fee per gas
    pub base_fee_per_gas: U64,
    /// The 32 byte block hash
    pub block_hash: B256,
    /// An array of transaction objects where each object is a byte list
    pub transactions: Vec<Bytes>,
    // /// An array of beaconchain withdrawals. Always empty as this exists only for L1 compatibility
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     pub withdrawals: Option<Vec<()>>,
    //     /// None if not present (pre-Ecotone)
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     pub blob_gas_used: Option<U64>,
    //     /// None if not present (pre-Ecotone)
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     pub excess_blob_gas: Option<U64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForkChoiceUpdate {
    /// Payload status.
    /// Note: values of the status field in the context of this method are restricted to the following subset: VALID, INVALID, SYNCING.
    pub payload_status: PayloadStatus,
    /// 8 byte identifier of the payload build process or null
    pub payload_id: Option<PayloadId>,
}
