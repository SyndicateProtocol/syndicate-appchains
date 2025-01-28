//! Integration tests for the metabased stack
#![allow(missing_docs)]

use alloy::{
    eips::{eip2718::Encodable2718, BlockNumberOrTag},
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, utils::parse_ether, Address, U256},
    providers::{Provider, ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner, Signer},
};
use block_builder::{connectors::anvil::MetaChainProvider, rollups::arbitrum};
use common::types::Block;
use contract_bindings::arbitrum::counter::Counter;
use e2e_tests::e2e_env::{wallet_from_private_key, TestEnv};
use eyre::{eyre, Result};
use serial_test::serial;
use std::time::Duration;
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

async fn launch_nitro_node() -> Result<(MetaChainProvider, Docker)> {
    let mchain = MetaChainProvider::start(Default::default()).await?;

    let nitro = Command::new("docker")
        .kill_on_drop(false) // kill via SIGTERM instead of SIGKILL
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg("offchainlabs/nitro-node:v3.4.0-rc.2-d896e9c-slim")
        .arg("--parent-chain.connection.url=".to_string() + mchain.anvil.endpoint_url().as_str())
        .arg("--node.dangerous.disable-blob-reader")
        .arg("--execution.forwarding-target=null")
        .arg("--execution.parent-chain-reader.old-header-timeout=1000h")
        .arg("--node.inbox-reader.check-delay=100ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg("--chain.info-json=".to_string() + &mchain.rollup_info)
        .arg("--http.addr=0.0.0.0")
        .arg("--http.port=8547")
        .arg("--log-level=DEBUG")
        .spawn()?;
    let rollup = ProviderBuilder::new().on_http("http://localhost:8547".parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    return timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        Ok::<_, eyre::Error>((mchain, Docker(nitro)))
    })
    .await?;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_nitro_batch() -> Result<()> {
    let (mchain, _nitro) = launch_nitro_node().await?;

    // deposit 1 eth
    let tx = mchain
        .rollup
        .depositEth(Address::default(), mchain.provider.default_signer_address(), parse_ether("1")?)
        .send()
        .await?
        .watch();
    mchain.mine_block(0).await?;
    tx.await?;

    // send a batch to sequence the deposit. include the init message as well.
    mchain
        .send_batch(&arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::Delayed; 2]))
        .await?;

    // wait 200ms for the batch to be processed
    tokio::time::sleep(Duration::from_millis(200)).await;
    let rollup = ProviderBuilder::new().on_http("http://localhost:8547".parse()?);
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
        .with_chain_id(13331370)
        .with_max_fee_per_gas(100000000)
        .with_max_priority_fee_per_gas(0)
        .build(&mchain.provider.wallet())
        .await?;

    inner_tx.encode_2718(&mut tx);
    let batch = arbitrum::batch::Batch(vec![arbitrum::batch::BatchMessage::L2(
        arbitrum::batch::L1IncomingMessage { header: Default::default(), l2_msg: vec![tx.into()] },
    )]);
    mchain.send_batch(&batch).await?;

    // wait 200ms for the batch to be processed
    tokio::time::sleep(Duration::from_millis(200)).await;
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
    // block should be mined deterministically - hash should be constant
    // update this hash whenever the test is modified
    assert_eq!(
        block.hash.to_string(),
        "0x2a1943b2a3a54f4855c2441b6864351e3cc282bd61a005fd0912467adf575951"
    );
    Ok(())
}
