//! Errors for the verifier

use alloy::primitives::FixedBytes;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum VerifierError {
    #[error("Receipts root mismatch at block index {index}: expected {expected}, got {actual}")]
    ReceiptsRootMismatch { index: usize, expected: FixedBytes<32>, actual: FixedBytes<32> },

    #[error("Final block hash mismatch: expected {expected}, got {actual}")]
    FinalBlockHashMismatch { expected: FixedBytes<32>, actual: FixedBytes<32> },

    #[error("Invalid parent hash at block {block_number}: expected {expected}, got {actual}")]
    InvalidParentHash { block_number: u64, expected: FixedBytes<32>, actual: FixedBytes<32> },

    #[error("Invalid block hash at block {block_number}: expected {expected}, got {actual}")]
    InvalidBlockHash { block_number: u64, expected: FixedBytes<32>, actual: FixedBytes<32> },

    #[error("No blocks provided")]
    NoBlocksProvided,

    #[error("No sequence blocks provided")]
    NoSequenceBlocksProvided,

    #[error("No settlement blocks provided")]
    NoSettlementBlocksProvided,

    #[error("No sequence receipts provided")]
    NoSequenceReceiptsProvided,

    #[error("Missing settlement blocks")]
    MissingSettlementBlocks,

    #[error(transparent)]
    Other(#[from] eyre::Report),
}
