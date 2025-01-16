use crate::application::{Metrics, Stopwatch};
use crate::domain::MetabasedSequencerChainService;
use crate::infrastructure::{PrometheusMetrics, SolMetabasedSequencerChainService, TokioStopwatch};
use alloy::network::{Ethereum, EthereumWallet};
use alloy::providers::fillers::{
    CachedNonceManager, ChainIdFiller, FillProvider, GasFiller, NonceFiller, WalletFiller,
};
use alloy::providers::{ReqwestProvider, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{Address, B256};
use std::fmt::Debug;
use url::Url;

#[derive(Debug)]
pub struct Services<Chain, M, S>
where
    Chain: MetabasedSequencerChainService + Debug,
    M: Metrics + Debug,
    S: Stopwatch + Debug,
{
    chain: Chain,
    metrics: M,
    stopwatch: S,
}

impl<Chain, M, S> Services<Chain, M, S>
where
    Chain: MetabasedSequencerChainService + Debug,
    M: Metrics + Debug,
    S: Stopwatch + Debug,
{
    pub fn new(chain: Chain, metrics: M, stopwatch: S) -> Self {
        Self {
            chain,
            metrics,
            stopwatch,
        }
    }

    pub fn chain_service(&self) -> &Chain {
        &self.chain
    }

    pub fn metrics_service(&self) -> &M {
        &self.metrics
    }

    pub fn stopwatch_service(&self) -> &S {
        &self.stopwatch
    }
}

pub fn create(
    chain_contract_address: Address,
    chain_rpc_address: Url,
    private_key: B256,
) -> anyhow::Result<
    Services<
        impl MetabasedSequencerChainService<Error = alloy::contract::Error>
            + Send
            + Sync
            + Debug
            + 'static,
        impl Metrics + Send + Sync + Debug + 'static,
        impl Stopwatch<Running: Send + Sync + Debug + 'static> + Send + Sync + Debug + 'static,
    >,
> {
    let chain = create_chain_service(chain_contract_address, chain_rpc_address, private_key)?;
    let metrics = PrometheusMetrics::new();
    let stopwatch = TokioStopwatch;

    Ok(Services::new(chain, metrics, stopwatch))
}

fn create_chain_service(
    chain_contract_address: Address,
    chain_rpc_address: Url,
    private_key: B256,
) -> anyhow::Result<
    impl MetabasedSequencerChainService<Error = alloy::contract::Error> + Send + Sync + Debug + 'static,
> {
    // Fillers automatically set some attributes for every transaction sent using this provider.
    // See https://alloy.rs/building-with-alloy/understanding-fillers.html
    let signer = PrivateKeySigner::from_bytes(&private_key)?;
    let wallet = EthereumWallet::from(signer);
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
