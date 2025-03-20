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
