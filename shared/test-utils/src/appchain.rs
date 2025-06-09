//! Appchain utils for the integration tests

use alloy::primitives::Address;
use synd_mchain::methods::common::{APPCHAIN_CONTRACT, MCHAIN_ID};

/// Get the nitro json configuration data for the appchain
pub fn appchain_info(appchain_config: &str, chain_name: &str) -> String {
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
              "chain-config": {appchain_config},
              "rollup": {{
                "bridge": "{APPCHAIN_CONTRACT}",
                "inbox": "{zero}",
                "sequencer-inbox": "{APPCHAIN_CONTRACT}",
                "deployed-at": {deployed_at},
                "rollup": "{zero}",
                "native-token": "{zero}",
                "upgrade-executor": "{zero}",
                "validator-wallet-creator": "{zero}"
              }}
            }}]"#
    )
}
