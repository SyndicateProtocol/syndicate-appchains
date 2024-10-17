use crate::application::{Metrics, PrometheusMetrics};
use crate::domain::primitives::Address;
use crate::domain::MetabasedSequencerChainService;
use crate::infrastructure::SolMetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::jsonrpc;
use alloy::providers::ProviderBuilder;
use jsonrpsee::server::{RpcServiceBuilder, Server, ServerHandle};
use jsonrpsee::RpcModule;
use std::net::SocketAddr;
use url::Url;

pub async fn run(
    port: u16,
    chain_contract_address: Address,
    chain_rpc_address: Url,
) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let rpc_middleware = RpcServiceBuilder::new();
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(format!("127.0.0.1:{port}"))
        .await?;
    let addr = server.local_addr()?;
    let rpc = ProviderBuilder::new().on_http(chain_rpc_address);
    let chain = SolMetabasedSequencerChainService::new(chain_contract_address, rpc);
    let metrics = PrometheusMetrics::new();
    let services = Services { chain, metrics };
    let module = create_eth_module(services)?;
    let handle = server.start(module);

    Ok((addr, handle))
}

fn create_eth_module<Chain, M>(
    services: Services<Chain, M>,
) -> anyhow::Result<RpcModule<Services<Chain, M>>>
where
    Chain: MetabasedSequencerChainService + Send + Sync + 'static,
    M: Metrics + Send + Sync + 'static,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
{
    let mut module = RpcModule::new(services);
    module.register_async_method("eth_sendRawTransaction", jsonrpc::send_raw_transaction)?;
    module.register_method("metrics", jsonrpc::metrics)?;
    Ok(module)
}

#[derive(Debug)]
pub struct Services<Chain, M> {
    chain: Chain,
    metrics: M,
}

impl<Chain: MetabasedSequencerChainService, M: Metrics> Services<Chain, M> {
    pub fn chain_service(&self) -> &Chain {
        &self.chain
    }

    pub fn metrics_service(&self) -> &M {
        &self.metrics
    }
}
