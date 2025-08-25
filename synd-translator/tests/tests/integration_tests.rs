//! Integration tests for the Syndicate appchains stack
use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, utils::parse_ether, Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use contract_bindings::synd::{arb_gas_info::ArbGasInfo, arb_owner_public::ArbOwnerPublic};
use eyre::{Ok, Result};
use std::{str::FromStr as _, time::Duration};
use synd_block_builder::appchains::arbitrum::{self, arbitrum_adapter::L1MessageType};
use synd_mchain::{
    client::Provider as _,
    db::{ArbitrumBatch, DelayedMessage, MBlock, Slot},
    methods::common::{APPCHAIN_CONTRACT, MCHAIN_ID},
};
use test_utils::{
    docker::{launch_nitro_node, start_mchain, NitroNodeArgs, NitroSequencerMode},
    nitro_chain::NitroDeployment,
    wait_until,
};

// an arbitrary chain id used for testing
const APPCHAIN_CHAIN_ID: u64 = 13331370;

// an arbitrary private key used for signing. does not contain funds.
const TEST_PRIVATE_KEY: &str = "0x8734a70a2db460d7aacc114ac822de780967cbe9c8b4ee4b5849b0b38fb0cc1c";

// an arbitrary eoa address. does not contain funds.
const TEST_ADDR: Address = address!("0x0000000000000000000000000000000000000001");

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

fn get_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str(TEST_PRIVATE_KEY)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {err}"))
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

    // Start the appchain's node
    let appchain_owner = address!("0x0000000000000000000000000000000000000001");
    let (mchain_url, _mchain, _) = start_mchain(APPCHAIN_CHAIN_ID, 0).await?;
    let chain_info = launch_nitro_node(NitroNodeArgs {
        chain_id: APPCHAIN_CHAIN_ID,
        chain_owner: appchain_owner,
        parent_chain_url: mchain_url,
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: NitroDeployment {
            bridge: APPCHAIN_CONTRACT,
            sequencer_inbox: APPCHAIN_CONTRACT,
            deployed_at: 1,
            ..Default::default()
        },
        sequencer_private_key: None,
    })
    .await?;
    let arb_owner_public = ArbOwnerPublic::new(ARB_OWNER_CONTRACT_ADDRESS, &chain_info.provider);
    assert_eq!(arb_owner_public.getAllChainOwners().call().await?, [appchain_owner]);
    Ok(())
}

#[tokio::test]
async fn no_l1_fees_test() -> Result<()> {
    const ARB_GAS_INFO_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006c");
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, 0).await?;
    let chain_info = launch_nitro_node(NitroNodeArgs {
        chain_id: APPCHAIN_CHAIN_ID,
        chain_owner: Address::ZERO,
        parent_chain_url: mchain_url,
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: NitroDeployment {
            bridge: APPCHAIN_CONTRACT,
            sequencer_inbox: APPCHAIN_CONTRACT,
            deployed_at: 1,
            ..Default::default()
        },
        sequencer_private_key: None,
    })
    .await?;
    let arb_gas_info = ArbGasInfo::new(ARB_GAS_INFO_CONTRACT_ADDRESS, &chain_info.provider);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?, U256::ZERO);
    // Make sure adding delayed messages with a non-zero base fee does not increase the l1 fee.
    // The l1 fee should only be updated when BatchPostingReport messages are sent, which we
    // explicitly block in the `synd-translator`.
    let qty = parse_ether("1")?;
    let msg = DelayedMessage { base_fee_l1: qty, ..deposit_eth(TEST_ADDR, TEST_ADDR, qty) };
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(
                arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
                vec![msg.clone()],
            )),
            timestamp: 100,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        })
        .await?;
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(
                arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
                vec![msg],
            )),
            timestamp: 200,
            slot: Slot { seq_block_number: 2, ..Default::default() },
        })
        .await?;
    wait_until!(chain_info.provider.get_block_number().await? == 2, Duration::from_secs(2));
    assert_eq!(chain_info.provider.get_balance(TEST_ADDR).await?, qty + qty);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?, U256::ZERO);
    Ok(())
}

/// This test tests that appchain blocks are properly derived from batches created
/// via the block builder code and posted to the dummy appchain contract.
#[tokio::test]
async fn test_nitro_batch() -> Result<()> {
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, 0).await?;

    let chain_info = launch_nitro_node(NitroNodeArgs {
        chain_id: APPCHAIN_CHAIN_ID,
        chain_owner: Address::ZERO,
        parent_chain_url: mchain_url,
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: NitroDeployment {
            bridge: APPCHAIN_CONTRACT,
            sequencer_inbox: APPCHAIN_CONTRACT,
            deployed_at: 1,
            ..Default::default()
        },
        sequencer_private_key: None,
    })
    .await?;

    let addr = get_signer().address();

    // deposit 1 eth
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(
                arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
                vec![deposit_eth(Address::ZERO, addr, parse_ether("1")?)],
            )),
            timestamp: 0,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        })
        .await?;

    // wait for the batch to be processed and for block 1 to be derived
    wait_until!(chain_info.provider.get_block_number().await? == 1, Duration::from_secs(1));

    // check that the deposit succeeded
    assert_eq!(chain_info.provider.get_balance(addr).await?, parse_ether("1")?);

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
    tx.insert(0, 4);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage { header: Default::default(), l2_msg: vec![tx.into()] },
    )]);
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(batch.encode()?, Default::default())),
            slot: Slot { seq_block_number: 2, ..Default::default() },
            ..Default::default()
        })
        .await?;

    // Wait for batch processing to complete and block 2 to be derived
    wait_until!(chain_info.provider.get_block_number().await? == 2, Duration::from_millis(500));

    // check that the tx was sequenced
    let block =
        chain_info.provider.get_block_by_number(BlockNumberOrTag::Number(2)).await?.unwrap();
    assert_eq!(block.transactions.len(), 2);
    // tx hash should match
    let transactions: Vec<_> = block.transactions.hashes().collect();
    assert_eq!(transactions[1], *inner_tx.tx_hash());

    Ok(())
}

/// Regression test
#[tokio::test]
async fn test_nitro_batch_two_tx() -> Result<()> {
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, 0).await?;
    let chain_info = launch_nitro_node(NitroNodeArgs {
        chain_id: APPCHAIN_CHAIN_ID,
        chain_owner: Address::ZERO,
        parent_chain_url: mchain_url,
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: NitroDeployment {
            bridge: APPCHAIN_CONTRACT,
            sequencer_inbox: APPCHAIN_CONTRACT,
            deployed_at: 1,
            ..Default::default()
        },
        sequencer_private_key: None,
    })
    .await?;
    let addr = get_signer().address();

    // deposit 1 eth
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(
                arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
                vec![deposit_eth(Address::ZERO, addr, parse_ether("1")?)],
            )),
            timestamp: 0,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        })
        .await?;

    // Wait for the batch to be processed and for block 1 to be derived
    wait_until!(chain_info.provider.get_block_number().await? == 1, Duration::from_millis(500));

    // check that the deposit succeeded
    assert_eq!(chain_info.provider.get_balance(addr).await?, parse_ether("1")?);

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
    tx2.insert(0, 4);

    inner_tx.encode_2718(&mut tx);
    tx.insert(0, 4);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage {
            header: Default::default(),
            l2_msg: vec![tx.clone().into(), tx.into(), vec![4].into(), tx2.into()],
        },
    )]);
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(batch.encode()?, Default::default())),
            slot: Slot { seq_block_number: 2, ..Default::default() },
            timestamp: 0,
        })
        .await?;

    // Wait for the batch to be processed and for block 2 to be derived
    wait_until!(chain_info.provider.get_block_number().await? == 2, Duration::from_millis(500));

    // check that the tx was sequenced
    let block =
        chain_info.provider.get_block_by_number(BlockNumberOrTag::Number(2)).await?.unwrap();
    assert_eq!(block.transactions.len(), 3);
    // tx hash should match
    let transactions: Vec<_> = block.transactions.hashes().collect();
    assert_eq!(transactions[1], *inner_tx.tx_hash());
    assert_eq!(transactions[2], *second_tx.tx_hash());

    Ok(())
}

#[tokio::test]
async fn test_nitro_end_of_block_tx() -> Result<()> {
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, 0).await?;
    let chain_info = launch_nitro_node(NitroNodeArgs {
        chain_id: APPCHAIN_CHAIN_ID,
        chain_owner: Address::ZERO,
        parent_chain_url: mchain_url,
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: NitroDeployment {
            bridge: APPCHAIN_CONTRACT,
            sequencer_inbox: APPCHAIN_CONTRACT,
            deployed_at: 1,
            ..Default::default()
        },
        sequencer_private_key: None,
    })
    .await?;

    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(
                arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?,
                vec![
                    DelayedMessage {
                        kind: L1MessageType::EndOfBlock as u8,
                        sender: Address::ZERO,
                        data: Default::default(),
                        base_fee_l1: Default::default(),
                    };
                    3
                ],
            )),
            timestamp: 0,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        })
        .await?;

    wait_until!(chain_info.provider.get_block_number().await? == 3, Duration::from_secs(1));
    Ok(())
}

#[tokio::test]
async fn test_nitro_delayed_message_after_batch() -> Result<()> {
    let (mchain_url, _mchain, mchain) = start_mchain(APPCHAIN_CHAIN_ID, 0).await?;
    let chain_info = launch_nitro_node(NitroNodeArgs {
        chain_id: APPCHAIN_CHAIN_ID,
        chain_owner: Address::ZERO,
        parent_chain_url: mchain_url,
        parent_chain_id: MCHAIN_ID,
        sequencer_mode: NitroSequencerMode::None,
        chain_name: "appchain".to_string(),
        deployment: NitroDeployment {
            bridge: APPCHAIN_CONTRACT,
            sequencer_inbox: APPCHAIN_CONTRACT,
            deployed_at: 1,
            ..Default::default()
        },
        sequencer_private_key: None,
    })
    .await?;

    let qty = parse_ether("1")?;
    let msg = deposit_eth(Address::ZERO, get_signer().address(), qty);
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(
                arbitrum::batch::Batch(vec![]).encode()?,
                vec![msg.clone()],
            )),
            timestamp: 0,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        })
        .await?;

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
    tx.insert(0, 4);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage { header: Default::default(), l2_msg: vec![tx.into()] },
    )]);

    let msg: DelayedMessage = deposit_eth(Address::ZERO, TEST_ADDR, U256::from(1));
    mchain
        .add_batch(&MBlock {
            payload: Some(ArbitrumBatch::new(batch.encode()?, vec![msg])),
            timestamp: 0,
            slot: Slot { seq_block_number: 2, ..Default::default() },
        })
        .await?;

    wait_until!(chain_info.provider.get_block_number().await? == 3, Duration::from_secs(1));
    let block =
        chain_info.provider.get_block_by_number(BlockNumberOrTag::Number(2)).await?.unwrap();
    assert_eq!(block.transactions.len(), 2);
    let transactions: Vec<_> = block.transactions.hashes().collect();
    assert_eq!(transactions[1], *inner_tx.tx_hash());
    assert_eq!(chain_info.provider.get_balance(TEST_ADDR).await?, U256::from(2));

    Ok(())
}
