//! Errors for the `synd-appchain-verifier`

use synd_block_builder::appchains::shared::sequencing_transaction_parser::SequencingParserError;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum VerifierError {
    #[error("Invalid sequencing chain input {reason}")]
    InvalidSequencingChainInputWithReason { reason: String },

    #[error("Invalid settlement chain input {reason}")]
    InvalidSettlementChainInputWithReason { reason: String },

    #[error("Invalid sequencing chain input: {reason}. Expected {expected:?}, got {actual:?}")]
    InvalidSequencingChainInput { reason: String, expected: String, actual: String },

    #[error("Invalid settlement chain input: {reason}. Expected {expected:?}, got {actual:?}")]
    InvalidSettlementChainInput { reason: String, expected: String, actual: String },

    #[error("Error verifying proof: {0}")]
    ErrorVerifyingProof(String),

    #[error("Invalid batch")]
    InvalidBatch,

    #[error(transparent)]
    SequencingParserError(#[from] SequencingParserError),

    #[error(transparent)]
    Other(#[from] eyre::Error),

    #[error("Unknown error")]
    Unknown,
}
