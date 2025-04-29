use alloy::primitives::Address;

pub(super) struct TcSequencerConfig {
    pub(crate) sequencer_port: u16,
    pub(crate) metrics_port: u16,
}

impl TcSequencerConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
        vec![]
    }
}
