//! Block builder service for processing and building L3 blocks.

use alloy::{
    primitives::Address,
    transports::{RpcError, TransportErrorKind},
};

#[allow(missing_docs)] // self-documenting
#[derive(Debug, thiserror::Error)]
pub enum BlockBuilderError {
    #[error("Failed to submit transaction to MetaChain: {0}")]
    SubmitTxnError(RpcError<TransportErrorKind>),

    #[error("Error getting current block number: {0}")]
    GetCurrentBlockNumber(String),

    #[error("Known block(slot) number {0} is greater than the current mchain block number {1}")]
    KnownBlockNumberGreaterThanCurrentBlockNumber(u64, u64),

    #[error("Cannot serialize empty l2 msg")]
    EmptyL2Message(),

    #[error("Error resuming from block: {0}")]
    ResumeFromBlock(String),

    #[error("Error mining block: {0}")]
    MineBlock(String),

    #[error("Block builder was shut down")]
    Shutdown,

    #[error("Rollup owner mismatch: configured owner {0} != mchain owner {1}")]
    RollupOwnerMismatch(Address, Address),
}
