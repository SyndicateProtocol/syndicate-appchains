use alloy::primitives::Address;

// needs to be different from the regular private key to prevent nonce collisions
// needs to match the owner of the proposer contract
// anvil account 0
pub(super) const PROPOSER_SEQUENCER_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug)]
pub(super) struct ProposerConfig {
    pub(crate) assertion_poster_contract_address: Address,
    pub(crate) settlement_rpc_url: String,
    pub(crate) appchain_rpc_url: String,
    pub(crate) metrics_port: u16,
    pub(crate) port: u16,
}

impl ProposerConfig {
    pub(super) fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            PROPOSER_SEQUENCER_PRIVATE_KEY.to_string(),
            "--appchain-rpc-url".to_string(),
            self.appchain_rpc_url.to_string(),
            "--assertion-poster-contract-address".to_string(),
            self.assertion_poster_contract_address.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--polling-interval".to_string(),
            "1h".to_string(),
            "--port".to_string(),
            self.port.to_string(),
        ]
    }
}
