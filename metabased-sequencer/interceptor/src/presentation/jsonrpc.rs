use crate::application;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::json_rpc_errors::Error::InvalidParams;
use alloy::hex;
use alloy::hex::ToHexExt;
use bytes::Bytes;
use jsonrpsee::types::{ErrorObject, Params};
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};

/// An error type for JSON-RPC endpoints.
///
/// It can be created from any error and converted into [`ErrorObject`]. This allows for the use of
/// `?` operator in the endpoint implementation and also its usage as an error return type. A
/// [`Result`] with this error is accepted by [`RpcModule::register_method`], assuming the [`Ok`]
/// type allows for [`IntoResponse`] implementation.
///
/// [`RpcModule::register_method`]: jsonrpsee::RpcModule::register_method
#[derive(Debug)]
pub struct JsonRpcError<S: Serialize> {
    pub code: i32,
    pub data: Option<S>,
    pub message: String,
}

impl<S: Serialize> Display for JsonRpcError<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl<E: Into<anyhow::Error>> From<E> for JsonRpcError<()> {
    fn from(value: E) -> Self {
        Self {
            code: -32602,
            message: value.into().to_string(),
            data: None,
        }
    }
}

impl<'error, S: Serialize> From<JsonRpcError<S>> for ErrorObject<'error> {
    fn from(value: JsonRpcError<S>) -> Self {
        ErrorObject::owned(value.code, value.message, value.data)
    }
}

impl From<Error> for JsonRpcError<()> {
    fn from(value: Error) -> Self {
        JsonRpcError {
            code: value.code(),
            message: value.to_string(),
            data: None,
        }
    }
}

/// The JSON-RPC endpoint for `eth_sendRawTransaction`.
///
/// # Parameters
/// Expects an array of a single element.
///
/// * Data: hex encoded string that contains signed and serialized transaction with an optional `0x`
///   prefix.
pub fn send_raw_transaction(
    params: Params,
    _ctx: &(),
    _ext: &http::Extensions,
) -> Result<String, JsonRpcError<()>> {
    let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap())?;
    let arr = json
        .as_array_mut()
        .ok_or(InvalidParams("unexpected parameter format".to_string()))?;
    if arr.len() != 1 {
        InvalidParams(format!("expected 1 parameter, got {}", arr.len()));
    }
    let item = arr
        .pop()
        .ok_or(InvalidParams("missing parameter".to_string()))?;
    let str = item
        .as_str()
        .ok_or(InvalidParams("Expected hex encoded string".to_string()))?;
    let bytes = hex::decode(str)?;
    let bytes = Bytes::from(bytes);

    Ok(application::send_raw_transaction(bytes)?.encode_hex_with_prefix())
}
