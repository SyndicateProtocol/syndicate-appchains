//!  The `submitter` submits information to the `AssertionPoster` contract

use crate::types::NitroBlock;
use eyre::{eyre, Error};
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, oneshot};
use tracing::info;

#[derive(Debug)]
struct Submitter {}

/// Starts the poller task
pub async fn run(
    blocks_rx: Receiver<Arc<NitroBlock>>,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let submitter = Submitter {};
    submitter.main_loop(blocks_rx, shutdown_rx).await
}

impl Submitter {
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
                    info!("Received block: {:?}", block);
                },
                _ = &mut shutdown_rx => {

                    info!("Shutting down Submitter...");
                    return Err(eyre!("Shut down"))
                }
            }
        }
    }
}
