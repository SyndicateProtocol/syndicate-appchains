use crate::app;
use alloy::hex;
use alloy::hex::ToHexExt;
use anyhow::anyhow;
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
        .ok_or(anyhow!("Unexpected parameter format"))?;
    let item = arr.pop().ok_or(anyhow!("Missing parameter"))?;
    if !arr.is_empty() {
        Err(anyhow!("Expected 1 parameter, got {}", arr.len() + 1))?;
    }
    let str = item
        .as_str()
        .ok_or(anyhow!("Expected hex encoded string"))?;
    let bytes = hex::decode(str)?;
    let bytes = Bytes::from(bytes);

    Ok(app::send_raw_transaction(bytes)?.encode_hex_with_prefix())
}
