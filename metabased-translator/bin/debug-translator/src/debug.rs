use common::types::{BlockAndReceipts, Slot};
use std::fmt::Debug;
use tokio::{
    io::{self, AsyncBufReadExt},
    sync::{mpsc, oneshot},
};
use tracing::{error, info};

struct DebugChannel<T: Debug> {
    rx: mpsc::Receiver<T>,
    tx: mpsc::Sender<T>,

    paused: bool,
}

impl<T: Debug> DebugChannel<T> {
    pub fn new(rx: mpsc::Receiver<T>, tx: mpsc::Sender<T>) -> Self {
        Self { rx, tx, paused: true }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn check_length(&self) -> usize {
        self.rx.len()
    }

    pub async fn step(&mut self) -> Result<(), mpsc::error::TryRecvError> {
        let item = self.rx.try_recv();
        match item {
            Ok(item) => {
                info!("Stepping: {:?}", item);
                self.tx.send(item).await.unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Debugger {
    sequencing_channel: DebugChannel<BlockAndReceipts>,
    settlement_channel: DebugChannel<BlockAndReceipts>,
    slot_channel: DebugChannel<Slot>,
}

impl Debugger {
    pub fn new(
        sequencing_rx: mpsc::Receiver<BlockAndReceipts>,
        settlement_rx: mpsc::Receiver<BlockAndReceipts>,
        debug_sequencing_tx: mpsc::Sender<BlockAndReceipts>,
        debug_settlement_tx: mpsc::Sender<BlockAndReceipts>,
        slot_rx: mpsc::Receiver<Slot>,
        debug_slot_tx: mpsc::Sender<Slot>,
    ) -> Self {
        let sequencing_channel = DebugChannel::new(sequencing_rx, debug_sequencing_tx);
        let settlement_channel = DebugChannel::new(settlement_rx, debug_settlement_tx);
        let slot_channel = DebugChannel::new(slot_rx, debug_slot_tx);
        Self { sequencing_channel, settlement_channel, slot_channel }
    }

    pub async fn run(&mut self, mut shutdown_rx: oneshot::Receiver<()>) {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        info!("Debugger running");

        loop {
            tokio::select! {
                result = stdin.next_line() => {
                    match result {
                        Ok(Some(line)) => {
                            match line.as_str() {
                                "step-seq" => {
                                    self.sequencing_channel.step().await.unwrap();
                                }
                                "step-set" => {
                                    self.settlement_channel.step().await.unwrap();
                                }
                                "step-slot" => {
                                    self.slot_channel.step().await.unwrap();
                                }
                                "info" => {
                                    info!("Sequencing channel length: {}", self.sequencing_channel.check_length());
                                    info!("Settlement channel length: {}", self.settlement_channel.check_length());
                                    info!("Slot channel length: {}", self.slot_channel.check_length());
                                }
                                "pause" => {
                                    self.sequencing_channel.set_paused(true);
                                    self.settlement_channel.set_paused(true);
                                    self.slot_channel.set_paused(true);
                                    info!("Channels paused");
                                }
                                "unpause" => {
                                    self.sequencing_channel.set_paused(false);
                                    self.settlement_channel.set_paused(false);
                                    self.slot_channel.set_paused(false);
                                    info!("Channels unpaused");
                                }
                                _ => info!("Unknown command: {}", line),
                            }
                        }
                        Ok(None) => {
                            info!("End of input stream.");
                            break;
                        }
                        Err(e) => {
                            error!("Error reading input: {}", e);
                            break;
                        }
                    }
                }
                item = self.sequencing_channel.rx.recv(), if !self.sequencing_channel.paused => {
                    if let Some(item) = item {
                        self.sequencing_channel.tx.send(item).await.unwrap();
                    }
                }
                item = self.settlement_channel.rx.recv(), if !self.settlement_channel.paused => {
                    if let Some(item) = item {
                        self.settlement_channel.tx.send(item).await.unwrap();
                    }
                }
                item = self.slot_channel.rx.recv(), if !self.slot_channel.paused => {
                    if let Some(item) = item {
                        self.slot_channel.tx.send(item).await.unwrap();
                    }
                }
                _ = &mut shutdown_rx => {
                    info!("Shutting down debugger...");
                    break;
                }
            }
        }
    }
}
