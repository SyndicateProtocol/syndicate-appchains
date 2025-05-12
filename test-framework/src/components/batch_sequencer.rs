use alloy::primitives::Address;

pub(super) struct BatchSequencerConfig {
    pub(crate) chain_id: u64,
    pub(crate) redis_url: String,
    pub(crate) private_key: String,
    pub(crate) wallet_pool_address: Address,
    pub(crate) sequencing_address: Address,
    pub(crate) sequencing_rpc_url: String,
    pub(crate) metrics_port: u16,
}

impl BatchSequencerConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
        vec![
            "--chain-id".to_string(),
            self.chain_id.to_string(),
            "--redis-url".to_string(),
            self.redis_url.to_string(),
            "--private-key".to_string(),
            self.private_key.to_string(),
            "--wallet-pool-address".to_string(),
            self.wallet_pool_address.to_string(),
            "--sequencing-address".to_string(),
            self.sequencing_address.to_string(),
            "--sequencing-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
        ]
    }
}
