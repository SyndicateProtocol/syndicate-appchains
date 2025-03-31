//! Integration tests for the metabased stack

use alloy::{
    eips::{BlockNumberOrTag, Encodable2718},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, Block, BlockTransactionsKind, TransactionRequest},
};
use contract_bindings::arbitrum::{
    arbgasinfo::ArbGasInfo, arbownerpublic::ArbOwnerPublic, iinbox::IInbox,
};
use eyre::{Error, Result};
use std::time::Duration;
use test_framework::components::{
    Components, ConfigurationOptions, ContractVersion, ImageTags, APPCHAIN_OWNER,
};
use test_utils::wait_until;
use tokio::time::sleep;
use tracing::info;

#[tokio::test(flavor = "multi_thread")]
async fn arb_owner_test() -> Result<()> {
    const ARB_OWNER_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006b");

    // Start the meta node
    let components = Components::new(None, None).await?;
    let arb_owner_public =
        ArbOwnerPublic::new(ARB_OWNER_CONTRACT_ADDRESS, &components.appchain_provider);
    assert_eq!(arb_owner_public.getAllChainOwners().call().await?._0, [APPCHAIN_OWNER]);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn no_l1_fees_test() -> Result<()> {
    const ARB_GAS_INFO_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006c");
    let components = Components::new(None, None).await?;
    let arb_gas_info =
        ArbGasInfo::new(ARB_GAS_INFO_CONTRACT_ADDRESS, &components.appchain_provider);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?._0, U256::ZERO);

    Ok(())
}

#[allow(clippy::unwrap_used)] // test code
async fn new_test_with_synced_chains() -> Result<Components> {
    // Start the meta node (pre-loaded with the full set of Arb contracts)
    let components = Components::new(
        Some(ContractVersion::V300),
        Some(ConfigurationOptions {
            sequencing_start_block: 3,
            settlement_start_block: 1,
            settlement_delay: 0,
        }),
    )
    .await?;

    // Sync the tips of the sequencing and settlement chains
    let block = components
        .settlement_provider
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
    components
        .sequencing_provider
        .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
        .await?;

    Ok(components)
}

/// This test sends different types of delayed messages
/// via the inbox contract and ensures that all of them
/// are sequenced via the metabased translator and show up on the rollup.
#[tokio::test(flavor = "multi_thread")]
async fn e2e_settlement_test() -> Result<()> {
    let components = new_test_with_synced_chains().await?;

    // Grab the wallet address for the test
    let wallet_address = APPCHAIN_OWNER;

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
    // wait_until!(
    //     components.appchain_provider.get_block_number().await? == 17,
    //     Duration::from_secs(2)
    // );

    sleep(Duration::from_secs(2)).await;
    let current_block = components.appchain_provider.get_block_number().await?;
    info!("CURRENT BLOCK {:?}", current_block);

    assert_eq!(
        components
            .appchain_provider
            .get_balance(components.settlement_provider.default_signer_address())
            .await?,
        parse_ether("4.6001407626")?
    );

    Ok(())
}
