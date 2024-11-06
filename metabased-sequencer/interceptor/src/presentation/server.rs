use crate::application::{Metrics, RunningStopwatch, Stopwatch};
use crate::domain::primitives::Address;
use crate::domain::MetabasedSequencerChainService;
use crate::infrastructure::{PrometheusMetrics, SolMetabasedSequencerChainService, TokioStopwatch};
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
    let stopwatch = TokioStopwatch;
    let services = Services::new(chain, metrics, stopwatch);

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

fn create_eth_module<Chain, M, S, S1>(
    services: Services<Chain, M, S>,
) -> anyhow::Result<RpcModule<Services<Chain, M, S>>>
where
    Chain: MetabasedSequencerChainService + Send + Sync + 'static,
    Error: From<<Chain as MetabasedSequencerChainService>::Error>,
    M: Metrics + Send + Sync + 'static,
    S: Stopwatch<Running = S1> + Send + Sync + 'static,
    S1: RunningStopwatch + Send + Sync + 'static,
{
    let mut module = RpcModule::new(services);
    module.register_async_method("eth_sendRawTransaction", jsonrpc::send_raw_transaction)?;
    module.register_method("metrics", jsonrpc::metrics)?;
    module.register_method("health", jsonrpc::health)?;
    Ok(module)
}

#[derive(Debug)]
pub struct Services<Chain, M, S> {
    chain: Chain,
    metrics: M,
    stopwatch: S,
}

impl<Chain: MetabasedSequencerChainService, M: Metrics, S: Stopwatch> Services<Chain, M, S> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::fillers::{FillerControlFlow, TxFiller};
    use alloy::providers::{Network, SendableTx};
    use alloy::rpc::types::TransactionRequest;
    use alloy::transports::TransportResult;

    #[derive(Clone, Copy, Debug)]
    pub struct DummyGasFiller;

    impl TxFiller<Ethereum> for DummyGasFiller {
        type Fillable = ();

        fn status(&self, _tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            FillerControlFlow::Finished
        }

        fn fill_sync(&self, tx: &mut SendableTx<Ethereum>) {
            if let SendableTx::Builder(tx) = tx {
                tx.gas.replace(1);
            }
        }

        async fn prepare<P, T>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable> {
            Ok(())
        }

        async fn fill(
            &self,
            _to_fill: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            Ok(tx)
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct DummyChainIdFiller;

    impl TxFiller<Ethereum> for DummyChainIdFiller {
        type Fillable = ();

        fn status(&self, _tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            FillerControlFlow::Finished
        }

        fn fill_sync(&self, tx: &mut SendableTx<Ethereum>) {
            if let SendableTx::Builder(tx) = tx {
                tx.chain_id.replace(2);
            }
        }

        async fn prepare<P, T>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable> {
            Ok(())
        }

        async fn fill(
            &self,
            _to_fill: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            Ok(tx)
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct DummyNonceFiller;

    impl TxFiller<Ethereum> for DummyNonceFiller {
        type Fillable = ();

        fn status(&self, _tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            FillerControlFlow::Finished
        }

        fn fill_sync(&self, tx: &mut SendableTx<Ethereum>) {
            if let SendableTx::Builder(tx) = tx {
                tx.nonce.replace(3);
            }
        }

        async fn prepare<P, T>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable> {
            Ok(())
        }

        async fn fill(
            &self,
            _to_fill: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            Ok(tx)
        }
    }

    #[test]
    fn test_join_fill_creates_filler_that_applies_all_given_fillers_on_transaction() {
        let tx_request = TransactionRequest::default();
        let mut actual_tx = SendableTx::Builder(tx_request);

        let filler = join_fill!(DummyChainIdFiller, DummyGasFiller, DummyNonceFiller);
        filler.fill_sync(&mut actual_tx);

        let actual_tx = actual_tx.as_builder().unwrap();
        let expected_tx = &TransactionRequest {
            nonce: Some(3),
            chain_id: Some(2),
            gas: Some(1),
            ..Default::default()
        };

        assert_eq!(actual_tx, expected_tx);
    }
}
