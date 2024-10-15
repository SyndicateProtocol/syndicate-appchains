use crate::domain::primitives::Address;
use crate::domain::MetabasedSequencerChainService;
use crate::infrastructure::SolMetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::jsonrpc;
use alloy::hex;
use alloy::providers::ProviderBuilder;
use jsonrpsee::server::{RpcServiceBuilder, Server, ServerHandle};
use jsonrpsee::RpcModule;
use std::net::SocketAddr;
use url::Url;

pub async fn run(port: u16) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let rpc_middleware = RpcServiceBuilder::new();
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(format!("127.0.0.1:{}", port))
        .await?;
    let addr = server.local_addr()?;
    let address = Address::new(hex!("0000000000000000000000000000000000000000"));
    let rpc = ProviderBuilder::new().on_http(Url::parse("http://127.0.0.1").unwrap());
    let writer = SolMetabasedSequencerChainService::new(address, rpc);
    let services = Services { chain: writer };
    let module = create_eth_module(services)?;
    let handle = server.start(module);

    Ok((addr, handle))
}

fn create_eth_module<Chain>(services: Services<Chain>) -> anyhow::Result<RpcModule<Services<Chain>>>
where
    Chain: MetabasedSequencerChainService + Send + Sync + 'static,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
{
    let mut module = RpcModule::new(services);
    module.register_async_method("eth_sendRawTransaction", jsonrpc::send_raw_transaction)?;
    Ok(module)
}

#[derive(Debug)]
pub struct Services<Chain> {
    chain: Chain,
}

impl<Chain: MetabasedSequencerChainService> Services<Chain> {
    pub fn chain_service(&self) -> &Chain {
        &self.chain
    }
}
