//! Components for the integration tests

use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    eips::{BlockNumberOrTag, Encodable2718},
    node_bindings::AnvilInstance,
    primitives::{Address, U256},
    providers::{ext::AnvilApi as _, Provider as _, RootProvider, WalletProvider},
    rpc::types::anvil::MineOptions,
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule, metabasedsequencerchain::MetabasedSequencerChain,
    },
};
use eyre::Result;
use mchain::mchain::{rollup_config, MProvider};
use reqwest::Client;
use std::{
    env,
    io::{stderr, Write},
    time::{Duration, SystemTime},
};
use test_utils::{
    anvil::{mine_block, start_anvil, start_anvil_with_args, FilledProvider},
    docker::{launch_nitro_node, start_component, start_mchain, Docker},
    port_manager::PortManager,
    preloaded_config::{
        PRELOAD_BRIDGE_ADDRESS_231, PRELOAD_BRIDGE_ADDRESS_300, PRELOAD_INBOX_ADDRESS_231,
        PRELOAD_INBOX_ADDRESS_300, PRELOAD_POSTER_ADDRESS_231, PRELOAD_POSTER_ADDRESS_300,
    },
};
use tokio::time::timeout;
use tracing::info;

// needs to be different from the regular private key to prevent nonce collisions
// needs to match the owner of the poster contract
// anvil account 0
const POSTER_SEQUENCER_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Components {
    /// Timer for latency measurement
    /// Keep this on top - the top element gets destroyed first
    _timer: TestTimer,

    /// Sequencing
    pub sequencing_provider: FilledProvider,

    /// Settlement
    pub settlement_provider: FilledProvider,
    pub chain_id: u64,
    pub bridge_address: Address,
    pub inbox_address: Address,

    /// Appchain
    pub appchain_provider: RootProvider,

    /// Mchain
    pub mchain_provider: MProvider,

    /// References to keep the processes/tasks alive
    _mchain: Docker,
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    _nitro_docker: Docker,
    _translator: Docker,
    _poster: Option<Docker>,
    _sequencer: Docker,
}

#[derive(Debug)]
struct TestTimer(SystemTime, Duration);

impl Drop for TestTimer {
    fn drop(&mut self) {
        let thread = std::thread::current();
        let elapsed = format!(
            "---- SYN ---- Test {:?} took: {:.2?} (setup: {:.2?})\n",
            thread.name().unwrap_or_default(),
            self.0.elapsed().unwrap_or_default(),
            self.1,
        );
        // Write directly to stderr (bypasses test harness output capture)
        let _ = stderr().write_all(elapsed.as_bytes());
    }
}

/// nitro contract version on the settlement chain used for testing
#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum ContractVersion {
    V213,
    V300,
}

#[derive(Debug)]
struct TranslatorConfig {
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_contract_address: Address,
    mchain_rpc_url: String,
    sequencing_rpc_url: String,
    settlement_rpc_url: String,
    metrics_port: u16,
    sequencing_start_block: u64,
    settlement_start_block: u64,
    settlement_delay: u64,
}

#[derive(Debug)]
struct PosterConfig {
    assertion_poster_contract_address: Address,
    settlement_rpc_url: String,
    appchain_rpc_url: String,
    metrics_port: u16,
}

#[derive(Debug)]
struct SequencerConfig {
    sequencing_contract_address: Address,
    sequencing_rpc_url: String,
    sequencer_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct ConfigurationOptions {
    pub pre_loaded: Option<ContractVersion>,
    pub sequencing_start_block: u64,
    pub settlement_start_block: u64,
    pub settlement_delay: u64,
    pub appchain_owner: Address,
    pub appchain_chain_id: u64,
    pub finality_delay: u64,
}

impl Default for ConfigurationOptions {
    fn default() -> Self {
        Self {
            pre_loaded: None,
            sequencing_start_block: 1, // skip the genesis block
            settlement_start_block: 1, // skip the genesis block
            settlement_delay: 0,
            appchain_owner: Address::ZERO,
            appchain_chain_id: 13331370,
            finality_delay: 60,
        }
    }
}

impl Components {
    pub(crate) async fn new(options: &ConfigurationOptions) -> Result<Self> {
        let start_time = SystemTime::now();

        info!("Starting components...");
        info!("Starting mchain...");
        let (mchain_rpc_url, _mchain, mchain_provider) =
            start_mchain(options.appchain_chain_id, options.appchain_owner, options.finality_delay)
                .await?;

        // Launch mock sequencing chain and deploy contracts
        info!("Starting sequencing chain...");
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(
            &seq_provider,
            U256::from(options.appchain_chain_id),
        )
        .send()
        .await?;
        let sequencing_contract_address = seq_provider.default_signer_address().create(0);
        _ = AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
        let always_allowed_module_address = seq_provider.default_signer_address().create(1);

        // Setup the sequencing contract
        let provider_clone = seq_provider.clone();
        let sequencing_contract =
            MetabasedSequencerChain::new(sequencing_contract_address, provider_clone);
        _ = sequencing_contract
            .initialize(seq_provider.default_signer_address(), always_allowed_module_address)
            .send()
            .await?;
        mine_block(&seq_provider, 0).await?;

        // Launch mock settlement chain
        info!("Starting settlement chain...");
        let (set_port, set_anvil, set_provider);
        if let Some(version) = &options.pre_loaded {
            let file = match version {
                ContractVersion::V300 => "anvil_300.json",
                ContractVersion::V213 => "anvil_213.json",
            };

            // If flag is set, load the anvil state from a file
            // This is the full set of Arb contracts
            let state_file =
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config").join(file);

            (set_port, set_anvil, set_provider) = start_anvil_with_args(
                31337,
                #[allow(clippy::unwrap_used)]
                &["--load-state", state_file.to_str().unwrap()],
            )
            .await?;

            // Sync the tips of the sequencing and settlement chains
            #[allow(clippy::unwrap_used)]
            let block = set_provider.get_block_by_number(BlockNumberOrTag::Latest).await?.unwrap();
            seq_provider
                .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
                .await?;
        } else {
            // If not use our mock Rollup contract for easier testing
            (set_port, set_anvil, set_provider) = start_anvil(20).await?;
            // Use the mock rollup contract for the test instead of deploying all the nitro rollup
            // contracts
            _ = Rollup::deploy_builder(
                &set_provider,
                U256::from(options.appchain_chain_id),
                rollup_config(options.appchain_chain_id, options.appchain_owner),
            )
            .nonce(0)
            .send()
            .await?;

            mine_block(&set_provider, 0).await?;
        }
        let sequencing_rpc_url = format!("http://localhost:{}", seq_port);
        let settlement_rpc_url = format!("http://localhost:{}", set_port);

        // Launch components
        info!("Starting sequencer...");
        let sequencer_config = SequencerConfig {
            sequencing_contract_address,
            sequencing_rpc_url: sequencing_rpc_url.clone(),
            sequencer_port: PortManager::instance().next_port(),
            metrics_port: PortManager::instance().next_port(),
        };
        let sequencer =
            start_component("metabased-sequencer", sequencer_config.cli_args(), Default::default())
                .await?;
        wait_for_service(sequencer_config.metrics_port).await?;

        info!("Starting translator...");
        let translator_config = TranslatorConfig {
            sequencing_start_block: options.sequencing_start_block,
            settlement_start_block: options.settlement_start_block,
            settlement_delay: options.settlement_delay,
            arbitrum_bridge_address: options.pre_loaded.as_ref().map_or_else(
                || set_provider.default_signer_address().create(0),
                |version| match version {
                    ContractVersion::V300 => PRELOAD_BRIDGE_ADDRESS_300,
                    ContractVersion::V213 => PRELOAD_BRIDGE_ADDRESS_231,
                },
            ),
            arbitrum_inbox_address: options.pre_loaded.as_ref().map_or_else(
                || set_provider.default_signer_address().create(0),
                |version| match version {
                    ContractVersion::V300 => PRELOAD_INBOX_ADDRESS_300,
                    ContractVersion::V213 => PRELOAD_INBOX_ADDRESS_231,
                },
            ),
            sequencing_contract_address,
            mchain_rpc_url: mchain_rpc_url.clone(),
            metrics_port: PortManager::instance().next_port(),
            sequencing_rpc_url,
            settlement_rpc_url: settlement_rpc_url.clone(),
        };
        let translator = start_component(
            "metabased-translator",
            translator_config.cli_args(),
            Default::default(),
        )
        .await?;
        wait_for_service(translator_config.metrics_port).await?;

        // Launch the nitro rollup
        info!("Starting nitro node...");
        let (nitro_docker, appchain_provider, nitro_url) = launch_nitro_node(
            options.appchain_chain_id,
            options.appchain_owner,
            &mchain_rpc_url,
            Some(sequencer_config.sequencer_port),
        )
        .await?;

        let mut poster = None;
        if options.pre_loaded.is_some() {
            info!("Starting poster...");
            let poster_config = PosterConfig {
                assertion_poster_contract_address: options.pre_loaded.as_ref().map_or(
                    Address::ZERO,
                    |version| match version {
                        ContractVersion::V300 => PRELOAD_POSTER_ADDRESS_300,
                        ContractVersion::V213 => PRELOAD_POSTER_ADDRESS_231,
                    },
                ),
                settlement_rpc_url: settlement_rpc_url.clone(),
                metrics_port: PortManager::instance().next_port(),
                appchain_rpc_url: nitro_url,
            };
            poster = Some(
                start_component("metabased-poster", poster_config.cli_args(), Default::default())
                    .await?,
            );
            wait_for_service(poster_config.metrics_port).await?;
        }

        // Launch sequencer
        Ok(Self {
            #[allow(clippy::unwrap_used)]
            _timer: TestTimer(SystemTime::now(), start_time.elapsed().unwrap()),

            sequencing_provider: seq_provider,
            settlement_provider: set_provider,
            appchain_provider,
            chain_id: options.appchain_chain_id,
            bridge_address: translator_config.arbitrum_bridge_address,
            inbox_address: translator_config.arbitrum_inbox_address,
            mchain_provider,

            _mchain,
            _seq_anvil: seq_anvil,
            _set_anvil: set_anvil,
            _nitro_docker: nitro_docker,
            _translator: translator,
            _poster: poster,
            _sequencer: sequencer,
        })
    }

    pub(crate) async fn mine_seq_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.sequencing_provider, delay).await?;
        Ok(())
    }

    pub(crate) async fn send_tx_and_mine_block(
        &self,
        tx: &EthereumTxEnvelope<TxEip4844Variant>,
        delay: u64,
    ) -> Result<()> {
        self.sequencing_provider.anvil_set_block_timestamp_interval(delay).await?;
        self.sequencing_provider.anvil_set_auto_mine(true).await?;
        _ = self.appchain_provider.send_raw_transaction(&tx.encoded_2718()).await?;
        self.sequencing_provider.anvil_set_auto_mine(false).await?;
        self.sequencing_provider.anvil_remove_block_timestamp_interval().await?;
        Ok(())
    }

    pub(crate) async fn mine_set_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.settlement_provider, delay).await?;
        Ok(())
    }

    pub(crate) async fn mine_both(&self, delay: u64) -> Result<()> {
        self.mine_seq_block(delay).await?;
        self.mine_set_block(delay).await?;
        Ok(())
    }
}

async fn wait_for_service(port: u16) -> Result<()> {
    let client = Client::new();
    timeout(Duration::from_secs(120), async {
        while !client
            .get(format!("http://localhost:{port}/metrics"))
            .send()
            .await
            .is_ok_and(|x| x.status().is_success())
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok(())
    })
    .await?
}

impl TranslatorConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--mchain-rpc-url".to_string(),
            self.mchain_rpc_url.to_string(),
            "--sequencing-contract-address".to_string(),
            self.sequencing_contract_address.to_string(),
            "--arbitrum-bridge-address".to_string(),
            self.arbitrum_bridge_address.to_string(),
            "--arbitrum-inbox-address".to_string(),
            self.arbitrum_inbox_address.to_string(),
            "--sequencing-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--sequencing-start-block".to_string(),
            self.sequencing_start_block.to_string(),
            "--settlement-start-block".to_string(),
            self.settlement_start_block.to_string(),
            "--settlement-delay".to_string(),
            self.settlement_delay.to_string(),
            "--sequencing-polling-interval".to_string(),
            "50ms".to_string(),
            "--settlement-polling-interval".to_string(),
            "50ms".to_string(),
        ]
    }
}

impl PosterConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            POSTER_SEQUENCER_PRIVATE_KEY.to_string(),
            "--appchain-rpc-url".to_string(),
            self.appchain_rpc_url.to_string(),
            "--assertion-poster-contract-address".to_string(),
            self.assertion_poster_contract_address.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--polling-interval".to_string(),
            "1s".to_string(),
        ]
    }
}

impl SequencerConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            POSTER_SEQUENCER_PRIVATE_KEY.to_string(),
            "--chain-contract-address".to_string(),
            self.sequencing_contract_address.to_string(),
            "--chain-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--port".to_string(),
            self.sequencer_port.to_string(),
        ]
    }
}
