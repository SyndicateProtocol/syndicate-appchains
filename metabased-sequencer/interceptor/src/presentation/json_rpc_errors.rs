use crate::presentation::transaction;
use alloy::hex;
use std::fmt;

// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidRequest(String),
    MethodNotFound(String),
    InvalidParams(String),
    Internal,
    Parse,
    InvalidInput(String),
    ResourceNotFound(String),
    ResourceUnavailable(String),
    TransactionRejected(String),
    MethodNotSupported(String),
    LimitExceeded,
    Server,
}

impl From<Error> for i32 {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidRequest(_) => -32600,
            Error::MethodNotFound(_) => -32601,
            Error::InvalidParams(_) => -32602,
            Error::Internal => -32603,
            Error::Parse => -32700,
            Error::InvalidInput(_) => -32000,
            Error::ResourceNotFound(_) => -32001,
            Error::ResourceUnavailable(_) => -32002,
            Error::TransactionRejected(_) => -32003,
            Error::MethodNotSupported(_) => -32004,
            Error::LimitExceeded => -32005,
            Error::Server => -32099,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidRequest(m) => write!(f, "Invalid request {}", m),
            Error::MethodNotFound(m) => write!(f, "Method not found: {}", m),
            Error::InvalidParams(m) => write!(f, "Invalid params: {}", m),
            Error::Internal => write!(f, "Internal error"),
            Error::Parse => write!(f, "Parse error"),
            Error::InvalidInput(m) => write!(f, "Invalid input: {}", m),
            Error::ResourceNotFound(m) => write!(f, "Resource not found: {}", m),
            Error::ResourceUnavailable(m) => write!(f, "Resource unavailable: {}", m),
            Error::TransactionRejected(m) => write!(f, "Transaction rejected: {}", m),
            Error::MethodNotSupported(m) => write!(f, "Method not supported: {}", m),
            Error::LimitExceeded => write!(f, "Limit exceeded"),
            Error::Server => write!(f, "Server error"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InvalidInput("Invalid JSON".to_string())
    }
}

impl From<hex::FromHexError> for Error {
    fn from(_: hex::FromHexError) -> Self {
        Error::InvalidParams("Invalid hex".to_string())
    }
}

impl From<alloy_primitives::private::alloy_rlp::Error> for Error {
    fn from(e: alloy_primitives::private::alloy_rlp::Error) -> Self {
        Error::InvalidInput(e.to_string())
    }
}

impl From<alloy_primitives::SignatureError> for Error {
    fn from(_: alloy_primitives::SignatureError) -> Self {
        Error::InvalidInput("Invalid transaction signature".to_string())
    }
}

impl From<transaction::CheckTxFeeError> for Error {
    fn from(e: transaction::CheckTxFeeError) -> Self {
        Error::TransactionRejected(e.to_string())
    }
}

impl<T> From<alloy_primitives::ruint::ToUintError<T>> for Error {
    fn from(_: alloy_primitives::ruint::ToUintError<T>) -> Self {
        Error::InvalidInput("Invalid uint".to_string())
    }
}

impl Error {
    pub fn code(&self) -> i32 {
        self.clone().into()
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}
