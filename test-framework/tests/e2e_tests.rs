//! Integration tests for the metabased stack

use alloy::{
    primitives::{address, Address, U256},
    rpc::types::Block,
};
use contract_bindings::arbitrum::{arbgasinfo::ArbGasInfo, arbownerpublic::ArbOwnerPublic};
use eyre::Result;
use test_framework::components::{Components, ContractVersion, ImageTags, APPCHAIN_OWNER};
use tracing::info;

#[tokio::test(flavor = "multi_thread")]
async fn arb_owner_test() -> Result<()> {
    const ARB_OWNER_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006b");

    // Start the meta node
    let components = Components::new(None).await?;
    let arb_owner_public =
        ArbOwnerPublic::new(ARB_OWNER_CONTRACT_ADDRESS, &components.appchain_provider);
    assert_eq!(arb_owner_public.getAllChainOwners().call().await?._0, [APPCHAIN_OWNER]);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn no_l1_fees_test() -> Result<()> {
    const ARB_GAS_INFO_CONTRACT_ADDRESS: Address =
        address!("0x000000000000000000000000000000000000006c");
    let components = Components::new(None).await?;
    let arb_gas_info =
        ArbGasInfo::new(ARB_GAS_INFO_CONTRACT_ADDRESS, &components.appchain_provider);
    assert_eq!(arb_gas_info.getL1BaseFeeEstimate().call().await?._0, U256::ZERO);

    Ok(())
}
