//! Errors for the `synd-appchain-verifier`

use synd_block_builder::appchains::shared::sequencing_transaction_parser::SequencingParserError;
use thiserror::Error;
use withdrawals_shared::error::VerifierError;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum AppchainVerifierError {
    #[error("Invalid sequencing chain input {reason}")]
    InvalidSequencingChainInputWithReason { reason: String },

    #[error("Invalid settlement chain input {reason}")]
    InvalidSettlementChainInputWithReason { reason: String },

    #[error(transparent)]
    VerifierError(#[from] VerifierError),

    #[error("Invalid batch")]
    InvalidBatch,

    #[error(transparent)]
    SequencingParserError(#[from] SequencingParserError),

    #[error(transparent)]
    Other(#[from] eyre::Error),

    #[error("Unknown error")]
    Unknown,
}
