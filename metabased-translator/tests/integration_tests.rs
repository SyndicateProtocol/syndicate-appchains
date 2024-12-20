mod e2e_env;
use metabased_translator::contract_bindings::translator_contracts::counter::Counter;

#[tokio::test]
async fn test_e2e_env() -> eyre::Result<()> {
    let config = e2e_env::TestEnvConfig {
        rollup_type: e2e_env::RollupType::Optimism,
        settlement_rpc: None, // Will spawn Anvil
        sequencing_rpc: None, // Will spawn Anvil
    };

    let env = e2e_env::TestEnv::new(config).await?;

    let counter_deploy_calldata = Counter::deploy_builder(env.l3).calldata();
    // env.sequence_tx(tx).await?;
    // TODO create tx to deploy counter to L3
    // TODO call env.SequenceTx to sequence it
    // TODO assert the contract is deployed
    // TODO try to increment the counter
    // TODO assert the counter is incremented
    Ok(())
}
