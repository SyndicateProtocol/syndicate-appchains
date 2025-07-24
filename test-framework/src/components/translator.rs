//! Translator for the integration tests
use alloy::primitives::Address;

#[derive(Debug, Default)]
pub(super) struct TranslatorConfig {
    pub(crate) arbitrum_bridge_address: Option<Address>,
    pub(crate) arbitrum_inbox_address: Option<Address>,
    pub(crate) sequencing_contract_address: Option<Address>,
    pub(crate) config_manager_address: Option<Address>,
    pub(crate) appchain_chain_id: Option<u64>,
    pub(crate) mchain_ws_url: String,
    pub(crate) sequencing_ws_url: Option<String>,
    pub(crate) settlement_ws_url: String,
    pub(crate) port: u16,
    pub(crate) sequencing_start_block: Option<u64>,
    pub(crate) settlement_start_block: Option<u64>,
    pub(crate) settlement_delay: Option<u64>,
}

impl TranslatorConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
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
