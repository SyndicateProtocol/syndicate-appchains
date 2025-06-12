//! Appchain utils for the integration tests

use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, Bytes, B256, U256},
    providers::{Provider, WalletProvider},
};
use contract_bindings::synd::{
    arbsys::ArbSys, ibridge::IBridge, iinbox::IInbox, ioutbox::IOutbox, irollupcore::IRollupCore,
    nodeinterface::NodeInterface, rollup::Rollup,
};
use eyre::Ok;
use serde::{Deserialize, Serialize};
use shared::types::FilledProvider;

/// Get the nitro json configuration data for the appchain
pub fn nitro_chain_info_json(
    chain_id: u64,
    parent_chain_id: u64,
    chain_owner: Address,
    chain_name: &str,
    bridge_address: Address,
    sequencer_inbox_address: Address,
) -> String {
    let appchain_config = chain_config(chain_id, chain_owner);
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
                "bridge": "{bridge_address}",
                "inbox": "{zero}",
                "sequencer-inbox": "{sequencer_inbox_address}",
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
pub fn chain_config(chain_id: u64, chain_owner: Address) -> String {
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

pub const ARB_SYS_PRECOMPILE_ADDRESS: Address =
    address!("0x0000000000000000000000000000000000000064");
pub const NODE_INTERFACE_PRECOMPILE_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000000c8");

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NitroBlock {
    pub hash: B256,
    pub send_root: B256,
    pub number: U256,
    pub l1_block_number: U256,
    pub timestamp: U256,
}

pub async fn init_withdrawal_tx(
    to_address: Address,
    withdrawal_value: U256,
    appchain_provider: &FilledProvider,
) -> eyre::Result<EthereumTxEnvelope<TxEip4844Variant>> {
    let from = appchain_provider.default_signer_address();
    let nonce = appchain_provider.get_transaction_count(from).await?;
    let tx = ArbSys::new(ARB_SYS_PRECOMPILE_ADDRESS, &appchain_provider)
        .withdrawEth(to_address)
        .value(withdrawal_value)
        .nonce(nonce)
        .gas(100_000u64)
        .chain_id(appchain_provider.get_chain_id().await?)
        .max_fee_per_gas(100_000_000u128)
        .max_priority_fee_per_gas(0)
        .into_transaction_request()
        .build(appchain_provider.wallet())
        .await?;
    Ok(tx)
}

pub async fn execute_withdrawal(
    to_address: Address,
    withdrawal_value: U256,
    bridge_address: Address,
    settlement_provider: &FilledProvider,
    appchain_provider: &FilledProvider,
) -> eyre::Result<()> {
    // Generate proof
    let node_interface = NodeInterface::new(NODE_INTERFACE_PRECOMPILE_ADDRESS, &appchain_provider);
    let proof = node_interface.constructOutboxProof(1, 0).call().await?;

    // Execute withdrawal
    let bridge = IBridge::new(bridge_address, &settlement_provider);
    let outbox = IOutbox::new(
        IRollupCore::new(bridge.rollup().call().await?._0, &settlement_provider)
            .outbox()
            .call()
            .await?
            ._0,
        &settlement_provider,
    );

    let block: NitroBlock =
        appchain_provider.raw_request("eth_getBlockByNumber".into(), ("latest", false)).await?;

    let _ = outbox
        .executeTransaction(
            proof.proof,                                  // proof
            U256::from(0),                                // index
            settlement_provider.default_signer_address(), // l2Sender
            to_address,                                   // to
            block.number,                                 // l2Block,
            block.l1_block_number,                        // l1Block,
            block.timestamp,                              // l2Timestamp,
            withdrawal_value,                             // value
            Bytes::new(),                                 // data (always empty)
        )
        .send()
        .await?;
    Ok(())
}
