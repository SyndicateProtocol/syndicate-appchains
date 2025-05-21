//! Errors for the verifier

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum VerifierError {
    #[error("Invalid sequencing chain input")]
    InvalidSequencingChainInput,

    #[error("Invalid settlement chain input")]
    InvalidSettlementChainInput,

    #[error(transparent)]
    Other(#[from] eyre::Report),
}
