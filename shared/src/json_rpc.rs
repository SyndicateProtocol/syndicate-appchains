//! The `json_rpc` module contains functionality relating to JSON-RPC request handling

use alloy::{
    contract, hex,
    primitives::{ruint::ToUintError, SignatureError},
    rlp,
};
use jsonrpsee::types::{error, ErrorObject, ErrorObjectOwned};
use std::convert::Infallible;
use thiserror::Error;

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
/// Primary error type for the metabased sequencer, following JSON-RPC error code mapping
#[derive(Debug, Error)]
pub enum Error {
    /// The JSON sent is not a valid Request object
    #[error("invalid request")]
    InvalidRequest,
    /// The method does not exist / is not available
    #[error("method not found: {0}")]
    MethodNotFound(String),
    /// Invalid method parameter(s)
    #[error("invalid params: {0}")]
    InvalidParams(InvalidParamsError),
    /// Internal JSON-RPC error
    #[error("internal error")]
    Internal,
    /// Invalid JSON was received by the server
    #[error("parse error")]
    Parse,
    /// Invalid input data provided
    #[error("invalid input: {0}")]
    InvalidInput(InvalidInputError),
    /// Requested resource not found
    #[error("resource not found")]
    ResourceNotFound,
    /// Requested resource not available
    #[error("resource unavailable")]
    ResourceUnavailable,
    /// Transaction was rejected
    #[error("transaction rejected: {0}")]
    TransactionRejected(Rejection),
    /// Method is not implemented
    #[error("method not supported")]
    MethodNotSupported,
    /// Request exceeds defined limits
    #[error("limit exceeded")]
    LimitExceeded,
    /// Server error
    #[error("server error")]
    Server,
    /// Contract-related error
    #[error("contract error: {0}")]
    Contract(contract::Error),
}

impl Error {
    /// Convert the error to a JSON-RPC error object
    pub fn to_json_rpc_error(&self) -> ErrorObjectOwned {
        match self {
            Self::InvalidRequest => {
                ErrorObject::owned(error::INVALID_REQUEST_CODE, "invalid request", None::<()>)
            }
            Self::MethodNotFound(m) => ErrorObject::owned(
                error::METHOD_NOT_FOUND_CODE,
                format!("method not found: {}", m),
                None::<()>,
            ),
            Self::InvalidParams(m) => ErrorObject::owned(
                error::INVALID_PARAMS_CODE,
                format!("invalid params: {}", m),
                None::<()>,
            ),
            Self::Internal => {
                ErrorObject::owned(error::INTERNAL_ERROR_CODE, "internal error", None::<()>)
            }
            Self::Parse => ErrorObject::owned(error::PARSE_ERROR_CODE, "parse error", None::<()>),
            Self::InvalidInput(m) => ErrorObject::owned(
                error::CALL_EXECUTION_FAILED_CODE,
                format!("invalid input: {}", m),
                None::<()>,
            ),
            Self::ResourceNotFound => {
                ErrorObject::owned(error::UNKNOWN_ERROR_CODE, "resource not found", None::<()>)
            }
            Self::ResourceUnavailable => {
                ErrorObject::owned(error::INVALID_REQUEST_CODE, "resource unavailable", None::<()>)
            }
            Self::TransactionRejected(m) => {
                ErrorObject::owned(-32003, format!("transaction rejected: {}", m), None::<()>)
            }
            Self::MethodNotSupported => {
                ErrorObject::owned(-32004, "method not supported", None::<()>)
            }
            Self::LimitExceeded => ErrorObject::owned(-32005, "limit exceeded", None::<()>),
            Self::Server => ErrorObject::owned(-32099, "server error", None::<()>),
            Self::Contract(e) => {
                ErrorObject::owned(-32099, format!("contract error: {}", e), None::<()>)
            }
        }
    }
}

/// Reasons for transaction rejection
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Rejection {
    /// Transaction fee is too high
    #[error("transaction fee too high")]
    FeeTooHigh,
}

/// Invalid parameter errors that can occur during request processing
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidParamsError {
    /// Signature validation failed
    #[error("bad signature")]
    BadSignature,
    /// Transaction nonce is lower than current nonce
    #[error("nonce too low")]
    NonceTooLow,
    /// Invalid hex string format
    #[error("invalid hex")]
    InvalidHex,
    /// Parameters must be provided as an array
    #[error("params must be an array")]
    NotAnArray,
    /// Incorrect number of parameters provided
    #[error("wrong number of params")]
    WrongParamCount(usize),
    /// Required parameter is missing
    #[error("missing param")]
    MissingParam,
    /// String is not hex encoded
    #[error("not a hex encoded string")]
    NotHexEncoded,
}

/// Invalid input errors that can occur during request processing
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidInputError {
    /// Invalid JSON format
    #[error("invalid JSON")]
    InvalidJson,
    /// Invalid unsigned integer value
    #[error("invalid uint")]
    InvalidUint,
    /// Transaction signature is invalid
    #[error("invalid transaction signature")]
    InvalidTransactionSignature,
    /// Failed to decode RLP data
    #[error("unable to RLP decode")]
    UnableToRLPDecode,
    /// Chain ID is missing
    #[error("missing chain ID")]
    MissingChainID,
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidJson)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(_: hex::FromHexError) -> Self {
        Self::InvalidParams(InvalidParamsError::InvalidHex)
    }
}

impl From<rlp::Error> for Error {
    fn from(_: rlp::Error) -> Self {
        Self::InvalidInput(InvalidInputError::UnableToRLPDecode)
    }
}

impl From<SignatureError> for Error {
    fn from(_: SignatureError) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidTransactionSignature)
    }
}

impl<T> From<ToUintError<T>> for Error {
    fn from(_: ToUintError<T>) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidUint)
    }
}

impl From<contract::Error> for Error {
    fn from(value: contract::Error) -> Self {
        Self::Contract(value)
    }
}

impl From<Infallible> for Error {
    fn from(_value: Infallible) -> Self {
        unreachable!("Cannot instantiate infallible")
    }
}
