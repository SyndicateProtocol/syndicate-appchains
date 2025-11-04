use crate::e2e::e2e_tests::Storage;
use alloy::{
    eips::BlockId,
    primitives::{utils::parse_ether, Address, U160, U256},
    providers::{ext::AnvilApi, Provider},
    rpc::types::TransactionRequest,
};
use contract_bindings::synd::{
    arb_owner::ArbOwner,
    i_bridge::IBridge,
    i_inbox::IInbox,
    i_sequencer_inbox::ISequencerInbox,
    syndicate_sequencing_chain::SyndicateSequencingChain::{
        self, SyndicateSequencingChainInstance,
    },
};
use eyre::Result;
use shared::types::FilledProvider;
use std::{path::PathBuf, time::Duration};
use synd_mchain::methods::common::{APPCHAIN_CONTRACT, MCHAIN_ID};
use synd_migration::migration::{get_migration_data, RollupState};
use test_framework::components::{
    chain_ingestor::ChainIngestorConfig,
    configuration::{setup_config_manager, ConfigurationOptions},
    test_components::{SEQUENCING_CHAIN_ID, SETTLEMENT_CHAIN_ID},
    translator::TranslatorConfig,
};
use test_utils::{
    anvil::{mine_block, start_anvil_with_args},
    chain_info::{test_account1, test_account8, ChainInfo},
    docker::{
        launch_nitro_node, start_component, start_eigenda_proxy, start_mchain, E2EProcess,
        NitroNodeArgs, NitroSequencerMode,
    },
    nitro_chain::{deploy_nitro_rollup, NitroDeployment, ARB_OWNER_PRECOMPILE_ADDRESS},
    port_manager::PortManager,
    utils::test_path,
    wait_until,
};
use tracing::info;

async fn start_base_chain(chain_id: u64) -> Result<ChainInfo> {
    let chain_info = start_anvil_with_args(chain_id, &[]).await?;
    chain_info.provider.anvil_set_auto_mine(true).await?;
    Ok(chain_info)
}

async fn deploy_sequencing_contract<P: Provider>(
    provider: P,
    appchain_chain_id: U256,
) -> Result<SyndicateSequencingChainInstance<P>> {
    let contract_instance = SyndicateSequencingChain::deploy(provider, appchain_chain_id).await?;
    assert!(contract_instance
        .updateRequirementModule(U160::from(1).into())
        .send()
        .await?
        .get_receipt()
        .await?
        .status());
    Ok(contract_instance)
}

#[allow(dead_code)]
struct SyndicateStack {
    mchain_rpc_url: String,
    mchain: E2EProcess,
    config_manager_address: Address,
    sequencing_chain_ingestor: E2EProcess,
    settlement_chain_ingestor: E2EProcess,
    translator: E2EProcess,
}

async fn spin_up_syndicate_stack(
    appchain_chain_id: u64,
    appchain_owner: Address,
    set_provider: &FilledProvider,
    sequencing_contract_address: Address,
    appchain_deployment: NitroDeployment,
    sequencing_rpc_url: String,
    settlement_rpc_url: String,
    migration_data: RollupState,
) -> Result<SyndicateStack> {
    let opt = ConfigurationOptions {
        appchain_chain_id,
        base_chains_type: test_framework::components::configuration::BaseChainsType::Nitro,
        rollup_owner: appchain_owner,
        ..Default::default()
    };
    let (mchain_rpc_url, mchain, mchain_provider) =
        start_mchain(appchain_chain_id, opt.finality_delay).await?;

    println!("mchain started");

    // Setup config manager and get chain config address
    let config_manager_address = setup_config_manager(
        set_provider,
        &opt,
        sequencing_contract_address,
        appchain_deployment.bridge,
        appchain_deployment.inbox,
        sequencing_rpc_url.clone(),
        "https://example.com/explorer".to_string(),
        Some(migration_data.clone()),
    )
    .await?;

    println!("config manager address: {config_manager_address}");

    let temp = test_path("chain_ingestor");
    let seq_chain_ingestor_cfg = ChainIngestorConfig {
        ws_urls: vec![sequencing_rpc_url.clone()],
        db_file: temp.clone() + "/sequencing_chain.db",
        start_block: 0,
        port: PortManager::instance().next_port().await,
        metrics_port: PortManager::instance().next_port().await,
    };

    let sequencing_chain_ingestor = start_component(
        "synd-chain-ingestor",
        seq_chain_ingestor_cfg.port,
        seq_chain_ingestor_cfg.cli_args(),
        Default::default(),
    )
    .await?;

    let set_chain_ingestor_cfg = ChainIngestorConfig {
        ws_urls: vec![settlement_rpc_url.clone()],
        db_file: temp + "/settlement_chain.db",
        start_block: migration_data.block_number,
        port: PortManager::instance().next_port().await,
        metrics_port: PortManager::instance().next_port().await,
    };

    let settlement_chain_ingestor = start_component(
        "synd-chain-ingestor",
        set_chain_ingestor_cfg.port,
        set_chain_ingestor_cfg.cli_args(),
        Default::default(),
    )
    .await?;

    let sequencing_ingestor_rpc_url = format!("ws://localhost:{}", seq_chain_ingestor_cfg.port);
    let settlement_ingestor_rpc_url = format!("ws://localhost:{}", set_chain_ingestor_cfg.port);

    println!("sequencing_ingestor_rpc_url: {sequencing_ingestor_rpc_url}");

    let translator_config = TranslatorConfig {
        settlement_ws_url: settlement_ingestor_rpc_url.clone(),
        config_manager_address: Some(config_manager_address),
        appchain_chain_id: Some(opt.appchain_chain_id),
        mchain_ws_url: mchain_rpc_url.clone(),
        port: PortManager::instance().next_port().await,
        // Needs to be provided as it needs to be the ingestor's URL
        sequencing_ws_url: Some(sequencing_ingestor_rpc_url.clone()),
        settlement_delay: Some(60),
        // NOTE: do not fill the values that are meant to be filled by the config manager
        // contract
        ..Default::default()
    };

    let translator = start_component(
        "synd-translator",
        translator_config.port,
        translator_config.cli_args(),
        vec![],
    )
    .await?;

    println!("translator started");

    let syndicate_stack = SyndicateStack {
        mchain_rpc_url,
        mchain,
        config_manager_address,
        sequencing_chain_ingestor,
        settlement_chain_ingestor,
        translator,
    };
    Ok(syndicate_stack)
}

#[tokio::test]
async fn e2e_migration() -> Result<()> {
    let appchain_chain_id = 15u64;
    let appchain_owner = test_account1();
    let batch_poster = test_account8();
    let test_user = test_account1(); // TODO try to use test_account9 instead

    let set_chain = start_base_chain(SETTLEMENT_CHAIN_ID).await?;
    let seq_chain = start_base_chain(SEQUENCING_CHAIN_ID).await?;
    let sequencing_contract =
        deploy_sequencing_contract(seq_chain.provider.clone(), U256::from(appchain_chain_id))
            .await?;

    let appchain_deployment = deploy_nitro_rollup(
        &set_chain.http_url,
        appchain_chain_id,
        appchain_owner.address,
        vec![batch_poster.address],
        true,
    )
    .await?;

    let (_instance, eigenda_proxy_url) = start_eigenda_proxy().await?;

    let data_dir = test_path("nitro");

    // setup a normal arb rollup on an anvil chain
    let appchain = launch_nitro_node(NitroNodeArgs {
        chain_id: appchain_chain_id,
        chain_owner: appchain_owner.address,
        parent_chain_url: set_chain.ws_url.clone(),
        parent_chain_id: SETTLEMENT_CHAIN_ID,
        sequencer_mode: NitroSequencerMode::EigenDASequencer(eigenda_proxy_url.clone()),
        chain_name: "appchain".to_string(),
        deployment: appchain_deployment.clone(),
        sequencer_private_key: Some(batch_poster.private_key.to_string()),
        data_dir: Some(data_dir.clone()),
    })
    .await?;

    // --

    // deposit some funds for the default signer
    let inbox = IInbox::new(appchain_deployment.inbox, &set_chain.provider);
    let _ = inbox.depositEth().value(parse_ether("10")?).send().await?;

    // wait until those funds arrive on the chain
    wait_until!(
        appchain.provider.get_balance(test_user.address).await? >= parse_ether("10")?,
        Duration::from_secs(10)
    );

    let storage_contract_address =
        *Storage::deploy(appchain.provider.clone(), U256::from(42)).await?.address();

    let arb_sequencer_inbox =
        ISequencerInbox::new(appchain_deployment.sequencer_inbox, set_chain.provider.clone());

    // wait for a batch to be posted
    wait_until!(arb_sequencer_inbox.batchCount().call().await? == 2, Duration::from_secs(20));

    let bridge = IBridge::new(appchain_deployment.bridge, &set_chain.provider);
    let delayed_msgs_count = bridge.delayedMessageCount().call().await?;
    let delayed_msgs_acc =
        bridge.delayedInboxAccs(delayed_msgs_count - U256::from(1)).call().await?;
    let batch_count = bridge.sequencerMessageCount().call().await?;
    info!("batch_count: {batch_count}");
    let batch_acc = bridge.sequencerInboxAccs(batch_count - U256::from(1)).call().await?;
    let before_batch_acc = bridge.sequencerInboxAccs(batch_count - U256::from(2)).call().await?;

    // shutdown the nitro node
    drop(appchain);

    // run the migration cli code to obtain migration data from the nitro node
    let mut migration_data: RollupState = Default::default();
    wait_until!(
        {
            match get_migration_data(&PathBuf::from(data_dir.clone()).join("appchain/nitro")).await
            {
                Ok(data) => {
                    migration_data = data;
                    true
                }
                Err(e) => {
                    println!("Failed to get migration data: {e}");
                    false
                }
            }
        },
        Duration::from_secs(10),
        Duration::from_secs(1)
    );

    assert!(migration_data.batch_acc == batch_acc);
    assert!(migration_data.batch_count == 2);
    assert!(migration_data.delayed_msgs_acc == delayed_msgs_acc);
    assert!(U256::from(migration_data.delayed_msgs_count) == delayed_msgs_count);

    // migrate the bridge contract
    // - TODO Remove validators and stakers
    // - TODO Set the upgradeExecutor role to the assertionPoster

    // spin up the syndicate stack
    let syndicate_stack = spin_up_syndicate_stack(
        appchain_chain_id,
        appchain_owner.address,
        &set_chain.provider.clone(),
        *sequencing_contract.address(),
        appchain_deployment.clone(),
        seq_chain.ws_url.clone(),
        set_chain.ws_url.clone(),
        migration_data.clone(),
    )
    .await?;

    let migrated_appchain_deployment = NitroDeployment {
        bridge: APPCHAIN_CONTRACT,
        inbox: APPCHAIN_CONTRACT,
        sequencer_inbox: APPCHAIN_CONTRACT,
        rollup: APPCHAIN_CONTRACT,
        native_token: Address::ZERO,
        upgrade_executor: APPCHAIN_CONTRACT,
        validator_utils: APPCHAIN_CONTRACT,
        validator_wallet_creator: APPCHAIN_CONTRACT,
        deployed_at: 1,
    };

    // Wake nitro up
    let migrated_appchain = launch_nitro_node(NitroNodeArgs {
        chain_id: appchain_chain_id,
        chain_owner: appchain_owner.address,
        parent_chain_url: syndicate_stack.mchain_rpc_url.clone(),
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: migrated_appchain_deployment.clone(),
        sequencer_private_key: None,
        data_dir: Some(data_dir.clone()),
    })
    .await?;

    let storage_contract =
        Storage::new(storage_contract_address, migrated_appchain.provider.clone());
    let initial_value = storage_contract.get().call().await?;
    assert_eq!(initial_value, U256::from(42));

    // assert new txs work
    let nonce = migrated_appchain.provider.get_transaction_count(test_user.address).await?;
    println!("JORGE<3POTATOS!!");
    let update_val_raw_tx = storage_contract
        .set(U256::from(43))
        .nonce(nonce)
        .gas(100_000)
        .max_fee_per_gas(100000000)
        .max_priority_fee_per_gas(0)
        .chain_id(appchain_chain_id)
        .build_raw_transaction(test_user.signer.clone())
        .await?;
    assert!(sequencing_contract
        .processTransaction(update_val_raw_tx.into())
        .send()
        .await?
        .get_receipt()
        .await?
        .status());
    wait_until!(storage_contract.get().call().await? == U256::from(43), Duration::from_secs(10));

    // TODO ^ this sometimes fails, look for `reorgingSequencer=true`... needs to be investigated

    // deposit again, assert it works
    // let _ = inbox.depositEth().value(parse_ether("10")?).send().await?;
    // mine_block(&set_chain.provider.clone(), 1).await?;
    // mine_block(&seq_chain.provider.clone(), 0).await?;
    // mine_block(&seq_chain.provider.clone(), 70).await?;
    // mine_block(&set_chain.provider.clone(), 70).await?;

    // we need to produce an extra tx to seal the slot with the deposit
    // let cur_ts =
    // set_chain.provider.get_block(BlockId::latest()).await?.unwrap().header.timestamp;
    // set_chain.provider.anvil_set_next_block_timestamp(cur_ts + 100).await?; // set_delay is 60
    // assert!(set_chain
    //     .provider
    //     .send_transaction(TransactionRequest::default().to(Address::ZERO).
    // value(U256::from(0u64)))     .await?
    //     .get_receipt()
    //     .await?
    //     .status());

    // mine_block(&seq_chain.provider.clone(), 70).await?;

    // println!("waiting for balance to be greater than 19");
    // println!(
    //     "test_user balance before: {}",
    //     migrated_appchain.provider.get_balance(test_user.address).await?
    // );
    // wait_until!(
    //     // 10 + 10 - gas fees
    //     {
    //         let balance = migrated_appchain.provider.get_balance(test_user.address).await?;
    //         println!("balance: {balance}");
    //         balance > parse_ether("19")?
    //     },
    //     Duration::from_secs(10)
    // );

    // assert `arbOwner.setL1PricePerUnit(0) ` works
    // assert!(ArbOwner::new(ARB_OWNER_PRECOMPILE_ADDRESS, &migrated_appchain.provider)
    //     .setL1PricePerUnit(U256::ZERO)
    //     .send()
    //     .await?
    //     .get_receipt()
    //     .await?
    //     .status());

    // assert new txs work after setPricePerUnit is called
    // (also assert the standard nitro -> sequencer flow works)
    // assert!(storage_contract.set(U256::from(44)).send().await?.get_receipt().await?.status());
    // assert!(storage_contract.get().call().await? == U256::from(44));
    // // TODO ^ this will not pass until we setup the sequencer (it's sending directly to nitro)

    // // assert sendL2MessageFromOrigin (WITHOUT THE custom event fork) works
    // let nonce = migrated_appchain.provider.get_transaction_count(test_user.address).await?;
    // let update_val_raw_tx = storage_contract
    //     .set(U256::from(45))
    //     .nonce(nonce)
    //     .gas(100_000)
    //     .max_fee_per_gas(100000000)
    //     .max_priority_fee_per_gas(0)
    //     .chain_id(appchain_chain_id)
    //     .build_raw_transaction(test_user.signer.clone())
    //     .await?;

    // // TODO this is not expected to pass yet
    // assert!(inbox
    //     .sendL2MessageFromOrigin(update_val_raw_tx.into())
    //     .send()
    //     .await?
    //     .get_receipt()
    //     .await?
    //     .status());
    // wait_until!(storage_contract.get().call().await? == U256::from(45), Duration::from_secs(10));

    // assert withdrawals work (TBD)

    // TODO InboxMessageDeliveredFromOrigin needs to be handled in the enclave too... test an
    // withdrawal triggered this way

    Ok(())
}
