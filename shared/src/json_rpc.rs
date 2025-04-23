//! The `json_rpc` module contains functionality relating to JSON-RPC request handling

use crate::json_rpc::{
    InvalidParamsError::{InvalidHex, MissingParam, NotAnArray, NotHexEncoded, WrongParamCount},
    RpcError::InvalidParams,
};
use alloy::{
    contract, hex,
    primitives::{ruint::ToUintError, Bytes, ChainId, SignatureError},
    rlp,
};
use jsonrpsee::types::{error, ErrorObject, ErrorObjectOwned, Params};
use std::convert::Infallible;
use thiserror::Error;

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
/// Primary error type for the metabased sequencer, following JSON-RPC error code mapping
#[derive(Debug, Error)]
pub enum RpcError {
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
    #[error("internal error: {0}")]
    Internal(String),
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

/// Parses the input parameters for sending a raw transaction.
///
/// This function converts the provided JSON-RPC request [`Params`] into a byte array
/// representation, used for submitting an Ethereum raw transaction. If the conversion fails, it
/// returns an `Error`.
pub fn parse_send_raw_transaction_params(params: Params<'static>) -> Result<Bytes, RpcError> {
    Bytes::try_from(ParamsWrapper::from(params))
}

/// Required to implement [`TryFrom`] trait below
#[derive(Debug)]
pub struct ParamsWrapper(Params<'static>);

impl From<Params<'static>> for ParamsWrapper {
    fn from(params: Params<'static>) -> Self {
        Self(params)
    }
}

impl TryFrom<ParamsWrapper> for Bytes {
    type Error = RpcError;

    fn try_from(wrapper: ParamsWrapper) -> Result<Self, Self::Error> {
        let ParamsWrapper(params) = wrapper;

        let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap_or("[]"))?;
        let arr = json.as_array_mut().ok_or(InvalidParams(NotAnArray))?;
        if arr.len() != 1 {
            return Err(InvalidParams(WrongParamCount(arr.len())));
        }
        let item = arr.pop().ok_or(InvalidParams(MissingParam))?;
        let raw_tx = item.as_str().ok_or(InvalidParams(NotHexEncoded))?.to_string();
        let tx_data = hex::decode(&raw_tx).map(Self::from)?;

        Ok(tx_data)
    }
}

impl From<RpcError> for ErrorObjectOwned {
    fn from(error: RpcError) -> Self {
        match error {
            RpcError::InvalidRequest => {
                ErrorObject::owned(error::INVALID_REQUEST_CODE, "invalid request", None::<()>)
            }
            RpcError::MethodNotFound(m) => ErrorObject::owned(
                error::METHOD_NOT_FOUND_CODE,
                format!("method not found: {}", m),
                None::<()>,
            ),
            InvalidParams(m) => ErrorObject::owned(
                error::INVALID_PARAMS_CODE,
                format!("invalid params: {}", m),
                None::<()>,
            ),
            RpcError::Internal(m) => ErrorObject::owned(
                error::INTERNAL_ERROR_CODE,
                format!("internal error: {}", m),
                None::<()>,
            ),
            RpcError::Parse => {
                ErrorObject::owned(error::PARSE_ERROR_CODE, "parse error", None::<()>)
            }
            RpcError::InvalidInput(m) => ErrorObject::owned(
                error::CALL_EXECUTION_FAILED_CODE,
                format!("invalid input: {}", m),
                None::<()>,
            ),
            RpcError::ResourceNotFound => {
                ErrorObject::owned(error::UNKNOWN_ERROR_CODE, "resource not found", None::<()>)
            }
            RpcError::ResourceUnavailable => {
                ErrorObject::owned(error::INVALID_REQUEST_CODE, "resource unavailable", None::<()>)
            }
            RpcError::TransactionRejected(m) => {
                ErrorObject::owned(-32003, format!("transaction rejected: {}", m), None::<()>)
            }
            RpcError::MethodNotSupported => {
                ErrorObject::owned(-32004, "method not supported", None::<()>)
            }
            RpcError::LimitExceeded => ErrorObject::owned(-32005, "limit exceeded", None::<()>),
            RpcError::Server => ErrorObject::owned(-32099, "server error", None::<()>),
            RpcError::Contract(e) => {
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
    ChainIdMissing,
    /// Chain ID is missing
    #[error("chain ID mismatch: expected {0} got {1}")]
    ChainIdMismatched(String, String),
    /// Unsupported Chain ID
    #[error("unsupported chain ID: {0}")]
    UnsupportedChainId(ChainId),
    /// Transaction too large
    #[error("transaction too large: limit {0} - got {1}")]
    TransactionTooLarge(String, String),
}

impl From<serde_json::Error> for RpcError {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidJson)
    }
}

impl From<hex::FromHexError> for RpcError {
    fn from(_: hex::FromHexError) -> Self {
        InvalidParams(InvalidHex)
    }
}

impl From<rlp::Error> for RpcError {
    fn from(_: rlp::Error) -> Self {
        Self::InvalidInput(InvalidInputError::UnableToRLPDecode)
    }
}

impl From<SignatureError> for RpcError {
    fn from(_: SignatureError) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidTransactionSignature)
    }
}

impl<T> From<ToUintError<T>> for RpcError {
    fn from(_: ToUintError<T>) -> Self {
        Self::InvalidInput(InvalidInputError::InvalidUint)
    }
}

impl From<contract::Error> for RpcError {
    fn from(value: contract::Error) -> Self {
        Self::Contract(value)
    }
}

impl From<Infallible> for RpcError {
    fn from(_value: Infallible) -> Self {
        unreachable!("Cannot instantiate infallible")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonrpsee::tokio;

    #[tokio::test]
    async fn test_send_raw_transaction_handler_invalid_params() {
        let invalid_params = Params::new(Some("[\"invalid_hex\"]"));

        let result = parse_send_raw_transaction_params(invalid_params);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_raw_transaction_handler_valid_params() {
        // Valid raw transaction hex
        let valid_tx = "[\"0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772\"]";
        let params = Params::new(Some(valid_tx));

        let result = parse_send_raw_transaction_params(params);

        assert!(result.is_ok());
    }
}
