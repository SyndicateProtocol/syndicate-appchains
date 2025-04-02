//! Integration tests for the metabased stack
use alloy::{
    eips::{eip2718::Encodable2718, BlockId::Number, BlockNumberOrTag},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, Bytes, B256, U256},
    providers::{ext::AnvilApi as _, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, BlockTransactionsKind, TransactionRequest},
    sol_types::SolCall,
};
use block_builder::{
    self,
    config::{get_default_private_key_signer, get_rollup_contract_address},
};
use common::types::{Block, BlockRef};
use contract_bindings::arbitrum::{
    arbgasinfo::ArbGasInfo, arbownerpublic::ArbOwnerPublic, arbsys::ArbSys,
    assertionposter::AssertionPoster, ibridge::IBridge, iinbox::IInbox, ioutbox::IOutbox,
    iownable::IOwnable, irollupadmin::IRollupAdmin, irollupcore::IRollupCore,
    isequencerinbox::ISequencerInbox, iupgradeexecutor::IUpgradeExecutor,
    nodeinterface::NodeInterface, rollup::Rollup,
};
use e2e_tests::full_meta_node::{ContractVersion, MetaNode};
use eyre::{Ok, Result};
use metabased_translator::config::MetabasedConfig;
use serde::{Deserialize, Serialize};
use shared::{logger::init_test_tracing, wait_until};
use std::time::Duration;
use tracing::Level;

const ARB_SYS_PRECOMPILE_ADDRESS: Address = address!("0x0000000000000000000000000000000000000064");
const NODE_INTERFACE_PRECOMPILE_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000000c8");

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

/// This test sends different types of delayed messages
/// via the inbox contract and ensures that all of them
/// are sequenced via the metabased translator and show up on the rollup.
#[tokio::test(flavor = "multi_thread")]
async fn e2e_settlement_test() -> Result<()> {
    let _ = init_test_tracing(Level::INFO);

    let meta_node = new_test_with_synced_chains().await?;

    // Grab the wallet address for the test
    let wallet_address = meta_node.settlement_provider.default_signer_address();

    // Send a deposit (unaliased address) delayed message
    // Deposit is from the arbos address and does not increment the nonce
    let inbox = IInbox::new(meta_node.inbox_address, &meta_node.settlement_provider);
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
    meta_node.mine_set_block(0).await?;

    // Mine a set block to process the slot
    meta_node.mine_set_block(1).await?;

    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_eq!(meta_node.metabased_rollup.get_block_number().await?, 17);
    // Process the slot - wait for block 17 to be reached
    wait_until!(meta_node.metabased_rollup.get_block_number().await? == 17, Duration::from_secs(2));

    assert_eq!(
        meta_node
            .metabased_rollup
            .get_balance(meta_node.settlement_provider.default_signer_address())
            .await?,
        parse_ether("4.6000316")?
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_test() -> Result<()> {
    let _ = init_test_tracing(Level::INFO);
    // Start the meta node (no pre-loaded contracts)
    let mut config = MetabasedConfig::default();
    config.sequencing.sequencing_start_block = 3; // skip the contract deployment blocks
    let meta_node = MetaNode::new(None, config.clone()).await?;
    // Setup the settlement rollup contract
    let set_rollup = Rollup::new(get_rollup_contract_address(), &meta_node.settlement_provider);

    // Send a deposit and a regular tx
    _ = set_rollup
        .depositEth(
            meta_node.sequencing_provider.default_signer_address(),
            meta_node.sequencing_provider.default_signer_address(),
            parse_ether("1")?,
        )
        .send()
        .await?;

    // Send a sequenced tx
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

    // due to the delay for settlement, we need to space the sequencing chain, so the following
    // tx arrives at the same time, or after the deposit
    meta_node.mine_seq_block(config.slotter.settlement_delay).await?;
    meta_node.mine_set_block(0).await?;

    // mine 1 set block to close the opened slot that contains another deposit
    let test_addr: Address = "0xA9ec1Ed7008fDfdE38978Dfef4cF2754A969E5FA".parse()?;
    _ = set_rollup
        .depositEth(
            meta_node.sequencing_provider.default_signer_address(),
            test_addr,
            parse_ether("1")?,
        )
        .send()
        .await?;
    meta_node.mine_set_block(1).await?;

    // Wait for blocks to be processed
    wait_until!(
        meta_node.metabased_rollup.get_block_number().await? == 2,
        Duration::from_millis(500)
    );
    assert_eq!(meta_node.mchain_provider.get_block_number().await?, 2);

    // check rollup blocks
    // check the first rollup block
    let rollup_block: Block = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(1), true))
        .await?;
    assert_eq!(rollup_block.timestamp, config.slotter.settlement_delay);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // check the second rollup block
    let rollup_block: Block = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    assert_eq!(rollup_block.timestamp, config.slotter.settlement_delay);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // tx hash should match
    assert_eq!(rollup_block.transactions[1].hash, *inner_tx.tx_hash());

    // sequence an empty slot
    meta_node.mine_seq_block(0).await?;

    // mine the pending settlement block (which contains a deposit tx)
    meta_node.mine_both(1).await?;

    let block_number = 3;
    // Wait for blocks to be processed
    wait_until!(
        meta_node.mchain_provider.get_block_number().await? >= block_number,
        Duration::from_millis(500)
    );

    // check mchain block
    assert_eq!(meta_node.mchain_provider.get_block_number().await?, block_number);

    // check rollup block
    wait_until!(meta_node.metabased_rollup.get_block_number().await? == 3, Duration::from_secs(1));
    let rollup_block: Block = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(3), true))
        .await?;
    assert_eq!(rollup_block.timestamp, config.slotter.settlement_delay + 1);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // balance should match
    assert_eq!(meta_node.metabased_rollup.get_balance(test_addr).await?, parse_ether("1")?);

    // check rollup contract state
    let known_state = meta_node
        .mchain_provider
        .get_safe_state(&meta_node.sequencing_client, &meta_node.settlement_client)
        .await?
        .unwrap();
    assert_eq!(
        known_state.mchain_block_number,
        // -1 because safe_state return the block that matches the verified state of the world
        // minus 1
        meta_node.mchain_provider.get_block_number().await? - 1
    );
    let seq_block = meta_node
        .sequencing_provider
        .get_block(
            Number(BlockNumberOrTag::Number(
                // -2 because it's the latest block that has transactions in it
                meta_node.sequencing_provider.get_block_number().await? - 2,
            )),
            BlockTransactionsKind::Hashes,
        )
        .await?
        .unwrap();
    assert_eq!(
        known_state.sequencing_block,
        BlockRef {
            number: seq_block.header.number,
            timestamp: seq_block.header.timestamp,
            hash: seq_block.header.hash
        }
    );
    // last block hasn't been processed yet
    let set_block = meta_node
        .settlement_provider
        .get_block(
            Number(BlockNumberOrTag::Number(
                // -2 because the latest block hasn't been processed yet
                meta_node.settlement_provider.get_block_number().await? - 2,
            )),
            BlockTransactionsKind::Hashes,
        )
        .await?
        .unwrap();
    assert_eq!(
        known_state.settlement_block,
        BlockRef {
            number: set_block.header.number,
            timestamp: set_block.header.timestamp,
            hash: set_block.header.hash
        }
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_settlement_fast_withdrawal_300() -> Result<()> {
    e2e_settlement_fast_withdrawal_base(ContractVersion::V300).await
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_settlement_fast_withdrawal_213() -> Result<()> {
    e2e_settlement_fast_withdrawal_base(ContractVersion::V213).await
}

async fn e2e_settlement_fast_withdrawal_base(version: ContractVersion) -> Result<()> {
    // 0. Set up
    let _ = init_test_tracing(Level::INFO);

    let mut config = MetabasedConfig::default();
    config.slotter.settlement_delay = 0;
    config.settlement.settlement_start_block = 1;
    config.sequencing.sequencing_start_block = 3;
    let meta_node = MetaNode::new(Some(version), config).await?;

    let block: Block = meta_node
        .settlement_provider
        .raw_request("eth_getBlockByNumber".into(), ("latest", true))
        .await?;
    meta_node
        .sequencing_provider
        .evm_mine(Some(MineOptions::Timestamp(Some(block.timestamp))))
        .await?;

    let inbox = IInbox::new(meta_node.inbox_address, &meta_node.settlement_provider);
    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;
    meta_node.mine_set_block(0).await?;
    meta_node.mine_set_block(1).await?;

    // Wait for deposit to be processed
    wait_until!(meta_node.metabased_rollup.get_block_number().await? > 0, Duration::from_secs(2));

    let bridge = IBridge::new(meta_node.bridge_address, &meta_node.settlement_provider);
    // 1. Deploy assertion poster contract & set assertion poster as the fast confirm owner
    let nonce = meta_node
        .settlement_provider
        .get_transaction_count(get_default_private_key_signer().address())
        .await?;
    _ = AssertionPoster::deploy_builder(
        &meta_node.settlement_provider,
        bridge.rollup().call().await?._0,
    )
    .nonce(nonce)
    .send()
    .await?;
    let assertion_poster = AssertionPoster::new(
        get_default_private_key_signer().address().create(nonce),
        &meta_node.settlement_provider,
    );
    let executor = IUpgradeExecutor::new(
        IOwnable::new(bridge.rollup().call().await?._0, &meta_node.settlement_provider)
            .owner()
            .call()
            .await?
            ._0,
        &meta_node.settlement_provider,
    );
    _ = executor
        .execute(*assertion_poster.address(), AssertionPoster::initializeCall::SELECTOR.into())
        .send()
        .await?;

    // 2. Send withdrawal transaction on the Appchain
    let mut tx = vec![];
    let arbsys = ArbSys::new(ARB_SYS_PRECOMPILE_ADDRESS, &meta_node.metabased_rollup);
    let gas_limit: u64 = 100_000;
    let max_fee_per_gas: u128 = 100_000_000;
    let withdrawal_value = parse_ether("0.1")?;
    let withdrawal_wallet = meta_node.sequencing_provider.wallet();
    let to_address = address!("0x0000000000000000000000000000000000000001");
    arbsys
        .withdrawEth(to_address)
        .value(withdrawal_value)
        .nonce(0)
        .gas(gas_limit)
        .chain_id(meta_node.chain_id)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(0)
        .into_transaction_request()
        .build(withdrawal_wallet)
        .await?
        .encode_2718(&mut tx);
    let _ = meta_node.sequencing_contract.processTransaction(tx.into()).send().await?;
    meta_node.mine_seq_block(0).await?;

    // Wait for the withdrawal transaction to be processed
    wait_until!(meta_node.metabased_rollup.get_block_number().await? == 10, Duration::from_secs(2));

    // 3. Build & confirm Assertion on the settlement chain
    // Helper struct
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct NitroBlock {
        hash: B256,
        send_root: B256,
        number: U256,
        l1_block_number: U256,
        timestamp: U256,
    }

    let block: NitroBlock = meta_node
        .metabased_rollup
        .raw_request("eth_getBlockByNumber".into(), ("latest", false))
        .await?;

    // submit multiple times to make sure repeated submissions work
    _ = assertion_poster.postAssertion(U256::from(1).into(), U256::from(2).into()).send().await?;
    _ = assertion_poster.postAssertion(U256::from(3).into(), U256::from(4).into()).send().await?;
    // post a batch to make sure this doesn't break anything
    let seq_inbox = ISequencerInbox::new(
        bridge.sequencerInbox().call().await?._0,
        &meta_node.settlement_provider,
    );
    _ = seq_inbox
        .addSequencerL2Batch(
            U256::MAX,
            Default::default(),
            U256::from(1),
            Address::ZERO,
            U256::ZERO,
            U256::ZERO,
        )
        .send()
        .await?;
    // modify the confirm period to make sure it still works
    _ = executor
        .executeCall(
            bridge.rollup().call().await?._0,
            IRollupAdmin::setConfirmPeriodBlocksCall { newConfirmPeriod: 99 }.abi_encode().into(),
        )
        .send()
        .await?;
    _ = assertion_poster.postAssertion(U256::from(5).into(), U256::from(6).into()).send().await?;
    _ = assertion_poster.postAssertion(block.hash, block.send_root).send().await?;

    // 3. Execute transaction (usually done by end-user)

    // Generate proof
    let node_interface =
        NodeInterface::new(NODE_INTERFACE_PRECOMPILE_ADDRESS, &meta_node.metabased_rollup);
    let proof = node_interface.constructOutboxProof(1, 0).call().await?;

    // Execute withdrawal
    let outbox = IOutbox::new(
        IRollupCore::new(bridge.rollup().call().await?._0, &meta_node.settlement_provider)
            .outbox()
            .call()
            .await?
            ._0,
        &meta_node.settlement_provider,
    );
    let _ = outbox
        .executeTransaction(
            proof.proof,                                            // proof
            U256::from(0),                                          // index
            meta_node.sequencing_provider.default_signer_address(), // l2Sender
            to_address,                                             // to
            block.number,                                           // l2Block,
            block.l1_block_number,                                  // l1Block,
            block.timestamp,                                        // l2Timestamp,
            withdrawal_value,                                       // value
            Bytes::new(),                                           // data (always empty)
        )
        .send()
        .await?;

    meta_node.mine_set_block(0).await?;

    // Assert new balance is equal to withdrawal amount
    let balance_after = meta_node.settlement_provider.get_balance(to_address).await?;
    assert_eq!(balance_after, withdrawal_value);

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
    let mchain_block_before_deposit = meta_node.mchain_provider.get_block_number().await?;
    assert_eq!(mchain_block_before_deposit, 3);

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
                .get_block_number()
                .await?;
            block == mchain_block_before_deposit
        ,
        Duration::from_millis(500)
    );

    // mchain should have reorged to a pre-deposit block
    assert_eq!(meta_node.mchain_provider.get_block_number().await?, mchain_block_before_deposit);
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
