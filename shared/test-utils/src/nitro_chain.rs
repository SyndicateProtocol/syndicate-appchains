//! Appchain utils for the integration tests

use crate::{chain_info::PRIVATE_KEY, docker::E2EProcess};
use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    network::TransactionBuilder,
    primitives::{address, Address, Bytes, B256, U256},
    providers::{Provider, WalletProvider},
};
use contract_bindings::synd::{
    arb_sys::ArbSys, i_bridge::IBridge, i_outbox::IOutbox, i_rollup_core::IRollupCore,
    node_interface::NodeInterface,
};
use eyre::Ok;
use serde::{Deserialize, Serialize};
use shared::types::{deserialize_address, FilledProvider};
use tokio::{fs, process::Command};

pub struct NitroChainInfoArgs {
    pub chain_id: u64,
    pub parent_chain_id: u64,
    pub chain_owner: Address,
    pub chain_name: String,
    pub deployment: NitroDeployment,
}

/// Get the nitro json configuration data for the appchain
pub fn nitro_chain_info_json(args: NitroChainInfoArgs) -> String {
    let NitroChainInfoArgs { chain_name, parent_chain_id, deployment, .. } = args;
    let NitroDeployment {
        bridge,
        inbox,
        sequencer_inbox,
        deployed_at,
        rollup,
        native_token,
        upgrade_executor,
        validator_wallet_creator,
        stake_token,
        ..
    } = deployment;

    let appchain_config = chain_config(args.chain_id, args.chain_owner);
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
                "bridge": "{bridge}",
                "inbox": "{inbox}",
                "sequencer-inbox": "{sequencer_inbox}",
                "deployed-at": {deployed_at},
                "rollup": "{rollup}",
                "native-token": "{native_token}",
                "upgrade-executor": "{upgrade_executor}",
                "validator-wallet-creator": "{validator_wallet_creator}",
                "stake-token": "{stake_token}"
              }}
            }}]"#
    )
}

/// Return the on-chain config for a rollup with a given chain id
pub fn chain_config(chain_id: u64, chain_owner: Address) -> String {
    // TODO SEQ-1032: DataAvailabilityCommittee might need to be true for set/seq chains with
    // eigenDA
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
        IRollupCore::new(bridge.rollup().call().await?, &settlement_provider)
            .outbox()
            .call()
            .await?,
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

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct NitroDeployment {
    #[serde(deserialize_with = "deserialize_address")]
    pub bridge: Address,

    #[serde(deserialize_with = "deserialize_address")]
    pub inbox: Address,

    #[serde(rename = "sequencer-inbox", deserialize_with = "deserialize_address")]
    pub sequencer_inbox: Address,

    #[serde(rename = "deployed-at")]
    pub deployed_at: u64,

    #[serde(deserialize_with = "deserialize_address")]
    pub rollup: Address,

    #[serde(rename = "native-token", deserialize_with = "deserialize_address")]
    pub native_token: Address,

    #[serde(rename = "upgrade-executor", deserialize_with = "deserialize_address")]
    pub upgrade_executor: Address,

    #[serde(rename = "validator-wallet-creator", deserialize_with = "deserialize_address")]
    pub validator_wallet_creator: Address,

    #[serde(rename = "stake-token", deserialize_with = "deserialize_address")]
    pub stake_token: Address,
}

#[allow(clippy::unwrap_used)]
/// Deploys a mock rollup contract. - It expects `yarn` to be installed.
/// NOTE: only used for the base chains when in Nitro mode, it expects `l1_provider` to have
/// automine enabled
pub async fn deploy_nitro_rollup(
    l1_rpc_url_http: &str,
    rollup_chain_id: u64,
    rollup_owner: Address,
) -> eyre::Result<NitroDeployment> {
    let project_root = env!("CARGO_WORKSPACE_DIR");
    let nitro_contracts_dir = format!("{project_root}/synd-contracts/lib/nitro-contracts");

    // install and build dependencies
    let status = E2EProcess::new(
        Command::new("yarn").current_dir(nitro_contracts_dir.clone()).arg("install"),
        "nitro-contracts-install",
    )?
    .wait()
    .await?;
    assert!(status.success(), "Failed to run `yarn install` in nitro contracts");

    let status = E2EProcess::new(
        Command::new("yarn").current_dir(nitro_contracts_dir.clone()).arg("build:all"),
        "nitro-contracts-build",
    )?
    .wait()
    .await?;
    assert!(status.success(), "Failed to run `yarn build` in nitro contracts");

    let chain_config_json = chain_config(rollup_chain_id, rollup_owner);
    let zero_address = Address::ZERO;

    // setup config.ts (unfortunately, the script expects the config to be in a specific path)
    let config_ts = format!(
        r#"
        import {{ ethers }} from 'ethers'

        // 90% of Geth's 128KB tx size limit, leaving ~13KB for proving
        // This need to be adjusted for Orbit chains
        export const maxDataSize = 117964

        export const isUsingFeeToken = false;

        export const config = {{
            rollupConfig: {{
                confirmPeriodBlocks: ethers.BigNumber.from('45818'),
                extraChallengeTimeBlocks: ethers.BigNumber.from('200'),
                stakeToken: ethers.constants.AddressZero,
                baseStake: ethers.utils.parseEther('1'),
                wasmModuleRoot:    '0xda4e3ad5e7feacb817c21c8d0220da7650fe9051ece68a3f0b1c5d38bbb27b21',
                owner: '{rollup_owner}',
                loserStakeEscrow: ethers.constants.AddressZero,
                chainId: ethers.BigNumber.from('{rollup_chain_id}'),
                minimumAssertionPeriod: 75,
                validatorAfkBlocks: 201600,
                chainConfig: '{chain_config_json}',
                genesisBlockNum: ethers.BigNumber.from('0'),
                sequencerInboxMaxTimeVariation: {{
                    delayBlocks: ethers.BigNumber.from('7200'),
                    futureBlocks: ethers.BigNumber.from('12'),
                    delaySeconds: ethers.BigNumber.from('86400'),
                    futureSeconds: ethers.BigNumber.from('3600'),
                }},
                bufferConfig: {{
                    threshold: ethers.BigNumber.from('600'),
                    max: ethers.BigNumber.from('14400'),
                    replenishRateInBasis: ethers.BigNumber.from('833'),
                }},
            }},
            // validators: [],
            // batchPosterManager: '{zero_address}',
            // batchPosters: []
        }}
    "#
    );
    let config_ts_path = format!("{nitro_contracts_dir}/scripts/config.ts");
    fs::write(config_ts_path, config_ts).await?;

    let l2_chain_config_path = format!("{nitro_contracts_dir}/l2_chain_config.json");
    fs::write(l2_chain_config_path, chain_config_json).await?;

    // deploy the rollup
    let status = E2EProcess::new(
        Command::new("yarn")
            .current_dir(nitro_contracts_dir.clone())
            .arg("run")
            .arg("create-rollup-testnode")
            .arg("--network")
            .arg("custom")
            .env("DEPLOYER_PRIVKEY", PRIVATE_KEY)
            .env("PARENT_CHAIN_RPC", l1_rpc_url_http)
            .env("PARENT_CHAIN_ID", 1.to_string())
            .env("CUSTOM_RPC_URL", l1_rpc_url_http)
            .env("CHILD_CHAIN_NAME", format!("local-rollup-{rollup_chain_id}"))
            .env("CHILD_CHAIN_CONFIG_PATH", "./l2_chain_config.json")
            .env("OWNER_ADDRESS", rollup_owner.to_string())
            .env("SEQUENCER_ADDRESS", rollup_owner.to_string()) // is set as the batch poster manager by default
            .env(
                "WASM_MODULE_ROOT",
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            ),
        "nitro-contracts-deploy-rollup",
    )?
    .wait()
    .await?;
    assert!(status.success(), "failed to deploy rollup");

    // Read the deploy.json file
    let deploy_json_path = format!("{nitro_contracts_dir}/deploy.json");
    let deploy_info: NitroDeployment =
        serde_json::from_reader(std::fs::File::open(deploy_json_path)?)?;

    // Cleanup -  reset the submodule repo - It's annoying to leave pending changes in the submodule
    let status = E2EProcess::new(
        Command::new("git")
            .current_dir(nitro_contracts_dir.clone())
            .arg("checkout")
            .arg("--")
            .arg("hardhat.config.ts"),
        "cleanup-nitro-contracts-submodule-checkout",
    )?
    .wait()
    .await?;
    assert!(status.success(), "failed to cleanup nitro contracts submodule");

    let status = E2EProcess::new(
        Command::new("git").current_dir(nitro_contracts_dir.clone()).arg("clean").arg("-fd"),
        "cleanup-nitro-contracts-submodule",
    )?
    .wait()
    .await?;
    assert!(status.success(), "failed to cleanup nitro contracts submodule");

    Ok(deploy_info)
}
