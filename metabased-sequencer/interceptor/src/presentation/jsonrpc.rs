use crate::application;
use crate::application::{Metrics, RunningStopwatch, SendRawTransactionParams, Stopwatch};
use crate::domain::primitives::Bytes;
use crate::domain::MetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::services::Services;
use alloy::hex;
use alloy::hex::ToHexExt;
use jsonrpsee::types::{ErrorObject, Params};
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
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
    let metrics = ctx.metrics_service();
    let start = ctx.stopwatch_service().start();
    let chain = ctx.chain_service();

    with_metrics(
        metrics,
        start,
        handle_send_raw_transaction(params, chain)
    ).await
}

pub async fn handle_send_raw_transaction<Chain>(
    params: Params<'static>,
    chain: &Chain,
) -> Result<String, Error>
where
    Chain: MetabasedSequencerChainService,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
{
    let params = SendRawTransactionParams::try_from(params)?;
    let bytes = hex::decode(&params.raw_tx)
        .map(Bytes::from)?;

    let tx_hash = application::send_raw_transaction(bytes, chain).await?;
    Ok(tx_hash.encode_hex_with_prefix())
}

// Capture whole content of function `f` for metric
async fn with_metrics<T>(
    metrics: &impl Metrics,
    start: impl RunningStopwatch,
    f: impl Future<Output = Result<T, Error>>,
) -> Result<T, JsonRpcError<()>> {
    let result = f.await;

    metrics.append_send_raw_transaction_with_duration(
        start.elapsed(),
        result.as_ref().err(),
    );

    result.map_err(JsonRpcError::from)
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
    use crate::infrastructure::error_to_static_str;
    use crate::presentation::services::Services;
    use alloy_primitives::Bytes;
    use async_trait::async_trait;
    use jsonrpsee::types::Params;
    use std::convert::Infallible;
    use std::time::Duration;

    #[tokio::test]
    async fn test_send_raw_transaction_success() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        let services_arc = Arc::new(services);
        // an arbitrary (valid) tx
        let params = Params::new(Some(
            r#"["0x02f871018319cb1d808502b95ddeef82520894e94f1fa4f27d9d288ffea234bb62e1fbc086ca0c877654752ccd929080c001a0fd107a1713c5b89e4affcf616b2bdc517a70ce9735c4d67d142fd9211f2c6d8ea032fac076f33f22c968380c02331be61da3f157f90e72a121d5fac80313745779"]"#,
        ));
        let result = send_raw_transaction(params, services_arc.clone(), Default::default())
            .await
            .unwrap();

        assert_eq!(
            result,
            "0x1111111111111111111111111111111111111111111111111111111111111111"
        );

        assert!(services_arc.metrics_service().metrics_called.get());
        assert_eq!(services_arc.metrics_service().last_error.get(), "none");
    }

    #[tokio::test]
    async fn test_send_raw_transaction_invalid_number_of_params() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        // an arbitrary (valid) tx, and an extra param
        let params = Params::new(Some(
            r#"["0x02f871018319cb1d808502b95ddeef82520894e94f1fa4f27d9d288ffea234bb62e1fbc086ca0c877654752ccd929080c001a0fd107a1713c5b89e4affcf616b2bdc517a70ce9735c4d67d142fd9211f2c6d8ea032fac076f33f22c968380c02331be61da3f157f90e72a121d5fac80313745779", "1"]"#,
        ));
        let services_arc = Arc::new(services);
        let err = send_raw_transaction(params, services_arc.clone(), Default::default())
            .await
            .unwrap_err();

        assert!(err
            .to_string()
            .contains("invalid params: wrong number of params"));

        assert!(services_arc.metrics_service().metrics_called.get());
        assert_eq!(services_arc.metrics_service().last_error.get(), "invalid_params.wrong_count");
    }

    #[tokio::test]
    async fn test_send_raw_transaction_invalid_payload() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        let params = Params::new(Some(r#"["0x not hex"]"#));

        let services_arc = Arc::new(services);
        let _err = send_raw_transaction(params, services_arc.clone(), Default::default())
            .await
            .unwrap_err();

        assert!(services_arc.metrics_service().metrics_called.get());
        assert_eq!(services_arc.metrics_service().last_error.get(), "invalid_params.invalid_hex");
    }

    #[tokio::test]
    async fn test_send_raw_transaction_invalid_tx() {
        let services = Services::new(MockChain, MockMetrics::new(), ());
        let params = Params::new(Some(r#"["0xdeadbeef"]"#));

        let services_arc = Arc::new(services);
        let err = send_raw_transaction(params, services_arc.clone(), Default::default())
            .await
            .unwrap_err();

        assert!(err
            .to_string()
            .contains("invalid input: unable to RLP decode"));


        assert!(services_arc.metrics_service().metrics_called.get());
        assert_eq!(services_arc.metrics_service().last_error.get(), "invalid_input.rlp_decode_error");
    }

    // handler tests
    #[tokio::test]
    async fn test_handle_valid_transaction() {
        let chain = MockChain;
        let params = Params::new(Some(r#"["0x02f871018319cb1d808502b95ddeef82520894e94f1fa4f27d9d288ffea234bb62e1fbc086ca0c877654752ccd929080c001a0fd107a1713c5b89e4affcf616b2bdc517a70ce9735c4d67d142fd9211f2c6d8ea032fac076f33f22c968380c02331be61da3f157f90e72a121d5fac80313745779"]"#));

        let result = handle_send_raw_transaction(params, &chain).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "0x1111111111111111111111111111111111111111111111111111111111111111"
        );
    }

    #[tokio::test]
    async fn test_handle_invalid_hex() {
        let chain = MockChain;
        let params = Params::new(Some(r#"["not hex"]"#));

        let result = handle_send_raw_transaction(params, &chain).await;
        assert!(result.is_err());
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

    // To appease Clippy
    unsafe impl Send for MockMetrics {}
    unsafe impl Sync for MockMetrics {}

    struct MockMetrics {
        metrics_called: std::cell::Cell<bool>,
        last_error: std::cell::Cell<&'static str>,
    }

    impl MockMetrics {
        fn new() -> Self {
            // Init to zero vals
            Self {
                metrics_called: std::cell::Cell::new(false),
                last_error: std::cell::Cell::new(""),
            }
        }
    }

    impl Display for MockMetrics {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MockMetrics")
        }
    }

    impl Metrics for MockMetrics {
        fn append_send_raw_transaction_with_duration(&self, _duration: Duration, error: Option<&Error>) {
            self.metrics_called.set(true);
            self.last_error.set(error_to_static_str(error));
        }

        fn encode(&self, _writer: &mut impl std::fmt::Write) -> std::fmt::Result {
            Ok(())
        }
    }
}
