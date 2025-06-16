//! e2e tests for the `synd-withdrawals`
use alloy::{
    contract::CallBuilder,
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::{address, utils::parse_ether, Address, B256},
    providers::{ext::DebugApi, Provider},
    rpc::types::trace::geth::GethDebugTracingOptions,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
};
use contract_bindings::synd::{
    assertionposter::AssertionPoster, teekeymanager::TeeKeyManager, teemodule::TeeModule,
};
use eyre::Result;
use synd_appchain_verifier::config::AppchainVerifierConfig;
use synd_seqchain_verifier::config::SeqchainVerifierConfig;
use test_framework::components::{
    configuration::{BaseChainsType, ConfigurationOptions},
    test_components::TestComponents,
};
use test_utils::{
    anvil::start_anvil,
    chain_info::default_signer,
    nitro_chain::{deploy_nitro_rollup, execute_withdrawal, init_withdrawal_tx},
};

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

// MockZkVerifier is a mock implementation of the IAttestationDocVerifier interface for testing
sol! {
    #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506102578061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c8063c22a96941461002d575b5f5ffd5b610047600480360381019061004291906100e5565b61005d565b60405161005491906101a2565b60405180910390f35b5f5f858581019061006e91906101f6565b905080915050949350505050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f8401126100a5576100a4610084565b5b8235905067ffffffffffffffff8111156100c2576100c1610088565b5b6020830191508360018202830111156100de576100dd61008c565b5b9250929050565b5f5f5f5f604085870312156100fd576100fc61007c565b5b5f85013567ffffffffffffffff81111561011a57610119610080565b5b61012687828801610090565b9450945050602085013567ffffffffffffffff81111561014957610148610080565b5b61015587828801610090565b925092505092959194509250565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61018c82610163565b9050919050565b61019c81610182565b82525050565b5f6020820190506101b55f830184610193565b92915050565b5f6101c582610163565b9050919050565b6101d5816101bb565b81146101df575f5ffd5b50565b5f813590506101f0816101cc565b92915050565b5f6020828403121561020b5761020a61007c565b5b5f610218848285016101e2565b9150509291505056fea2646970667358221220b1c06a0ba5fc1213a39916c7bdb363a0d04daeb334eda966e59fd82ee313512c64736f6c634300081e0033")] // TODO
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

#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn e2e_tee_withdrawal() -> Result<()> {
    TestComponents::run(
        // NOTE: important to use base chains type == Nitro
        &ConfigurationOptions { base_chains_type: BaseChainsType::Nitro, ..Default::default() },
        |components| async move {
            // deploy contracts:
            // - Tee module
            // - key manager
            // - mock zk-verifier
            // - set the TEE module as the owner for the assertion poster contract (call
            //   `transferOwnership`)

            let signer = default_signer();

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

            let appchain_config = AppchainVerifierConfig {
                sequencing_contract_address: *components.sequencing_contract.address(),
                settlement_delay: ConfigurationOptions::default().settlement_delay,
            };

            let seq_config =
                SeqchainVerifierConfig { arbitrum_bridge_address: components.bridge_address };

            let tee_module_addr = deploy_on_settlement_chain(
                &components,
                TeeModule::deploy_builder(
                    &components.settlement_provider,
                    components.assertion_poster_address,
                    components.bridge_address,
                    appchain_config.hash_verifier_config_sha256(),
                    appchain_start_block_hash,
                    seq_config.hash_verifier_config_sha256(),
                    seq_start_block_hash,
                    // TODO try to set this to not 0
                    B256::ZERO,
                    Address::ZERO,
                    60 * 60, // challenge_window_duration - 1 hour
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

            // add the TEE signer address to the key manager using a mock proof
            let key_mgr = TeeKeyManager::new(key_mgr_addr, &components.settlement_provider);
            let public_values = components.tee_signer_address.abi_encode();
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

            // initiate withdrawal from the appchain
            let withdrawal_value = parse_ether("0.1")?;
            let to_address = address!("0x0000000000000000000000000000000000000001");
            let tx =
                init_withdrawal_tx(to_address, withdrawal_value, &components.appchain_provider)
                    .await?;

            let tx_hash =
                components.appchain_provider.send_transaction(tx.into()).await?.watch().await?;
            let receipt =
                components.appchain_provider.get_transaction_receipt(tx_hash).await?.unwrap();
            assert!(receipt.status());

            // wait until the withdrawal root is posted
            // TODO look for `AssertionConfirmed` event on the `RollupCore` contract
            // TODO call `closeChallengeWindow` on the TEEmodule - to force the assertion to be
            // posted

            // finish the withdrawal on the settlement chain
            execute_withdrawal(
                to_address,
                withdrawal_value,
                components.bridge_address,
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
async fn deploy_on_settlement_chain<T, P, D>(
    components: &TestComponents,
    call_builder: CallBuilder<T, P, D, Ethereum>,
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
        .max_fee_per_gas(100_000_000)
        .max_priority_fee_per_gas(0)
        .send()
        .await?
        .watch()
        .await?;

    let receipt = components.settlement_provider.get_transaction_receipt(tx_hash).await?.unwrap();
    if !receipt.status() {
        println!("receipt: {:?}", receipt);
        let trace = components
            .settlement_provider
            .debug_trace_transaction(tx_hash, GethDebugTracingOptions::default())
            .await
            .unwrap();
        println!("trace: {:?}", trace);
    }
    assert!(receipt.status());
    Ok(receipt.contract_address.unwrap())
}
