use tokio::sync::oneshot::{self, Receiver, Sender};

pub struct ShutdownTx {
    pub sequencer: Sender<()>,
    pub settlement: Sender<()>,
    pub slotter: Sender<()>,
}

pub struct ShutdownRx {
    pub sequencing: Receiver<()>,
    pub settlement: Receiver<()>,
    pub slotter: Receiver<()>,
}

/// Main channel plus paired channels for each component
pub struct ShutdownChannels {
    pub tx: ShutdownTx,
    pub rx: ShutdownRx,
}

impl ShutdownChannels {
    pub fn new() -> Self {
        let (seq_tx, seq_rx) = oneshot::channel();
        let (settle_tx, settle_rx) = oneshot::channel();
        let (slot_tx, slot_rx) = oneshot::channel();

        Self {
            tx: ShutdownTx { sequencer: seq_tx, settlement: settle_tx, slotter: slot_tx },
            rx: ShutdownRx { sequencing: seq_rx, settlement: settle_rx, slotter: slot_rx },
        }
    }

    pub fn split(self) -> (ShutdownTx, ShutdownRx) {
        (self.tx, self.rx)
    }
}

impl Default for ShutdownChannels {
    fn default() -> Self {
        Self::new()
    }
}
