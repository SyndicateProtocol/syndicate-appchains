//! e2e tests for the metabased stack
use alloy::{
    eips::{BlockNumberOrTag, Encodable2718},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, Bytes, B256, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, Block, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
};
use chain_ingestor::client::IngestorProvider;
use contract_bindings::arbitrum::{
    arbsys::ArbSys, ibridge::IBridge, iinbox::IInbox, ioutbox::IOutbox, irollupcore::IRollupCore,
    nodeinterface::NodeInterface, rollup::Rollup,
};
use eyre::Result;
use mchain::client::Provider as _;
use serde::{Deserialize, Serialize};
use std::{str::FromStr as _, time::Duration};
use test_framework::components::{
    configuration::{ConfigurationOptions, ContractVersion},
    test_components::TestComponents,
};
use test_utils::{anvil::PRIVATE_KEY, wait_until};

const ARB_SYS_PRECOMPILE_ADDRESS: Address = address!("0x0000000000000000000000000000000000000064");
const NODE_INTERFACE_PRECOMPILE_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000000c8");

// an arbitrary eoa address used for testing
const TEST_ADDR: Address = address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380");

#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

#[tokio::test]
async fn e2e_send_transaction() -> Result<()> {
    let config = ConfigurationOptions { settlement_delay: 60, ..Default::default() };
    TestComponents::run(&config, |components| async move {
        // Set up the settlement rollup contract
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
        assert_eq!(
            components.appchain_provider.get_balance(wallet_address).await?,
            parse_ether("1")?
        );

        // Send a sequenced tx
        let tx = TransactionRequest::default()
            .with_to(TEST_ADDR)
            .with_value(U256::from(1))
            .with_nonce(0)
            .with_gas_limit(100_000)
            .with_chain_id(components.appchain_chain_id)
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
        wait_until!(
            components.mchain_provider.get_block_number().await >= 4,
            Duration::from_secs(10)
        );

        // check mchain block
        assert_eq!(components.mchain_provider.get_block_number().await, 4);

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
    })
    .await
}

#[tokio::test]
async fn e2e_deposit_300() -> Result<()> {
    e2e_deposit_base(ContractVersion::V300).await
}

#[tokio::test]
async fn e2e_deposit_213() -> Result<()> {
    e2e_deposit_base(ContractVersion::V213).await
}

/// This test sends different types of delayed messages
/// via the inbox contract and ensures that all of them
/// are sequenced via the metabased translator and show up on the rollup.
async fn e2e_deposit_base(version: ContractVersion) -> Result<()> {
    // Sequencer fees go to the zero address
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: Some(version), ..Default::default() },
        |components| async move {
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
                .with_chain_id(components.appchain_chain_id)
                .with_max_fee_per_gas(max_fee_per_gas)
                .with_max_priority_fee_per_gas(0)
                .build(components.settlement_provider.wallet())
                .await?
                .encode_2718(&mut inner_tx);
            let mut tx = vec![L2_MESSAGE_KIND_SIGNED_TX];
            tx.append(&mut inner_tx);
            _ = inbox.sendL2Message(tx.into()).send().await?;
            // Message From Origin - should be ignored by the translator
            inner_tx = vec![];
            TransactionRequest::default()
                .with_to(Address::ZERO)
                .with_value(parse_ether("0.1")?)
                .with_nonce(1)
                .with_gas_limit(gas_limit)
                .with_chain_id(components.appchain_chain_id)
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
                parse_ether("4.7000337")?
            );

            Ok(())
        },
    )
    .await
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
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: Some(version), ..Default::default() },
        |components| async move {
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
                .chain_id(components.appchain_chain_id)
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
            let node_interface = NodeInterface::new(
                NODE_INTERFACE_PRECOMPILE_ADDRESS,
                &components.appchain_provider,
            );
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
        },
    )
    .await
}

#[tokio::test]
async fn e2e_settlement_reorg() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: Some(ContractVersion::V300), ..Default::default() },
        |components| async move {
            // NOTE: at this point the mchain is on block 1 (initial mchain block) - we can't reorg
            // to genesis, so we need to create two slots on top of it before the reorg
            // happens.

            components.mine_both(1).await?; //mine mchain block 2 (only works because there are delayed txs to proccess, otherwise
                                            // the mchain block would be empty/skipped)
                                            // Wait for mchain to reach block 2

            wait_until!(
                components.mchain_provider.get_block_number().await == 2,
                Duration::from_secs(10)
            );

            let wallet_address = components.settlement_provider.default_signer_address();
            let inbox = IInbox::new(components.inbox_address, &components.settlement_provider);

            // create a deposit1 (that won't be rolled back) that will fit on mchain's block 3
            _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

            components.mine_both(100).await?;
            components.mine_both(1).await?; // extra blocks to close the slot

            // Wait for deposit1 to be processed
            wait_until!(
                components.appchain_provider.get_balance(wallet_address).await? ==
                    parse_ether("1")?,
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
                components.appchain_provider.get_balance(wallet_address).await? ==
                    parse_ether("2")?,
                Duration::from_secs(10)
            );
            assert_eq!(components.mchain_provider.get_block_number().await, 4);

            // the rollup head has not been updated yet
            let rollup_head_before_reorg = components
                .appchain_provider
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await?
                .unwrap();

            // reorg, assert that the L3 balance has disappeared
            let reorg_depth = 2;
            let current_block = components.settlement_provider.get_block_number().await?;
            components.settlement_provider.anvil_rollback(Some(reorg_depth)).await?;
            assert_eq!(
                components.settlement_provider.get_block_number().await?,
                current_block - reorg_depth
            );

            //NOTE: mine an extra block. (the ingestor currently polls by block number, so it won't
            // detect a reorg at a given block height, until a block with a bigger
            // height is mined) if we switch to a different ingestor implementation this
            // could be removed.

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
            let balance_after_reorg =
                components.appchain_provider.get_balance(wallet_address).await?;
            assert_eq!(balance_after_reorg, parse_ether("1.01")?);

            Ok(())
        },
    )
    .await
}

// simple storage contract for testing
sol! {
    #[sol(rpc, bytecode = "6080604052348015600e575f80fd5b5060405161020f38038061020f8339818101604052810190602e9190606b565b805f81905550506091565b5f80fd5b5f819050919050565b604d81603d565b81146056575f80fd5b50565b5f815190506065816046565b92915050565b5f60208284031215607d57607c6039565b5b5f6088848285016059565b91505092915050565b6101718061009e5f395ff3fe608060405234801561000f575f80fd5b506004361061003f575f3560e01c80633fa4f2451461004357806360fe47b1146100615780636d4ce63c1461007d575b5f80fd5b61004b61009b565b60405161005891906100c9565b60405180910390f35b61007b60048036038101906100769190610110565b6100a0565b005b6100856100a9565b60405161009291906100c9565b60405180910390f35b5f5481565b805f8190555050565b5f8054905090565b5f819050919050565b6100c3816100b1565b82525050565b5f6020820190506100dc5f8301846100ba565b92915050565b5f80fd5b6100ef816100b1565b81146100f9575f80fd5b50565b5f8135905061010a816100e6565b92915050565b5f60208284031215610125576101246100e2565b5b5f610132848285016100fc565b9150509291505056fea26469706673582212203294a1485ad6035630092feab3c12235dae68a8920350d2678d9097f36cce44364736f6c634300081a0033")]
    contract Storage {
        uint256 public value;

        constructor(uint256 _value) {
            value = _value;
        }

        function set(uint256 _value) public {
            value = _value;
        }

        function get() public view returns (uint256) {
            return value;
        }
    }
}

#[tokio::test]
async fn e2e_sequencing_reorg() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: Some(ContractVersion::V300), ..Default::default() },
        |components| async move {
            // deposit some funds to the appchain

            let signer = PrivateKeySigner::from_str(PRIVATE_KEY).unwrap();
            let wallet_address = components.settlement_provider.default_signer_address();
            let inbox = IInbox::new(components.inbox_address, &components.settlement_provider);

            // create a deposit1 (that won't be rolled back) that will fit on mchain's block 3
            _ = inbox.depositEth().value(parse_ether("10")?).send().await?;

            components.mine_set_block(0).await?;
            components.mine_both(1).await?;
            components.mine_set_block(1000).await?; // mine a set block far in the future so that sequencing blocks get slotted immediately

            wait_until!(
                components.mchain_provider.get_block_number().await == 2,
                Duration::from_secs(10)
            );
            wait_until!(
                components.appchain_provider.get_balance(wallet_address).await? ==
                    parse_ether("10")?,
                Duration::from_secs(10)
            );

            // deploy a storate contract on the appchain
            let initial_value = U256::from(42);
            let deploy_storage_tx =
                Storage::deploy_builder(&components.appchain_provider, initial_value)
                    .nonce(0)
                    .chain_id(components.appchain_chain_id)
                    .gas(100_000_000)
                    .max_fee_per_gas(100_000_000)
                    .max_priority_fee_per_gas(0)
                    .build_raw_transaction(signer.clone())
                    .await?;

            let receipt = components.sequence_tx(&deploy_storage_tx, 1, true).await?.unwrap();
            assert!(receipt.status());

            let storage_address = receipt.contract_address.unwrap();
            let storage = Storage::new(storage_address, &components.appchain_provider);

            let current_value = storage.get().call().await?._0;
            assert_eq!(current_value, initial_value);

            // update the stored value (this will be reorged later)
            let update_storage_tx = storage
                .set(U256::from(43))
                .nonce(1)
                .chain_id(components.appchain_chain_id)
                .gas(100_000_000)
                .max_fee_per_gas(100_000_000)
                .max_priority_fee_per_gas(0)
                .build_raw_transaction(signer.clone())
                .await?;

            let receipt = components.sequence_tx(&update_storage_tx, 1, true).await?.unwrap();
            assert!(receipt.status());

            let current_value = storage.get().call().await?._0;
            assert_eq!(current_value, U256::from(43));

            // reorg the sequencing chain by 1 block
            components.sequencing_provider.anvil_rollback(Some(1)).await?;

            // build the sequencing chain to a height above the reorg (so it is detected)
            components.sequence_tx(b"potato", 10, false).await?;
            components.sequence_tx(b"potato", 10, false).await?;

            // state is correctly rolled back
            wait_until!(storage.get().call().await?._0 == U256::from(42), Duration::from_secs(10));

            assert_eq!(
                components.appchain_provider.get_transaction_count(wallet_address).await?,
                1
            );

            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn e2e_reboot_without_settlement_processed() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: None, ..Default::default() },
        |components| async move {
            let set_offset = components.settlement_provider.get_block_number().await?;
            // mchain is on genesis (block 1)
            assert_eq!(components.mchain_provider.get_block_number().await, 1);

            // sequence any tx
            components.sequence_tx(b"my_tx_calldata", 10, false).await?;

            // mine a set block to close the slot, but without any transactions
            components.mine_set_block(100000).await?;

            // mchain should be on block 2
            wait_until!(
                components.mchain_provider.get_block_number().await == 2,
                Duration::from_secs(10)
            );
            let (slot, block_number) = components
                .mchain_provider
                .get_source_chains_processed_blocks(BlockNumberOrTag::Pending)
                .await?;
            assert_eq!(slot.seq_block_number, 2);
            assert_eq!(slot.set_block_number, 1 + set_offset);
            assert_eq!(block_number, 3);

            let (slot, block_number) = components
                .mchain_provider
                .get_source_chains_processed_blocks(BlockNumberOrTag::Number(block_number - 1))
                .await?;
            assert_eq!(slot.seq_block_number, 2);
            assert_eq!(slot.set_block_number, 1 + set_offset);
            assert_eq!(block_number, 2);

            // assert that restarting and rolling back here will not make mchain go back to block 1
            let seq_mchain_client =
                IngestorProvider::new(&components.sequencing_rpc_url, Duration::from_secs(1)).await;
            let settlement_client =
                IngestorProvider::new(&components.settlement_rpc_url, Duration::from_secs(1)).await;

            components
                .mchain_provider
                .reconcile_mchain_with_source_chains(&seq_mchain_client, &settlement_client)
                .await?;

            // mchain should be on the same block since no reorgs occurred
            assert_eq!(components.mchain_provider.get_block_number().await, 2);

            let (slot, block_number) = components
                .mchain_provider
                .get_source_chains_processed_blocks(BlockNumberOrTag::Pending)
                .await?;
            assert_eq!(slot.seq_block_number, 2);
            assert_eq!(slot.set_block_number, 1 + set_offset);
            assert_eq!(block_number, 3);

            let (slot, block_number) = components
                .mchain_provider
                .get_source_chains_processed_blocks(BlockNumberOrTag::Number(block_number - 1))
                .await?;
            assert_eq!(slot.seq_block_number, 2);
            assert_eq!(slot.set_block_number, 1 + set_offset);
            assert_eq!(block_number, 2);
            Ok(())
        },
    )
    .await
}

#[tokio::test]
async fn e2e_maestro_batch_sequencer_translator() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions { pre_loaded: None, use_write_loop: true, ..Default::default() },
        |components| async move {
            components.sequencing_provider.anvil_set_block_timestamp_interval(0).await?;
            components.sequencing_provider.anvil_set_auto_mine(true).await?;
            // Send a deposit to the appchain to make sure the from address has funds
            let wallet_address = components.sequencing_provider.default_signer_address();
            let value = parse_ether("0.01")?;
            let inbox = Rollup::new(components.inbox_address, &components.settlement_provider);
            _ = inbox.depositEth(wallet_address, wallet_address, value).send().await?;
            components.mine_set_block(0).await?;
            components.mine_set_block(1).await?;

            // Wait for deposit to be processed
            wait_until!(
                components.appchain_provider.get_balance(wallet_address).await? > U256::from(0),
                Duration::from_secs(10)
            );

            let chain_id = components.appchain_chain_id;
            let nonce = components.appchain_provider.get_transaction_count(wallet_address).await?;
            let tx = TransactionRequest::default()
                .from(wallet_address)
                .with_to(TEST_ADDR)
                .with_value(U256::from(0))
                .with_nonce(nonce)
                .with_gas_limit(100_000)
                .with_chain_id(chain_id)
                .with_max_fee_per_gas(100000000)
                .with_max_priority_fee_per_gas(0)
                .build(components.sequencing_provider.wallet())
                .await?;

            let tx_hash = components.send_maestro_tx_successful(&tx.encoded_2718()).await?;

            wait_until!(
                components.appchain_provider.get_transaction_count(wallet_address).await? ==
                    nonce + 1,
                Duration::from_secs(10)
            );

            // Verify that the transaction was processed
            let receipt = components.appchain_provider.get_transaction_receipt(tx_hash).await?;
            assert!(receipt.is_some());
            assert!(receipt.clone().unwrap().status());
            assert_eq!(receipt.unwrap().from, wallet_address);
            Ok(())
        },
    )
    .await
}
