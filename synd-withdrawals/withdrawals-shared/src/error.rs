//! Shared errors for withdrawals
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum VerifierError {
    #[error("Invalid input: {reason}. Expected {expected:?}, got {actual:?}")]
    InvalidInput { reason: String, expected: String, actual: String },

    #[error("Error verifying proof: {0}")]
    ErrorVerifyingProof(String),
}
