//! Errors for the `synd-seqchain-verifier`

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum VerifierError {
    #[error("Invalid L1 chain input {reason}")]
    InvalidL1ChainInputWithReason { reason: String },

    #[error("Invalid L1 chain input: {reason}. Expected {expected:?}, got {actual:?}")]
    InvalidL1ChainInput { reason: String, expected: String, actual: String },

    #[error("Error verifying proof: {0}")]
    ErrorVerifyingProof(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}
