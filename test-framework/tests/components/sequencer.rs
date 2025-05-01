use crate::components::poster::POSTER_SEQUENCER_PRIVATE_KEY;
use alloy::primitives::Address;

#[derive(Debug)]
pub(super) struct SequencerConfig {
    pub(crate) sequencing_contract_address: Address,
    pub(crate) sequencing_rpc_url: String,
    pub(crate) sequencer_port: u16,
    pub(crate) metrics_port: u16,
}

impl SequencerConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            POSTER_SEQUENCER_PRIVATE_KEY.to_string(),
            "--chain-contract-address".to_string(),
            self.sequencing_contract_address.to_string(),
            "--chain-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--port".to_string(),
            self.sequencer_port.to_string(),
        ]
    }
}
