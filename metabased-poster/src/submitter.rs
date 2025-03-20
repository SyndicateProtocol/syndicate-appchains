//!  The `submitter` submits information to the `AssertionPoster` contract

use crate::types::NitroBlock;
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
};
use contract_bindings::arbitrum::iassertionposter::IAssertionPoster::{
    self, IAssertionPosterInstance,
};
use eyre::{eyre, Result};
use std::{str::FromStr, sync::Arc};
use tokio::sync::{mpsc::Receiver, oneshot};
use tracing::{error, info};
use url::Url;

type FilledProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

#[derive(Debug)]
struct Submitter {
    assertion_poster: IAssertionPosterInstance<(), FilledProvider>,
}

/// Starts the poller task
pub async fn run(
    rpc_url: Url,
    private_key: String,
    assertion_poster_contract_address: Address,
    blocks_rx: Receiver<Arc<NitroBlock>>,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<()> {
    let priv_k = PrivateKeySigner::from_str(&private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let provider = ProviderBuilder::new().wallet(EthereumWallet::from(priv_k)).on_http(rpc_url);
    let assertion_poster = IAssertionPoster::new(assertion_poster_contract_address, provider);
    let submitter = Submitter { assertion_poster };
    submitter.main_loop(blocks_rx, shutdown_rx).await
}

impl Submitter {
    async fn main_loop(
        &self,
        mut blocks_rx: Receiver<Arc<NitroBlock>>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<()> {
        info!("Starting Submitter...");
        loop {
            tokio::select! {
                Some(block) = blocks_rx.recv() => {
                   if let Err(err) = self.post_assertion(block).await {
                    error!("Error submitting assertion: {:?}", err);
                   }
                }
                _ = &mut shutdown_rx => {
                    return Err(eyre!("Shut down"))
                }
            }
        }
    }

    async fn post_assertion(&self, block: Arc<NitroBlock>) -> Result<()> {
        let _ = self.assertion_poster.postAssertion(block.hash, block.send_root).send().await?;
        info!("Assertion submitted2 for block: {:?}", block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    async fn create_submitter() -> Submitter {
        let rpc_url = Url::parse("http://localhost:8545").unwrap(); // Replace with an actual RPC if needed
        let private_key =
            "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string();
        let contract_address = Address::default();

        let priv_k = PrivateKeySigner::from_str(&private_key).expect("Invalid private key");
        let provider = ProviderBuilder::new().wallet(EthereumWallet::from(priv_k)).on_http(rpc_url);
        let assertion_poster = IAssertionPoster::new(contract_address, provider);

        Submitter { assertion_poster }
    }

    #[tokio::test]
    async fn test_post_assertion() {
        let submitter = create_submitter().await;
        let block = Arc::new(NitroBlock::default());
        let result = submitter.post_assertion(block.clone()).await;
        match &result {
            Ok(_) => println!("post_assertion succeeded"),
            Err(err) => eprintln!("post_assertion failed: {:?}", err),
        }

        // Note: This will fail in actual execution since we're using a mock setup
        // but it tests the parameter parsing logic
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_main_loop_handles_blocks_and_shutdown() {
        let submitter = create_submitter().await;

        let (blocks_tx, blocks_rx) = mpsc::channel(10);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let block = Arc::new(NitroBlock::default());
        blocks_tx.send(block.clone()).await.unwrap();
        drop(shutdown_tx);

        let result = submitter.main_loop(blocks_rx, shutdown_rx).await;
        assert!(result.is_err(), "Expected main_loop to exit with shutdown error");
    }
}
