//! e2e tests for the `synd-appchains` stack
use alloy::{
    eips::{BlockId, BlockNumberOrTag, Encodable2718},
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        address,
        utils::{parse_ether, parse_units},
        Address, U160, U256,
    },
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, Block, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
};
use contract_bindings::synd::{dummy_poster::DummyPoster, i_inbox::IInbox, rollup::Rollup};
use eyre::Result;
use std::time::Duration;
use synd_chain_ingestor::client::{IngestorProvider, IngestorProviderConfig};
use synd_mchain::client::Provider as _;
use test_framework::components::{
    configuration::{BaseChainsType, ConfigurationOptions},
    test_components::{TestComponents, SETTLEMENT_CHAIN_ID},
};
use test_utils::{chain_info::test_account1, preloaded_config::ContractVersion, wait_until};

// an arbitrary eoa address used for testing
const TEST_ADDR: Address = address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380");

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

#[tokio::test]
async fn e2e_send_transaction() -> Result<()> {
    let config = ConfigurationOptions { settlement_delay: 60, ..Default::default() };
    TestComponents::run(&config, |components| async move {
        // Set up the settlement appchain contract
        let set_appchain =
            Rollup::new(components.appchain_deployment.inbox, &components.settlement_provider);
        let wallet_address = components.settlement_provider.default_signer_address();

        // Send a deposit
        let _ = set_appchain
            .depositEth(wallet_address, wallet_address, parse_ether("1")?)
            .send()
            .await?;

        let set_timestamp = components
            .settlement_provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .unwrap()
            .header
            .timestamp;

        components
            .sequencing_provider
            .evm_mine(Some(MineOptions::Timestamp(Some(set_timestamp + config.settlement_delay))))
            .await?;
        components.mine_set_block(0).await?;
        // mine 1 set block to close the opened slot that contains the other deposit
        let test_addr: Address = "0xA9ec1Ed7008fDfdE38978Dfef4cF2754A969E5FA".parse()?;
        let _ =
            set_appchain.depositEth(wallet_address, test_addr, parse_ether("1")?).send().await?;
        components.mine_set_block(1).await?;

        // Wait for the deposit to arrive
        wait_until!(
            components.appchain_provider.get_block_number().await? == 1,
            Duration::from_secs(1)
        );

        // check the first appchain block
        let appchain_block: Block =
            components.appchain_provider.get_block_by_number(1.into()).await?.unwrap();

        assert_eq!(appchain_block.header.timestamp, set_timestamp + config.settlement_delay);
        // the first transaction is the startBlock transaction
        assert_eq!(appchain_block.transactions.len(), 2);
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
        _ = components
            .sequencing_contract
            .processTransaction(tx.encoded_2718().into())
            .send()
            .await?;
        _ = components
            .sequencing_contract
            .processTransaction(tx.encoded_2718().into())
            .send()
            .await?;
        components.mine_seq_block(0).await?;

        // Wait for the tx to arrive
        wait_until!(
            components.appchain_provider.get_block_number().await? == 2,
            Duration::from_secs(20)
        );

        // check the second appchain block
        let appchain_block: Block =
            components.appchain_provider.get_block_by_number(2.into()).await?.unwrap();

        assert_eq!(appchain_block.header.timestamp, set_timestamp + config.settlement_delay);
        // the first transaction is the startBlock transaction
        assert_eq!(appchain_block.transactions.len(), 2);
        // tx hash should match
        let hashes: Vec<_> = appchain_block.transactions.hashes().collect();
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

        // check appchain block
        wait_until!(
            components.appchain_provider.get_block_number().await? == 3,
            Duration::from_secs(10)
        );
        let appchain_block: Block =
            components.appchain_provider.get_block_by_number(3.into()).await?.unwrap();
        assert_eq!(appchain_block.header.timestamp, set_timestamp + config.settlement_delay + 1);
        // the first transaction is the startBlock transaction
        assert_eq!(appchain_block.transactions.len(), 2);
        // balance should match
        assert_eq!(components.appchain_provider.get_balance(test_addr).await?, parse_ether("1")?);

        Ok(())
    })
    .await
}

#[tokio::test]
async fn e2e_unsigned_tx() -> Result<()> {
    TestComponents::run(&Default::default(), |components| async move {
        let offset = components.sequencing_contract.OFFSET().call().await?;

        // Send a deposit
        let set_appchain =
            Rollup::new(components.appchain_deployment.inbox, &components.settlement_provider);
        let wallet_address = components.settlement_provider.default_signer_address();
        let alias_address =
            Address::from(U160::from_be_slice(&wallet_address[..]).wrapping_add(offset));
        let _ = set_appchain
            .depositEth(wallet_address, alias_address, parse_ether("1")?)
            .send()
            .await?;
        components.mine_both(0).await?;
        components.mine_set_block(1).await?;

        // Send unsigned tx
        let _ = components
            .sequencing_contract
            .sendUnsignedTransaction(
                1e6 as u64,
                parse_units("1", "gwei")?.into(),
                U256::ZERO,
                TEST_ADDR,
                parse_ether("0.1")?,
                Default::default(),
            )
            .send()
            .await?;

        // Send unsigned tx with the wrong nonce
        let _ = components
            .sequencing_contract
            .sendUnsignedTransaction(
                1e6 as u64,
                parse_units("1", "gwei")?.into(),
                U256::from(2),
                TEST_ADDR,
                parse_ether("0.1")?,
                Default::default(),
            )
            .send()
            .await?;

        // Try deploying a contract via an unsigned tx with the correct nonce
        let _ = components
            .sequencing_contract
            .sendUnsignedTransaction(
                1e6 as u64,
                parse_units("1", "gwei")?.into(),
                U256::ONE,
                Address::ZERO,
                Default::default(),
                DummyPoster::BYTECODE.clone(),
            )
            .send()
            .await?;

        // Wait for the tx to arrive
        components.mine_seq_block(0).await?;
        wait_until!(
            components.appchain_provider.get_block_number().await? == 2,
            Duration::from_secs(20)
        );

        // Check balance & nonce
        assert_eq!(components.appchain_provider.get_balance(TEST_ADDR).await?, parse_ether("0.1")?);
        assert_eq!(
            components.appchain_provider.get_code_at(alias_address.create(1)).await?,
            DummyPoster::DEPLOYED_BYTECODE
        );
        assert_eq!(components.appchain_provider.get_transaction_count(wallet_address).await?, 0);
        assert_eq!(
            components.sequencing_contract.contractNonce(wallet_address).call().await?,
            U256::ZERO
        );
        assert_eq!(
            components.sequencing_contract.contractNonce(alias_address).call().await?,
            U256::ZERO
        );
        assert_eq!(components.appchain_provider.get_transaction_count(alias_address).await?, 2);

        Ok(())
    })
    .await
}

#[tokio::test]
async fn e2e_contract_tx() -> Result<()> {
    TestComponents::run(&Default::default(), |components| async move {
        let offset = components.sequencing_contract.OFFSET().call().await?;

        // Send a deposit
        let set_appchain =
            Rollup::new(components.appchain_deployment.inbox, &components.settlement_provider);
        let wallet_address = components.settlement_provider.default_signer_address();
        let alias_address =
            Address::from(U160::from_be_slice(&wallet_address[..]).wrapping_add(offset));
        let _ = set_appchain
            .depositEth(wallet_address, alias_address, parse_ether("1")?)
            .send()
            .await?;
        components.mine_both(0).await?;
        components.mine_set_block(1).await?;

        // Send contract tx
        let _ = components
            .sequencing_contract
            .sendContractTransaction(
                1e6 as u64,
                parse_units("1", "gwei")?.into(),
                TEST_ADDR,
                parse_ether("0.1")?,
                Default::default(),
            )
            .send()
            .await?;

        // Send contract tx with the wrong balance
        let _ = components
            .sequencing_contract
            .sendContractTransaction(
                1e6 as u64,
                parse_units("1", "gwei")?.into(),
                TEST_ADDR,
                parse_ether("10")?,
                Default::default(),
            )
            .send()
            .await?;

        // Try deploying a contract via a contract tx
        let _ = components
            .sequencing_contract
            .sendContractTransaction(
                1e6 as u64,
                parse_units("1", "gwei")?.into(),
                Address::ZERO,
                Default::default(),
                DummyPoster::BYTECODE.clone(),
            )
            .send()
            .await?;

        // Wait for the txs to arrive
        components.mine_seq_block(0).await?;
        wait_until!(
            components.appchain_provider.get_block_number().await? >= 2,
            Duration::from_secs(20)
        );

        // Check balance, nonce, and code
        assert_eq!(components.appchain_provider.get_balance(TEST_ADDR).await?, parse_ether("0.1")?);
        assert_eq!(
            components.appchain_provider.get_code_at(alias_address.create(1)).await?,
            DummyPoster::DEPLOYED_BYTECODE
        );
        assert_eq!(components.appchain_provider.get_transaction_count(wallet_address).await?, 0);
        assert_eq!(
            components.sequencing_contract.contractNonce(wallet_address).call().await?,
            U256::from(3)
        );
        assert_eq!(
            components.sequencing_contract.contractNonce(alias_address).call().await?,
            U256::ZERO
        );
        assert_eq!(components.appchain_provider.get_transaction_count(alias_address).await?, 2);

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
/// are sequenced via the `synd-translator` and show up on the appchain.
async fn e2e_deposit_base(version: ContractVersion) -> Result<()> {
    // Sequencer fees go to the zero address
    TestComponents::run(
        &ConfigurationOptions {
            base_chains_type: BaseChainsType::PreLoaded(version),
            ..Default::default()
        },
        |components| async move {
            let wallet_address = components.settlement_provider.default_signer_address();

            // Send a deposit (unaliased address) delayed message
            // Deposit is from the arbos address and does not increment the nonce
            let inbox =
                IInbox::new(components.appchain_deployment.inbox, &components.settlement_provider);
            let _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

            const L2_MESSAGE_KIND_SIGNED_TX: u8 = 4;
            let gas_limit: u64 = 100_000;
            let max_fee_per_gas: u128 = 100_000_000;

            // Send l2 signed messages (unaliased address)
            // Message (not from origin)
            let mut tx = TransactionRequest::default()
                .with_to(Address::ZERO)
                .with_value(parse_ether("0.1")?)
                .with_nonce(0)
                .with_gas_limit(gas_limit)
                .with_chain_id(components.appchain_chain_id)
                .with_max_fee_per_gas(max_fee_per_gas)
                .with_max_priority_fee_per_gas(0)
                .build(components.settlement_provider.wallet())
                .await?
                .encoded_2718();
            tx.insert(0, L2_MESSAGE_KIND_SIGNED_TX);
            let _ = inbox.sendL2Message(tx.into()).send().await?;

            // Send retryable tickets that are automatically redeemed (aliased address)
            // Safe Retryable Ticket
            let _ = inbox
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
            let _ = inbox
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
            let _ = inbox
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
            let _ = inbox
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
            let _ = inbox
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
            let _ = inbox
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

            // Process the slot - wait for block 16 to be reached
            wait_until!(
                components.appchain_provider.get_block_number().await? == 16,
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
async fn e2e_settlement_reorg() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions {
            base_chains_type: BaseChainsType::PreLoaded(ContractVersion::V300),
            ..Default::default()
        },
        |components| async move {
            // NOTE: at this point the synd-mchain is on block 1 (initial mchain block) - we
            // can't reorg to genesis, so we need to create two slots on top of it
            // before the reorg happens.

            components.mine_both(1).await?; //mine mchain block 2 (only works because there are delayed txs to process,
                                            // otherwise the synd-mchain block would be
                                            // empty/skipped) Wait for synd-mchain to
                                            // reach block 2

            wait_until!(
                components.mchain_provider.get_block_number().await == 2,
                Duration::from_secs(10)
            );

            let wallet_address = components.settlement_provider.default_signer_address();
            let inbox =
                IInbox::new(components.appchain_deployment.inbox, &components.settlement_provider);

            // create a deposit1 (that won't be rolled back) that will fit on synd-mchain's block 3
            let _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

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

            let _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

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

            // the appchain head has not been updated yet
            let appchain_head_before_reorg = components
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

            // the appchain should have reorged to a pre-deposit block

            // create new slots

            // NOTE: build the tx manually, instead of using the much simpler
            // let _ = inbox.depositEth().value(parse_ether("0.01")?).send().await?;
            // this is because the contract_instance gets confused after a reorg and fails the
            // tests... re-creating the contract instance after reorg did not help.
            // (this is a bug in alloy.)
            // https://github.com/alloy-rs/alloy/issues/2668
            let deposit_tx = inbox
                .depositEth()
                .value(parse_ether("0.01")?)
                .nonce(
                    components
                        .settlement_provider
                        .get_transaction_count(test_account1().address)
                        .await?,
                )
                .gas(10_000_000)
                .max_fee_per_gas(10_000_000)
                .max_priority_fee_per_gas(0)
                .chain_id(SETTLEMENT_CHAIN_ID)
                .build_raw_transaction(test_account1().signer.clone())
                .await?;
            let _ = components.settlement_provider.send_raw_transaction(&deposit_tx).await?;

            components.mine_both(500).await?;
            components.mine_both(500).await?; // build `synd-mchain` to a height above what the appchain has seen before the reorg

            wait_until!(let appchain_head = components
                .appchain_provider
                .get_block_by_number(BlockNumberOrTag::Latest)
                .await?
                .unwrap();
                appchain_head.header.number == appchain_head_before_reorg.header.number &&
                appchain_head.header.hash != appchain_head_before_reorg.header.hash,
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
        &ConfigurationOptions {
            base_chains_type: BaseChainsType::PreLoaded(ContractVersion::V300),
            ..Default::default()
        },
        |components| async move {
            // deposit some funds to the appchain

            let signer = test_account1().signer.clone();
            let wallet_address = components.settlement_provider.default_signer_address();
            let inbox =
                IInbox::new(components.appchain_deployment.inbox, &components.settlement_provider);

            // create a deposit (that won't be rolled back) that will fit on synd-mchain's block 3
            let _ = inbox.depositEth().value(parse_ether("10")?).send().await?;

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

            // deploy a storage contract on the appchain
            let initial_value = U256::from(42);
            let deploy_storage_tx: Vec<u8> =
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

            let current_value = storage.get().call().await?;
            assert_eq!(current_value, initial_value);

            // update the stored value (this will be reorged later)
            let update_storage_tx: Vec<u8> = storage
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

            let current_value = storage.get().call().await?;
            assert_eq!(current_value, U256::from(43));

            // reorg the sequencing chain by 1 block
            components.sequencing_provider.anvil_rollback(Some(1)).await?;

            // sequence a valid tx so that an appchain batch is created - nitro will not reorg until
            // the new batch is created.
            let appchain_tx = TransactionRequest::default()
                .nonce(0)
                .gas_limit(0)
                .to(Address::ZERO)
                .gas_price(0)
                .build(&EthereumWallet::from(PrivateKeySigner::random()))
                .await?
                .encoded_2718();

            components.sequence_tx(&appchain_tx, 10, false).await?;

            // state is correctly rolled back
            wait_until!(storage.get().call().await? == U256::from(42), Duration::from_secs(10));

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
    TestComponents::run(&Default::default(), |components| async move {
        let set_offset = components.settlement_provider.get_block_number().await?;
        // synd-mchain is on genesis (block 1)
        assert_eq!(components.mchain_provider.get_block_number().await, 1);

        // sequence a valid tx so that an appchain batch & mchain block is created
        let appchain_tx = TransactionRequest::default()
            .nonce(0)
            .gas_limit(0)
            .to(Address::ZERO)
            .gas_price(0)
            .build(&EthereumWallet::from(PrivateKeySigner::random()))
            .await?
            .encoded_2718();
        components.sequence_tx(&appchain_tx, 10, false).await?;
        let seq_block = components.sequencing_provider.get_block_number().await?;

        // mine a set block to close the slot, but without any transactions
        components.mine_set_block(100000).await?;

        // synd-mchain should be on block 2
        wait_until!(
            components.mchain_provider.get_block_number().await == 2,
            Duration::from_secs(10)
        );
        let (slot, block_number) = components
            .mchain_provider
            .get_source_chains_processed_blocks(BlockNumberOrTag::Pending)
            .await?;
        assert_eq!(slot.seq_block_number, seq_block);
        assert_eq!(slot.set_block_number, 1 + set_offset);
        assert_eq!(block_number, 3);

        let (slot, block_number) = components
            .mchain_provider
            .get_source_chains_processed_blocks(BlockNumberOrTag::Number(block_number - 1))
            .await?;
        assert_eq!(slot.seq_block_number, seq_block);
        assert_eq!(slot.set_block_number, 1 + set_offset);
        assert_eq!(block_number, 2);

        // assert that restarting and rolling back here will not make synd-mchain go back to
        // block 1
        let seq_mchain_client = IngestorProvider::new(
            components.sequencing_ingestor_rpc_url.as_ref(),
            IngestorProviderConfig { timeout: Duration::from_secs(1), ..Default::default() },
        )
        .await;
        let settlement_client = IngestorProvider::new(
            components.settlement_ingestor_rpc_url.as_ref(),
            IngestorProviderConfig { timeout: Duration::from_secs(1), ..Default::default() },
        )
        .await;

        components
            .mchain_provider
            .reconcile_mchain_with_source_chains(&seq_mchain_client, &settlement_client)
            .await?;

        // synd-mchain should be on the same block since no reorgs occurred
        assert_eq!(components.mchain_provider.get_block_number().await, 2);

        let (slot, block_number) = components
            .mchain_provider
            .get_source_chains_processed_blocks(BlockNumberOrTag::Pending)
            .await?;
        assert_eq!(slot.seq_block_number, seq_block);
        assert_eq!(slot.set_block_number, 1 + set_offset);
        assert_eq!(block_number, 3);

        let (slot, block_number) = components
            .mchain_provider
            .get_source_chains_processed_blocks(BlockNumberOrTag::Number(block_number - 1))
            .await?;
        assert_eq!(slot.seq_block_number, seq_block);
        assert_eq!(slot.set_block_number, 1 + set_offset);
        assert_eq!(block_number, 2);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn e2e_maestro_batch_sequencer_translator() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions { use_write_loop: true, ..Default::default() },
        |components| async move {
            components.sequencing_provider.anvil_set_block_timestamp_interval(0).await?;
            components.sequencing_provider.anvil_set_auto_mine(true).await?;
            // Send a deposit to the appchain to make sure the from address has funds
            let wallet_address = components.sequencing_provider.default_signer_address();
            let value = parse_ether("0.01")?;
            let inbox =
                Rollup::new(components.appchain_deployment.inbox, &components.settlement_provider);
            let _ = inbox.depositEth(wallet_address, wallet_address, value).send().await?;
            components.mine_set_block(0).await?;
            components.mine_seq_block(1).await?;
            components.mine_set_block(1000).await?;

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

#[tokio::test]
async fn e2e_maestro_reorg_handling() -> Result<()> {
    let config_opts = ConfigurationOptions {
        use_write_loop: true,
        // tx takes 3s to be considered final, and that is checked every second
        maestro_finalization_duration: Some(Duration::from_secs(5)),
        maestro_finalization_checker_interval: Some(Duration::from_secs(1)),
        ..Default::default()
    };

    TestComponents::run(&config_opts, |components| async move {
        let wallet_address = components.sequencing_provider.default_signer_address();
        let chain_id = components.appchain_chain_id;

        // 1. Deposit funds to ensure the wallet can send a transaction
        let deposit_value = parse_ether("0.01")?;
        let inbox =
            Rollup::new(components.appchain_deployment.inbox, &components.settlement_provider);
        let _ = inbox.depositEth(wallet_address, wallet_address, deposit_value).send().await?;
        components.mine_set_block(0).await?;
        components.mine_set_block(10000000).await?; // mine a set block far in the future so that sequencing blocks get slotted immediately
        wait_until!(
            components.appchain_provider.get_balance(wallet_address).await? > U256::ZERO,
            Duration::from_secs(10)
        );

        // 2. Send an initial transaction via Maestro
        let initial_nonce =
            components.appchain_provider.get_transaction_count(wallet_address).await?;
        let tx = TransactionRequest::default()
            .from(wallet_address)
            .with_to(TEST_ADDR)
            .with_value(U256::from(0))
            .with_nonce(initial_nonce)
            .with_gas_limit(100_000)
            .with_chain_id(chain_id)
            .with_max_fee_per_gas(100_000_000)
            .with_max_priority_fee_per_gas(0)
            .build(components.sequencing_provider.wallet())
            .await?;
        let tx_encoded = tx.encoded_2718();
        let tx_hash = components.send_maestro_tx_successful(&tx_encoded).await?;

        // mine sequencing blocks until we see the tx from the batcher being included
        wait_until!(
            components.mine_seq_block(1).await?;
            components.sequencing_provider.get_block(BlockId::Number(BlockNumberOrTag::Latest)).await?.unwrap().transactions.len() == 1,
            Duration::from_secs(10)
        );
        // 3. Verify the initial transaction was processed
        wait_until!(
            components.appchain_provider.get_transaction_count(wallet_address).await? ==
                initial_nonce + 1,
            Duration::from_secs(10)
        );
        let receipt_before_reorg =
            components.appchain_provider.get_transaction_receipt(tx_hash).await?;
        assert!(receipt_before_reorg.is_some(), "Receipt for initial tx should exist before reorg");
        assert!(
            receipt_before_reorg.clone().unwrap().status(),
            "Initial tx should be successful before reorg"
        );

        // 4. Simulate a reorg on the sequencing chain
        components.sequencing_provider.anvil_rollback(Some(1)).await?;
        // re-build a block on top, must submit a tx so the reorg is detected by nitro
        components.sequence_tx(b"potato", 10, false).await?; 

        // 5. Verify the appchain reflects the reorg
        wait_until!(
            components.appchain_provider.get_transaction_count(wallet_address).await? ==
                initial_nonce,
            Duration::from_secs(10)
        );
        let receipt_after_reorg =
            components.appchain_provider.get_transaction_receipt(tx_hash).await?;
        assert!(receipt_after_reorg.is_none(), "Receipt should be gone after reorg");

        // mine sequencing blocks until we see the re-submitted tx from the batcher being included
        wait_until!(
            components.mine_seq_block(1).await?;
            components.sequencing_provider.get_block(BlockId::Number(BlockNumberOrTag::Latest)).await?.unwrap().transactions.len() == 1,
            Duration::from_secs(10)
        );

        // 6. Wait for Maestro's finalization background task to detect and re-submit the tx,
        // Verify the transaction is sequenced again
        wait_until!(
            components.appchain_provider.get_transaction_count(wallet_address).await? ==
                initial_nonce + 1,
            Duration::from_secs(10)
        );
        let receipt_after_resubmission =
            components.appchain_provider.get_transaction_receipt(tx_hash).await?;
        assert!(
            receipt_after_resubmission.is_some(),
            "Receipt should exist again after Maestro resubmission"
        );
        assert!(
            receipt_after_resubmission.clone().unwrap().status(),
            "Resubmitted tx should be successful"
        );
        assert_eq!(
            receipt_after_resubmission.unwrap().from,
            wallet_address,
            "From address should match on resubmitted tx"
        );

        Ok(())
    })
    .await
}
