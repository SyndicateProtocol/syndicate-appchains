// Source: https://github.com/MetaMask/rpc-errors/blob/main/src/errors.ts
#[derive(Debug, Clone, PartialEq)]
pub enum JsonRpcErrorCode {
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    ParseError = -32700,
    InvalidInput = -32000,
    ResourceNotFound = -32001,
    ResourceUnavailable = -32002,
    TransactionRejected = -32003,
    MethodNotSupported = -32004,
    LimitExceeded = -32005,
    ServerError = -32099, // We'll use this as the base for server errors
}

impl JsonRpcErrorCode {
    // TODO - bring back if needed
    // pub fn message(&self) -> &'static str {
    //     match self {
    //         Self::InvalidRequest => "Invalid request",
    //         Self::MethodNotFound => "Method not found",
    //         Self::InvalidParams => "Invalid params",
    //         Self::InternalError => "Internal error",
    //         Self::ParseError => "Parse error",
    //         Self::InvalidInput => "Invalid input",
    //         Self::ResourceNotFound => "Resource not found",
    //         Self::ResourceUnavailable => "Resource unavailable",
    //         Self::TransactionRejected => "Transaction rejected",
    //         Self::MethodNotSupported => "Method not supported",
    //         Self::LimitExceeded => "Limit exceeded",
    //         Self::ServerError => "Server error",
    //     }
    // }
}

impl From<i32> for JsonRpcErrorCode {
    fn from(value: i32) -> Self {
        match value {
            -32700 => Self::ParseError,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::InternalError,
            -32000 => Self::InvalidInput,
            -32001 => Self::ResourceNotFound,
            -32002 => Self::ResourceUnavailable,
            -32003 => Self::TransactionRejected,
            -32004 => Self::MethodNotSupported,
            -32005 => Self::LimitExceeded,
            _ if (-32099..=-32000).contains(&value) => Self::ServerError,
            _ => Self::InternalError, // Default case
        }
    }
}

impl JsonRpcErrorCode {
    pub fn code(&self) -> i32 {
        self.to_owned() as i32
    }
}
