//! Rollup utils for the integration tests

use alloy::primitives::Address;
use mchain::server::{MCHAIN_ID, ROLLUP};

/// Get the nitro json configuration data for the rollup
pub fn rollup_info(rollup_config: &str, chain_name: &str) -> String {
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
                "bridge": "{ROLLUP}",
                "inbox": "{zero}",
                "sequencer-inbox": "{ROLLUP}",
                "deployed-at": {deployed_at},
                "rollup": "{zero}",
                "native-token": "{zero}",
                "upgrade-executor": "{zero}",
                "validator-wallet-creator": "{zero}"
              }}
            }}]"#
    )
}
