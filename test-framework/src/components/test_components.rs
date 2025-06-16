//! Components for the integration tests
#![allow(missing_docs)]

use crate::components::{
    batch_sequencer::BatchSequencerConfig,
    chain_ingestor::ChainIngestorConfig,
    configuration::{setup_config_manager, BaseChainsType, ConfigurationOptions},
    maestro::MaestroConfig,
    proposer::ProposerConfig,
    timer::TestTimer,
    translator::TranslatorConfig,
};
use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    eips::{BlockNumberOrTag, Encodable2718},
    primitives::{address, hex, keccak256, utils::parse_ether, Address, Bytes, TxHash, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::{anvil::MineOptions, TransactionReceipt},
};
use contract_bindings::synd::{
    alwaysallowedmodule::AlwaysAllowedModule,
    assertionposter::AssertionPoster,
    iinbox::IInbox,
    rollup::Rollup,
    syndicatesequencingchain::SyndicateSequencingChain::{self, SyndicateSequencingChainInstance},
};
use eyre::Result;
use serde_json::{json, Value};
use shared::{
    parse::parse_address,
    types::{deserialize_address, FilledProvider},
};
use std::{
    env,
    future::Future,
    str::FromStr,
    time::{Duration, SystemTime},
};
use synd_maestro::server::HEADER_CHAIN_ID;
use synd_mchain::{
    client::MProvider,
    methods::common::{APPCHAIN_CONTRACT, MCHAIN_ID},
};
use test_utils::{
    anvil::{mine_block, start_anvil, start_anvil_with_args},
    chain_info::{default_signer, ChainInfo, ProcessInstance, PRIVATE_KEY},
    docker::{
        launch_enclave_server, launch_nitro_node, start_component, start_mchain, start_valkey,
        E2EProcess, NitroNodeArgs, NitroSequencerMode,
    },
    nitro_chain::{chain_config, deploy_nitro_rollup, NitroDeployment},
    port_manager::PortManager,
    preloaded_config::{
        get_anvil_file, get_assertion_poster_address, get_bridge_address, get_inbox_address,
    },
    utils::test_path,
    wait_until,
};
use tokio::{fs, process::Command, time::sleep};
use tracing::info;

#[derive(Debug)]
struct ComponentHandles {
    l1_chain: Option<ProcessInstance>,
    seq_chain: ProcessInstance,
    set_chain: ProcessInstance,
    appchain_chain: ProcessInstance,
    mchain: E2EProcess,
    translator: E2EProcess,
    proposer: Option<E2EProcess>,
    enclave_server: Option<E2EProcess>,
    sequencing_chain_ingestor: E2EProcess,
    settlement_chain_ingestor: E2EProcess,

    // Write loop
    batch_sequencer: Option<E2EProcess>,
    valkey: Option<E2EProcess>,
    maestro: Option<E2EProcess>,
}

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub struct TestComponents {
    /// Timer for latency measurement
    /// Keep this on top - the top element gets destroyed first
    _timer: TestTimer,

    pub l1_provider: Option<FilledProvider>,

    /// Sequencing
    pub sequencing_provider: FilledProvider,
    pub sequencing_rpc_url: String,
    pub sequencing_contract: SyndicateSequencingChainInstance<(), FilledProvider>,

    /// Settlement
    pub settlement_provider: FilledProvider,
    pub settlement_rpc_url: String,
    pub bridge_address: Address,
    pub inbox_address: Address,
    pub assertion_poster_address: Address,

    /// Appchain
    pub appchain_provider: FilledProvider,
    pub appchain_chain_id: u64,

    /// Mchain
    pub mchain_provider: MProvider,
    pub proposer_url: String,

    pub maestro_url: String,
    pub valkey_url: String,

    #[allow(dead_code)]
    pub appchain_block_explorer_url: String,

    pub tee_signer_address: Address,
}

impl TestComponents {
    #[allow(clippy::unwrap_used)]
    pub async fn run<Fut: Future<Output = Result<()>> + Send>(
        options: &ConfigurationOptions,
        test: impl FnOnce(Self) -> Fut + Send,
    ) -> Result<()> {
        let (components, mut handles) = Self::new(options).await?;
        let proposer = handles.proposer.take();
        let enclave_server = handles.enclave_server.take();
        let maestro = handles.maestro.take();
        let batch_sequencer = handles.batch_sequencer.take();
        let valkey = handles.valkey.take();
        tokio::select! {
            biased;
            e = handles.sequencing_chain_ingestor.wait() => panic!("sequencing ingestor died: {:#?}", e),
            e = handles.settlement_chain_ingestor.wait() => panic!("settlement ingestor died: {:#?}", e),
            e = handles.mchain.wait() => panic!("synd-mchain died: {:#?}", e),
            e = async {handles.l1_chain.as_mut().unwrap().wait().await}, if handles.l1_chain.is_some() => panic!("l1 chain died: {:#?}", e),
            e = handles.seq_chain.wait() => panic!("sequencing chain died: {:#?}", e),
            e = handles.set_chain.wait() => panic!("settlement chain died: {:#?}", e),
            e = handles.appchain_chain.wait() => panic!("nitro died: {:#?}", e),
            e = handles.translator.wait() => panic!("synd-translator died: {:#?}", e),
            e = async {proposer.unwrap().wait().await}, if proposer.is_some() => panic!("synd-proposer died: {:#?}", e),
            e = async {enclave_server.unwrap().wait().await}, if enclave_server.is_some() => panic!("enclave server died: {:#?}", e),
            e = async {maestro.unwrap().wait().await}, if maestro.is_some() => panic!("synd-maestro died: {:#?}", e),
            e = async {batch_sequencer.unwrap().wait().await}, if batch_sequencer.is_some() => panic!("synd-batch-sequencer died: {:#?}", e),
            e = async {valkey.unwrap().wait().await}, if valkey.is_some() => panic!("valkey died: {:#?}", e),
            r = test(components) => r
        }
    }

    #[allow(clippy::unwrap_used)]
    async fn new(options: &ConfigurationOptions) -> Result<(Self, ComponentHandles)> {
        let mut options = options.clone();
        let start_time = SystemTime::now();

        let l1_info = match options.base_chains_type {
            BaseChainsType::Anvil | BaseChainsType::PreLoaded(_) => None,
            BaseChainsType::Nitro => {
                let info = start_anvil(11111).await?;
                // avoid "latest L1 block is old" error log from nitro
                let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
                info.provider.evm_mine(Some(MineOptions::Timestamp(Some(now)))).await?;
                info.provider.anvil_set_auto_mine(true).await?; //auto-mine enabled
                Some(info)
            }
        };

        // Launch mock sequencing chain and deploy contracts
        info!("Starting sequencing chain...");
        let ChainInfo {
            ws_url: seq_rpc_url,
            instance: seq_instance,
            provider: seq_provider,
            http_url: _,
        } = match options.base_chains_type {
            BaseChainsType::Anvil | BaseChainsType::PreLoaded(_) => start_anvil(15).await?,
            BaseChainsType::Nitro => {
                let chain_id = 15;
                let l1_info = l1_info.as_ref().unwrap();

                let deployment = deploy_nitro_rollup(&l1_info.http_url, chain_id).await?;

                // TODO remove this

                println!("DEBUG INFO");
                println!("parent chain id: {}", l1_info.provider.get_chain_id().await?);
                println!("deployment: {:?}", deployment);
                println!(
                    "rollup code: {:?}",
                    l1_info.provider.get_code_at(deployment.rollup).await?
                );
                println!("inbox code: {:?}", l1_info.provider.get_code_at(deployment.inbox).await?);
                println!(
                    "bridge code: {:?}",
                    l1_info.provider.get_code_at(deployment.bridge).await?
                );
                println!(
                    "sequencer inbox code: {:?}",
                    l1_info.provider.get_code_at(deployment.sequencer_inbox).await?
                );
                println!(
                    "native token code: {:?}",
                    l1_info.provider.get_code_at(deployment.native_token).await?
                );
                println!(
                    "upgrade executor code: {:?}",
                    l1_info.provider.get_code_at(deployment.upgrade_executor).await?
                );
                println!(
                    "validator utils code: {:?}",
                    l1_info.provider.get_code_at(deployment.validator_utils).await?
                );
                println!(
                    "validator wallet creator code: {:?}",
                    l1_info.provider.get_code_at(deployment.validator_wallet_creator).await?
                );
                println!("deployed at: {}", deployment.deployed_at);
                println!("--------------------------------");
                // ------------------------------------------------------------

                info!("Starting sequencing chain's nitro node...");
                let chain_info = launch_nitro_node(NitroNodeArgs {
                    chain_id,
                    chain_owner: default_signer().address(),
                    parent_chain_url: l1_info.ws_url.clone(),
                    parent_chain_id: l1_info.provider.get_chain_id().await?,
                    sequencer_mode: NitroSequencerMode::Sequencer,
                    chain_name: "sequencing".to_string(),
                    deployment: deployment.clone(),
                })
                .await?;

                // deposit some funds for the default signer
                let inbox = IInbox::new(deployment.inbox, &l1_info.provider);
                let _ = inbox.depositEth().value(parse_ether("10")?).send().await?;

                // wait until those funds arrive on the sequencing chain
                wait_until!(
                    chain_info.provider.get_balance(default_signer().address()).await? ==
                        parse_ether("10")?,
                    Duration::from_secs(10)
                );

                chain_info
            }
        };

        info!("Sequencing chain Nitro URL: {}", seq_rpc_url);

        let _ = SyndicateSequencingChain::deploy_builder(
            &seq_provider,
            U256::from(options.appchain_chain_id),
        )
        .send()
        .await?;
        let sequencing_contract_address = seq_provider.default_signer_address().create(0);
        let _ = AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
        let always_allowed_module_address = seq_provider.default_signer_address().create(1);

        // Setup the sequencing contract
        let provider_clone = seq_provider.clone();
        let sequencing_contract =
            SyndicateSequencingChain::new(sequencing_contract_address, provider_clone);
        let _ = sequencing_contract
            .initialize(seq_provider.default_signer_address(), always_allowed_module_address)
            .send()
            .await?;

        mine_block(&seq_provider, 0).await?;

        // Launch mock settlement chain
        info!("Starting settlement chain...");
        let ChainInfo {
            ws_url: set_rpc_url,
            instance: set_instance,
            provider: set_provider,
            http_url: _,
        } = match options.base_chains_type {
            BaseChainsType::Anvil => {
                let chain_info = start_anvil(20).await?;
                // Use the mock rollup contract for the test instead of deploying all the nitro
                // rollup contracts
                let _ = Rollup::deploy_builder(
                    &chain_info.provider,
                    U256::from(options.appchain_chain_id),
                    "null".to_string(),
                )
                .nonce(0)
                .send()
                .await?;

                mine_block(&chain_info.provider, 0).await?;
                chain_info
            }
            BaseChainsType::Nitro => {
                let chain_id = 20;
                let l1_info = l1_info.as_ref().unwrap();

                let deployment = deploy_nitro_rollup(&l1_info.http_url, chain_id).await?;

                info!("Starting settlement chain's nitro node...");
                let chain_info = launch_nitro_node(NitroNodeArgs {
                    chain_id,
                    chain_owner: default_signer().address(),
                    parent_chain_url: l1_info.ws_url.clone(),
                    parent_chain_id: l1_info.provider.get_chain_id().await?,
                    sequencer_mode: NitroSequencerMode::Sequencer,
                    chain_name: "settlement".to_string(),
                    deployment: deployment.clone(),
                })
                .await?;

                // deposit some funds for the default signer
                let inbox = IInbox::new(deployment.inbox, &l1_info.provider);
                let _ = inbox.depositEth().value(parse_ether("10")?).send().await?;

                // wait until those funds arrive on the sequencing chain
                wait_until!(
                    chain_info.provider.get_balance(default_signer().address()).await? ==
                        parse_ether("10")?,
                    Duration::from_secs(10)
                );

                // deploy the rollup contract for the appchain on the settlement chain
                let _ = Rollup::deploy_builder(
                    &chain_info.provider,
                    U256::from(options.appchain_chain_id),
                    "null".to_string(),
                )
                .nonce(0)
                .send()
                .await?;

                chain_info
            }
            BaseChainsType::PreLoaded(version) => {
                // If flag is set, load the anvil state from a file
                // This is the full set of Arb contracts
                let state_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("config")
                    .join(get_anvil_file(&version));

                let chain_info =
                    start_anvil_with_args(31337, &["--load-state", state_file.to_str().unwrap()])
                        .await?;

                // Sync the tips of the sequencing and settlement chains
                let block = chain_info
                    .provider
                    .get_block_by_number(BlockNumberOrTag::Latest)
                    .await?
                    .unwrap();
                seq_provider
                    .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
                    .await?;
                chain_info
            }
        };

        let (arbitrum_bridge_address, arbitrum_inbox_address) = match options.base_chains_type {
            BaseChainsType::Anvil | BaseChainsType::Nitro => (
                set_provider.default_signer_address().create(0),
                set_provider.default_signer_address().create(0),
            ),
            BaseChainsType::PreLoaded(version) => {
                (get_bridge_address(&version), get_inbox_address(&version))
            }
        };

        info!("sequencing_rpc_url: {}", seq_rpc_url);
        info!("settlement_rpc_url: {}", set_rpc_url);

        // overwrite the rollup owner in case it's not set (cannot be empty in config manager)
        if options.rollup_owner == Address::ZERO {
            options.rollup_owner = address!("0x0000000000000000000000000000000000000064");
        }

        info!("Starting components...");
        info!("Starting synd-mchain...");
        let (mchain_rpc_url, mchain, mchain_provider) =
            start_mchain(options.appchain_chain_id, options.finality_delay).await?;

        // Setup config manager and get chain config address
        let appchain_block_explorer_url = "https://example.com/explorer".to_string();
        let config_manager_address = setup_config_manager(
            &set_provider,
            &options,
            sequencing_contract_address,
            arbitrum_bridge_address,
            arbitrum_inbox_address,
            &seq_rpc_url,
            &appchain_block_explorer_url,
        )
        .await?;

        info!("Starting chain ingestors...");
        let temp = test_path("chain_ingestor");
        let seq_chain_ingestor_cfg = ChainIngestorConfig {
            rpc_url: seq_rpc_url.to_string(),
            db_file: temp.clone() + "/sequencing_chain.db",
            start_block: 0,
            port: PortManager::instance().next_port().await,
            metrics_port: PortManager::instance().next_port().await,
        };
        let sequencing_chain_ingestor = start_component(
            "synd-chain-ingestor",
            seq_chain_ingestor_cfg.metrics_port,
            seq_chain_ingestor_cfg.cli_args(),
            Default::default(),
        )
        .await?;

        let set_chain_ingestor_cfg = ChainIngestorConfig {
            rpc_url: set_rpc_url.clone(),
            db_file: temp + "/settlement_chain.db",
            start_block: 0,
            port: PortManager::instance().next_port().await,
            metrics_port: PortManager::instance().next_port().await,
        };

        let settlement_chain_ingestor = start_component(
            "synd-chain-ingestor",
            set_chain_ingestor_cfg.metrics_port,
            set_chain_ingestor_cfg.cli_args(),
            Default::default(),
        )
        .await?;

        let sequencing_rpc_url = format!("ws://localhost:{}", seq_chain_ingestor_cfg.port);
        let settlement_rpc_url = format!("ws://localhost:{}", set_chain_ingestor_cfg.port);

        info!("Starting translator...");
        let translator_config = TranslatorConfig {
            settlement_rpc_url: settlement_rpc_url.clone(),
            config_manager_address: Some(config_manager_address),
            appchain_chain_id: Some(options.appchain_chain_id),
            mchain_rpc_url: mchain_rpc_url.clone(),
            metrics_port: PortManager::instance().next_port().await,
            arbitrum_bridge_address: None,
            arbitrum_inbox_address: None,
            sequencing_contract_address: None,
            sequencing_rpc_url: Some(sequencing_rpc_url.clone()),
            appchain_block_explorer_url: Some(appchain_block_explorer_url.clone()),
            sequencing_start_block: None,
            settlement_start_block: None,
            settlement_delay: None,
        };

        let translator = start_component(
            "synd-translator",
            translator_config.metrics_port,
            translator_config.cli_args(),
            Default::default(),
        )
        .await?;

        // Launch the appchain nitro node
        info!("Starting appchain nitro node...");
        let ChainInfo {
            instance: appchain_instance,
            provider: appchain_provider,
            ws_url: appchain_rpc_url,
            http_url: _,
        } = launch_nitro_node(NitroNodeArgs {
            chain_id: options.appchain_chain_id,
            chain_owner: options.rollup_owner,
            parent_chain_url: mchain_rpc_url.clone(),
            parent_chain_id: MCHAIN_ID,
            sequencer_mode: NitroSequencerMode::None,
            chain_name: "appchain".to_string(),
            deployment: NitroDeployment {
                bridge: APPCHAIN_CONTRACT,
                sequencer_inbox: APPCHAIN_CONTRACT,
                deployed_at: 1,
                ..Default::default()
            },
        })
        .await?;
        info!("Nitro URL: {}", appchain_rpc_url);

        let assertion_poster_contract_address = match options.base_chains_type {
            BaseChainsType::Anvil => Address::ZERO,
            BaseChainsType::PreLoaded(version) => get_assertion_poster_address(&version),
            BaseChainsType::Nitro => {
                _ = AssertionPoster::deploy_builder(&set_provider, arbitrum_bridge_address)
                    .nonce(1)
                    .send()
                    .await?;
                mine_block(&set_provider, 0).await?;
                set_provider.default_signer_address().create(1)
            }
        };

        let (proposer_instance, proposer_url, enclave_server_instance, tee_signer_address) =
            match options.base_chains_type {
                BaseChainsType::Anvil => (None, String::new(), None, Address::ZERO),
                BaseChainsType::PreLoaded(_) | BaseChainsType::Nitro => {
                    // TODO do not launch enclave if not in Nitro mode. It takes a long time
                    let (enclave_server_instance, enclave_rpc_url, tee_public_key) =
                        launch_enclave_server().await?;

                    info!("Starting proposer...");
                    let proposer_config = ProposerConfig {
                        ethereum_rpc_url: l1_info
                            .as_ref()
                            .map_or(set_rpc_url.clone(), |info| info.ws_url.clone()),
                        assertion_poster_contract_address, // TODO remove
                        tee_module_contract_address: Default::default(), // TODO fill this in
                        arbitrum_bridge_address,
                        inbox_address: arbitrum_inbox_address,
                        sequencer_inbox_address: arbitrum_inbox_address,
                        settlement_rpc_url: set_rpc_url.clone(),
                        metrics_port: PortManager::instance().next_port().await,
                        port: PortManager::instance().next_port().await,
                        appchain_rpc_url: appchain_rpc_url.clone(),
                        sequencing_rpc_url: sequencing_rpc_url.clone(),
                        enclave_rpc_url,
                        polling_interval: "1m".to_string(), // TODO needs to be much lower
                    };

                    let proposer_instance = start_component(
                        "synd-proposer",
                        proposer_config.metrics_port,
                        proposer_config.cli_args(),
                        Default::default(),
                    )
                    .await?;
                    (
                        Some(proposer_instance),
                        format!("http://localhost:{}", proposer_config.port),
                        Some(enclave_server_instance),
                        tee_public_key,
                    )
                }
            };

        let (mut valkey, mut maestro, mut batch_sequencer) = (None, None, None);
        let mut valkey_url_init = String::new();
        let mut maestro_url = Default::default();
        if options.use_write_loop {
            info!("Starting Write Loop components...");
            info!("Starting valkey...");
            let (valkey_instance, valkey_url) = start_valkey().await?;
            valkey_url_init = valkey_url.clone();
            valkey = Some(valkey_instance);
            info!("Starting maestro...");
            let maestro_config = MaestroConfig {
                port: PortManager::instance().next_port().await,
                valkey_url: valkey_url.clone(),
                chain_rpc_urls: format!(
                    "{{\"{}\":\"{}\"}}",
                    options.appchain_chain_id, appchain_rpc_url
                ),
                metrics_port: PortManager::instance().next_port().await,
                finalization_duration: options.maestro_finalization_duration,
                finalization_checker_interval: options.maestro_finalization_checker_interval,
            };
            let maestro_instance = start_component(
                "synd-maestro",
                // `/health` is proxied to RPC method
                maestro_config.port,
                maestro_config.cli_args(),
                Default::default(),
            )
            .await?;
            maestro = Some(maestro_instance);
            maestro_url = format!("http://localhost:{}", maestro_config.port);
            info!("Starting batch sequencer...");
            let batch_sequencer_config = BatchSequencerConfig {
                chain_id: options.appchain_chain_id,
                valkey_url: valkey_url.clone(),
                private_key: PRIVATE_KEY.to_string(),
                sequencing_address: sequencing_contract_address,
                sequencing_rpc_url: seq_rpc_url.to_string(),
                metrics_port: PortManager::instance().next_port().await,
            };
            let batch_sequencer_instance = start_component(
                "synd-batch-sequencer",
                batch_sequencer_config.metrics_port,
                batch_sequencer_config.cli_args(),
                Default::default(),
            )
            .await?;
            batch_sequencer = Some(batch_sequencer_instance);
        }

        let l1_provider = l1_info.as_ref().map(|info| info.provider.clone());
        let l1_instance = l1_info.map(|info| info.instance);

        Ok((
            Self {
                _timer: TestTimer(SystemTime::now(), start_time.elapsed().unwrap()),

                l1_provider,

                sequencing_provider: seq_provider,
                sequencing_rpc_url,
                sequencing_contract,

                settlement_provider: set_provider,
                settlement_rpc_url,
                appchain_provider,
                appchain_chain_id: options.appchain_chain_id,
                bridge_address: arbitrum_bridge_address,
                inbox_address: arbitrum_inbox_address,
                assertion_poster_address: assertion_poster_contract_address,

                mchain_provider,
                proposer_url,
                maestro_url,
                valkey_url: valkey_url_init,
                appchain_block_explorer_url,

                tee_signer_address,
            },
            ComponentHandles {
                l1_chain: l1_instance,
                seq_chain: seq_instance,
                set_chain: set_instance,
                sequencing_chain_ingestor,
                settlement_chain_ingestor,
                mchain,
                appchain_chain: appchain_instance,
                translator,
                proposer: proposer_instance,
                enclave_server: enclave_server_instance,
                batch_sequencer,
                valkey,
                maestro,
            },
        ))
    }

    pub async fn mine_seq_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.sequencing_provider, delay).await?;
        Ok(())
    }

    /// Use this if you intend for the txn to succeed
    /// Returns [`TxHash`]
    #[allow(clippy::unwrap_used)]
    pub async fn send_maestro_tx_successful(&self, raw_tx: &[u8]) -> Result<TxHash> {
        let client = reqwest::Client::new();
        let tx_hex = hex::encode_prefixed(raw_tx);
        info!("tx_hex: {}", tx_hex);
        let response = client
            .post(self.maestro_url.clone())
            .header(HEADER_CHAIN_ID, self.appchain_chain_id.to_string())
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": [tx_hex]
            }))
            .send()
            .await?;
        assert!(response.status().is_success(), "EIP-1559 transaction request failed");
        let json_resp: Value = response.json().await?;

        assert!(
            json_resp.get("result").is_some(),
            "Transaction response missing 'result' field: {}",
            json_resp
        );
        let result = json_resp.get("result").unwrap().as_str().unwrap();
        let tx_hash = TxHash::from_str(result).unwrap();

        Ok(tx_hash)
    }

    /// Use this instead of `send_maestro_tx_successful()` to inspect the JSON `error` field
    #[allow(clippy::unwrap_used)]
    pub async fn send_maestro_tx_could_be_unsuccessful(
        &self,
        tx: &EthereumTxEnvelope<TxEip4844Variant>,
    ) -> Result<Value> {
        let client = reqwest::Client::new();
        let encoded_tx = tx.encoded_2718();
        let tx_hex = hex::encode_prefixed(encoded_tx);
        info!("tx_hex: {}", tx_hex);
        let response = client
            .post(self.maestro_url.clone())
            .header(HEADER_CHAIN_ID, self.appchain_chain_id.to_string())
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": [tx_hex]
            }))
            .send()
            .await?;
        // assert!(response.status().is_success(), "EIP-1559 transaction request failed");
        let json_resp: Value = response.json().await?;
        Ok(json_resp)
    }

    pub async fn mine_set_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.settlement_provider, delay).await?;
        Ok(())
    }

    pub async fn mine_both(&self, delay: u64) -> Result<()> {
        self.mine_seq_block(delay).await?;
        self.mine_set_block(delay).await?;
        Ok(())
    }

    /// sequences a valid appchain raw transaction and mines the sequencing block
    /// returns the appchain's transaction receipt
    #[allow(clippy::unwrap_used)]
    pub async fn sequence_tx(
        &self,
        tx: &[u8],
        seq_delay: u64,
        wait_for_appchain_receipt: bool,
    ) -> Result<Option<TransactionReceipt>> {
        let tx_hash = keccak256(tx);
        let tx_bytes = Bytes::from(tx.to_vec());
        let seq_tx =
            self.sequencing_contract.processTransactionUncompressed(tx_bytes).send().await?;
        self.mine_seq_block(seq_delay).await?;
        let seq_receipt =
            self.sequencing_provider.get_transaction_receipt(*seq_tx.tx_hash()).await?.unwrap();
        assert!(seq_receipt.status(), "Sequence transaction failed");

        match wait_for_appchain_receipt {
            true => {
                let mut receipt = None;
                wait_until!(
                receipt = self.appchain_provider.get_transaction_receipt(tx_hash).await?;
                receipt.is_some(),
                        Duration::from_secs(10)
                    );
                Ok(receipt)
            }
            false => Ok(None),
        }
    }
}
