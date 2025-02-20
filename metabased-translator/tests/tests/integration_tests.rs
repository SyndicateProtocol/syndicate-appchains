//! Integration tests for the metabased stack

use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{ext::AnvilApi as _, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, TransactionRequest},
};
use block_builder::{
    config::{get_rollup_contract_address, BlockBuilderConfig},
    connectors::{
        anvil::{MetaChainProvider, MCHAIN_ID},
        metrics::MChainMetrics,
    },
    rollups::arbitrum,
};
use common::types::Block;
use contract_bindings::arbitrum::{iinbox::IInbox, rollup::Rollup};
use e2e_tests::full_meta_node::{
    launch_nitro_node, mine_block, start_reth, MetaNode, PRELOAD_INBOX_ADDRESS,
};
use eyre::{eyre, Result};
use metabased_translator::config::MetabasedConfig;
use metrics::metrics::MetricsState;
use prometheus_client::registry::Registry;
use serial_test::serial;
use std::time::Duration;
use tokio::time::sleep;

/// This test sends different types of delayed messages
/// via the inbox contract and ensures that all of them
/// are sequenced via the metabased translator and show up on the rollup.
/// TODO: remove the serial attribute once the reth port conflicts are fixed.
#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn e2e_settlement_test() -> Result<()> {
    // Start the meta node (port index 0, pre-loaded with the full set of Arb contracts)
    let mut config = MetabasedConfig::default();
    config.slotter.settlement_delay = 0;
    config.slotter.slot_duration = 1000;
    config.slotter.start_slot_timestamp = 1736824187; // timestamp of settlement block 1
    config.settlement.settlement_start_block = 1;
    config.sequencing.sequencing_start_block = 3;
    let meta_node = MetaNode::new(true, config).await?;

    // Sync the tips of the sequencing and settlement chains
    let seq_block: Block = meta_node
        .settlement_provider
        .raw_request("eth_getBlockByNumber".into(), ("latest", true))
        .await?;
    meta_node
        .sequencing_provider
        .evm_mine(Some(MineOptions::Timestamp(Some(seq_block.timestamp))))
        .await?;

    // Grab the wallet address for the test
    let wallet_address = meta_node.settlement_provider.default_signer_address();

    // Send a deposit (unaliased address) delayed message
    // Deposit is from the arbos address and does not increment the nonce
    let inbox = IInbox::new(PRELOAD_INBOX_ADDRESS, &meta_node.settlement_provider);
    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

    const L2_MESSAGE_KIND_SIGNED_TX: u8 = 4;
    let gas_limit: u64 = 100_000;
    let max_fee_per_gas: u128 = 100_000_000;

    // Send l2 signed messages (unaliased address)
    // Message (not from origin)
    let mut inner_tx = vec![];
    TransactionRequest::default()
        .with_to(Address::ZERO)
        .with_value(parse_ether("0.1")?)
        .with_nonce(0)
        .with_gas_limit(gas_limit)
        .with_chain_id(meta_node.chain_id)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(0)
        .build(meta_node.settlement_provider.wallet())
        .await?
        .encode_2718(&mut inner_tx);
    let mut tx = vec![L2_MESSAGE_KIND_SIGNED_TX];
    tx.append(&mut inner_tx);
    _ = inbox.sendL2Message(tx.into()).send().await?;
    // Message From Origin
    inner_tx = vec![];
    TransactionRequest::default()
        .with_to(Address::ZERO)
        .with_value(parse_ether("0.1")?)
        .with_nonce(1)
        .with_gas_limit(gas_limit)
        .with_chain_id(meta_node.chain_id)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(0)
        .build(meta_node.settlement_provider.wallet())
        .await?
        .encode_2718(&mut inner_tx);
    tx = vec![L2_MESSAGE_KIND_SIGNED_TX];
    tx.append(&mut inner_tx);
    _ = inbox.sendL2MessageFromOrigin(tx.into()).send().await?;

    // Send retryable tickets that are automatically redeemed (aliased address)
    // Safe Retryable Ticket
    _ = inbox
        .createRetryableTicket(
            wallet_address,
            U256::ZERO,
            parse_ether("0.00001")?,
            wallet_address,
            wallet_address,
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;
    // Unsafe Retryable Ticket
    _ = inbox
        .unsafeCreateRetryableTicket(
            wallet_address,
            U256::ZERO,
            parse_ether("0.00001")?,
            wallet_address,
            wallet_address,
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;

    // Send 2 l2 unsigned messages (aliased address)
    // Unsigned Transaction
    _ = inbox
        .sendUnsignedTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            U256::from(2),
            wallet_address,
            parse_ether("0.9")?,
            Default::default(),
        )
        .send()
        .await?;
    // Contract Transaction
    _ = inbox
        .sendContractTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            wallet_address,
            parse_ether("0.9")?,
            Default::default(),
        )
        .send()
        .await?;

    // Send 2 l2 funded by l1 messages (aliased address)
    // Funded Unsigned Transaction
    _ = inbox
        .sendL1FundedUnsignedTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            U256::from(4),
            wallet_address,
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;
    // Funded Contract Transaction
    _ = inbox
        .sendL1FundedContractTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            wallet_address,
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;
    meta_node.mine_set_blocks(0).await?;

    // Mine blocks to process the slot
    meta_node.mine_next_slot().await?;
    sleep(Duration::from_millis(1000)).await;

    assert_eq!(meta_node.metabased_rollup.get_block_number().await?, 17);
    assert_eq!(
        meta_node
            .metabased_rollup
            .get_balance(meta_node.settlement_provider.default_signer_address())
            .await?,
        parse_ether("4.6000316")?
    );

    Ok(())
}

/// This test tests the sequencing contract to make sure
/// blocks sequenced via the sequencing contract show up
/// on the rollup. It also checks to make sure missing slots
/// sequence a mchain block that does not include a batch.
/// TODO: remove the serial attribute once the reth port conflicts are fixed.
#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn e2e_test() -> Result<()> {
    // Start the meta node (port index 1, no pre-loaded contracts)
    let mut config = MetabasedConfig::default();
    config.slotter.settlement_delay = 0;
    let meta_node = MetaNode::new(false, config).await?;
    // Setup the settlement rollup contract
    let set_rollup = Rollup::new(get_rollup_contract_address(), &meta_node.settlement_provider);

    // Send a deposit and a regular tx at slot 1 (mchain block 2, rollup block 1)
    _ = set_rollup
        .depositEth(
            meta_node.sequencing_provider.default_signer_address(),
            meta_node.sequencing_provider.default_signer_address(),
            parse_ether("1")?,
        )
        .send()
        .await?;

    // Send a sequenced tx at slot 1 (mchain block 2, rollup 2)
    let mut tx = vec![];
    let inner_tx = TransactionRequest::default()
        .with_to(address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"))
        .with_value(U256::from(1))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(meta_node.chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(meta_node.sequencing_provider.wallet())
        .await?;
    inner_tx.encode_2718(&mut tx);
    _ = meta_node.sequencing_contract.processTransaction(tx.into()).send().await?;

    // insert a block to slot 1 @ ts 0, slot 2 @ ts 1. mine slot 1 -> mchain block 2.
    meta_node.mine_both(0).await?;
    meta_node.mine_next_slot().await?;
    sleep(Duration::from_millis(1000)).await;

    // check mchain blocks
    assert_eq!(meta_node.mchain_provider.get_block_number().await?, 2);
    let mchain_block: Block = meta_node
        .mchain_provider
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    assert_eq!(mchain_block.timestamp, 0);
    assert_eq!(mchain_block.transactions.len(), 2);
    // check rollup blocks
    assert_eq!(meta_node.metabased_rollup.get_block_number().await?, 2);
    // check the first rollup block
    let rollup_block: Block = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(1), true))
        .await?;
    assert_eq!(rollup_block.timestamp, 0);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // check the second rollup block
    let rollup_block: Block = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    assert_eq!(rollup_block.timestamp, 0);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // tx hash should match
    assert_eq!(rollup_block.transactions[1].hash, *inner_tx.tx_hash());

    let test_addr: Address = "0xA9ec1Ed7008fDfdE38978Dfef4cF2754A969E5FA".parse()?;

    // Send another delayed message one slot out. The missing slot should sequence an empty block.
    _ = set_rollup
        .depositEth(
            meta_node.sequencing_provider.default_signer_address(),
            test_addr,
            parse_ether("1")?,
        )
        .send()
        .await?;
    meta_node.mine_next_slot().await?;
    meta_node.mine_next_slot().await?;
    sleep(Duration::from_millis(2000)).await;

    // check mchain blocks
    assert_eq!(meta_node.mchain_provider.get_block_number().await?, 4);
    // check mchain block 3
    let mchain_block: Block = meta_node
        .mchain_provider
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(3), true))
        .await?;
    assert_eq!(mchain_block.timestamp, meta_node.slot_duration);
    assert_eq!(mchain_block.transactions.len(), 0);
    // check mchain block 4
    let mchain_block: Block = meta_node
        .mchain_provider
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(4), true))
        .await?;
    assert_eq!(mchain_block.timestamp, meta_node.slot_duration * 2);
    assert_eq!(mchain_block.transactions.len(), 2);
    // check rollup block 3
    assert_eq!(meta_node.metabased_rollup.get_block_number().await?, 3);
    let rollup_block: Block = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(3), true))
        .await?;
    assert_eq!(rollup_block.timestamp, meta_node.slot_duration * 2);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // balance should match
    assert_eq!(meta_node.metabased_rollup.get_balance(test_addr).await?, parse_ether("1")?);

    Ok(())
}

/// This test tests that rollup blocks are properly derived from batches created
/// via the block builder code and posted to the dummy rollup contract.
/// TODO: remove the serial attribute once the reth port conflicts are fixed
#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_nitro_batch() -> Result<()> {
    let block_builder_cfg =
        BlockBuilderConfig { mchain_url: "http://127.0.0.1:8388".parse()?, ..Default::default() };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = MChainMetrics::new(&mut metrics_state.registry);
    let _mchain = start_reth(block_builder_cfg.mchain_url.port().unwrap(), MCHAIN_ID).await?;
    let mchain = MetaChainProvider::start(&block_builder_cfg, &metrics).await?;
    let (_nitro, rollup) = launch_nitro_node(&mchain, 8347).await?;

    // deposit 1 eth
    _ = mchain
        .rollup
        .depositEth(Address::default(), mchain.provider.default_signer_address(), parse_ether("1")?)
        .send()
        .await?;
    mine_block(&mchain.provider, 0).await?;

    // send a batch to sequence the deposit.
    _ = mchain
        .rollup
        .postBatch(arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?)
        .send()
        .await?;
    mine_block(&mchain.provider, 0).await?;

    // wait 20ms for the batch to be processed
    sleep(Duration::from_millis(20)).await;
    if rollup.get_block_number().await? != 1 {
        return Err(eyre!("block derivation failed - not on block 1"));
    }

    // check that the deposit succeeded
    assert_eq!(
        rollup.get_balance(mchain.provider.default_signer_address()).await?,
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
        .build(&mchain.provider.wallet())
        .await?;

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage { header: Default::default(), l2_msg: vec![tx.into()] },
    )]);
    _ = mchain.rollup.postBatch(batch.encode()?).send().await?;
    mine_block(&mchain.provider, 0).await?;

    // wait for the batch to be processed
    sleep(Duration::from_millis(200)).await;
    if rollup.get_block_number().await? != 2 {
        return Err(eyre!("block derivation failed - not on block 2"));
    }

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
/// TODO: remove the serial attribute once the reth port conflicts are fixed
#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_nitro_batch_two_tx() -> Result<()> {
    let block_builder_cfg =
        BlockBuilderConfig { mchain_url: "http://127.0.0.1:8488".parse()?, ..Default::default() };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = MChainMetrics::new(&mut metrics_state.registry);
    let _mchain = start_reth(block_builder_cfg.mchain_url.port().unwrap(), MCHAIN_ID).await?;
    let mchain = MetaChainProvider::start(&block_builder_cfg, &metrics).await?;
    let (_nitro, rollup) = launch_nitro_node(&mchain, 8447).await?;

    // deposit 1 eth
    _ = mchain
        .rollup
        .depositEth(Address::default(), mchain.provider.default_signer_address(), parse_ether("1")?)
        .send()
        .await?;
    mine_block(&mchain.provider, 0).await?;

    // send a batch to sequence the deposit.
    _ = mchain
        .rollup
        .postBatch(arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?)
        .send()
        .await?;
    mine_block(&mchain.provider, 0).await?;

    // wait 20ms for the batch to be processed
    sleep(Duration::from_millis(20)).await;
    if rollup.get_block_number().await? != 1 {
        return Err(eyre!("block derivation failed - not on block 1"));
    }

    // check that the deposit succeeded
    assert_eq!(
        rollup.get_balance(mchain.provider.default_signer_address()).await?,
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
        .build(&mchain.provider.wallet())
        .await?;

    let second_tx = TransactionRequest::default()
        .with_to(address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"))
        .with_value(U256::from(0))
        .with_nonce(1)
        .with_gas_limit(100_000)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&mchain.provider.wallet())
        .await?;

    second_tx.encode_2718(&mut tx2);

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage {
            header: Default::default(),
            l2_msg: vec![tx.clone().into(), tx.into(), tx2.into()],
        },
    )]);
    _ = mchain.rollup.postBatch(batch.encode()?).send().await?;
    mine_block(&mchain.provider, 0).await?;

    // wait 20ms for the batch to be processed
    sleep(Duration::from_millis(20)).await;
    if rollup.get_block_number().await? != 2 {
        return Err(eyre!("block derivation failed - not on block 2"));
    }

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
