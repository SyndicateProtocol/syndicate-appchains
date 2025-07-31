use alloy::primitives::Address;

#[derive(Debug)]
pub struct ProposerConfig {
    pub ethereum_rpc_url: String,
    pub settlement_rpc_url: String,
    pub settlement_chain_id: u64,
    pub sequencing_rpc_url: String,
    pub appchain_rpc_url: String,
    pub enclave_rpc_url: String,
    pub eigen_rpc_url: String,
    pub tee_module_contract_address: Address,
    pub appchain_bridge_address: Address,
    pub sequencing_contract_address: Address,
    pub sequencing_bridge_address_on_l1: Address,
    pub settlement_delay: u64,
    pub polling_interval: String,
    pub close_challenge_interval: String,
    pub port: u16,
    pub private_key: String,
}

impl ProposerConfig {
    pub fn cli_args(&self) -> Vec<String> {
        vec![
            "--ethereum-rpc-url".to_string(),
            self.ethereum_rpc_url.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--settlement-chain-id".to_string(),
            self.settlement_chain_id.to_string(),
            "--sequencing-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--appchain-rpc-url".to_string(),
            self.appchain_rpc_url.to_string(),
            "--enclave-rpc-url".to_string(),
            self.enclave_rpc_url.to_string(),
            "--eigen-rpc-url".to_string(),
            self.eigen_rpc_url.to_string(),
            "--tee-module-contract-address".to_string(),
            self.tee_module_contract_address.to_string(),
            "--appchain-bridge-address".to_string(),
            self.appchain_bridge_address.to_string(),
            "--sequencing-contract-address".to_string(),
            self.sequencing_contract_address.to_string(),
            "--sequencing-bridge-address".to_string(),
            self.sequencing_bridge_address_on_l1.to_string(),
            "--settlement-delay".to_string(),
            self.settlement_delay.to_string(),
            "--private-key".to_string(),
            self.private_key.to_string(),
            "--polling-interval".to_string(),
            self.polling_interval.to_string(),
            "--close-challenge-interval".to_string(),
            self.close_challenge_interval.to_string(),
            "--port".to_string(),
            self.port.to_string(),
            "--mtls-enabled-enclave".to_string(),
            "false".to_string(),
        ]
    }
}
