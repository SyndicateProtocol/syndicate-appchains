//! Errors for the `synd-appchain-verifier`

use synd_block_builder::appchains::shared::sequencing_transaction_parser::SequencingParserError;
use thiserror::Error;
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum VerifierError {
    #[error("Invalid sequencing chain input")]
    InvalidSequencingChainInput,

    #[error("Invalid settlement chain input")]
    InvalidSettlementChainInput,

    #[error("Invalid batch")]
    InvalidBatch,

    #[error(transparent)]
    SequencingParserError(#[from] SequencingParserError),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}
