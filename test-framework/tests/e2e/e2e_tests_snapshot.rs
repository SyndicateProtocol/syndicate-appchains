//! e2e tests for m-chain snapshot functionality

use alloy::{
    primitives::utils::parse_ether,
    providers::{Provider, WalletProvider},
};
use contract_bindings::synd::i_inbox::IInbox;
use eyre::Result;
use std::{fs, time::Duration};
use synd_mchain::client::MchainProvider as _;
use test_framework::components::{
    configuration::{BaseChainsType, ConfigurationOptions},
    test_components::TestComponents,
};
use test_utils::{
    nitro_chain::ArbContractVersion,
    port_manager::PortManager,
    tar::{create_tar_gz, start_file_server},
    utils::test_path,
    wait_until,
};
use tokio::time::sleep;

#[ctor::ctor]
fn init() {
    shared::tracing::setup_global_logging();
}

#[tokio::test]
async fn e2e_snapshot_restore() -> Result<()> {
    TestComponents::run(
        &ConfigurationOptions {
            base_chains_type: BaseChainsType::PreLoaded(ArbContractVersion::V311),
            ..Default::default()
        },
        |components| async move {
            let wallet_address = components.settlement_provider.default_signer_address();

            let inbox =
                IInbox::new(components.appchain_deployment.inbox, &components.settlement_provider);
            let _ = inbox.depositEth().value(parse_ether("1")?).send().await?;

            components.mine_set_block(0).await?;
            components.mine_set_block(1).await?;

            wait_until!(
                components.appchain_provider.get_balance(wallet_address).await? ==
                    parse_ether("1")?,
                Duration::from_secs(10)
            );

            let mchain_block_before_snapshot = components.mchain_provider.get_block_number().await;
            assert!(mchain_block_before_snapshot > 1, "Should have processed some blocks");

            let mchain_datadir = std::path::PathBuf::from(&components.mchain_datadir);

            assert!(mchain_datadir.exists(), "Mchain datadir should exist at {mchain_datadir:?}");

            // Wait a bit to ensure RocksDB has flushed all data to disk
            sleep(Duration::from_secs(2)).await;

            let temp_dir = std::env::temp_dir();
            let test_id = format!("e2e_snapshot_{}", std::process::id());
            let snapshot_file = temp_dir.join(format!("{test_id}_snapshot.tar.gz"));

            create_tar_gz(&mchain_datadir, &snapshot_file)?;
            let snapshot_size = fs::metadata(&snapshot_file)?.len();

            assert!(snapshot_size > 1000, "Snapshot seems too small (only {snapshot_size} bytes), likely didn't capture RocksDB files");

            let port = PortManager::instance().next_port().await;
            let _server_handle = start_file_server(&snapshot_file, port).await?;

            let snapshot_url = format!("http://127.0.0.1:{port}/snapshot.tar.gz");

            // Now we need to test restarting mchain with the snapshot
            // Since TestComponents manages the lifecycle, we'll create a new mchain instance
            // with the snapshot URL in a separate test path
            let restore_datadir = test_path("synd-mchain-restore", None);
            let _ = fs::remove_dir_all(&restore_datadir);

            let restore_port = PortManager::instance().next_port().await;
            let restore_metric_port = PortManager::instance().next_port().await;

            let args = vec![
                "--appchain-chain-id".to_string(),
                components.appchain_chain_id.to_string(),
                "--port".to_string(),
                restore_port.to_string(),
                "--metrics-port".to_string(),
                restore_metric_port.to_string(),
                "--finality-delay".to_string(),
                "60".to_string(),
                "--snapshot-url".to_string(),
                snapshot_url.clone(),
                "--datadir".to_string(),
                restore_datadir.to_string_lossy().to_string(),
            ];

            let _restore_mchain_docker =
                test_utils::docker::start_component("synd-mchain", restore_port, args, vec![])
                    .await?;

            let restore_mchain_url = format!("ws://localhost:{restore_port}");
            let restore_mchain_provider =
                synd_mchain::client::MProvider::new(&restore_mchain_url).await?;

            wait_until!(
                restore_mchain_provider.get_block_number().await > 0,
                Duration::from_secs(20)
            );

            let restored_block_number = restore_mchain_provider.get_block_number().await;
            assert!(
                restored_block_number == mchain_block_before_snapshot,
                "Restored mchain should have the same block number as the snapshot. Got {restored_block_number}, expected {mchain_block_before_snapshot }"
            );


            let current_block_number = restore_mchain_provider.get_block_number().await;
            assert!(
                current_block_number >= mchain_block_before_snapshot,
                "Should be able to query restored state"
            );
            Ok(())
        },
    )
    .await
}
