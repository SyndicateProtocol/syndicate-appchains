use crate::application::{Metrics, PrometheusMetrics};
use crate::domain::primitives::Address;
use crate::domain::MetabasedSequencerChainService;
use crate::infrastructure::SolMetabasedSequencerChainService;
use crate::presentation::json_rpc_errors::Error;
use crate::presentation::jsonrpc;
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::B256;
use alloy::providers::fillers::{
    CachedNonceManager, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{ReqwestProvider, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use jsonrpsee::server::{RpcServiceBuilder, Server, ServerHandle};
use jsonrpsee::RpcModule;
use std::net::SocketAddr;
use url::Url;

/// Combines `n` [filler]s into one [filler].
///
/// It achieves this by nesting [`JoinFill`] providers into a binary tree structure. In this tree,
/// every node with 2 children is a [`JoinFill`] provider while every leaf node is a provider from
/// the given arguments.
///
/// [filler]: alloy::providers::fillers::TxFiller
macro_rules! join_fill {
    ($x:expr, $($y:tt)+) => {
        JoinFill::new($x, join_fill!($($y)+))
    };
    ($x:expr $(,)?) => {
        $x
    };
}

pub async fn run(
    port: u16,
    chain_contract_address: Address,
    chain_rpc_address: Url,
    private_key: B256,
) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let chain = create_chain_service(chain_contract_address, chain_rpc_address, private_key)?;
    let metrics = PrometheusMetrics::new();
    let services = Services::new(chain, metrics);

    let rpc_middleware = RpcServiceBuilder::new();
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(format!("127.0.0.1:{port}"))
        .await?;

    let addr = server.local_addr()?;
    let module = create_eth_module(services)?;
    let handle = server.start(module);

    Ok((addr, handle))
}

fn create_chain_service(
    chain_contract_address: Address,
    chain_rpc_address: Url,
    private_key: B256,
) -> anyhow::Result<
    impl MetabasedSequencerChainService<Error = alloy::contract::Error> + Send + Sync + 'static,
> {
    let signer = PrivateKeySigner::from_bytes(&private_key)?;
    let wallet = EthereumWallet::from(signer);
    // Fillers automatically set some attributes for every transaction sent using this provider.
    // See https://alloy.rs/building-with-alloy/understanding-fillers.html
    let filler = join_fill!(
        NonceFiller::new(CachedNonceManager::default()),
        WalletFiller::new(wallet),
        GasFiller,
        ChainIdFiller::new(None),
    );
    let rpc: RootProvider<_, Ethereum> = ReqwestProvider::new_http(chain_rpc_address);
    let rpc = FillProvider::new(rpc, filler);

    Ok(SolMetabasedSequencerChainService::new(
        chain_contract_address,
        rpc,
    ))
}

fn create_eth_module<Chain, M>(
    services: Services<Chain, M>,
) -> anyhow::Result<RpcModule<Services<Chain, M>>>
where
    Chain: MetabasedSequencerChainService + Send + Sync + 'static,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
    M: Metrics + Send + Sync + 'static,
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
    pub fn new(chain: Chain, metrics: M) -> Self {
        Self { chain, metrics }
    }

    pub fn chain_service(&self) -> &Chain {
        &self.chain
    }

    pub fn metrics_service(&self) -> &M {
        &self.metrics
    }
}
