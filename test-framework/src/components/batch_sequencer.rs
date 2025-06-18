use alloy::primitives::Address;

pub(super) struct BatchSequencerConfig {
    pub(crate) chain_id: u64,
    pub(crate) valkey_url: String,
    pub(crate) private_key: String,
    pub(crate) sequencing_address: Address,
    pub(crate) sequencing_rpc_url: String,
    pub(crate) metrics_port: u16,
}

impl BatchSequencerConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
        vec![
            "--chain-id".to_string(),
            self.chain_id.to_string(),
            "--valkey-url".to_string(),
            self.valkey_url.to_string(),
            "--private-key".to_string(),
            self.private_key.to_string(),
            "--sequencing-address".to_string(),
            self.sequencing_address.to_string(),
            "--sequencing-ws-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
        ]
    }
}
