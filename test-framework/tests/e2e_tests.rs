//! e2e tests for the metabased stack

use alloy::{
    eips::{BlockNumberOrTag, Encodable2718},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, Bytes, B256, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, Block, TransactionRequest},
};
use components::{Components, ConfigurationOptions, ContractVersion};
use contract_bindings::arbitrum::{
    arbsys::ArbSys, ibridge::IBridge, iinbox::IInbox, ioutbox::IOutbox, irollupcore::IRollupCore,
    nodeinterface::NodeInterface, rollup::Rollup,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use test_utils::wait_until;

mod components;

const ARB_SYS_PRECOMPILE_ADDRESS: Address = address!("0x0000000000000000000000000000000000000064");
const NODE_INTERFACE_PRECOMPILE_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000000c8");

// an arbitrary eoa address used for testing
const TEST_ADDR: Address = address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380");

#[cfg(test)]
#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

#[tokio::test]
async fn e2e_send_transaction() -> Result<()> {
    let config = ConfigurationOptions { settlement_delay: 60, ..Default::default() };
    let components = Components::new(&config).await?;

    // Setup the settlement rollup contract
    let set_rollup = Rollup::new(components.inbox_address, &components.settlement_provider);
    let wallet_address = components.settlement_provider.default_signer_address();

    // Send a deposit
    _ = set_rollup.depositEth(wallet_address, wallet_address, parse_ether("1")?).send().await?;
    components.mine_seq_block(config.settlement_delay).await?;
    components.mine_set_block(0).await?;
    // mine 1 set block to close the opened slot that contains the other deposit
    let test_addr: Address = "0xA9ec1Ed7008fDfdE38978Dfef4cF2754A969E5FA".parse()?;
    _ = set_rollup.depositEth(wallet_address, test_addr, parse_ether("1")?).send().await?;
    components.mine_set_block(1).await?;

    // Wait for the deposit to arrive
    wait_until!(
        components.appchain_provider.get_block_number().await? == 1,
        Duration::from_secs(1)
    );

    // check the first rollup block
    let rollup_block: Block = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Number(1))
        .await?
        .unwrap();
    assert_eq!(rollup_block.header.timestamp, config.settlement_delay);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    assert_eq!(components.appchain_provider.get_balance(wallet_address).await?, parse_ether("1")?);

    // Send a sequenced tx
    let tx = TransactionRequest::default()
        .with_to(TEST_ADDR)
        .with_value(U256::from(1))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(components.chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(components.sequencing_provider.wallet())
        .await?;
    components.send_tx_and_mine_block(&tx, 0).await?;

    // Wait for the tx to arrive
    wait_until!(
        components.appchain_provider.get_block_number().await? == 2,
        Duration::from_secs(1)
    );

    // check the second rollup block
    let rollup_block: Block = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Number(2))
        .await?
        .unwrap();
    assert_eq!(rollup_block.header.timestamp, config.settlement_delay);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // tx hash should match
    let hashes: Vec<_> = rollup_block.transactions.hashes().collect();
    assert_eq!(hashes[1], *tx.tx_hash());

    // sequence an empty slot
    components.mine_seq_block(0).await?;

    // mine the pending settlement block (which contains a deposit tx)
    components.mine_both(1).await?;

    // Wait for blocks to be processed
    wait_until!(components.mchain_provider.get_block_number().await >= 3, Duration::from_secs(10));

    // check mchain block
    assert_eq!(components.mchain_provider.get_block_number().await, 3);

    // check rollup block
    wait_until!(
        components.appchain_provider.get_block_number().await? == 3,
        Duration::from_secs(10)
    );
    let rollup_block: Block = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Number(3))
        .await?
        .unwrap();
    assert_eq!(rollup_block.header.timestamp, config.settlement_delay + 1);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // balance should match
    assert_eq!(components.appchain_provider.get_balance(test_addr).await?, parse_ether("1")?);

    Ok(())
}

/// This test sends different types of delayed messages
/// via the inbox contract and ensures that all of them
/// are sequenced via the metabased translator and show up on the rollup.
#[tokio::test]
async fn e2e_deposit() -> Result<()> {
    // Sequencer fees go to the zero address
    let components = Components::new(&ConfigurationOptions {
        pre_loaded: Some(ContractVersion::V300),
        ..Default::default()
    })
    .await?;

    let wallet_address = components.settlement_provider.default_signer_address();

    // Send a deposit (unaliased address) delayed message
    // Deposit is from the arbos address and does not increment the nonce
    let inbox = IInbox::new(components.inbox_address, &components.settlement_provider);
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
        .with_chain_id(components.chain_id)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(0)
        .build(components.settlement_provider.wallet())
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
        .with_chain_id(components.chain_id)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(0)
        .build(components.settlement_provider.wallet())
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
    components.mine_set_block(0).await?;

    // Mine a set block to process the slot
    components.mine_set_block(1).await?;

    // Process the slot - wait for block 17 to be reached
    wait_until!(
        components.appchain_provider.get_block_number().await? == 17,
        Duration::from_secs(10)
    );

    assert_eq!(
        components.appchain_provider.get_balance(wallet_address).await?,
        parse_ether("4.6000316")?
    );

    Ok(())
}

#[tokio::test]
async fn e2e_fast_withdrawal_300() -> Result<()> {
    e2e_fast_withdrawal_base(ContractVersion::V300).await
}

#[tokio::test]
async fn e2e_fast_withdrawal_213() -> Result<()> {
    e2e_fast_withdrawal_base(ContractVersion::V213).await
}

async fn e2e_fast_withdrawal_base(version: ContractVersion) -> Result<()> {
    let components =
        Components::new(&ConfigurationOptions { pre_loaded: Some(version), ..Default::default() })
            .await?;

    let block: Block = components
        .settlement_provider
        .raw_request("eth_getBlockByNumber".into(), ("latest", true))
        .await?;
    components
        .sequencing_provider
        .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
        .await?;

    let wallet_address = components.settlement_provider.default_signer_address();
    let value = parse_ether("1")?;
    let inbox = IInbox::new(components.inbox_address, &components.settlement_provider);
    _ = inbox.depositEth().value(value).send().await?;
    components.mine_set_block(0).await?;
    components.mine_set_block(1).await?;

    // Wait for deposit to be processed
    wait_until!(
        components.appchain_provider.get_balance(wallet_address).await? >= value,
        Duration::from_secs(10)
    );
    assert_eq!(components.appchain_provider.get_block_number().await?, 9);

    let bridge = IBridge::new(components.bridge_address, &components.settlement_provider);

    // 2. Send withdrawal transaction on the Appchain
    let arbsys = ArbSys::new(ARB_SYS_PRECOMPILE_ADDRESS, &components.appchain_provider);
    let gas_limit: u64 = 100_000;
    let max_fee_per_gas: u128 = 100_000_000;
    let withdrawal_value = parse_ether("0.1")?;
    let withdrawal_wallet = components.sequencing_provider.wallet();
    let to_address = address!("0x0000000000000000000000000000000000000001");
    let tx = arbsys
        .withdrawEth(to_address)
        .value(withdrawal_value)
        .nonce(0)
        .gas(gas_limit)
        .chain_id(components.chain_id)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(0)
        .into_transaction_request()
        .build(withdrawal_wallet)
        .await?;
    components.send_tx_and_mine_block(&tx, 0).await?;

    // Wait for the withdrawal transaction to be processed
    wait_until!(
        components.appchain_provider.get_block_number().await? == 10,
        Duration::from_secs(10)
    );

    // 2. Poster service posts
    let client = reqwest::Client::new();
    let response = client.post(format!("{}/post", components.poster_url)).send().await?;
    assert!(response.status().is_success(), "Expected 200 OK, got {}", response.status());

    // 3. Execute transaction (usually done by end-user)

    // Generate proof
    let node_interface =
        NodeInterface::new(NODE_INTERFACE_PRECOMPILE_ADDRESS, &components.appchain_provider);
    let proof = node_interface.constructOutboxProof(1, 0).call().await?;

    // Execute withdrawal
    let outbox = IOutbox::new(
        IRollupCore::new(bridge.rollup().call().await?._0, &components.settlement_provider)
            .outbox()
            .call()
            .await?
            ._0,
        &components.settlement_provider,
    );

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct NitroBlock {
        hash: B256,
        send_root: B256,
        number: U256,
        l1_block_number: U256,
        timestamp: U256,
    }

    let block: NitroBlock = components
        .appchain_provider
        .raw_request("eth_getBlockByNumber".into(), ("latest", false))
        .await?;

    let _ = outbox
        .executeTransaction(
            proof.proof,           // proof
            U256::from(0),         // index
            wallet_address,        // l2Sender
            to_address,            // to
            block.number,          // l2Block,
            block.l1_block_number, // l1Block,
            block.timestamp,       // l2Timestamp,
            withdrawal_value,      // value
            Bytes::new(),          // data (always empty)
        )
        .send()
        .await?;

    components.mine_set_block(0).await?;

    // Assert new balance is equal to withdrawal amount
    let balance_after = components.settlement_provider.get_balance(to_address).await?;
    assert_eq!(balance_after, withdrawal_value);

    Ok(())
}

#[tokio::test]
async fn e2e_settlement_reorg() -> Result<()> {
    let components = Components::new(&ConfigurationOptions {
        pre_loaded: Some(ContractVersion::V300),
        ..Default::default()
    })
    .await?;

    let counter = tokio::sync::Mutex::new(0);
    wait_until!(*counter.lock().await += 1; *counter.lock().await >= 3, Duration::from_secs(1));

    // NOTE: at this point the mchain is on block 1 (initial mchain block) - we can't reorg to
    // genesis, so we need to create two slots on top of it before the reorg happens.

    components.mine_both(1).await?; //mine mchain block 2 (only works because there are delayed txs to proccess, otherwise the
                                    // mchain block would be empty/skipped)
                                    // Wait for mchain to reach block 2

    wait_until!(components.mchain_provider.get_block_number().await == 2, Duration::from_secs(10));

    let wallet_address = components.settlement_provider.default_signer_address();
    let inbox = IInbox::new(components.inbox_address, &components.settlement_provider);

    // create a deposit1 (that won't be rolled back) that will fit on mchain's block 3
    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

    components.mine_both(100).await?;
    components.mine_both(1).await?; // extra blocks to close the slot

    // Wait for deposit1 to be processed
    wait_until!(
        components.appchain_provider.get_balance(wallet_address).await? == parse_ether("1")?,
        Duration::from_secs(10)
    );

    // send a deposit2 that will be reorged
    let mchain_block_before_deposit = components.mchain_provider.get_block_number().await;
    assert_eq!(mchain_block_before_deposit, 3);

    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

    // make this deposit2 fit into a slot that will be reorged
    components.mine_both(200).await?; // will be reorged leaving this slot opened
    components.mine_both(1).await?; // mine an extra block to close the slot (will be reorged later)

    // Wait for deposit2 to be processed
    wait_until!(
        components.appchain_provider.get_balance(wallet_address).await? == parse_ether("2")?,
        Duration::from_secs(10)
    );
    assert_eq!(components.mchain_provider.get_block_number().await, 4);

    // the rollup head has not been updated yet
    let rollup_head_before_reorg =
        components.appchain_provider.get_block_by_number(BlockNumberOrTag::Latest).await?.unwrap();

    // reorg, assert that the L3 balance has disappeared
    let reorg_depth = 2;
    let current_block = components.settlement_provider.get_block_number().await?;
    components.settlement_provider.anvil_rollback(Some(reorg_depth)).await?;
    assert_eq!(
        components.settlement_provider.get_block_number().await?,
        current_block - reorg_depth
    );

    //NOTE: mine an extra block. (the ingestor currently polls by block number, so it won't detect
    // a reorg at a given block height, until a block with a bigger height is mined)
    // if we switch to a different ingestor implementation this could be removed.

    for _ in 0..reorg_depth + 1 {
        components.mine_set_block(0).await?;
    }

    // Wait for reorg to be processed
    wait_until!(
            let block = components
                .mchain_provider
                .get_block_number()
                .await;
            block == mchain_block_before_deposit
        ,
        Duration::from_secs(10)
    );

    // the rollup should have reorged to a pre-deposit block

    // create new slots
    _ = inbox.depositEth().value(parse_ether("0.01")?).send().await?;
    components.mine_both(500).await?;
    components.mine_both(500).await?; // build mchain to an height above what the rollup has seen before the reorg

    wait_until!(let rollup_head = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .unwrap();
        rollup_head.header.number == rollup_head_before_reorg.header.number &&
        rollup_head.header.hash != rollup_head_before_reorg.header.hash,
        Duration::from_secs(10)
    );

    assert_eq!(components.mchain_provider.get_block_number().await, 4);

    // balance should be correct (reorged deposit is not included)
    let balance_after_reorg = components.appchain_provider.get_balance(wallet_address).await?;
    assert_eq!(balance_after_reorg, parse_ether("1.01")?);

    Ok(())
}
