//! Integration tests for the metabased stack
use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, utils::parse_ether, Address, BlockHash, U256},
    providers::{ext::AnvilApi as _, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, BlockTransactionsKind, TransactionRequest},
};
use block_builder::{
    config::{get_default_private_key_signer, BlockBuilderConfig},
    connectors::mchain::{MetaChainProvider, MCHAIN_ID},
    metrics::BlockBuilderMetrics,
    rollups::{
        arbitrum::{self, arbitrum_adapter::ArbitrumAdapter},
        shared::RollupAdapter,
    },
};
use common::{tracing::init_test_tracing, types::Block};
use contract_bindings::arbitrum::{
    arbgasinfo::ArbGasInfo, arbownerpublic::ArbOwnerPublic, iinbox::IInbox,
};
use e2e_tests::full_meta_node::{launch_nitro_node, start_reth, ContractVersion, MetaNode};
use eyre::{Ok, Result};
use metabased_translator::config::MetabasedConfig;
use metrics::metrics::MetricsState;
use prometheus_client::registry::Registry;
use std::time::Duration;
use test_utils::wait_until;
use tracing::Level;
/// mine a mchain block with a delay - for testing only
async fn mine_block(
    provider: &MetaChainProvider<impl RollupAdapter>,
    delay: u64,
) -> Result<BlockHash> {
    #[allow(clippy::expect_used)]
    let ts = provider
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .expect("failed to get latest block")
        .header
        .timestamp;
    provider.mine_block(ts + delay).await
}

#[tokio::test(flavor = "multi_thread")]
async fn test_rollback() -> Result<()> {
    let (node, _mchain) = start_reth(MCHAIN_ID).await?;
    let block_builder_cfg = BlockBuilderConfig {
        mchain_ipc_path: node.ipc,
        mchain_auth_ipc_path: node.auth_ipc,
        ..Default::default()
    };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
    let mchain = MetaChainProvider::start(
        &block_builder_cfg,
        metrics,
        ArbitrumAdapter::new(&block_builder_cfg),
    )
    .await?;

    let b1 = mchain
        .get_block_by_number(BlockNumberOrTag::Number(1), BlockTransactionsKind::Hashes)
        .await?
        .expect("could not find first block")
        .header
        .hash;
    let b2 = mchain.mine_block(1).await?;
    mchain.mine_block(2).await?;
    let b4 = mchain.mine_block(3).await?;
    let b5 = mchain.mine_block(4).await?;

    assert_eq!(mchain.get_block_number().await?, 5);
    mchain.rollback_to_block(b5).await?;
    assert_eq!(mchain.get_block_number().await?, 5);
    mchain.rollback_to_block(b4).await?;
    assert_eq!(mchain.get_block_number().await?, 4);
    mchain.rollback_to_block(b2).await?;
    assert_eq!(mchain.get_block_number().await?, 2);
    mchain.mine_block(1).await?;
    assert_eq!(mchain.get_block_number().await?, 3);
    mchain.rollback_to_block(b1).await?;
    assert_eq!(mchain.get_block_number().await?, 1);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn arb_owner_test() -> Result<()> {
    const ARB_OWNER_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006b");
    let _ = init_test_tracing(Level::INFO);
    let mut cfg = MetabasedConfig::default();
    let owner = address!("0x0000000000000000000000000000000000000001");
    cfg.block_builder.rollup_owner_address = owner;
    // Start the meta node
    let meta_node = MetaNode::new(None, cfg).await?;
    let arb_owner_public =
        ArbOwnerPublic::new(ARB_OWNER_CONTRACT_ADDRESS, &meta_node.metabased_rollup);
    assert_eq!(arb_owner_public.getAllChainOwners().call().await?._0, [owner]);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn no_l1_fees_test() -> Result<()> {
    const ARB_GAS_INFO_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006c");
    let _ = init_test_tracing(Level::INFO);
    // Start the meta node
    let meta_node = MetaNode::new(None, MetabasedConfig::default()).await?;
    let arb_gas_info = ArbGasInfo::new(ARB_GAS_INFO_CONTRACT_ADDRESS, &meta_node.metabased_rollup);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?._0, U256::ZERO);

    Ok(())
}

// regression test
#[tokio::test(flavor = "multi_thread")]
async fn test_rollback_regression() -> Result<()> {
    let (node, _mchain) = start_reth(MCHAIN_ID).await?;
    let block_builder_cfg = BlockBuilderConfig {
        mchain_ipc_path: node.ipc,
        mchain_auth_ipc_path: node.auth_ipc,
        ..Default::default()
    };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
    let mchain = MetaChainProvider::start(
        &block_builder_cfg,
        metrics,
        ArbitrumAdapter::new(&block_builder_cfg),
    )
    .await?;

    let b1 = mchain.mine_block(1).await?;
    for _ in 0..100 {
        mchain.mine_block(1).await?;
    }
    mchain.rollback_to_block(b1).await?;
    assert_eq!(mchain.get_block_number().await?, 2);

    Ok(())
}

#[allow(clippy::unwrap_used)] // test code
async fn new_test_with_synced_chains() -> Result<MetaNode> {
    // Start the meta node (pre-loaded with the full set of Arb contracts)
    let mut config = MetabasedConfig::default();
    config.slotter.settlement_delay = 0;
    config.settlement.settlement_start_block = 1;
    config.sequencing.sequencing_start_block = 3;
    let meta_node = MetaNode::new(Some(ContractVersion::V300), config).await?;

    // Sync the tips of the sequencing and settlement chains
    let block = meta_node
        .settlement_provider
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
    meta_node
        .sequencing_provider
        .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
        .await?;

    Ok(meta_node)
}
/// This test tests that rollup blocks are properly derived from batches created
/// via the block builder code and posted to the dummy rollup contract.
#[tokio::test(flavor = "multi_thread")]
async fn test_nitro_batch() -> Result<()> {
    let (node, _mchain) = start_reth(MCHAIN_ID).await?;
    let block_builder_cfg = BlockBuilderConfig {
        mchain_ipc_path: node.ipc,
        mchain_auth_ipc_path: node.auth_ipc,
        ..Default::default()
    };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
    let mchain = MetaChainProvider::start(
        &block_builder_cfg,
        metrics,
        ArbitrumAdapter::new(&block_builder_cfg),
    )
    .await?;
    let (_nitro, rollup) = launch_nitro_node(
        block_builder_cfg.target_chain_id,
        block_builder_cfg.rollup_owner_address,
        node.http_port,
    )
    .await?;

    let rollup_contract = mchain.get_rollup();

    // deposit 1 eth
    _ = rollup_contract
        .depositEth(
            Address::default(),
            get_default_private_key_signer().address(),
            parse_ether("1")?,
        )
        .send()
        .await?;
    mine_block(&mchain, 0).await?;

    // send a batch to sequence the deposit.
    _ = rollup_contract
        .postBatch(
            arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
            0,
            U256::from(0),
            0,
            U256::from(0),
        )
        .send()
        .await?;
    mine_block(&mchain, 0).await?;

    // Wait for the batch to be processed and for block 1 to be derived
    wait_until!(rollup.get_block_number().await? == 1, Duration::from_secs(1));

    // check that the deposit succeeded
    assert_eq!(
        rollup.get_balance(get_default_private_key_signer().address()).await?,
        parse_ether("1")?
    );

    // include a tx in a batch
    let mut tx = vec![];
    let inner_tx = TransactionRequest::default()
        .with_to(address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"))
        .with_value(U256::from(1))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&EthereumWallet::from(get_default_private_key_signer()))
        .await?;

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage { header: Default::default(), l2_msg: vec![tx.into()] },
    )]);
    _ = rollup_contract
        .postBatch(batch.encode()?, 0, U256::from(0), 0, U256::from(0))
        .send()
        .await?;
    mchain.mine_block(0).await?;

    // Wait for batch processing to complete and block 2 to be derived
    wait_until!(rollup.get_block_number().await? == 2, Duration::from_millis(500));

    // check that the tx was sequenced
    let block: Block = rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    assert_eq!(block.transactions.len(), 2);
    // tx hash should match
    assert_eq!(block.transactions[1].hash, *inner_tx.tx_hash());

    Ok(())
}

/// Regression test
#[tokio::test(flavor = "multi_thread")]
async fn test_nitro_batch_two_tx() -> Result<()> {
    let (node, _mchain) = start_reth(MCHAIN_ID).await?;
    let block_builder_cfg = BlockBuilderConfig {
        mchain_ipc_path: node.ipc,
        mchain_auth_ipc_path: node.auth_ipc,
        ..Default::default()
    };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
    let mchain = MetaChainProvider::start(
        &block_builder_cfg,
        metrics,
        ArbitrumAdapter::new(&block_builder_cfg),
    )
    .await?;
    let (_nitro, rollup) = launch_nitro_node(
        block_builder_cfg.target_chain_id,
        block_builder_cfg.rollup_owner_address,
        node.http_port,
    )
    .await?;
    let rollup_contract = mchain.get_rollup();

    // deposit 1 eth
    _ = rollup_contract
        .depositEth(
            Address::default(),
            get_default_private_key_signer().address(),
            parse_ether("1")?,
        )
        .send()
        .await?;
    mine_block(&mchain, 0).await?;

    // send a batch to sequence the deposit.
    _ = rollup_contract
        .postBatch(
            arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
            0,
            U256::from(0),
            0,
            U256::from(0),
        )
        .send()
        .await?;
    mine_block(&mchain, 0).await?;

    // Wait for the batch to be processed and for block 1 to be derived
    wait_until!(rollup.get_block_number().await? == 1, Duration::from_millis(500));

    // check that the deposit succeeded
    assert_eq!(
        rollup.get_balance(get_default_private_key_signer().address()).await?,
        parse_ether("1")?
    );

    // include two tx in a batch
    let mut tx = vec![];
    let mut tx2 = vec![];
    let inner_tx = TransactionRequest::default()
        .with_to(address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"))
        .with_value(U256::from(0))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&EthereumWallet::from(get_default_private_key_signer()))
        .await?;

    let second_tx = TransactionRequest::default()
        .with_to(address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"))
        .with_value(U256::from(0))
        .with_nonce(1)
        .with_gas_limit(100_000)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&EthereumWallet::from(get_default_private_key_signer()))
        .await?;

    second_tx.encode_2718(&mut tx2);

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage {
            header: Default::default(),
            l2_msg: vec![tx.clone().into(), tx.into(), tx2.into()],
        },
    )]);
    _ = rollup_contract
        .postBatch(batch.encode()?, 0, U256::from(0), 0, U256::from(0))
        .send()
        .await?;
    mchain.mine_block(0).await?;

    // Wait for the batch to be processed and for block 2 to be derived
    wait_until!(rollup.get_block_number().await? == 2, Duration::from_millis(500));

    // check that the tx was sequenced
    let block: Block = rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    assert_eq!(block.transactions.len(), 3);
    // tx hash should match
    assert_eq!(block.transactions[1].hash, *inner_tx.tx_hash());
    assert_eq!(block.transactions[2].hash, *second_tx.tx_hash());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_settlement_reorg() -> Result<()> {
    let _ = init_test_tracing(Level::DEBUG);
    let meta_node = new_test_with_synced_chains().await?;

    // NOTE: at this point the mchain is on block 1 (initial mchain block) - we can't reorg to
    // genesis, so we need to create two slots on top of it before the reorg happens.

    meta_node.mine_both(1).await?; //mine mchain block 2 (only works because there are delayed txs to proccess, otherwise the
                                   // mchain block would be empty/skipped)
                                   // Wait for mchain to reach block 2
    wait_until!(
        meta_node.mchain_provider.get_block_number().await? == 2,
        Duration::from_millis(500)
    );

    let wallet_address = meta_node.settlement_provider.default_signer_address();
    let inbox = IInbox::new(meta_node.inbox_address, &meta_node.settlement_provider);

    // create a deposit1 (that won't be rolled back) that will fit on mchain's block 3
    let balance_before_deposit = meta_node.metabased_rollup.get_balance(wallet_address).await?;
    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

    meta_node.mine_both(100).await?;
    meta_node.mine_both(1).await?; // extra blocks to close the slot

    // Wait for deposit1 to be processed
    wait_until!(
        meta_node.metabased_rollup.get_balance(wallet_address).await? ==
            balance_before_deposit + parse_ether("1")?,
        Duration::from_millis(500)
    );

    // send a deposit2 that will be reorged
    let mchain_block_before_deposit = meta_node
        .mchain_provider
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
    assert_eq!(mchain_block_before_deposit.header.number, 3);

    let balance_before_deposit = meta_node.metabased_rollup.get_balance(wallet_address).await?;

    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

    // make this deposit2 fit into a slot that will be reorged
    meta_node.mine_both(200).await?; // will be reorged leaving this slot opened
    meta_node.mine_both(1).await?; // mine an extra block to close the slot (will be reorged later)

    // Wait for deposit2 to be processed
    wait_until!(
        meta_node.metabased_rollup.get_balance(wallet_address).await? ==
            balance_before_deposit + parse_ether("1")?,
        Duration::from_millis(500)
    );

    assert_eq!(meta_node.mchain_provider.get_block_number().await?, 4);

    // the rollup head has not been updated yet
    let rollup_head_before_reorg = meta_node
        .metabased_rollup
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .unwrap();

    // reorg, assert that the L3 balance has disappeared
    let reorg_depth = 2;
    let current_block = meta_node.settlement_provider.get_block_number().await?;
    meta_node.settlement_provider.anvil_rollback(Some(reorg_depth)).await?;
    assert_eq!(
        meta_node.settlement_provider.get_block_number().await?,
        current_block - reorg_depth
    );

    //NOTE: mine an extra block. (the ingestor currently polls by block number, so it won't detect
    // a reorg at a given block height, until a block with a bigger height is mined)
    // if we switch to a different ingestor implementation this could be removed.

    for _ in 0..reorg_depth + 1 {
        meta_node.mine_set_block(0).await?;
    }

    // Wait for reorg to be processed
    wait_until!(
            let block = meta_node
                .mchain_provider
                .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
                .await?
                .unwrap();
            block == mchain_block_before_deposit
        ,
        Duration::from_millis(500)
    );

    // mchain should have reorged to a pre-deposit block
    assert_eq!(
        meta_node
            .mchain_provider
            .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
            .await?
            .unwrap(),
        mchain_block_before_deposit
    );
    // the rollup should have reorged to a pre-deposit block

    // create new slots
    _ = inbox.depositEth().value(parse_ether("0.01")?).send().await?;
    meta_node.mine_both(500).await?;
    meta_node.mine_both(500).await?; // build mchain to an height above what the rollup has seen before the reorg

    wait_until!(let rollup_head = meta_node
        .metabased_rollup
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
        rollup_head.header.number == rollup_head_before_reorg.header.number &&
        rollup_head.header.hash != rollup_head_before_reorg.header.hash,
        Duration::from_secs(10)
    );
    assert_eq!(meta_node.mchain_provider.get_block_number().await?, 4);

    // balance should be correct (reorged deposit is not included)
    let balance_after_reorg = meta_node.metabased_rollup.get_balance(wallet_address).await?;
    assert_eq!(balance_after_reorg, balance_before_deposit + parse_ether("0.01")?);

    Ok(())
}
