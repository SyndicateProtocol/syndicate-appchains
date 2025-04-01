//! Integration tests for the metabased stack
use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, utils::parse_ether, Address, BlockHash, U256},
    providers::Provider,
    rpc::types::{BlockTransactionsKind, TransactionRequest},
};
use block_builder::{
    config::BlockBuilderConfig,
    connectors::mchain::{MetaChainProvider, MCHAIN_ID},
    metrics::BlockBuilderMetrics,
    rollups::{
        arbitrum::{self, arbitrum_adapter::ArbitrumAdapter},
        shared::RollupAdapter,
    },
};
use common::types::Block;
use eyre::{Ok, Result};
use metrics::metrics::MetricsState;
use prometheus_client::registry::Registry;
use std::time::Duration;
use test_utils::{
    docker::{launch_nitro_node, start_reth},
    preloaded_config::get_default_private_key_signer,
    utils::assert_eventually,
    wait_until,
};

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
    let (node, _mchain) = start_reth(MCHAIN_ID, "2.0.0").await?;
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

// regression test
#[tokio::test(flavor = "multi_thread")]
async fn test_rollback_regression() -> Result<()> {
    let (node, _mchain) = start_reth(MCHAIN_ID, "2.0.0").await?;
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

/// This test tests that rollup blocks are properly derived from batches created
/// via the block builder code and posted to the dummy rollup contract.
#[tokio::test(flavor = "multi_thread")]
async fn test_nitro_batch() -> Result<()> {
    let (node, _mchain) = start_reth(MCHAIN_ID, "2.0.0").await?;
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
    let (_nitro, rollup, _url) = launch_nitro_node(
        block_builder_cfg.target_chain_id,
        block_builder_cfg.rollup_owner_address,
        node.http_port,
        None,
        "v3.4.0-d896e9c-slim",
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
    let (node, _mchain) = start_reth(MCHAIN_ID, "2.0.0").await?;
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
    let (_nitro, rollup, _url) = launch_nitro_node(
        block_builder_cfg.target_chain_id,
        block_builder_cfg.rollup_owner_address,
        node.http_port,
        None,
        "v3.4.0-d896e9c-slim",
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
