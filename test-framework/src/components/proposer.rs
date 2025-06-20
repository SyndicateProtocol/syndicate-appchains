use alloy::primitives::Address;

// needs to be different from the regular private key to prevent nonce collisions
// needs to match the owner of the proposer contract
// anvil account 0
pub const PROPOSER_SEQUENCER_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug)]
pub struct ProposerConfig {
    pub ethereum_rpc_url: String,
    pub settlement_rpc_url: String,
    pub settlement_chain_id: u64,
    pub sequencing_rpc_url: String,
    pub appchain_rpc_url: String,
    pub enclave_rpc_url: String,
    pub tee_module_contract_address: Address,
    pub arbitrum_bridge_address: Address,
    pub inbox_address: Address,
    pub sequencer_inbox_address: Address,
    pub polling_interval: String,
    pub close_challenge_interval: String,
    pub metrics_port: u16,
}

impl ProposerConfig {
    pub fn cli_args(&self) -> Vec<String> {
        vec![
            "--ethereum-rpc-url".to_string(),
            self.ethereum_rpc_url.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--sequencing-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--appchain-rpc-url".to_string(),
            self.appchain_rpc_url.to_string(),
            "--enclave-rpc-url".to_string(),
            self.enclave_rpc_url.to_string(),
            "--tee-module-contract-address".to_string(),
            self.tee_module_contract_address.to_string(),
            "--arbitrum-bridge-address".to_string(),
            self.arbitrum_bridge_address.to_string(),
            "--inbox-address".to_string(),
            self.inbox_address.to_string(),
            "--sequencer-inbox-address".to_string(),
            self.sequencer_inbox_address.to_string(),
            "--private-key".to_string(),
            PROPOSER_SEQUENCER_PRIVATE_KEY.to_string(),
            "--polling-interval".to_string(),
            self.polling_interval.to_string(),
            "--close-challenge-interval".to_string(),
            self.close_challenge_interval.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--settlement-chain-id".to_string(),
            self.settlement_chain_id.to_string(),
        ]
    }
}
