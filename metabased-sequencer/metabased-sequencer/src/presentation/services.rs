use crate::{
    application::{Metrics, Stopwatch},
    domain::MetabasedSequencerChainService,
    infrastructure::{PrometheusMetrics, SolMetabasedSequencerChainService, TokioStopwatch},
};
use alloy::{
    network::{Ethereum, EthereumWallet, NetworkWallet},
    primitives::{Address, B256},
    providers::{
        fillers::{
            CachedNonceManager, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        ReqwestProvider, RootProvider,
    },
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
};
use eyre::Report;
use std::fmt::Debug;
use url::Url;

type RpcFillProvider = FillProvider<
    JoinFill<
        NonceFiller<CachedNonceManager>,
        JoinFill<WalletFiller<EthereumWallet>, JoinFill<GasFiller, ChainIdFiller>>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

pub type DefaultChainService =
    SolMetabasedSequencerChainService<RpcFillProvider, Http<Client>, Ethereum>;

pub type DefaultMetrics = PrometheusMetrics;
pub type DefaultStopwatch = TokioStopwatch;
pub type DefaultServices = Services<DefaultChainService, DefaultMetrics, DefaultStopwatch>;

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
        Self { chain, metrics, stopwatch }
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

impl DefaultServices {
    pub fn create(
        chain_contract_address: Address,
        chain_rpc_address: Url,
        private_key: B256,
    ) -> eyre::Result<Self> {
        let chain =
            Self::create_chain_service(chain_contract_address, chain_rpc_address, private_key)?;
        let metrics = PrometheusMetrics::new();
        let stopwatch = TokioStopwatch;

        Ok(Services::new(chain, metrics, stopwatch))
    }

    fn create_chain_service(
        chain_contract_address: Address,
        chain_rpc_address: Url,
        private_key: B256,
    ) -> eyre::Result<DefaultChainService> {
        // Fillers automatically set some attributes for every transaction sent using this provider.
        // See https://alloy.rs/building-with-alloy/understanding-fillers.html
        let wallet = Self::from(&private_key)?;

        let wallet_address =
            <EthereumWallet as NetworkWallet<Ethereum>>::default_signer_address(&wallet);

        let rpc_provider = Self::new_rpc_provider(chain_rpc_address, wallet);

        Ok(SolMetabasedSequencerChainService::new(
            chain_contract_address,
            wallet_address,
            rpc_provider,
        ))
    }

    fn from(private_key: &B256) -> Result<EthereumWallet, Report> {
        Ok(EthereumWallet::from(PrivateKeySigner::from_bytes(private_key)?))
    }

    fn new_rpc_provider(chain_rpc_address: Url, wallet: EthereumWallet) -> RpcFillProvider {
        let filler = join_fill!(
            NonceFiller::new(CachedNonceManager::default()),
            WalletFiller::new(wallet),
            GasFiller,
            ChainIdFiller::new(None),
        );

        let rpc_client: RootProvider<_, Ethereum> = ReqwestProvider::new_http(chain_rpc_address);

        FillProvider::new(rpc_client, filler)
    }
}
