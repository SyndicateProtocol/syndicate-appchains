// use block_builder::contract_bindings::counter::Counter;
// use e2e_tests::contract_bindings::metabasedsequencerchain::MetabasedSequencerChain;
// use e2e_tests::e2e_env::TestEnv;

#[tokio::test]
async fn test_e2e_env() -> eyre::Result<()> {
    // let env = TestEnv::new().await?;

    // let counter_deploy_calldata = Counter::deploy_builder(env.l3).calldata();
    // MetabasedSequencerChain::processTransactionRawCall(env.l3, counter_deploy_calldata).await?;
    // let sequencer_contract = MetabasedSequencerChain::new(env,env.sequencing_chain);
    // sequencer_contract
    //     .processTransactionRaw(counter_deploy_calldata)
    //     .await?;
    // TODO create tx to deploy counter to L3
    // TODO call env.SequenceTx to sequence it
    // TODO assert the contract is deployed
    // TODO try to increment the counter
    // TODO assert the counter is incremented
    Ok(())
}
