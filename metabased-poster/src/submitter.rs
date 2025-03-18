//!  The `submitter` submits information to the `AssertionPoster` contract

use crate::types::NitroBlock;
use alloy::{
    primitives::Address,
    providers::{ProviderBuilder, RootProvider},
};
use contract_bindings::arbitrum::iassertionposter::IAssertionPoster::{
    self, IAssertionPosterInstance,
};
use eyre::{eyre, Error};
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, oneshot};
use tracing::info;
use url::Url;

#[derive(Debug)]
struct Submitter<'a> {
    assertion_poster: IAssertionPosterInstance<(), &'a RootProvider>,
}

/// Starts the poller task
pub async fn run(
    rpc_url: Url,
    assertion_poster_contract_address: Address,
    blocks_rx: Receiver<Arc<NitroBlock>>,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let provider = ProviderBuilder::default().on_http(rpc_url);
    let assertion_poster = IAssertionPoster::new(assertion_poster_contract_address, &provider);
    let submitter = Submitter { assertion_poster };
    submitter.main_loop(blocks_rx, shutdown_rx).await
}

impl Submitter<'_> {
    async fn main_loop(
        self,
        mut blocks_rx: Receiver<Arc<NitroBlock>>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), Error> {
        info!("Starting Submitter...");
        loop {
            tokio::select! {
                Some(block) = blocks_rx.recv() => {
                    // TODO (SEQ-689): Implement submitter
                    info!("Submitting block: {:?}", block);
                    let _ = self.assertion_poster.postAssertion(block.hash, block.send_root).send().await?;
                },
                _ = &mut shutdown_rx => {
                    info!("Shutting down Submitter...");
                    return Err(eyre!("Shut down"))
                }
            }
        }
    }
}
