use crate::application;
use crate::application::{Metrics, RunningStopwatch, Stopwatch};
use crate::domain::primitives::Bytes;
use crate::domain::MetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::json_rpc_errors::Error::InvalidParams;
use crate::presentation::json_rpc_errors::InvalidParamsError::{
    MissingParam, NotAnArray, NotHexEncoded, WrongParamCount,
};
use crate::presentation::server::Services;
use alloy::hex;
use alloy::hex::ToHexExt;
use jsonrpsee::types::{ErrorObject, Params};
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

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
        let code = Self::valid_eth_rpc_codes(&value);
        JsonRpcError {
            code,
            message: value.to_string(),
            data: None,
        }
    }
}

impl JsonRpcError<()> {
    fn valid_eth_rpc_codes(value: &Error) -> i32 {
        match value {
            Error::InvalidRequest => -32600,
            Error::MethodNotFound(_) => -32601,
            Error::InvalidParams(_) => -32602,
            Error::Internal => -32603,
            Error::Parse => -32700,
            Error::InvalidInput(_) => -32000,
            Error::ResourceNotFound => -32001,
            Error::ResourceUnavailable => -32002,
            Error::TransactionRejected(_) => -32003,
            Error::MethodNotSupported => -32004,
            Error::LimitExceeded => -32005,
            Error::Server | Error::Contract(_) => -32099,
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
pub async fn send_raw_transaction<Chain, M, S>(
    params: Params<'static>,
    ctx: Arc<Services<Chain, M, S>>,
    _ext: http::Extensions,
) -> Result<String, JsonRpcError<()>>
where
    Chain: MetabasedSequencerChainService,
    M: Metrics,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
    S: Stopwatch,
{
    let mut json: serde_json::Value = serde_json::from_str(params.as_str().unwrap())?;
    let arr = json.as_array_mut().ok_or(InvalidParams(NotAnArray))?;
    if arr.len() != 1 {
        return Err(InvalidParams(WrongParamCount(arr.len())).into());
    }
    let item = arr.pop().ok_or(InvalidParams(MissingParam))?; // should be impossible
    let str = item.as_str().ok_or(InvalidParams(NotHexEncoded))?;
    let bytes = hex::decode(str)?;
    let bytes = Bytes::from(bytes);
    let chain = ctx.chain_service();
    let metrics = ctx.metrics_service();
    let start = ctx.stopwatch_service().start();

    let result = application::send_raw_transaction(bytes, chain).await;

    metrics.append_send_raw_transaction_with_duration(start.elapsed());

    Ok(result?.encode_hex_with_prefix())
}

/// The JSON-RPC endpoint for Prometheus metrics scraper to collect data from.
pub fn metrics<Chain, M, S>(
    _params: Params,
    ctx: &Services<Chain, M, S>,
    _ext: &http::Extensions,
) -> String
where
    Chain: MetabasedSequencerChainService,
    M: Metrics,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
    S: Stopwatch,
{
    application::metrics(ctx.metrics_service())
}

/// The JSON-RPC endpoint for health check.
pub fn health<Chain, M, S>(
    _params: Params,
    _ctx: &Services<Chain, M, S>,
    _ext: &http::Extensions,
) -> Result<String, JsonRpcError<()>>
where
    Chain: MetabasedSequencerChainService,
    M: Metrics,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
    S: Stopwatch,
{
    Ok("ok".to_string())
}