use crate::application;
use crate::application::{Metrics, RunningStopwatch, Stopwatch};
use crate::domain::primitives::Bytes;
use crate::domain::MetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::json_rpc_errors::Error::InvalidParams;
use crate::presentation::json_rpc_errors::InvalidParamsError::{
    MissingParam, NotAnArray, NotHexEncoded, WrongParamCount,
};
use crate::presentation::services::Services;
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

    // TODO (SEQ-352): differentiate on error
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::Metrics;
    use crate::domain::primitives::TxHash;
    use crate::domain::MetabasedSequencerChainService;
    use crate::presentation::services::Services;
    use alloy_primitives::Bytes;
    use async_trait::async_trait;
    use jsonrpsee::types::Params;
    use std::convert::Infallible;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[tokio::test]
    async fn test_send_raw_transaction_success() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        // an arbitrary (valid) tx
        let params = Params::new(Some(
            r#"["0x02f871018319cb1d808502b95ddeef82520894e94f1fa4f27d9d288ffea234bb62e1fbc086ca0c877654752ccd929080c001a0fd107a1713c5b89e4affcf616b2bdc517a70ce9735c4d67d142fd9211f2c6d8ea032fac076f33f22c968380c02331be61da3f157f90e72a121d5fac80313745779"]"#,
        ));
        let result = send_raw_transaction(params, Arc::new(services), Default::default())
            .await
            .unwrap();

        assert_eq!(
            result,
            "0x1111111111111111111111111111111111111111111111111111111111111111"
        );
    }

    #[tokio::test]
    async fn test_send_raw_transaction_invalid_number_of_params() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        // an arbitrary (valid) tx, and an extra param
        let params = Params::new(Some(
            r#"["0x02f871018319cb1d808502b95ddeef82520894e94f1fa4f27d9d288ffea234bb62e1fbc086ca0c877654752ccd929080c001a0fd107a1713c5b89e4affcf616b2bdc517a70ce9735c4d67d142fd9211f2c6d8ea032fac076f33f22c968380c02331be61da3f157f90e72a121d5fac80313745779", "1"]"#,
        ));
        let err = send_raw_transaction(params, Arc::new(services), Default::default())
            .await
            .unwrap_err();

        assert!(err
            .to_string()
            .contains("invalid params: wrong number of params"));
    }

    #[tokio::test]
    async fn test_send_raw_transaction_invalid_payload() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        let params = Params::new(Some(r#"["0x not hex"]"#));

        let err = send_raw_transaction(params, Arc::new(services), Default::default())
            .await
            .unwrap_err();

        assert!(err.to_string().contains("invalid character"));
    }

    #[tokio::test]
    async fn test_send_raw_transaction_invalid_tx() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        let params = Params::new(Some(r#"["0xdeadbeef"]"#));

        let err = send_raw_transaction(params, Arc::new(services), Default::default())
            .await
            .unwrap_err();

        assert!(err
            .to_string()
            .contains("invalid input: unable to RLP decode"));
    }

    #[derive(Default)]
    struct MockChain;

    #[async_trait]
    impl MetabasedSequencerChainService for MockChain {
        type Error = Infallible;

        async fn process_transaction(&self, _tx: Bytes) -> Result<TxHash, Self::Error> {
            Ok(TxHash::repeat_byte(0x11))
        }

        async fn process_bulk_transactions(&self, _tx: Vec<Bytes>) -> Result<TxHash, Self::Error> {
            unimplemented!()
        }
    }

    struct MockMetrics;

    impl MockMetrics {
        fn new() -> Self {
            METRICS_CALL_COUNTER.store(0, Ordering::Relaxed);
            Self
        }
    }

    impl Display for MockMetrics {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MockMetrics")
        }
    }

    
    static METRICS_CALL_COUNTER: AtomicUsize = AtomicUsize::new(0);

    // TODO (SEQ-352): make this deterministic
    impl Metrics for MockMetrics {
        fn append_send_raw_transaction_with_duration(&self, _duration: Duration) {
            METRICS_CALL_COUNTER.fetch_add(1, Ordering::Relaxed);
        }
        fn encode(&self, _writer: &mut impl std::fmt::Write) -> std::fmt::Result {
            Ok(())
        }
    }
}
