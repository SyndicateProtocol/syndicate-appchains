use crate::{
    application::{Metrics, RunningStopwatch, Stopwatch},
    domain::{primitives::Address, MetabasedSequencerChainService},
    presentation::{
        json_rpc_errors::Error,
        jsonrpc,
        server::Endpoint::{Health, Metrics as MetricsEndpoint},
        services::{self, Services},
        tower::UnescapeJsonLayer,
    },
};
use alloy::primitives::B256;
use http::Method;
use jsonrpsee::{
    server::{middleware::http::ProxyGetRequestLayer, RpcServiceBuilder, Server, ServerHandle},
    RpcModule,
};
use std::{fmt::Debug, net::SocketAddr};
use tracing::info;
use url::Url;

pub async fn run(
    port: u16,
    chain_contract_address: Address,
    chain_rpc_address: Url,
    private_key: B256,
) -> eyre::Result<(SocketAddr, ServerHandle)> {
    let rpc_middleware = RpcServiceBuilder::new();
    let http_middleware = tower::ServiceBuilder::new()
        .layer(ProxyGetRequestLayer::new(Health.http_path(), Health.rpc_method())?)
        .layer(UnescapeJsonLayer::new(|request| {
            request.uri() == MetricsEndpoint.http_path() && request.method() == Method::GET
        }))
        .layer(ProxyGetRequestLayer::new(
            MetricsEndpoint.http_path(),
            MetricsEndpoint.rpc_method(),
        )?);

    let server = Server::builder()
        .set_http_middleware(http_middleware)
        .set_rpc_middleware(rpc_middleware)
        .build(format!("127.0.0.1:{port}"))
        .await?;

    let services = services::create(chain_contract_address, chain_rpc_address, private_key)?;
    let module = create_eth_module(services)?;

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {
    Metrics,
    Health,
}

impl Endpoint {
    pub const fn http_path(&self) -> &'static str {
        match self {
            Self::Metrics => "/metrics",
            Health => "/health",
        }
    }

    pub const fn rpc_method(&self) -> &'static str {
        match self {
            Self::Metrics => "metrics",
            Health => "health",
        }
    }
}

fn create_eth_module<Chain, M, S>(
    services: Services<Chain, M, S>,
) -> eyre::Result<RpcModule<Services<Chain, M, S>>>
where
    Chain: MetabasedSequencerChainService + Send + Sync + Debug + 'static,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
    M: Metrics + Send + Sync + Debug + 'static,
    S: Stopwatch<Running: RunningStopwatch + Send + Sync + Debug + 'static>
        + Send
        + Sync
        + Debug
        + 'static,
{
    let mut module = RpcModule::new(services);
    module.register_async_method("eth_sendRawTransaction", jsonrpc::send_raw_transaction)?;
    module.register_method(MetricsEndpoint.rpc_method(), jsonrpc::metrics)?;
    module.register_method(Health.rpc_method(), |_, _, _| serde_json::json!({ "health": true }))?;

    info!("Registered RPC methods: {:#?}", module.method_names().collect::<Vec<_>>());
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Dummy objects are passed around but never actually used. Usually they are just used to fill
    /// parameter lists.
    mod dummy {
        use super::*;
        use alloy::primitives::{Bytes, TxHash};
        use async_trait::async_trait;
        use std::{convert::Infallible, time::Duration};

        #[async_trait]
        impl MetabasedSequencerChainService for () {
            type Error = Infallible;

            async fn process_transaction(&self, _tx: Bytes) -> Result<TxHash, Self::Error> {
                unimplemented!("Dummy implementation not expected to be invoked")
            }

            async fn process_bulk_transactions(
                &self,
                _tx: Vec<Bytes>,
            ) -> Result<TxHash, Self::Error> {
                unimplemented!("Dummy implementation not expected to be invoked")
            }
        }

        impl Stopwatch for () {
            type Running = ();

            fn start(&self) -> Self::Running {}
        }

        impl RunningStopwatch for () {
            fn elapsed(&self) -> Duration {
                Duration::from_secs(1)
            }
        }
    }

    /// Stubs provide canned answers to calls made during the test, usually not responding at all to
    /// anything outside what's programmed in for the test.
    mod stub {
        use crate::{application::Metrics, presentation::json_rpc_errors::Error};
        use std::{fmt::Write, time::Duration};

        impl Metrics for &'static str {
            fn append_send_raw_transaction_with_duration(
                &self,
                _duration: Duration,
                _error: Option<&Error>,
            ) {
                unimplemented!("Unexpected call")
            }

            fn encode(&self, _writer: &mut impl Write) -> std::fmt::Result {
                unimplemented!("Unexpected call, should use `Display` implementation")
            }
        }
    }

    #[tokio::test]
    async fn test_metrics_endpoint_encodes_collected_metrics_successfully() {
        let metrics = "test data";
        let services = Services::new((), metrics, ());
        let rpc_module = create_eth_module(services);

        let expected_response = metrics;
        let actual_response = rpc_module
            .unwrap()
            .call::<[(); 0], String>(MetricsEndpoint.rpc_method(), [])
            .await
            .expect("Metrics RPC invocation should not fail");

        assert_eq!(actual_response, expected_response)
    }

    #[tokio::test]
    async fn test_metrics_endpoint_ignores_params() {
        let metrics = "test data";
        let services = Services::new((), metrics, ());
        let rpc_module = create_eth_module(services);

        let expected_response = metrics;
        let actual_response = rpc_module
            .unwrap()
            .call::<[i32; 1], String>(MetricsEndpoint.rpc_method(), [0])
            .await
            .expect("Metrics RPC invocation should not fail");

        assert_eq!(actual_response, expected_response)
    }
}
