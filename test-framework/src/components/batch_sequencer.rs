use alloy::primitives::Address;

pub(super) struct BatchSequencerConfig {
    pub(crate) chain_id: u64,
    pub(crate) valkey_url: String,
    pub(crate) private_key: String,
    pub(crate) sequencing_address: Address,
    pub(crate) sequencing_rpc_url: String,
    pub(crate) port: u16,
    pub(crate) wait_for_receipt: bool,
    pub(crate) max_fee_per_gas: u128,
}

impl BatchSequencerConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
        let mut args = vec![
            "--chain-id".to_string(),
            self.chain_id.to_string(),
            "--valkey-url".to_string(),
            self.valkey_url.clone(),
            "--private-key".to_string(),
            self.private_key.clone(),
            "--sequencing-address".to_string(),
            self.sequencing_address.to_string(),
            "--sequencing-rpc-urls".to_string(),
            self.sequencing_rpc_url.clone(),
            "--port".to_string(),
            self.port.to_string(),
            "--max-fee-per-gas".to_string(),
            self.max_fee_per_gas.to_string(),
        ];

        if self.wait_for_receipt {
            args.push("--wait-for-receipt".to_string());
        }

        args
    }
}
