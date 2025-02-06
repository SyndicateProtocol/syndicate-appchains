//! Integration tests for the metabased stack
#![allow(missing_docs)]

use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::{Anvil, AnvilInstance},
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{ext::AnvilApi as _, Provider, ProviderBuilder, RootProvider, WalletProvider},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner, Signer},
    transports::http::Http,
};
use block_builder::{
    block_builder::BlockBuilder,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::{
        anvil::{FilledProvider, MetaChainProvider},
        metrics::MChainMetrics,
    },
    metrics::BlockBuilderMetrics,
    rollups::arbitrum,
};
use common::{
    db::DummyStore,
    tracing::init_test_tracing,
    types::{Block, Chain},
};
use contract_bindings::{
    arbitrum::{counter::Counter, iinbox::IInbox, rollup::Rollup},
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule, metabasedsequencerchain::MetabasedSequencerChain,
    },
};
use e2e_tests::e2e_env::{wallet_from_private_key, TestEnv};
use eyre::{eyre, Result};
use ingestor::{
    config::{ChainIngestorConfig, IngestionPipelineConfig},
    eth_client::{EthClient, RPCClient},
    ingestor::Ingestor,
    metrics::IngestorMetrics,
};
use metrics::metrics::MetricsState;
use prometheus_client::registry::Registry;
use reqwest::Client;
use slotting::{config::SlotterConfig, metrics::SlotterMetrics, slotting::Slotter};
use std::{sync::Arc, time::Duration};
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::timeout,
};
/// Simple test scenario:
/// Bob tries to deploy a counter contract to L3, then tries to increment it
/// Bob's transactions are sequenced on the sequencing chain
/// Assert that the counter contract is deployed and that the counter is incremented on the L3 chain
#[tokio::test]
#[cfg_attr(not(feature = "e2e-tests"), ignore)]
async fn test_e2e_counter_contract() -> Result<()> {
    let env = TestEnv::new().await?;

    let bob_wallet = wallet_from_private_key(&env.accounts().bob.private_key, env.l3_chain_id());

    //
    // create and sign a transaction to deploy the counter contract
    let nonce = env.l3_chain().get_transaction_count(env.accounts().bob.address).await?;

    let counter_deploy_tx = TransactionRequest::default()
        .with_to(env.accounts().bob.address)
        .with_nonce(nonce)
        .with_chain_id(env.l3_chain_id())
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000)
        .with_input(Counter::BYTECODE.clone())
        .build(&bob_wallet)
        .await?;

    //
    // send bob's raw tx to be sequenced
    env.sequence_tx(counter_deploy_tx.encoded_2718().into()).await?;

    //TODO we might need to wait for the L3 to pick up the tx

    //
    // assert the tx was picked up by the L3 and the contract was deployed
    let receipt = env
        .l3_chain()
        .get_transaction_receipt(counter_deploy_tx.tx_hash().to_owned())
        .await?
        .unwrap();
    assert!(receipt.status(), "Contract deployment failed");

    let l3_counter_address = receipt.contract_address.unwrap();
    let counter = Counter::new(l3_counter_address, env.l3_chain());
    let number = counter.number().call().await?._0.to::<u64>();
    assert_eq!(number, 0, "Initial counter value should be 0");

    let increment_tx = TransactionRequest::default()
        .with_to(env.accounts().bob.address)
        .with_nonce(nonce + 1)
        .with_chain_id(env.l3_chain_id())
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000)
        .with_input(counter.increment().calldata().clone())
        .build(&bob_wallet)
        .await?;

    env.sequence_tx(increment_tx.encoded_2718().into()).await?;

    //
    // assert the tx was picked up by the L3 and the counter was incremented
    let receipt =
        env.l3_chain().get_transaction_receipt(increment_tx.tx_hash().to_owned()).await?.unwrap();
    assert!(receipt.status(), "Counter increment failed");

    let number = counter.number().call().await?._0.to::<u64>();
    assert_eq!(number, 1, "Counter should be incremented to 1");

    Ok(())
}

/// This test is to ensure that the system can resist garbage data being fed to the sequencing
/// contract
#[tokio::test]
#[cfg_attr(not(feature = "e2e-tests"), ignore)]
async fn test_e2e_resist_garbage_data() -> Result<()> {
    let env = TestEnv::new().await?;

    //
    //try to sequence an invalid transaction (unsigned)
    env.sequence_tx(b"foobar".into()).await?;

    //
    //try to sequence an invalid transaction (signed, but no balance)
    let signer_without_balance = PrivateKeySigner::from(SigningKey::from_slice(&[0u8; 32])?)
        .with_chain_id(Some(env.l3_chain_id()));
    let address_without_balance = signer_without_balance.address();
    let wallet_without_balance = EthereumWallet::from(signer_without_balance);

    let invalid_tx = TransactionRequest::default()
        .with_to(env.accounts().bob.address)
        .with_nonce(0)
        .with_chain_id(env.l3_chain_id())
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000)
        .with_input(Counter::BYTECODE.clone())
        .build(&wallet_without_balance)
        .await?;

    env.sequence_tx(invalid_tx.encoded_2718().into()).await?;

    //
    // now try to sequence a valid transaction
    // create and sign a transaction to deploy the counter contract
    let bob_wallet = wallet_from_private_key(&env.accounts().bob.private_key, env.l3_chain_id());
    let nonce = env.l3_chain().get_transaction_count(env.accounts().bob.address).await?;

    let counter_deploy_tx = TransactionRequest::default()
        .with_to(env.accounts().bob.address)
        .with_nonce(nonce)
        .with_chain_id(env.l3_chain_id())
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000)
        .with_input(Counter::BYTECODE.clone())
        .build(&bob_wallet)
        .await?;

    //
    // send bob's raw tx to be sequenced
    env.sequence_tx(counter_deploy_tx.encoded_2718().into()).await?;

    //TODO we might need to wait for the L3 to pick up the tx

    //
    // the system is expected to be resilient to garbage data, so only the valid tx should be
    // included in the L3 assert the valid tx was picked up by the L3 and the contract was
    // deployed
    let receipt = env
        .l3_chain()
        .get_transaction_receipt(counter_deploy_tx.tx_hash().to_owned())
        .await?
        .unwrap();
    assert!(receipt.status(), "Contract deployment failed");

    // assert the transaction count for the account without balance is 0
    assert_eq!(
        env.l3_chain().get_transaction_count(address_without_balance).await?,
        0,
        "Transaction count for the account without balance should be 0"
    );

    Ok(())
}

struct Docker(Child);

impl Drop for Docker {
    fn drop(&mut self) {
        if let Some(x) = self.0.id() {
            _ = std::process::Command::new("kill").arg(x.to_string()).output();
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    _ = self.0.wait().await;
                })
            })
        }
    }
}

struct Task(task::JoinHandle<()>);

impl Drop for Task {
    fn drop(&mut self) {
        self.0.abort();
    }
}

async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    provider.anvil_set_block_timestamp_interval(delay).await?;
    provider.evm_mine(None).await?;
    Ok(())
}

async fn launch_nitro_node(
    mchain: &MetaChainProvider,
    port: u64,
) -> Result<(Docker, RootProvider<Http<Client>>)> {
    let nitro = Command::new("docker")
        .kill_on_drop(false) // kill via SIGTERM instead of SIGKILL
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg("offchainlabs/nitro-node:v3.4.0-d896e9c-slim")
        .arg("--parent-chain.connection.url=".to_string() + mchain.anvil.endpoint_url().as_str())
        .arg("--node.dangerous.disable-blob-reader")
        .arg("--execution.forwarding-target=null")
        .arg("--execution.parent-chain-reader.old-header-timeout=1000h")
        .arg("--node.inbox-reader.check-delay=10ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg("--chain.info-json=".to_string() + &mchain.rollup_info("test"))
        .arg("--http.addr=0.0.0.0")
        .arg("--http.port=".to_string() + &port.to_string())
        .arg("--log-level=info")
        .spawn()?;
    let rollup = ProviderBuilder::new()
        .on_http(("http://localhost:".to_string() + &port.to_string()).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((Docker(nitro), rollup))
    })
    .await?
}

/// This test sends different types of delayed messages
/// via the inbox contract and ensures that all of them
/// are sequenced via the metabased translator and show up on the rollup.
#[tokio::test(flavor = "multi_thread")]
async fn e2e_settlement_test() -> Result<()> {
    init_test_tracing("info")?;
    let block_builder_cfg = BlockBuilderConfig {
        bridge_address: address!("0x199Beb469aEf45CBC2B5Fb1BE58690C9D12f45E2"),
        inbox_address: address!("0xD82DEBC6B9DEebee526B4cb818b3ff2EAa136899"),
        mchain_url: "http://127.0.0.1:8188".parse()?,
        sequencing_contract_address: get_rollup_contract_address(),
        ..Default::default()
    };

    // Launch mock sequencing chain and deploy contracts
    let (_seq_anvil, seq_provider) = start_anvil(8145, 15).await?;
    _ = MetabasedSequencerChain::deploy_builder(
        &seq_provider,
        U256::from(block_builder_cfg.target_chain_id),
        seq_provider.default_signer_address(),
        seq_provider.default_signer_address().create(1),
    )
    .send()
    .await?;
    _ = AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
    mine_block(&seq_provider, 0).await?;

    // Load settlement chain from state dump
    let (_set_anvil, set_provider) = load_anvil(8146).await?;

    // Launch ingestors for the sequencer and settlement chains
    let mut ingestor_config = IngestionPipelineConfig::default();
    ingestor_config.sequencing.sequencing_rpc_url = "http://localhost:8145".to_string();
    ingestor_config.sequencing.sequencing_polling_interval = Duration::from_millis(10);
    ingestor_config.settlement.settlement_rpc_url = "http://localhost:8146".to_string();
    ingestor_config.settlement.settlement_polling_interval = Duration::from_millis(10);
    let seq_config: ChainIngestorConfig = (&ingestor_config.sequencing).into();
    let set_config: ChainIngestorConfig = (&ingestor_config.settlement).into();
    let mut metrics_state = MetricsState { registry: Registry::default() };
    let sequencing_client: Arc<dyn RPCClient> =
        Arc::new(EthClient::new(&ingestor_config.sequencing.sequencing_rpc_url).await?);
    let settlement_client: Arc<dyn RPCClient> =
        Arc::new(EthClient::new(&ingestor_config.settlement.settlement_rpc_url).await?);
    let (sequencing_ingestor, sequencer_rx) = Ingestor::new(
        Chain::Sequencing,
        sequencing_client,
        &seq_config,
        IngestorMetrics::new(&mut metrics_state.registry),
    )
    .await?;
    let (settlement_ingestor, settlement_rx) = Ingestor::new(
        Chain::Settlement,
        settlement_client,
        &set_config,
        IngestorMetrics::new(&mut metrics_state.registry),
    )
    .await?;
    let (_dumy, dummy) = tokio::sync::oneshot::channel();
    let _seq_ingestor_task = Task(tokio::spawn(async move {
        sequencing_ingestor.start_polling(dummy).await.unwrap();
    }));
    let (_dummy, dummy) = tokio::sync::oneshot::channel();
    let _set_ingestor_task = Task(tokio::spawn(async move {
        settlement_ingestor.start_polling(dummy).await.unwrap();
    }));

    // start slotter at the genesis timestamp
    let slotter_cfg =
        SlotterConfig { start_slot_timestamp: GENESIS_TIMESTAMP, ..Default::default() };

    // Launch the slotter, block builder, and nitro rollup
    let (slotter, slotter_rx) = Slotter::new(
        &slotter_cfg,
        None,
        Box::new(DummyStore {}),
        SlotterMetrics::new(&mut metrics_state.registry),
    );
    let (_dummy, dummy) = tokio::sync::oneshot::channel();
    let _slotter_task = Task(tokio::spawn(async move {
        slotter.start(sequencer_rx, settlement_rx, dummy).await;
    }));
    let block_builder = BlockBuilder::new(
        slotter_rx,
        &block_builder_cfg,
        BlockBuilderMetrics::new(&mut metrics_state.registry),
    )
    .await?;
    let (_nitro, rollup) = launch_nitro_node(&block_builder.mchain, 8147).await?;
    let (_dummy, dummy) = tokio::sync::oneshot::channel();
    let _block_builder_task = Task(tokio::spawn(async move {
        block_builder.start(None, dummy).await;
    }));

    // sequence a deposit (unaliased address)
    // deposit is from the arbos address and does not increment the nonce
    let inbox = IInbox::new(address!("0xD82DEBC6B9DEebee526B4cb818b3ff2EAa136899"), &set_provider);
    _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

    // sequence l2 signed messages (unaliased address)
    const L2_MESSAGE_KIND_SIGNED_TX: u8 = 4;
    let gas_limit: u64 = 100_000;
    let max_fee_per_gas: u128 = 100_000_000;
    let mut inner_tx = vec![];
    TransactionRequest::default()
        .with_to(Address::ZERO)
        .with_value(parse_ether("0.1")?)
        .with_nonce(0)
        .with_gas_limit(gas_limit)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(0)
        .build(set_provider.wallet())
        .await?
        .encode_2718(&mut inner_tx);
    let mut tx = vec![L2_MESSAGE_KIND_SIGNED_TX];
    tx.append(&mut inner_tx);
    _ = inbox.sendL2Message(tx.into()).send().await?;

    inner_tx = vec![];
    TransactionRequest::default()
        .with_to(Address::ZERO)
        .with_value(parse_ether("0.1")?)
        .with_nonce(1)
        .with_gas_limit(gas_limit)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(0)
        .build(set_provider.wallet())
        .await?
        .encode_2718(&mut inner_tx);
    tx = vec![L2_MESSAGE_KIND_SIGNED_TX];
    tx.append(&mut inner_tx);
    _ = inbox.sendL2MessageFromOrigin(tx.into()).send().await?;

    // sequence retryable tickets that are automatically redeemed (aliased address)
    _ = inbox
        .createRetryableTicket(
            set_provider.default_signer_address(),
            U256::ZERO,
            parse_ether("0.00001")?,
            set_provider.default_signer_address(),
            set_provider.default_signer_address(),
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;
    _ = inbox
        .unsafeCreateRetryableTicket(
            set_provider.default_signer_address(),
            U256::ZERO,
            parse_ether("0.00001")?,
            set_provider.default_signer_address(),
            set_provider.default_signer_address(),
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;

    // sequence l2 unsigned messages (aliased address)
    _ = inbox
        .sendUnsignedTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            U256::from(2),
            set_provider.default_signer_address(),
            parse_ether("0.9")?,
            Default::default(),
        )
        .send()
        .await?;
    _ = inbox
        .sendContractTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            set_provider.default_signer_address(),
            parse_ether("0.9")?,
            Default::default(),
        )
        .send()
        .await?;

    // sequence l2fundedbyl1 messages (aliased address)
    _ = inbox
        .sendL1FundedUnsignedTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            U256::from(4),
            set_provider.default_signer_address(),
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;
    _ = inbox
        .sendL1FundedContractTransaction(
            U256::from(gas_limit),
            U256::from(max_fee_per_gas),
            set_provider.default_signer_address(),
            Default::default(),
        )
        .value(parse_ether("1")?)
        .send()
        .await?;

    mine_block(&set_provider, 0).await?;
    set_provider
        .evm_mine(Some(alloy::rpc::types::anvil::MineOptions::Options {
            timestamp: Some(GENESIS_TIMESTAMP + 10),
            blocks: None,
        }))
        .await?;
    let _seq_chain = MetabasedSequencerChain::new(get_rollup_contract_address(), &seq_provider);

    seq_provider.evm_mine(None).await?;

    // process slots
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert_eq!(rollup.get_block_number().await?, 17);
    assert_eq!(
        rollup.get_balance(set_provider.default_signer_address()).await?,
        parse_ether("4.6000316")?
    );
    Ok(())
}

/// This test tests the sequencing contract to make sure
/// blocks sequenced via the sequencing contract show up
/// on the rollup. It also checks to make sure missing slots
/// sequence a mchain block that does not include a batch.
#[tokio::test(flavor = "multi_thread")]
async fn e2e_test() -> Result<()> {
    init_test_tracing("info")?;
    let block_builder_cfg = BlockBuilderConfig {
        bridge_address: get_rollup_contract_address(),
        inbox_address: get_rollup_contract_address(),
        mchain_url: "http://127.0.0.1:8288".parse()?,
        sequencing_contract_address: get_rollup_contract_address(),
        ..Default::default()
    };

    // Launch mock sequencing chain and deploy contracts
    let (_seq_anvil, seq_provider) = start_anvil(8245, 15).await?;
    _ = MetabasedSequencerChain::deploy_builder(
        &seq_provider,
        U256::from(block_builder_cfg.target_chain_id),
        seq_provider.default_signer_address(),
        seq_provider.default_signer_address().create(1),
    )
    .send()
    .await?;
    _ = AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
    mine_block(&seq_provider, 0).await?;

    // Launch mock settlement chain and deploy contracts
    let (_set_anvil, set_provider) = start_anvil(8246, 20).await?;
    // Use the mock rollup contract for the test instead of deploying all the nitro rollup contracts
    _ = Rollup::deploy_builder(
        &set_provider,
        U256::from(block_builder_cfg.target_chain_id),
        MetaChainProvider::rollup_config(block_builder_cfg.target_chain_id),
    )
    .nonce(0)
    .send()
    .await?;
    mine_block(&set_provider, 0).await?;
    let set_rollup = Rollup::new(get_rollup_contract_address(), &set_provider);

    // Launch ingestors for the sequencer and settlement chains
    let mut ingestor_config = IngestionPipelineConfig::default();
    ingestor_config.sequencing.sequencing_rpc_url = "http://localhost:8245".to_string();
    ingestor_config.sequencing.sequencing_polling_interval = Duration::from_millis(10);
    ingestor_config.settlement.settlement_rpc_url = "http://localhost:8246".to_string();
    ingestor_config.settlement.settlement_polling_interval = Duration::from_millis(10);
    let seq_config: ChainIngestorConfig = (&ingestor_config.sequencing).into();
    let set_config: ChainIngestorConfig = (&ingestor_config.settlement).into();
    let mut metrics_state = MetricsState { registry: Registry::default() };
    let sequencing_client: Arc<dyn RPCClient> =
        Arc::new(EthClient::new(&ingestor_config.sequencing.sequencing_rpc_url).await?);
    let settlement_client: Arc<dyn RPCClient> =
        Arc::new(EthClient::new(&ingestor_config.settlement.settlement_rpc_url).await?);
    let (sequencing_ingestor, sequencer_rx) = Ingestor::new(
        Chain::Sequencing,
        sequencing_client,
        &seq_config,
        IngestorMetrics::new(&mut metrics_state.registry),
    )
    .await?;
    let (settlement_ingestor, settlement_rx) = Ingestor::new(
        Chain::Settlement,
        settlement_client,
        &set_config,
        IngestorMetrics::new(&mut metrics_state.registry),
    )
    .await?;
    let (_dumy, dummy) = tokio::sync::oneshot::channel();
    let _seq_ingestor_task = Task(tokio::spawn(async move {
        sequencing_ingestor.start_polling(dummy).await.unwrap();
    }));
    let (_dummy, dummy) = tokio::sync::oneshot::channel();
    let _set_ingestor_task = Task(tokio::spawn(async move {
        settlement_ingestor.start_polling(dummy).await.unwrap();
    }));

    let slotter_cfg =
        SlotterConfig { start_slot_timestamp: GENESIS_TIMESTAMP, ..Default::default() };

    let slot_duration = slotter_cfg.slot_duration;
    // Launch the slotter, block builder, and nitro rollup
    let (slotter, slotter_rx) = Slotter::new(
        &slotter_cfg,
        None,
        Box::new(DummyStore {}),
        SlotterMetrics::new(&mut metrics_state.registry),
    );
    let (_dummy, dummy) = tokio::sync::oneshot::channel();
    let _slotter_task = Task(tokio::spawn(async move {
        slotter.start(sequencer_rx, settlement_rx, dummy).await;
    }));
    let block_builder = BlockBuilder::new(
        slotter_rx,
        &block_builder_cfg,
        BlockBuilderMetrics::new(&mut metrics_state.registry),
    )
    .await?;
    let mchain = block_builder.mchain.provider.clone();
    let (_nitro, rollup) = launch_nitro_node(&block_builder.mchain, 8247).await?;
    let (_dummy, dummy) = tokio::sync::oneshot::channel();
    let _block_builder_task = Task(tokio::spawn(async move {
        block_builder.start(None, dummy).await;
    }));

    // sequence a deposit and regular tx at slot 1 (mchain block 2, rollup block 1-2)
    _ = set_rollup
        .depositEth(
            seq_provider.default_signer_address(),
            seq_provider.default_signer_address(),
            parse_ether("1")?,
        )
        .send()
        .await?;
    let seq_chain = MetabasedSequencerChain::new(get_rollup_contract_address(), &seq_provider);
    let mut tx = vec![];
    let inner_tx = TransactionRequest::default()
        .with_to(address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"))
        .with_value(U256::from(1))
        .with_nonce(0)
        .with_gas_limit(100_000)
        .with_chain_id(block_builder_cfg.target_chain_id)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(seq_provider.wallet())
        .await?;
    inner_tx.encode_2718(&mut tx);
    _ = seq_chain.processTransaction(tx.into()).send().await?;
    // insert a block to slot 1 @ ts 0, slot 2 @ ts 1. mine slot 1 -> mchain block 2.
    mine_block(&seq_provider, 0).await?;
    mine_block(&set_provider, 0).await?;
    mine_block(&seq_provider, 1).await?;
    mine_block(&set_provider, 1).await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    // check mchain blocks
    assert_eq!(mchain.get_block_number().await?, 2);
    let mchain_block: Block = mchain
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    // assert_eq!(mchain_block.timestamp, block_builder_cfg.genesis_timestamp); // why this assert?
    assert_eq!(mchain_block.transactions.len(), 2);
    // check rollup blocks
    assert_eq!(rollup.get_block_number().await?, 2);
    // check the first rollup block
    let rollup_block: Block = rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(1), true))
        .await?;
    assert_eq!(rollup_block.timestamp, block_builder_cfg.genesis_timestamp);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // check the second rollup block
    let rollup_block: Block = rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    assert_eq!(rollup_block.timestamp, block_builder_cfg.genesis_timestamp);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // tx hash should match
    assert_eq!(rollup_block.transactions[1].hash, *inner_tx.tx_hash());

    let test_addr: Address = "0xA9ec1Ed7008fDfdE38978Dfef4cF2754A969E5FA".parse()?;

    // Send another delayed message one slot out. The missing slot should sequence an empty block.
    _ = set_rollup
        .depositEth(seq_provider.default_signer_address(), test_addr, parse_ether("1")?)
        .send()
        .await?;
    mine_block(&seq_provider, slot_duration).await?;
    mine_block(&set_provider, slot_duration).await?;
    mine_block(&seq_provider, slot_duration).await?;
    mine_block(&set_provider, slot_duration).await?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    // check mchain blocks
    assert_eq!(mchain.get_block_number().await?, 4);
    // check mchain block 3
    let mchain_block: Block = mchain
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(3), true))
        .await?;
    assert_eq!(mchain_block.timestamp, block_builder_cfg.genesis_timestamp + slot_duration);
    assert_eq!(mchain_block.transactions.len(), 0);
    // check mchain block 4
    let mchain_block: Block = mchain
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(4), true))
        .await?;
    assert_eq!(mchain_block.timestamp, block_builder_cfg.genesis_timestamp + slot_duration * 2);
    assert_eq!(mchain_block.transactions.len(), 2);
    // check rollup block 3
    assert_eq!(rollup.get_block_number().await?, 3);
    let rollup_block: Block = rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(3), true))
        .await?;
    assert_eq!(rollup_block.timestamp, block_builder_cfg.genesis_timestamp + slot_duration * 2);
    // the first transaction is the startBlock transaction
    assert_eq!(rollup_block.transactions.len(), 2);
    // balance should match
    assert_eq!(rollup.get_balance(test_addr).await?, parse_ether("1")?);

    Ok(())
}

const GENESIS_TIMESTAMP: u64 = 1736824187;

// a dummy block before the genesis of the rollup is included to make sure the slotter can handle
// this case and ignore the block.
async fn start_anvil(port: u16, chain_id: u64) -> Result<(AnvilInstance, FilledProvider)> {
    let timestamp = GENESIS_TIMESTAMP.to_string();
    let args = vec![
        "--base-fee",
        "0",
        "--gas-limit",
        "30000000",
        "--timestamp",
        &timestamp,
        "--no-mining",
    ];

    let anvil = Anvil::new().port(port).chain_id(chain_id).args(args).try_spawn()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    mine_block(&provider, 50000).await?;
    mine_block(&provider, 50000).await?;
    provider.anvil_set_block_timestamp_interval(0).await?;
    Ok((anvil, provider))
}

async fn load_anvil(port: u16) -> Result<(AnvilInstance, FilledProvider)> {
    let state_file =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config").join("anvil.json");

    let timestamp = GENESIS_TIMESTAMP.to_string();

    #[allow(clippy::unwrap_used)]
    let args = vec![
        "--base-fee",
        "0",
        "--gas-limit",
        "30000000",
        "--timestamp",
        &timestamp, //NOTE: why is this necessary? shouldn't load_state have this info?
        "--no-mining",
        "--load-state",
        state_file.to_str().unwrap(),
    ];

    let anvil = Anvil::new().port(port).chain_id(31337).args(args).try_spawn()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    provider.anvil_set_block_timestamp_interval(0).await?;
    Ok((anvil, provider))
}

/// This test tests that rollup blocks are properly derived from batches created
/// via the block builder code and posted to the dummy rollup contract.
#[tokio::test(flavor = "multi_thread")]
async fn test_nitro_batch() -> Result<()> {
    let block_builder_cfg =
        BlockBuilderConfig { mchain_url: "http://127.0.0.1:8388".parse()?, ..Default::default() };

    let mut metrics_state = MetricsState { registry: Registry::default() };
    let metrics = MChainMetrics::new(&mut metrics_state.registry);

    let mchain = MetaChainProvider::start(&block_builder_cfg, &metrics).await?;
    mchain.provider.anvil_set_block_timestamp_interval(1).await?;
    let (_nitro, rollup) = launch_nitro_node(&mchain, 8347).await?;

    // deposit 1 eth
    _ = mchain
        .rollup
        .depositEth(Address::default(), mchain.provider.default_signer_address(), parse_ether("1")?)
        .send()
        .await?;
    mchain.mine_block(0).await?;

    // send a batch to sequence the deposit.
    _ = mchain
        .rollup
        .postBatch(arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed]).encode()?)
        .send()
        .await?;
    mchain.mine_block(0).await?;

    // wait 20ms for the batch to be processed
    tokio::time::sleep(Duration::from_millis(20)).await;
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
    mchain.mine_block(0).await?;

    // wait 20ms for the batch to be processed
    tokio::time::sleep(Duration::from_millis(20)).await;
    if rollup.get_block_number().await? != 2 {
        return Err(eyre!("block derivation failed - not on block 2"));
    }

    // check that the tx was sequenced
    let block: Block = rollup
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Number(2), true))
        .await?;
    // the first transaction is the startBlock transaction
    println!("{:#?}", block.transactions);
    assert_eq!(block.transactions.len(), 2);
    // tx hash should match
    assert_eq!(block.transactions[1].hash, *inner_tx.tx_hash());
    Ok(())
}
