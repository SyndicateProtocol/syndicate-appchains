use crate::e2e::e2e_tests::Storage;
use alloy::{
    primitives::{utils::parse_ether, U160, U256},
    providers::{ext::AnvilApi, Provider},
};
use contract_bindings::synd::{
    i_inbox::IInbox,
    i_sequencer_inbox::ISequencerInbox,
    syndicate_sequencing_chain::SyndicateSequencingChain::{
        self, SyndicateSequencingChainInstance,
    },
};
use eyre::Result;
use std::time::Duration;
use test_framework::components::test_components::{SEQUENCING_CHAIN_ID, SETTLEMENT_CHAIN_ID};
use test_utils::{
    anvil::start_anvil_with_args,
    chain_info::{test_account1, test_account8, test_account9, ChainInfo},
    docker::{launch_nitro_node, start_eigenda_proxy, NitroNodeArgs},
    nitro_chain::deploy_nitro_rollup,
    wait_until,
};

async fn start_base_chain(chain_id: u64) -> Result<ChainInfo> {
    let chain_info = start_anvil_with_args(chain_id, &[]).await?;
    chain_info.provider.anvil_set_auto_mine(true).await?;
    Ok(chain_info)
}

async fn deploy_sequencing_contract<P: Provider>(
    provider: P,
    appchain_chain_id: U256,
) -> Result<SyndicateSequencingChainInstance<P>> {
    let contract_instance = SyndicateSequencingChain::deploy(provider, appchain_chain_id).await?;
    assert!(contract_instance
        .updateRequirementModule(U160::from(1).into())
        .send()
        .await?
        .get_receipt()
        .await?
        .status());
    Ok(contract_instance)
}

#[tokio::test]
async fn e2e_migration() -> Result<()> {
    let appchain_chain_id = 15u64;
    let appchain_owner = test_account1();
    let batch_poster = test_account8();
    let test_user = test_account9();

    let set_chain = start_base_chain(SETTLEMENT_CHAIN_ID).await?;
    let seq_chain = start_base_chain(SEQUENCING_CHAIN_ID).await?;
    let sequencing_contract =
        deploy_sequencing_contract(seq_chain.provider.clone(), U256::from(appchain_chain_id))
            .await?;

    let appchain_deployment = deploy_nitro_rollup(
        &set_chain.http_url,
        appchain_chain_id,
        appchain_owner.address,
        vec![batch_poster.address],
        true,
    )
    .await?;

    let (_instance, eigenda_proxy_url) = start_eigenda_proxy().await?;

    let appchain = launch_nitro_node(NitroNodeArgs {
        chain_id: appchain_chain_id,
        chain_owner: appchain_owner.address,
        parent_chain_url: set_chain.ws_url.clone(),
        parent_chain_id: SETTLEMENT_CHAIN_ID,
        sequencer_mode: test_utils::docker::NitroSequencerMode::EigenDASequencer(
            eigenda_proxy_url.clone(),
        ),
        chain_name: "appchain".to_string(),
        deployment: appchain_deployment.clone(),
        sequencer_private_key: Some(batch_poster.private_key.to_string()),
    })
    .await?;

    // --

    // deposit some funds for the default signer
    let inbox = IInbox::new(appchain_deployment.inbox, &set_chain.provider);
    let _ = inbox.depositEth().value(parse_ether("10")?).send().await?;

    // wait until those funds arrive on the chain
    wait_until!(
        appchain.provider.get_balance(test_account1().address).await? >= parse_ether("10")?,
        Duration::from_secs(10)
    );

    let storage_contract = Storage::deploy(appchain.provider.clone(), U256::from(42)).await?;

    let arb_sequencer_inbox =
        ISequencerInbox::new(appchain_deployment.sequencer_inbox, set_chain.provider);

    wait_until!(arb_sequencer_inbox.batchCount().call().await? == 1, Duration::from_secs(10));

    //tear nitro down
    drop(appchain);

    Ok(())

    // setup a normal arb rollup on an anvil chain

    // send a few txs / deposits

    // wait for a batch to be posted

    // shutdown the nitro node

    // run the migration cli code to obtain migration data from the nitro node

    // migrate the bridge contract

    // spin up the syndicate stack

    // assert new txs work

    // assert `arbOwner.setL1PricePerUnit(0) ` works

    // assert new txs work after setPricePerUnit is called

    // assert sendL2MessageFromOrigin (WITHOUT THE custom event fork) works
    //
    // assert withdrawals work (TBD)

    // TODO InboxMessageDeliveredFromOrigin needs to be handled in the enclave too... test an
    // withdrawal triggered this way
}
