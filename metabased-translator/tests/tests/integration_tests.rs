//! Integration tests for the metabased stack
use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, utils::parse_ether, Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use block_builder::rollups::arbitrum::{self, arbitrum_adapter::L1MessageType};
use contract_bindings::arbitrum::{arbgasinfo::ArbGasInfo, arbownerpublic::ArbOwnerPublic};
use eyre::{Ok, Result};
use mchain::db::{DelayedMessage, MBlock};
use std::{str::FromStr as _, time::Duration};
use test_utils::{
    docker::{launch_nitro_node, start_mchain},
    wait_until,
};

// an arbitrary chain id used for testing
const APPCHAIN_CHAIN_ID: u64 = 13331370;

// an arbitrary private key used for signing. does not contain funds.
const TEST_PRIVATE_KEY: &str = "0x8734a70a2db460d7aacc114ac822de780967cbe9c8b4ee4b5849b0b38fb0cc1c";

// an arbitrary eoa address. does not contain funds.
const TEST_ADDR: Address = address!("0x0000000000000000000000000000000000000001");

#[cfg(test)]
#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

fn get_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str(TEST_PRIVATE_KEY)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}

fn deposit_eth(src: Address, dest: Address, value: U256) -> DelayedMessage {
    DelayedMessage {
        kind: L1MessageType::EthDeposit as u8,
        sender: src,
        data: (dest, value).abi_encode_packed().into(),
        base_fee_l1: U256::ZERO,
    }
}

#[tokio::test]
async fn arb_owner_test() -> Result<()> {
    const ARB_OWNER_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006b");

    // Start the meta node
    let appchain_owner = address!("0x0000000000000000000000000000000000000001");
    let (mchain_url, _mchain, _) = start_mchain(APPCHAIN_CHAIN_ID, appchain_owner, 0).await?;
    let (_nitro, rollup, _) =
        launch_nitro_node(APPCHAIN_CHAIN_ID, appchain_owner, &mchain_url, None).await?;
    let arb_owner_public = ArbOwnerPublic::new(ARB_OWNER_CONTRACT_ADDRESS, &rollup);
    assert_eq!(arb_owner_public.getAllChainOwners().call().await?._0, [appchain_owner]);
    Ok(())
}

#[tokio::test]
async fn no_l1_fees_test() -> Result<()> {
    const ARB_GAS_INFO_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006c");
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, Address::ZERO, 0).await?;
    let (_nitro, rollup, _) =
        launch_nitro_node(APPCHAIN_CHAIN_ID, Address::ZERO, &mchain_url, None).await?;
    let arb_gas_info = ArbGasInfo::new(ARB_GAS_INFO_CONTRACT_ADDRESS, &rollup);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?._0, U256::ZERO);
    // Make sure adding delayed messages with a non-zero base fee does not increase the l1 fee.
    // The l1 fee should only be updated when BatchPostingReport messages are sent, which we
    // explicitly block in the metabased translator.
    let qty = parse_ether("1")?;
    let msg = DelayedMessage { base_fee_l1: qty, ..deposit_eth(TEST_ADDR, TEST_ADDR, qty) };
    mchain
        .add_batch(MBlock {
            messages: vec![msg.clone()],
            batch: arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
            timestamp: 100,
            ..Default::default()
        })
        .await?;
    mchain
        .add_batch(MBlock {
            messages: vec![msg],
            batch: arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
            timestamp: 200,
            ..Default::default()
        })
        .await?;
    wait_until!(rollup.get_block_number().await? == 2, Duration::from_secs(2));
    assert_eq!(rollup.get_balance(TEST_ADDR).await?, qty + qty);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?._0, U256::ZERO);
    Ok(())
}

/// This test tests that rollup blocks are properly derived from batches created
/// via the block builder code and posted to the dummy rollup contract.
#[tokio::test]
async fn test_nitro_batch() -> Result<()> {
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, Address::ZERO, 0).await?;

    let (_nitro, rollup, _) =
        launch_nitro_node(APPCHAIN_CHAIN_ID, Address::ZERO, &mchain_url, None).await?;

    let addr = get_signer().address();

    // deposit 1 eth
    mchain
        .add_batch(MBlock {
            messages: vec![deposit_eth(Address::ZERO, addr, parse_ether("1")?)],
            batch: arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
            ..Default::default()
        })
        .await?;

    // wait for the batch to be processed and for block 1 to be derived
    wait_until!(rollup.get_block_number().await? == 1, Duration::from_secs(1));

    // check that the deposit succeeded
    assert_eq!(rollup.get_balance(addr).await?, parse_ether("1")?);

    // include a tx in a batch
    let mut tx = vec![];
    let inner_tx = TransactionRequest::default()
        .with_to(TEST_ADDR)
        .with_value(U256::from(1))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(APPCHAIN_CHAIN_ID)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&EthereumWallet::from(get_signer()))
        .await?;

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage { header: Default::default(), l2_msg: vec![tx.into()] },
    )]);
    mchain.add_batch(MBlock { batch: batch.encode()?, ..Default::default() }).await?;

    // Wait for batch processing to complete and block 2 to be derived
    wait_until!(rollup.get_block_number().await? == 2, Duration::from_millis(500));

    // check that the tx was sequenced
    let block = rollup.get_block_by_number(BlockNumberOrTag::Number(2)).await?.unwrap();
    assert_eq!(block.transactions.len(), 2);
    // tx hash should match
    let transactions: Vec<_> = block.transactions.hashes().collect();
    assert_eq!(transactions[1], *inner_tx.tx_hash());

    Ok(())
}

/// Regression test
#[tokio::test]
async fn test_nitro_batch_two_tx() -> Result<()> {
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, Address::ZERO, 0).await?;
    let (_nitro, rollup, _) =
        launch_nitro_node(APPCHAIN_CHAIN_ID, Address::ZERO, &mchain_url, None).await?;
    let addr = get_signer().address();

    // deposit 1 eth
    mchain
        .add_batch(MBlock {
            messages: vec![deposit_eth(Address::ZERO, addr, parse_ether("1")?)],
            batch: arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
            ..Default::default()
        })
        .await?;

    // Wait for the batch to be processed and for block 1 to be derived
    wait_until!(rollup.get_block_number().await? == 1, Duration::from_millis(500));

    // check that the deposit succeeded
    assert_eq!(rollup.get_balance(addr).await?, parse_ether("1")?);

    // include two tx in a batch
    let mut tx = vec![];
    let mut tx2 = vec![];
    let inner_tx = TransactionRequest::default()
        .with_to(TEST_ADDR)
        .with_value(U256::from(0))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(APPCHAIN_CHAIN_ID)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&EthereumWallet::from(get_signer()))
        .await?;

    let second_tx = TransactionRequest::default()
        .with_to(TEST_ADDR)
        .with_value(U256::from(0))
        .with_nonce(1)
        .with_gas_limit(100_000)
        .with_chain_id(APPCHAIN_CHAIN_ID)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&EthereumWallet::from(get_signer()))
        .await?;

    second_tx.encode_2718(&mut tx2);

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage {
            header: Default::default(),
            l2_msg: vec![tx.clone().into(), tx.into(), tx2.into()],
        },
    )]);
    mchain.add_batch(MBlock { batch: batch.encode()?, ..Default::default() }).await?;

    // Wait for the batch to be processed and for block 2 to be derived
    wait_until!(rollup.get_block_number().await? == 2, Duration::from_millis(500));

    // check that the tx was sequenced
    let block = rollup.get_block_by_number(BlockNumberOrTag::Number(2)).await?.unwrap();
    assert_eq!(block.transactions.len(), 3);
    // tx hash should match
    let transactions: Vec<_> = block.transactions.hashes().collect();
    assert_eq!(transactions[1], *inner_tx.tx_hash());
    assert_eq!(transactions[2], *second_tx.tx_hash());

    Ok(())
}
