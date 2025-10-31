//! Translator for the integration tests
use alloy::primitives::Address;

#[derive(Debug, Default)]
pub struct TranslatorConfig {
    pub arbitrum_bridge_address: Option<Address>,
    pub arbitrum_inbox_address: Option<Address>,
    pub sequencing_contract_address: Option<Address>,
    pub config_manager_address: Option<Address>,
    pub appchain_chain_id: Option<u64>,
    pub mchain_ws_url: String,
    pub sequencing_ws_url: Option<String>,
    pub settlement_ws_url: String,
    pub port: u16,
    pub sequencing_start_block: Option<u64>,
    pub settlement_start_block: Option<u64>,
    pub settlement_delay: Option<u64>,
}

impl TranslatorConfig {
    pub fn cli_args(&self) -> Vec<String> {
        let mut args = vec![
            "--mchain-ws-url".to_string(),
            self.mchain_ws_url.to_string(),
            "--settlement-ws-url".to_string(),
            self.settlement_ws_url.to_string(),
            "--port".to_string(),
            self.port.to_string(),
        ];

        if let Some(url) = &self.sequencing_ws_url {
            args.extend(vec!["--sequencing-ws-url".to_string(), url.to_string()]);
        }

        if let Some(addr) = self.arbitrum_bridge_address {
            args.extend(vec!["--arbitrum-bridge-address".to_string(), addr.to_string()]);
        }

        if let Some(addr) = self.arbitrum_inbox_address {
            args.extend(vec!["--arbitrum-inbox-address".to_string(), addr.to_string()]);
        }

        if let Some(block) = self.sequencing_start_block {
            args.extend(vec!["--sequencing-start-block".to_string(), block.to_string()]);
        }

        if let Some(block) = self.settlement_start_block {
            args.extend(vec!["--settlement-start-block".to_string(), block.to_string()]);
        }

        if let Some(delay) = self.settlement_delay {
            args.extend(vec!["--settlement-delay".to_string(), delay.to_string()]);
        }

        if let Some(addr) = self.sequencing_contract_address {
            args.extend(vec!["--sequencing-contract-address".to_string(), addr.to_string()]);
        }

        if let Some(addr) = self.config_manager_address {
            args.extend(vec!["--config-manager-address".to_string(), addr.to_string()]);
        }

        if let Some(chain_id) = self.appchain_chain_id {
            args.extend(vec!["--appchain-chain-id".to_string(), chain_id.to_string()]);
        }

        args
    }
}
