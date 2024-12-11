use crate::presentation::json_rpc_errors::InvalidInputError::{
    InvalidJson, InvalidTransactionSignature, InvalidUint, MissingGasPrice, UnableToRLPDecode, MissingChainID
};
use crate::presentation::json_rpc_errors::InvalidParamsError::InvalidHex;
use crate::presentation::json_rpc_errors::Rejection::FeeTooHigh;
use crate::presentation::transaction;
use alloy::hex;
use alloy_primitives::private::alloy_rlp;
use std::convert::Infallible;
use std::fmt;

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
#[derive(Debug)]
pub enum Error {
    // Parent errors with a JSON-RPC error code mapping
    InvalidRequest,
    MethodNotFound(String),
    InvalidParams(InvalidParamsError),
    Internal,
    Parse,
    InvalidInput(InvalidInputError),
    ResourceNotFound,
    ResourceUnavailable,
    TransactionRejected(Rejection),
    MethodNotSupported,
    LimitExceeded,
    Server,
    Contract(alloy::contract::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rejection {
    FeeTooHigh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidParamsError {
    BadSignature,
    NonceTooLow,
    InvalidHex,
    NotAnArray,
    WrongParamCount(usize),
    MissingParam,
    NotHexEncoded,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidInputError {
    InvalidJson,
    InvalidUint,
    InvalidTransactionSignature,
    MissingGasPrice,
    UnableToRLPDecode,
    MissingChainID
}

impl fmt::Display for InvalidParamsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidParamsError::BadSignature => write!(f, "bad signature"),
            InvalidParamsError::NonceTooLow => write!(f, "nonce too low"),
            InvalidHex => write!(f, "invalid hex"),
            InvalidParamsError::NotAnArray => write!(f, "params must be an array"),
            InvalidParamsError::WrongParamCount(_) => write!(f, "wrong number of params"),
            InvalidParamsError::MissingParam => write!(f, "missing param"),
            InvalidParamsError::NotHexEncoded => write!(f, "not a hex encoded string"),
        }
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidJson => write!(f, "invalid JSON"),
            InvalidUint => write!(f, "invalid uint"),
            InvalidTransactionSignature => write!(f, "invalid transaction signature"),
            MissingGasPrice => write!(f, "transaction missing gas price"),
            UnableToRLPDecode => write!(f, "unable to RLP decode"),
            MissingChainID => {write!(f, "missing chain ID")}
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidRequest => write!(f, "invalid request",),
            Error::MethodNotFound(m) => write!(f, "method not found: {}", m),
            Error::InvalidParams(m) => write!(f, "invalid params: {}", m),
            Error::Internal => write!(f, "internal error"),
            Error::Parse => write!(f, "parse error"),
            Error::InvalidInput(m) => write!(f, "invalid input: {}", m),
            Error::ResourceNotFound => write!(f, "resource not found",),
            Error::ResourceUnavailable => write!(f, "resource unavailable",),
            Error::TransactionRejected(m) => write!(f, "transaction rejected: {}", m),
            Error::MethodNotSupported => write!(f, "method not supported"),
            Error::LimitExceeded => write!(f, "limit exceeded"),
            Error::Server => write!(f, "server error"),
            Error::Contract(error) => error.fmt(f),
        }
    }
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeeTooHigh => write!(f, "transaction fee too high"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InvalidInput(InvalidJson)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(_: hex::FromHexError) -> Self {
        Error::InvalidParams(InvalidHex)
    }
}

impl From<alloy_rlp::Error> for Error {
    fn from(_: alloy_rlp::Error) -> Self {
        Error::InvalidInput(UnableToRLPDecode)
    }
}

impl From<alloy_primitives::SignatureError> for Error {
    fn from(_: alloy_primitives::SignatureError) -> Self {
        Error::InvalidInput(InvalidTransactionSignature)
    }
}

impl From<transaction::TransactionFeeTooHigh> for Error {
    fn from(_: transaction::TransactionFeeTooHigh) -> Self {
        Error::TransactionRejected(FeeTooHigh)
    }
}

impl<T> From<alloy_primitives::ruint::ToUintError<T>> for Error {
    fn from(_: alloy_primitives::ruint::ToUintError<T>) -> Self {
        Error::InvalidInput(InvalidUint)
    }
}

impl From<alloy::contract::Error> for Error {
    fn from(value: alloy::contract::Error) -> Self {
        Self::Contract(value)
    }
}

impl From<Infallible> for Error {
    fn from(_value: Infallible) -> Self {
        unreachable!("Cannot instantiate infallible")
    }
}
