//! The `errors` module contains the error types for the metabased sequencer.
// TODO [SEQ-662]: Simplify error file

use alloy::{
    contract, hex,
    primitives::{ruint::ToUintError, SignatureError},
    rlp,
};
use jsonrpsee::types::{error, ErrorObject, ErrorObjectOwned};
use std::{convert::Infallible, fmt};

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    /// Transaction fee is too high
    FeeTooHigh,
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

impl fmt::Display for InvalidParamsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadSignature => write!(f, "bad signature"),
            Self::NonceTooLow => write!(f, "nonce too low"),
            Self::InvalidHex => write!(f, "invalid hex"),
            Self::NotAnArray => write!(f, "params must be an array"),
            Self::WrongParamCount(_) => write!(f, "wrong number of params"),
            Self::MissingParam => write!(f, "missing param"),
            Self::NotHexEncoded => write!(f, "not a hex encoded string"),
        }
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson => write!(f, "invalid JSON"),
            Self::InvalidUint => write!(f, "invalid uint"),
            Self::InvalidTransactionSignature => {
                write!(f, "invalid transaction signature")
            }
            Self::UnableToRLPDecode => write!(f, "unable to RLP decode"),
            Self::MissingChainID => {
                write!(f, "missing chain ID")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest => write!(f, "invalid request",),
            Self::MethodNotFound(m) => write!(f, "method not found: {}", m),
            Self::InvalidParams(m) => write!(f, "invalid params: {}", m),
            Self::Internal => write!(f, "internal error"),
            Self::Parse => write!(f, "parse error"),
            Self::InvalidInput(m) => write!(f, "invalid input: {}", m),
            Self::ResourceNotFound => write!(f, "resource not found",),
            Self::ResourceUnavailable => write!(f, "resource unavailable",),
            Self::TransactionRejected(m) => write!(f, "transaction rejected: {}", m),
            Self::MethodNotSupported => write!(f, "method not supported"),
            Self::LimitExceeded => write!(f, "limit exceeded"),
            Self::Server => write!(f, "server error"),
            Self::Contract(error) => error.fmt(f),
        }
    }
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FeeTooHigh => write!(f, "transaction fee too high"),
        }
    }
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
