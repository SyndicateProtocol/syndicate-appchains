//! e2e tests for the `synd-withdrawals`
use alloy::{
    contract::CallBuilder,
    eips::{BlockNumberOrTag, Encodable2718},
    network::Ethereum,
    primitives::{address, keccak256, utils::parse_ether, Address, B256},
    providers::{ext::DebugApi, Provider},
    rpc::types::trace::geth::GethDebugTracingOptions,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
};
use contract_bindings::synd::{
    assertion_poster::AssertionPoster, i_inbox::IInbox, i_rollup::IRollup,
    tee_key_manager::TeeKeyManager, tee_module::TeeModule,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, time::Duration};
use test_framework::components::{
    configuration::{BaseChainsType, ConfigurationOptions},
    proposer::ProposerConfig,
    test_components::TestComponents,
};
use test_utils::{
    chain_info::test_account1,
    docker::{launch_enclave_server, start_component},
    nitro_chain::{execute_withdrawal, init_withdrawal_tx},
    port_manager::PortManager,
    wait_until,
};
use tokio::time;

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

#[tokio::test]
#[allow(clippy::unwrap_used)]
#[ignore] // TODO remove once the proposer is ready
async fn e2e_tee_withdrawal() -> Result<()> {
    let close_challenge_interval = Duration::from_secs(1);
    TestComponents::run(
        // NOTE: important to use base chains type == Nitro
        &ConfigurationOptions {
            base_chains_type: BaseChainsType::Nitro,
            rollup_owner: test_account1().address,
            settlement_delay: 2,
            close_challenge_interval,
            ..Default::default()
        },
        |components| async move {
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

            // let l1_start_block_hash = components
            //     .l1_provider
            //     .unwrap()
            //     .get_block_by_number(BlockNumberOrTag::Number(0))
            //     .await?
            //     .unwrap()
            //     .header
            //     .hash;

            let config_hash = VerifierConfig {
                sequencing_contract_address: *components.sequencing_contract.address(),
                sequencing_bridge_address: components
                    .sequencing_deployment
                    .as_ref()
                    .unwrap()
                    .bridge,
                settlement_delay: ConfigurationOptions::default().settlement_delay,
            }
            .hash();
            let tee_module_addr = deploy_on_settlement_chain(
                &components,
                TeeModule::deploy_builder(
                    &components.settlement_provider,
                    components.assertion_poster_address,
                    components.appchain_deployment.bridge,
                    config_hash,
                    appchain_start_block_hash,
                    seq_start_block_hash,
                    // TODO try to set this to not 0 (after the test passes with the proposer in
                    // place)
                    B256::ZERO,
                    Address::ZERO,
                    true,
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
            let tx_hash =
                assertion_poster.transferOwnership(tee_module_addr).send().await?.watch().await?;
            let receipt =
                components.settlement_provider.get_transaction_receipt(tx_hash).await?.unwrap();
            assert!(receipt.status());

            // start enclave and proposer, obtain the tee public key
            let (mut enclave_server_instance, enclave_rpc_url, tee_public_key) =
                launch_enclave_server().await?;

            let proposer_config = ProposerConfig {
                ethereum_rpc_url: components.l1_ws_rpc_url.clone(),
                tee_module_contract_address: tee_module_addr,
                arbitrum_bridge_address: components.appchain_deployment.bridge,
                inbox_address: components.appchain_deployment.inbox,
                sequencer_inbox_address: components.appchain_deployment.sequencer_inbox,
                settlement_rpc_url: components.settlement_rpc_url.clone(),
                metrics_port: PortManager::instance().next_port().await,
                appchain_rpc_url: components.appchain_ws_rpc_url.clone(),
                sequencing_rpc_url: components.sequencing_rpc_url.clone(),
                enclave_rpc_url,
                polling_interval: "1m".to_string(),
                close_challenge_interval: format!("{}s", close_challenge_interval.as_secs()),
                settlement_chain_id: 84532,
            };

            let mut proposer_instance = start_component(
                "synd-proposer",
                proposer_config.metrics_port,
                proposer_config.cli_args(),
                Default::default(),
            )
            .await?;

            // fail the tests if the services die
            tokio::spawn(async move {
                tokio::select! {
                    e = proposer_instance.wait() => panic!("synd-proposer died: {:#?}", e),
                    e = enclave_server_instance.wait() => panic!("enclave server died: {:#?}", e),
                }
            });

            // add the TEE signer address to the key manager using a mock proof
            let key_mgr = TeeKeyManager::new(key_mgr_addr, &components.settlement_provider);
            let public_values = tee_public_key.abi_encode();
            let proof_bytes = vec![];
            let tx_hash = key_mgr
                .addKey(public_values.into(), proof_bytes.into())
                .send()
                .await?
                .watch()
                .await?;
            let receipt =
                components.settlement_provider.get_transaction_receipt(tx_hash).await?.unwrap();
            assert!(receipt.status());

            // deposit funds to the appchain
            let inbox =
                IInbox::new(components.appchain_deployment.inbox, &components.settlement_provider);
            let tx_hash = inbox.depositEth().value(parse_ether("1")?).send().await?.watch().await?;
            let receipt =
                components.settlement_provider.get_transaction_receipt(tx_hash).await?.unwrap();
            assert!(receipt.status());

            // send a dummy tx so that the sequencing chain progresses and the deposit is
            // slotted in
            components.sequence_tx(b"dummy_tx", 0, false).await?;

            wait_until!(
                components.appchain_provider.get_balance(test_account1().address).await? >=
                    parse_ether("1")?,
                Duration::from_secs(20)
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

            // call `closeChallengeWindow` on the TEEmodule - to force the assertion to be posted
            time::sleep(Duration::from_secs(1)).await; // wait 1s (challenge window duration) to elapse
            let tee_module = TeeModule::new(tee_module_addr, &components.settlement_provider);
            let tx_hash = tee_module.closeChallengeWindow().send().await?.watch().await?;
            let receipt =
                components.settlement_provider.get_transaction_receipt(tx_hash).await?.unwrap();
            assert!(receipt.status());

            // look for `AssertionConfirmed` event on the `RollupCore` contract
            // NOTE: `AssertionConfirmed` is only available on nito-contracts V3.... since this test
            // uses the legacy version we must check for `NodeConfirmed` event instead
            let rollup_core = IRollup::new(
                components.appchain_deployment.rollup,
                &components.settlement_provider,
            );

            wait_until!(
                rollup_core
                    .AssertionConfirmed_filter()
                    .query()
                    .await?
                    .iter()
                    .any(|event| event.0.blockHash == appchain_block_hash_to_prove),
                Duration::from_secs(20)
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

    let tx_hash = call_builder
        .nonce(nonce)
        .chain_id(chain_id)
        .gas(100_000_000)
        .max_priority_fee_per_gas(0)
        .send()
        .await?
        .watch()
        .await?;

    let receipt = components.settlement_provider.get_transaction_receipt(tx_hash).await?.unwrap();
    if !receipt.status() {
        println!("receipt: {receipt:?}");
        let trace = components
            .settlement_provider
            .debug_trace_transaction(tx_hash, GethDebugTracingOptions::default())
            .await
            .unwrap();
        println!("trace: {trace:?}");
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
pub struct VerifierConfig {
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
    pub fn hash(&self) -> B256 {
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
