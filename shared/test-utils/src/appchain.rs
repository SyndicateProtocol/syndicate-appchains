//! Appchain utils for the integration tests

use alloy::primitives::Address;
use synd_mchain::server::APPCHAIN;

/// Get the nitro json configuration data for the appchain
pub fn appchain_info_json(
    chain_id: u64,
    parent_chain_id: u64,
    chain_owner: Address,
    chain_name: &str,
) -> String {
    let appchain_config = appchain_config(chain_id, chain_owner);
    let deployed_at: u64 = 1;
    let zero = Address::ZERO;
    format!(
        r#"[{{
              "chain-name": "{chain_name}",
              "parent-chain-id": {parent_chain_id},
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

/// Return the on-chain config for a rollup with a given chain id
fn appchain_config(chain_id: u64, chain_owner: Address) -> String {
    let mut cfg = format!(
        r#"{{
          "chainId": {chain_id},
          "homesteadBlock": 0,
          "daoForkBlock": null,
          "daoForkSupport": true,
          "eip150Block": 0,
          "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
          "eip155Block": 0,
          "eip158Block": 0,
          "byzantiumBlock": 0,
          "constantinopleBlock": 0,
          "petersburgBlock": 0,
          "istanbulBlock": 0,
          "muirGlacierBlock": 0,
          "berlinBlock": 0,
          "londonBlock": 0,
          "clique": {{
          "period": 0,
          "epoch": 0
          }},
          "arbitrum": {{
          "EnableArbOS": true,
          "AllowDebugPrecompiles": false,
          "DataAvailabilityCommittee": false,
          "InitialArbOSVersion": 32,
          "InitialChainOwner": "{chain_owner}",
          "GenesisBlockNum": 0
          }}
      }}"#
    );
    cfg.retain(|c| !c.is_whitespace());
    cfg.shrink_to_fit();
    cfg
}
