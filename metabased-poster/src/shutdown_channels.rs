//! The `spawn` module handles shutdown signals

use tokio::{
    signal::unix::{signal, SignalKind},
    sync::oneshot::{self, Receiver, Sender},
};
use tracing::{error, info};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct ShutdownTx {
    pub poller: Sender<()>,
    pub submitter: Sender<()>,
}
#[derive(Debug)]
#[allow(missing_docs)]
pub struct ShutdownRx {
    pub poller: Receiver<()>,
    pub submitter: Receiver<()>,
}

#[derive(Debug)]
#[allow(missing_docs)]
/// Main channel plus paired channels for each component
pub struct ShutdownChannels {
    pub main: Receiver<()>,
    pub tx: ShutdownTx,
    pub rx: ShutdownRx,
}

impl ShutdownChannels {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let (main_tx, main_rx) = oneshot::channel();
        let (poller_tx, poller_rx) = oneshot::channel();
        let (submitter_tx, submitter_rx) = oneshot::channel();

        init_signal_handler(main_tx);

        Self {
            main: main_rx,
            tx: ShutdownTx { poller: poller_tx, submitter: submitter_tx },
            rx: ShutdownRx { poller: poller_rx, submitter: submitter_rx },
        }
    }
}

impl Default for ShutdownChannels {
    fn default() -> Self {
        Self::new()
    }
}

fn init_signal_handler(main_shutdown_tx: Sender<()>) {
    tokio::spawn(async move {
        // Attempt to register SIGINT handler
        let mut sigint = match signal(SignalKind::interrupt()) {
            Ok(sig) => sig,
            Err(e) => {
                error!("Failed to register SIGINT handler: {}", e);
                return;
            }
        };

        // Attempt to register SIGTERM handler
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(sig) => sig,
            Err(e) => {
                error!("Failed to register SIGTERM handler: {}", e);
                return;
            }
        };
        tokio::select! {
            _ = sigint.recv() => {
                info!("Received SIGINT (Ctrl+C), initiating shutdown...");
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM, initiating shutdown...");
            }
        }

        assert!(main_shutdown_tx.send(()).is_err(), "Failed to send shutdown signal")
    });
}
