//! Rollup utils for the integration tests

use crate::preloaded_config::get_default_private_key_signer;
use alloy::primitives::Address;

pub const MCHAIN_ID: u64 = 84532;

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

/// Return the on-chain config for a rollup with a given chain id
pub fn rollup_config(chain_id: u64, chain_owner: Address) -> String {
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

pub fn chain_config(chain_id: u64) -> String {
    r#"{"config": {
    "chainId": "#
        .to_string() +
        &chain_id.to_string() +
        r#",
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "constantinopleBlock": 0,
    "petersburgBlock": 0,
    "istanbulBlock": 0,
    "berlinBlock": 0,
    "londonBlock": 0,
    "terminalTotalDifficulty": 0,
    "shanghaiTime": 0,
    "cancunTime": 0
  },
  "nonce": "0x0",
  "timestamp": "0x0",
  "extraData": "0x",
  "gasLimit": "0x1c9c380",
  "difficulty": "0x0",
  "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "coinbase": "0x0000000000000000000000000000000000000000",
  "alloc": {
    "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
      "balance": "0xD3C21BCECCEDA1000000"
    }
  },
  "number": "0x0",
  "gasUsed": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}"#
}
