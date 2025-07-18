//! e2e tests for the `synd-withdrawals`
use alloy::{
    contract::CallBuilder,
    eips::{BlockNumberOrTag, Encodable2718},
    network::Ethereum,
    primitives::{address, keccak256, utils::parse_ether, Address, B256, U256},
    providers::{
        ext::{AnvilApi, DebugApi},
        Provider, ProviderBuilder,
    },
    rpc::types::{
        anvil::MineOptions,
        trace::geth::{GethDebugTracingOptions, GethTrace},
    },
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
};
use contract_bindings::synd::{
    assertion_poster::AssertionPoster, i_bridge::IBridge, i_inbox::IInbox, i_rollup::IRollup,
    tee_key_manager::TeeKeyManager, tee_module::TeeModule,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, time::Duration};
use test_framework::components::{
    configuration::{BaseChainsType, ConfigurationOptions},
    proposer::ProposerConfig,
    test_components::{TestComponents, SETTLEMENT_CHAIN_ID},
};
use test_utils::{
    chain_info::{test_account1, test_account2, test_account3, PRIVATE_KEY3},
    docker::{launch_enclave_server, start_component},
    nitro_chain::{execute_withdrawal, init_withdrawal_tx},
    port_manager::PortManager,
    wait_until,
};
use tokio::task::JoinHandle;

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

#[tokio::test]
async fn e2e_tee_withdrawal_with_eigenda() -> Result<()> {
    e2e_tee_withdrawal(BaseChainsType::NitroWithEigenda).await
}

#[tokio::test]
async fn e2e_tee_withdrawal_with_calldata() -> Result<()> {
    e2e_tee_withdrawal(BaseChainsType::Nitro).await
}

#[allow(clippy::unwrap_used)]
async fn e2e_tee_withdrawal(base_chains_type: BaseChainsType) -> Result<()> {
    let close_challenge_interval = Duration::from_secs(1);
    let settlement_delay = 2;
    TestComponents::run(
        &ConfigurationOptions {
            base_chains_type,
            rollup_owner: test_account1().address,
            settlement_delay,
            close_challenge_interval,
            ..Default::default()
        },
        |components| async move {
            // deposit funds to the appchain
            let inbox =
                IInbox::new(components.appchain_deployment.inbox, &components.settlement_provider);

            // NOTE: manually setting the nonce shouldn't be necessary, likey an artifact of: https://github.com/alloy-rs/alloy/issues/2668
            let receipt = inbox
                .depositEth()
                .value(parse_ether("1")?)
                .nonce(
                    components
                        .settlement_provider
                        .get_transaction_count(test_account1().address)
                        .await?,
                )
                .send()
                .await?
                .get_receipt()
                .await?;
            assert!(receipt.status());

            // deploy contracts:
            // - Tee module
            // - key manager
            // - mock zk-verifier
            // - set the TEE module as the owner for the assertion poster contract (call
            //   `transferOwnership`)

            let signer = test_account1().signer.clone();

            let zk_verifier_addr = deploy_on_settlement_chain(
                &components,
                MockZkVerifier::deploy_builder(&components.settlement_provider),
                &signer,
            )
            .await?;

            let key_mgr_addr = deploy_on_settlement_chain(
                &components,
                TeeKeyManager::deploy_builder(&components.settlement_provider, zk_verifier_addr),
                &signer,
            )
            .await?;

            let appchain_start_block_hash = components
                .appchain_provider
                .get_block_by_number(BlockNumberOrTag::Number(0))
                .await?
                .unwrap()
                .header
                .hash;

            let seq_start_block_hash = components
                .sequencing_provider
                .get_block_by_number(BlockNumberOrTag::Number(0))
                .await?
                .unwrap()
                .header
                .hash;

            let sequencing_bridge_address =
                components.sequencing_deployment.as_ref().unwrap().bridge;
            let config_hash = VerifierConfig {
                sequencing_contract_address: *components.sequencing_contract.address(),
                sequencing_bridge_address,
                settlement_delay,
            }
            .hash();

            let l1_provider = components.l1_provider.as_ref().unwrap();
            let sequencing_bridge = IBridge::new(sequencing_bridge_address, l1_provider);

            let l1_start_batch_acc =
                sequencing_bridge.sequencerInboxAccs(U256::from(U256::ZERO)).call().await?;

            // setup an oracle to push L1 data onto the settlement chain (could potentially be
            // removed if we use actual optimism stack (SEQ-1079))

            // NOTE: use test_account2 for the oracle to avoid race condition with test_account1
            // nonces
            let tx = IInbox::new(
                components.settlement_deployment.as_ref().unwrap().inbox,
                l1_provider.clone(),
            )
            .depositEth()
            .value(parse_ether("10")?)
            .nonce(l1_provider.get_transaction_count(test_account2().address).await?)
            .gas(100_000)
            .max_fee_per_gas(100_000_000)
            .max_priority_fee_per_gas(0)
            .build_raw_transaction(test_account2().signer.clone())
            .await?;
            _ = l1_provider.send_raw_transaction(&tx).await?;
            wait_until!(
                components.settlement_provider.get_balance(test_account2().address).await? >=
                    parse_ether("10")?,
                Duration::from_secs(20)
            );

            let oracle_settlement_provider = ProviderBuilder::new()
                .wallet(test_account2().signer.clone())
                .connect(&components.settlement_rpc_url)
                .await?;
            let (_l1_oracle_handle, l1_oracle_address) =
                setup_l1_oracle(l1_provider.clone(), oracle_settlement_provider).await;

            // Mine a block on L1 to trigger the oracle update after subscription is established
            l1_provider
                .evm_mine(Some(MineOptions::Options { timestamp: None, blocks: Some(1) }))
                .await?;

            // wait until the oracle is operational
            wait_until!(
                L1BlockOracle::new(l1_oracle_address, components.settlement_provider.clone())
                    .timestamp()
                    .call()
                    .await? >
                    0,
                Duration::from_secs(20)
            );

            let tee_module_addr = deploy_on_settlement_chain(
                &components,
                TeeModule::deploy_builder(
                    &components.settlement_provider,
                    components.assertion_poster_address,
                    components.appchain_deployment.bridge,
                    config_hash,
                    appchain_start_block_hash,
                    seq_start_block_hash,
                    l1_start_batch_acc,
                    l1_oracle_address,
                    false,
                    1, // challenge_window_duration - 1 second
                    key_mgr_addr,
                ),
                &signer,
            )
            .await?;

            // set the TEE module as the owner for the assertion poster contract
            let assertion_poster = AssertionPoster::new(
                components.assertion_poster_address,
                &components.settlement_provider,
            );

            // NOTE: manually setting the nonce shouldn't be necessary, likey an artifact of: https://github.com/alloy-rs/alloy/issues/2668
            let receipt = assertion_poster
                .transferOwnership(tee_module_addr)
                .nonce(
                    components
                        .settlement_provider
                        .get_transaction_count(test_account1().address)
                        .await?,
                )
                .send()
                .await?
                .get_receipt()
                .await?;
            assert!(receipt.status());

            // start enclave and proposer, obtain the tee public key
            let (mut enclave_server_instance, enclave_rpc_url, tee_public_key) =
                launch_enclave_server().await?;

            // deposit funds for the proposer to use on the settlement chain
            let tx = IInbox::new(
                components.settlement_deployment.as_ref().unwrap().inbox,
                l1_provider.clone(),
            )
            .depositEth()
            .value(parse_ether("10")?)
            .nonce(l1_provider.get_transaction_count(test_account3().address).await?)
            .gas(100_000)
            .max_fee_per_gas(100_000_000)
            .max_priority_fee_per_gas(0)
            .build_raw_transaction(test_account3().signer.clone())
            .await?;
            _ = l1_provider.send_raw_transaction(&tx).await?;
            wait_until!(
                components.settlement_provider.get_balance(test_account3().address).await? >=
                    parse_ether("10")?,
                Duration::from_secs(20)
            );

            let proposer_config = ProposerConfig {
                ethereum_rpc_url: components.l1_ws_rpc_url.clone(),
                tee_module_contract_address: tee_module_addr,
                settlement_rpc_url: components.settlement_rpc_url.clone(),
                port: PortManager::instance().next_port().await,
                appchain_rpc_url: components.appchain_ws_rpc_url.clone(),
                sequencing_rpc_url: components.sequencing_rpc_url.clone(),
                enclave_rpc_url,
                polling_interval: "1s".to_string(),
                close_challenge_interval: format!("{}s", close_challenge_interval.as_secs()),
                settlement_chain_id: SETTLEMENT_CHAIN_ID,
                eigen_rpc_url: components
                    .eigenda_proxy_url
                    .clone()
                    .unwrap_or_else(|| "http://localhost:0000".to_string()),
                appchain_bridge_address: components.appchain_deployment.bridge,
                sequencing_contract_address: *components.sequencing_contract.address(),
                sequencing_bridge_address_on_l1: sequencing_bridge_address,
                settlement_delay,
                private_key: PRIVATE_KEY3.to_string().strip_prefix("0x").unwrap().to_string(),
            };

            // add the TEE signer address to the key manager using a mock proof
            let key_mgr = TeeKeyManager::new(key_mgr_addr, components.settlement_provider.clone());
            let public_values = tee_public_key.abi_encode();
            let proof_bytes = vec![];

            // NOTE: manually setting the nonce shouldn't be necessary, likey an artifact of: https://github.com/alloy-rs/alloy/issues/2668
            let receipt = key_mgr
                .addKey(public_values.into(), proof_bytes.into())
                .nonce(
                    components
                        .settlement_provider
                        .get_transaction_count(test_account1().address)
                        .await?,
                )
                .send()
                .await?
                .get_receipt()
                .await?;
            assert!(receipt.status());

            let is_valid = key_mgr.isKeyValid(tee_public_key).call().await?;
            assert!(is_valid);

            let mut proposer_instance = start_component(
                "synd-proposer",
                proposer_config.port,
                proposer_config.cli_args(),
                Default::default(),
            )
            .await?;

            // Catch if the services die (NOTE: this won't kill the test, just log it)
            tokio::spawn(async move {
                tokio::select! {
                    e = proposer_instance.wait() => {
                        panic!("synd-proposer died: {e:#?}")
                    },
                    e = enclave_server_instance.wait() => panic!("enclave server died: {e:#?}"),
                }
            });

            // send a dummy tx so that the sequencing chain progresses and the deposit is
            // slotted in
            components.sequence_tx(b"dummy_tx", 0, false).await?;

            wait_until!(
                components.appchain_provider.get_balance(test_account1().address).await? >=
                    parse_ether("1")?,
                Duration::from_secs(60)
            );

            // initiate a withdrawal from the appchain to an empty wallet
            let withdrawal_value = parse_ether("0.1")?;
            let to_address = address!("0x0000000000000000000000000000000000000001");
            let tx =
                init_withdrawal_tx(to_address, withdrawal_value, &components.appchain_provider)
                    .await?;

            let receipt = components.sequence_tx(tx.encoded_2718().as_slice(), 0, true).await?;
            let appchain_block_hash_to_prove = receipt.unwrap().block_hash.unwrap();

            //
            // now wait until the withdrawal root is posted

            // look for `AssertionConfirmed` event on the `RollupCore` contract
            // NOTE: `AssertionConfirmed` is only available on nito-contracts V3.... since this test
            // uses the legacy version we must check for `NodeConfirmed` event instead
            let rollup_core = IRollup::new(
                components.appchain_deployment.rollup,
                &components.settlement_provider,
            );

            wait_until!(
                rollup_core
                    .NodeConfirmed_filter()
                    .query()
                    .await?
                    .iter()
                    .any(|event| event.0.blockHash == appchain_block_hash_to_prove),
                Duration::from_secs(120)
            );

            // finish the withdrawal on the settlement chain
            execute_withdrawal(
                to_address,
                withdrawal_value,
                components.appchain_deployment.bridge,
                &components.settlement_provider,
                &components.appchain_provider,
            )
            .await?;

            // Assert new balance is equal to withdrawal amount
            let balance_after = components.settlement_provider.get_balance(to_address).await?;
            assert_eq!(balance_after, withdrawal_value);

            Ok(())
        },
    )
    .await
}

#[allow(clippy::unwrap_used)]
async fn deploy_on_settlement_chain<P, D>(
    components: &TestComponents,
    call_builder: CallBuilder<P, D, Ethereum>,
    signer: &PrivateKeySigner,
) -> Result<Address>
where
    P: Provider<Ethereum>,
    D: alloy::contract::CallDecoder,
{
    let chain_id = components.settlement_provider.get_chain_id().await?;
    let nonce = components.settlement_provider.get_transaction_count(signer.address()).await?;

    let receipt = call_builder
        .nonce(nonce)
        .chain_id(chain_id)
        .gas(100_000_000)
        .max_priority_fee_per_gas(0)
        .send()
        .await?
        .get_receipt()
        .await?;

    if !receipt.status() {
        println!(
            "Deployment failed. Receipt: hash={:?}, gas_used={:?}",
            receipt.transaction_hash, receipt.gas_used
        );

        let trace = components
            .settlement_provider
            .debug_trace_transaction(receipt.transaction_hash, GethDebugTracingOptions::default())
            .await
            .unwrap();

        // Extract just the error information from the trace
        match trace {
            GethTrace::Default(frame) => {
                if frame.failed {
                    println!("Transaction failed with return value: {}", frame.return_value);
                    // Look for error in struct logs
                    for log in &frame.struct_logs {
                        if let Some(error) = &log.error {
                            println!("EVM Error: {error}");
                        }
                    }
                } else {
                    println!("Transaction succeeded but receipt shows failure");
                }
            }
            GethTrace::CallTracer(frame) => {
                if let Some(error) = &frame.error {
                    println!("Call failed with error: {error}");
                }
                if let Some(revert_reason) = &frame.revert_reason {
                    println!("Revert reason: {revert_reason}");
                }
            }
            _ => {
                println!("Unexpected trace type for failed transaction");
            }
        }
    }
    assert!(receipt.status());
    Ok(receipt.contract_address.unwrap())
}

// MockZkVerifier is a mock implementation of the IAttestationDocVerifier interface for testing
sol! {
    #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506102578061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c8063c22a96941461002d575b5f5ffd5b610047600480360381019061004291906100e5565b61005d565b60405161005491906101a2565b60405180910390f35b5f5f858581019061006e91906101f6565b905080915050949350505050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126100a5576100a4610084565b5b8235905067ffffffffffffffff8111156100c2576100c1610088565b5b6020830191508360018202830111156100de576100dd61008c565b5b9250929050565b5f5f5f5f604085870312156100fd576100fc61007c565b5b5f85013567ffffffffffffffff81111561011a57610119610080565b5b61012687828801610090565b9450945050602085013567ffffffffffffffff81111561014957610148610080565b5b61015587828801610090565b925092505092959194509250565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61018c82610163565b9050919050565b61019c81610182565b82525050565b5f6020820190506101b55f830184610193565b92915050565b5f6101c582610163565b9050919050565b6101d5816101bb565b81146101df575f5ffd5b50565b5f813590506101f0816101cc565b92915050565b5f6020828403121561020b5761020a61007c565b5b5f610218848285016101e2565b9150509291505056fea2646970667358221220b1c06a0ba5fc1213a39916c7bdb363a0d04daeb334eda966e59fd82ee313512c64736f6c634300081e0033")]
    contract MockZkVerifier {
    function verifyAttestationDocProof(bytes calldata _publicValues, bytes calldata _proofBytes)
    external
    view
    returns (address){
      address addr = abi.decode(_publicValues, (address));
      return addr;
    }
  }
}

/// Configuration for the verifier
#[derive(Clone, Debug, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct VerifierConfig {
    /// Sequencing contract address on the sequencing chain
    pub sequencing_contract_address: Address,

    /// Sequencing bridge address on the l1 chain
    pub sequencing_bridge_address: Address,

    /// Settlement delay in seconds
    pub settlement_delay: u64,
}

impl VerifierConfig {
    /// Hash the verifier config
    #[allow(clippy::unwrap_used)]
    fn hash(&self) -> B256 {
        keccak256(
            (
                self.sequencing_contract_address,
                self.sequencing_bridge_address,
                self.settlement_delay,
            )
                .abi_encode_packed(),
        )
    }
}

// simple oracle implementation to push L1 data onto the settlementchain - it implements the
// IL1Block interface

sol! {
    #[sol(rpc,bytecode="6080604052348015600e575f5ffd5b506102298061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c806309bd5a60146100435780630abc584214610061578063b80777ea1461007d575b5f5ffd5b61004b61009b565b6040516100589190610109565b60405180910390f35b61007b6004803603810190610076919061018d565b6100a4565b005b6100856100d6565b60405161009291906101da565b60405180910390f35b5f600154905090565b815f5f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550806001819055505050565b5f5f5f9054906101000a900467ffffffffffffffff16905090565b5f819050919050565b610103816100f1565b82525050565b5f60208201905061011c5f8301846100fa565b92915050565b5f5ffd5b5f67ffffffffffffffff82169050919050565b61014281610126565b811461014c575f5ffd5b50565b5f8135905061015d81610139565b92915050565b61016c816100f1565b8114610176575f5ffd5b50565b5f8135905061018781610163565b92915050565b5f5f604083850312156101a3576101a2610122565b5b5f6101b08582860161014f565b92505060206101c185828601610179565b9150509250929050565b6101d481610126565b82525050565b5f6020820190506101ed5f8301846101cb565b9291505056fea26469706673582212207dfc106a5b282ab6edfdf15ae2ce7a3bd07aaf732188f1110f35115fbc35639e64736f6c634300081e0033")]
    contract L1BlockOracle {
        uint64 private _timestamp;
        bytes32 private _hash;

        // Single setter for both timestamp and hash
        function setL1Block(uint64 newTimestamp, bytes32 newHash) external {
            _timestamp = newTimestamp;
            _hash = newHash;
        }

        // Getter for timestamp (implements IL1Block)
        function timestamp() external view override returns (uint64) {
            return _timestamp;
        }

        // Getter for hash (implements IL1Block)
        function hash() external view override returns (bytes32) {
            return _hash;
        }
    }
}

#[allow(clippy::unwrap_used)]
async fn setup_l1_oracle<T: Provider<Ethereum> + Clone + Send + Sync + 'static>(
    l1_provider: T,
    target_chain_provider: T,
) -> (JoinHandle<()>, Address) {
    // NOTE: manually constructing the deployment tx shouldn't be necessary, likey an artifact of: https://github.com/alloy-rs/alloy/issues/2668
    // instead should just be:
    // let oracle_contract = L1BlockOracle::deploy(target_chain_provider).await.unwrap();
    let receipt = L1BlockOracle::deploy_builder(target_chain_provider.clone())
        .nonce(target_chain_provider.get_transaction_count(test_account2().address).await.unwrap())
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    let oracle_contract =
        L1BlockOracle::new(receipt.contract_address.unwrap(), target_chain_provider.clone());
    // ---
    let oracle_contract_address = *oracle_contract.address();
    let handle = tokio::spawn(async move {
        let mut l1_block_sub = l1_provider.subscribe_blocks().await.unwrap();
        loop {
            // push the l1 block to the oracle contract on every new L1 block
            let l1_block = l1_block_sub.recv().await.unwrap();
            println!("l1 block: {l1_block:?}");
            let l1_hash = l1_block.hash;
            let l1_timestamp = l1_block.timestamp;
            // NOTE: manually setting the nonce shouldn't be necessary, likey an artifact of: https://github.com/alloy-rs/alloy/issues/2668
            let receipt = oracle_contract
                .setL1Block(l1_timestamp, l1_hash)
                .nonce(
                    target_chain_provider
                        .get_transaction_count(test_account2().address)
                        .await
                        .unwrap(),
                )
                .send()
                .await
                .unwrap()
                .get_receipt()
                .await
                .unwrap();
            assert!(receipt.status());
        }
    });
    (handle, oracle_contract_address)
}
