//! Rollup utils for the integration tests

use crate::preloaded_config::get_default_private_key_signer;
use alloy::primitives::Address;
use mchain::mchain::MCHAIN_ID;

pub fn get_rollup_contract_address() -> Address {
    get_default_private_key_signer().address().create(0)
}

/// Get the nitro json configuration data for the rollup
pub fn rollup_info(rollup_config: &str, chain_name: &str) -> String {
    let rollup = get_rollup_contract_address();
    let deployed_at: u64 = 1;
    let zero = Address::ZERO;
    format!(
        r#"[{{
              "chain-name": "{chain_name}",
              "parent-chain-id": {MCHAIN_ID},
              "parent-chain-is-arbitrum": false,
              "sequencer-url": "",
              "secondary-forwarding-target": "",
              "feed-url": "",
              "secondary-feed-url": "",
              "das-index-url": "",
              "has-genesis-state": false,
              "chain-config": {rollup_config},
              "rollup": {{
                "bridge": "{rollup}",
                "inbox": "{zero}",
                "sequencer-inbox": "{rollup}",
                "deployed-at": {deployed_at},
                "rollup": "{zero}",
                "native-token": "{zero}",
                "upgrade-executor": "{zero}",
                "validator-wallet-creator": "{zero}"
              }}
            }}]"#
    )
}
