use alloy::primitives::Address;
/// Configuration for the batch sequencer
#[derive(Debug, Clone)]
pub struct BatchSequencerConfig {
    /// The chain ID of the batch sequencer
    pub chain_id: u64,
    /// The URL of the valkey
    pub valkey_url: String,
    /// The private key of the batch sequencer
    pub private_key: String,
    /// The address of the sequencing contract
    pub sequencing_address: Address,
    /// The RPC URL of the sequencing contract
    pub sequencing_rpc_url: String,
    /// The port of the batch sequencer
    pub port: u16,
    /// Whether to wait for the receipt of the transaction
    pub wait_for_receipt: bool,
}

impl BatchSequencerConfig {
    pub fn cli_args(&self) -> Vec<String> {
        let mut args = vec![
            "--chain-id".to_string(),
            self.chain_id.to_string(),
            "--valkey-url".to_string(),
            self.valkey_url.to_string(),
            "--private-key".to_string(),
            self.private_key.to_string(),
            "--sequencing-address".to_string(),
            self.sequencing_address.to_string(),
            "--sequencing-rpc-urls".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--port".to_string(),
            self.port.to_string(),
        ];

        if self.wait_for_receipt {
            args.push("--wait-for-receipt".to_string());
        }

        args
    }
}
