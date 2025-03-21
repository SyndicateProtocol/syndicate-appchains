//! The `errors` module contains the error types for Maestro.

use crate::errors::Error::InvalidParams;
use alloy::contract;
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
/// Primary error type for the metabased sequencer, following JSON-RPC error code mapping
#[derive(Debug)]
pub enum Error {
    /// The JSON sent is not a valid Request object
    InvalidRequest,
    /// The method does not exist / is not available
    MethodNotFound(String),
    /// Invalid method parameter(s)
    InvalidParams(InvalidParamsError),
    /// Internal JSON-RPC error
    Internal,
    /// Invalid JSON was received by the server
    Parse,
    /// Invalid input data provided
    InvalidInput(InvalidInputError),
    /// Requested resource not found
    ResourceNotFound,
    /// Requested resource not available
    ResourceUnavailable,
    /// Transaction was rejected
    TransactionRejected(Rejection),
    /// Method is not implemented
    MethodNotSupported,
    /// Request exceeds defined limits
    LimitExceeded,
    /// Server error
    Server,
    /// Contract-related error
    Contract(contract::Error),
}

impl Error {
    /// Convert the error to a JSON-RPC error object
    pub fn to_json_rpc_error(&self) -> ErrorObjectOwned {
        // TODO implement this
        ErrorObject::owned(-32000, "Internal error", None::<()>)
    }
}

/// Invalid parameter errors that can occur during request processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidParamsError {
    /// Signature validation failed
    BadSignature,
    /// Transaction nonce is lower than current nonce
    NonceTooLow,
    /// Invalid hex string format
    InvalidHex,
    /// Parameters must be provided as an array
    NotAnArray,
    /// Incorrect number of parameters provided
    WrongParamCount(usize),
    /// Required parameter is missing
    MissingParam,
    /// String is not hex encoded
    NotHexEncoded,
}

//TODO remove me?
/// Reasons for transaction rejection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    /// Transaction fee is too high
    FeeTooHigh,
}

/// Invalid input errors that can occur during request processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidInputError {
    /// Invalid JSON format
    InvalidJson,
    /// Invalid unsigned integer value
    InvalidUint,
    /// Transaction signature is invalid
    InvalidTransactionSignature,
    /// Failed to decode RLP data
    UnableToRLPDecode,
    /// Chain ID is missing
    MissingChainID,
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidJson)
    }
}

impl From<alloy::hex::FromHexError> for Error {
    fn from(_: alloy::hex::FromHexError) -> Self {
        InvalidParams(InvalidParamsError::InvalidHex)
    }
}
