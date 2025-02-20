use tokio::{
    signal::unix::{signal, SignalKind},
    sync::{oneshot, oneshot::Sender},
};
use tracing::info;

pub struct ShutdownTx {
    pub sequencer: Sender<()>,
    pub settlement: Sender<()>,
    pub slotter: Sender<()>,
    pub builder: Sender<()>,
}

pub struct ShutdownRx {
    pub sequencer: oneshot::Receiver<()>,
    pub settlement: oneshot::Receiver<()>,
    pub slotter: oneshot::Receiver<()>,
    pub builder: oneshot::Receiver<()>,
}

/// Main channel plus paired channels for each component
pub struct ShutdownChannels {
    pub main: oneshot::Receiver<()>,
    pub tx: ShutdownTx,
    pub rx: ShutdownRx,
}

impl ShutdownChannels {
    pub fn new() -> Self {
        let (main_tx, main_rx) = oneshot::channel();
        let (seq_tx, seq_rx) = oneshot::channel();
        let (settle_tx, settle_rx) = oneshot::channel();
        let (slot_tx, slot_rx) = oneshot::channel();
        let (build_tx, build_rx) = oneshot::channel();

        init_signal_handler(main_tx);

        Self {
            main: main_rx,
            tx: ShutdownTx {
                sequencer: seq_tx,
                settlement: settle_tx,
                slotter: slot_tx,
                builder: build_tx,
            },
            rx: ShutdownRx {
                sequencer: seq_rx,
                settlement: settle_rx,
                slotter: slot_rx,
                builder: build_rx,
            },
        }
    }

    pub fn split(self) -> (oneshot::Receiver<()>, ShutdownTx, ShutdownRx) {
        (self.main, self.tx, self.rx)
    }
}

impl Default for ShutdownChannels {
    fn default() -> Self {
        Self::new()
    }
}

fn init_signal_handler(main_shutdown_tx: Sender<()>) {
    tokio::spawn(async move {
        // SIGINT is triggered when the user presses Ctrl+C in the terminal
        let mut sigint =
            signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
        // SIGTERM is typically sent when stopping a Docker container
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                info!("Received SIGINT (Ctrl+C), initiating shutdown...");
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM, initiating shutdown...");
            }
        }

        if main_shutdown_tx.send(()).is_err() {
            panic!("Failed to send shutdown signal");
        }
    });
}
