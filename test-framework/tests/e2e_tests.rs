//! Integration tests for the metabased stack

use alloy::{
    eips::{BlockNumberOrTag, Encodable2718},
    network::TransactionBuilder,
    primitives::{address, utils::parse_ether, Address, Bytes, B256, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, Block, BlockTransactionsKind, TransactionRequest},
};
use contract_bindings::arbitrum::{
    arbgasinfo::ArbGasInfo, arbownerpublic::ArbOwnerPublic, arbsys::ArbSys, ibridge::IBridge,
    iinbox::IInbox, ioutbox::IOutbox, irollupcore::IRollupCore, nodeinterface::NodeInterface,
    rollup::Rollup,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use test_framework::components::{
    get_rollup_contract_address, Components, ConfigurationOptions, ContractVersion, APPCHAIN_OWNER,
};
use test_utils::{utils::assert_eventually, wait_until};
use tokio::time::sleep;

const ARB_SYS_PRECOMPILE_ADDRESS: Address = address!("0x0000000000000000000000000000000000000064");
const NODE_INTERFACE_PRECOMPILE_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000000c8");

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
async fn new_test_with_synced_chains(
    config: Option<ConfigurationOptions>,
    contract_version: Option<ContractVersion>,
) -> Result<Components> {
    // Start the meta node (pre-loaded with the full set of Arb contracts)
    let config_options = config.unwrap_or(ConfigurationOptions {
        sequencing_start_block: 3,
        settlement_start_block: 1,
        settlement_delay: 0,
    });
    let components = Components::new(contract_version, Some(config_options)).await?;

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
    let components = new_test_with_synced_chains(None, Some(ContractVersion::V300)).await?;

    // Grab the wallet address for the test
    // Sequencer fees go to the owner address
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
    wait_until!(
        components.appchain_provider.get_block_number().await? == 17,
        Duration::from_secs(2)
    );

    assert_eq!(
        components
            .appchain_provider
            .get_balance(components.settlement_provider.default_signer_address())
            .await?,
        parse_ether("4.6001407626")?
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_test() -> Result<()> {
    let config = ConfigurationOptions {
        sequencing_start_block: 3,
        settlement_start_block: 1,
        settlement_delay: 60,
    };
    let components = new_test_with_synced_chains(Some(config), None).await?;

    // Setup the settlement rollup contract
    let set_rollup = Rollup::new(get_rollup_contract_address(), &components.settlement_provider);

    // Send a deposit and a regular tx
    _ = set_rollup
        .depositEth(
            components.sequencing_provider.default_signer_address(),
            components.sequencing_provider.default_signer_address(),
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
        .with_chain_id(components.chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(components.sequencing_provider.wallet())
        .await?;
    inner_tx.encode_2718(&mut tx);
    _ = components.sequencing_contract.processTransaction(tx.into()).send().await?;

    // due to the delay for settlement, we need to space the sequencing chain, so the following
    // tx arrives at the same time, or after the deposit
    components.mine_seq_block(config.settlement_delay).await?;
    components.mine_set_block(0).await?;

    // mine 1 set block to close the opened slot that contains another deposit
    let test_addr: Address = "0xA9ec1Ed7008fDfdE38978Dfef4cF2754A969E5FA".parse()?;
    _ = set_rollup
        .depositEth(
            components.sequencing_provider.default_signer_address(),
            test_addr,
            parse_ether("1")?,
        )
        .send()
        .await?;
    components.mine_set_block(1).await?;

    // Wait for blocks to be processed
    wait_until!(
        components.appchain_provider.get_block_number().await? == 2,
        Duration::from_secs(3)
    );

    // check rollup blocks
    // check the first rollup block
    let rollup_block: Block = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Number(1), BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
    // assert_eq!(rollup_block.header.timestamp, config.slotter.settlement_delay);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // check the second rollup block
    let rollup_block: Block = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Number(2), BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
    // assert_eq!(rollup_block.header..timestamp, config.slotter.settlement_delay);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // tx hash should match
    let hashes: Vec<_> = rollup_block.transactions.hashes().collect();
    assert_eq!(hashes[1], *inner_tx.tx_hash());

    // sequence an empty slot
    components.mine_seq_block(0).await?;

    // mine the pending settlement block (which contains a deposit tx)
    components.mine_both(1).await?;

    // check rollup block
    wait_until!(
        components.appchain_provider.get_block_number().await? == 3,
        Duration::from_secs(3)
    );
    let rollup_block: Block = components
        .appchain_provider
        .get_block_by_number(BlockNumberOrTag::Number(3), BlockTransactionsKind::Hashes)
        .await?
        .unwrap();
    assert_eq!(rollup_block.header.timestamp, config.settlement_delay + 1);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // balance should match
    assert_eq!(components.appchain_provider.get_balance(test_addr).await?, parse_ether("1")?);

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
    let components = new_test_with_synced_chains(None, Some(version)).await?;

    let block: Block = components
        .settlement_provider
        .raw_request("eth_getBlockByNumber".into(), ("latest", true))
        .await?;
    components
        .sequencing_provider
        .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
        .await?;

    let inbox = IInbox::new(components.inbox_address, &components.settlement_provider);
    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;
    components.mine_set_block(0).await?;
    components.mine_set_block(1).await?;

    // Wait for deposit to be processed
    wait_until!(components.appchain_provider.get_block_number().await? > 0, Duration::from_secs(4));

    let bridge = IBridge::new(components.bridge_address, &components.settlement_provider);

    // 2. Send withdrawal transaction on the Appchain
    let mut tx = vec![];
    let arbsys = ArbSys::new(ARB_SYS_PRECOMPILE_ADDRESS, &components.appchain_provider);
    let gas_limit: u64 = 100_000;
    let max_fee_per_gas: u128 = 100_000_000;
    let withdrawal_value = parse_ether("0.1")?;
    let withdrawal_wallet = components.sequencing_provider.wallet();
    let to_address = address!("0x0000000000000000000000000000000000000001");
    arbsys
        .withdrawEth(to_address)
        .value(withdrawal_value)
        .nonce(0)
        .gas(gas_limit)
        .chain_id(components.chain_id)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(0)
        .into_transaction_request()
        .build(withdrawal_wallet)
        .await?
        .encode_2718(&mut tx);
    let _ = components.sequencing_contract.processTransaction(tx.into()).send().await?;
    components.mine_seq_block(0).await?;

    // Wait for the withdrawal transaction to be processed
    wait_until!(
        components.appchain_provider.get_block_number().await? == 10,
        Duration::from_secs(2)
    );

    // 2. Poster service posts
    sleep(Duration::from_secs(2)).await;

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
            proof.proof,                                             // proof
            U256::from(0),                                           // index
            components.sequencing_provider.default_signer_address(), // l2Sender
            to_address,                                              // to
            block.number,                                            // l2Block,
            block.l1_block_number,                                   // l1Block,
            block.timestamp,                                         // l2Timestamp,
            withdrawal_value,                                        // value
            Bytes::new(),                                            // data (always empty)
        )
        .send()
        .await?;

    components.mine_set_block(0).await?;

    // Assert new balance is equal to withdrawal amount
    let balance_after = components.settlement_provider.get_balance(to_address).await?;
    assert_eq!(balance_after, withdrawal_value);

    Ok(())
}
