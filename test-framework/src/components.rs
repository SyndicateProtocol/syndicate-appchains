//! Components for the integration tests

use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{address, Address, U256},
    providers::{
        ext::AnvilApi as _,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, IpcConnect, Provider, ProviderBuilder, RootProvider, WalletProvider,
    },
    rpc::types::{anvil::MineOptions, Block, BlockTransactionsKind},
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule,
        metabasedsequencerchain::MetabasedSequencerChain::{self, MetabasedSequencerChainInstance},
    },
};
use eyre::{eyre, Result};
use reqwest::Client;
use std::{
    env,
    error::Error,
    fmt::{self, Display},
    str::FromStr,
    time::Duration,
};
use test_utils::port_manager::PortManager;
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::{sleep, timeout},
};
use tracing::{info, Level};
use tracing_subscriber::{fmt as subscriber_fmt, EnvFilter};

#[derive(Debug)]
enum TracingError {
    SubscriberInit(String),
}

impl Display for TracingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubscriberInit(msg) => {
                write!(f, "Failed to initialize subscriber: {}", msg)
            }
        }
    }
}

impl Error for TracingError {}
/// Initializes a tracing subscriber for testing purposes
fn init_test_tracing(level: Level) -> Result<(), TracingError> {
    subscriber_fmt()
        .with_env_filter(EnvFilter::new(level.to_string()))
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;
    Ok(())
}

const PRELOAD_INBOX_ADDRESS_300: Address = address!("0x26eE2349212255614edCc046DD9472F2a5b7EF2b");
const PRELOAD_BRIDGE_ADDRESS_300: Address = address!("0xa0e810a42086da4Ebc5C49fEd626cA6A75B06437");
const PRELOAD_POSTER_ADDRESS_300: Address = address!("0x67d269191c92Caf3cD7723F116c85e6E9bf55933");

const PRELOAD_INBOX_ADDRESS_271: Address = address!("0x7e2d5FCC5E02cBF2b9f860052C0226104E23F9c7");
const PRELOAD_BRIDGE_ADDRESS_271: Address = address!("0x8dAF17A20c9DBA35f005b6324F493785D239719d");
const PRELOAD_POSTER_ADDRESS_271: Address = address!("0x09635F643e140090A9A8Dcd712eD6285858ceBef");

const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // address = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

const MCHAIN_ID: u64 = 84532;
const APPCHAIN_CHAIN_ID: u64 = 13331370;
pub const APPCHAIN_OWNER: Address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

#[derive(Debug)]
pub struct Docker(Child);

impl Drop for Docker {
    fn drop(&mut self) {
        if let Some(x) = self.0.id() {
            _ = std::process::Command::new("kill").arg(x.to_string()).output();
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    _ = self.0.wait().await;
                })
            })
        }
    }
}

#[allow(missing_docs)]
pub type FilledProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

fn chain_config(chain_id: u64) -> String {
    r#"{"config": {
    "chainId": "#
        .to_string() +
        &chain_id.to_string() +
        r#",
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "constantinopleBlock": 0,
    "petersburgBlock": 0,
    "istanbulBlock": 0,
    "berlinBlock": 0,
    "londonBlock": 0,
    "terminalTotalDifficulty": 0,
    "shanghaiTime": 0,
    "cancunTime": 0
  },
  "nonce": "0x0",
  "timestamp": "0x0",
  "extraData": "0x",
  "gasLimit": "0x1c9c380",
  "difficulty": "0x0",
  "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "coinbase": "0x0000000000000000000000000000000000000000",
  "alloc": {
    "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
      "balance": "0xD3C21BCECCEDA1000000"
    }
  },
  "number": "0x0",
  "gasUsed": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}"#
}

#[derive(Debug)]
pub struct NodeInfo {
    pub ipc: String,
    pub auth_ipc: String,
    pub http_port: u16,
}

pub async fn start_reth(
    chain_id: u64,
    tag: &str,
) -> Result<(NodeInfo, (Docker, Option<(Docker, Docker, Docker, Docker)>))> {
    let manager = PortManager::instance();
    let port = manager.next_port();
    let auth_port = manager.next_port();
    let http_port = manager.next_port();
    let dir = env!("CARGO_MANIFEST_DIR");
    let ipc = format!("{dir}/{port}.ipc");
    let auth_ipc = format!("{dir}/{auth_port}.ipc");
    let chain_cfg = chain_config(chain_id);
    let reth = Docker(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("--entrypoint")
            .arg("/bin/sh")
            .arg("-v")
            .arg(if cfg!(target_os = "macos") { "ipc" } else { dir }.to_string() + ":/ipc")
            .arg("-p")
            .arg(format!("{http_port}:{http_port}"))
            .arg(format!("ghcr.io/syndicateprotocol/reth:{tag}"))
            .arg("-c")
            .arg(format!("umask 0 && exec reth node --http --http.addr=0.0.0.0 --http.port={http_port} --ipcpath=/ipc/{port}.ipc --auth-ipc --auth-ipc.path=/ipc/{auth_port}.ipc --chain='{chain_cfg}'"))
            .spawn()?
    );
    #[cfg(not(target_os = "macos"))]
    let socat = None;

    // on mac, use socat to route traffic over tcp as unix domain sockets cannot cross the os
    // boundary. host port.ipc -> host socat -> container socat -> container port.ipc
    // on linux, the port.ipc socket file can be shared between the docker container and host os
    // directly.
    #[cfg(target_os = "macos")]
    let socat = Some((
        Docker(
            Command::new("socat")
                .arg(format!("UNIX-LISTEN:{ipc},reuseaddr,fork"))
                .arg(format!("TCP4:127.0.0.1:{port}"))
                .spawn()?,
        ),
        Docker(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("-p")
                .arg(format!("{port}:{port}"))
                .arg("-v")
                .arg("ipc:/ipc")
                .arg("alpine/socat:1.8.0.1")
                .arg(format!("TCP4-LISTEN:{port},reuseaddr,fork,bind=0.0.0.0"))
                .arg(format!("UNIX-CONNECT:/ipc/{port}.ipc,retry=1200,interval=0.1"))
                .spawn()?,
        ),
        Docker(
            Command::new("socat")
                .arg(format!("UNIX-LISTEN:{auth_ipc},reuseaddr,fork"))
                .arg(format!("TCP4:127.0.0.1:{auth_port}"))
                .spawn()?,
        ),
        Docker(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("-p")
                .arg(format!("{auth_port}:{auth_port}"))
                .arg("-v")
                .arg("ipc:/ipc")
                .arg("alpine/socat:1.8.0.1")
                .arg(format!("TCP4-LISTEN:{auth_port},reuseaddr,fork,bind=0.0.0.0"))
                .arg(format!("UNIX-CONNECT:/ipc/{auth_port}.ipc,retry=1200,interval=0.1"))
                .spawn()?,
        ),
    ));
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        let mut rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(ipc.clone())).await;
        while rollup.is_err() {
            sleep(Duration::from_millis(100)).await;
            rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(ipc.clone())).await;
        }
        let mut auth_rollup =
            ProviderBuilder::new().on_ipc(IpcConnect::new(auth_ipc.clone())).await;
        while auth_rollup.is_err() {
            sleep(Duration::from_millis(100)).await;
            auth_rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(auth_ipc.clone())).await;
        }
        #[allow(clippy::unwrap_used)]
        let r = rollup.unwrap();
        while r.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }
        #[allow(clippy::unwrap_used)]
        let r = auth_rollup.unwrap();
        while r.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }
        Ok::<_, eyre::Error>((NodeInfo { ipc, auth_ipc, http_port }, (reth, socat)))
    })
    .await?
}

pub async fn start_anvil(chain_id: u64) -> Result<(u16, AnvilInstance, FilledProvider)> {
    start_anvil_with_args(chain_id, Default::default()).await
}

pub fn get_default_private_key_signer() -> LocalSigner<SigningKey> {
    PrivateKeySigner::from_str(DEFAULT_PRIVATE_KEY_SIGNER)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}

pub fn get_rollup_contract_address() -> Address {
    get_default_private_key_signer().address().create(0)
}
pub async fn start_anvil_with_args(
    chain_id: u64,
    args: &[&str],
) -> Result<(u16, AnvilInstance, FilledProvider)> {
    let port = PortManager::instance().next_port();
    let mut cmd =
        vec!["--base-fee", "0", "--gas-limit", "30000000", "--timestamp", "0", "--no-mining"];
    cmd.extend_from_slice(args);
    let anvil = Anvil::new().port(port).chain_id(chain_id).args(cmd).try_spawn()?;

    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    Ok((port, anvil, provider))
}

/// mine a block with a delay
async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    let block: Block = provider
        .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
        .await?
        .ok_or_else(|| eyre!("Block not found"))?;
    provider.evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp + delay)))).await?;
    Ok(())
}

/// Get the nitro json configuration data for the rollup
fn rollup_info(rollup_config: &str, chain_name: &str) -> String {
    let rollup = get_rollup_contract_address();
    let deployed_at: u64 = 1;
    let zero = Address::ZERO;
    format!(
        r#"[{{
              "chain-name": "{chain_name}",
              "parent-chain-id": {MCHAIN_ID},
              "parent-chain-is-arbitrum": false,
              "sequencer-url": "",
              "secondary-forwarding-target": "",
              "feed-url": "",
              "secondary-feed-url": "",
              "das-index-url": "",
              "has-genesis-state": false,
              "chain-config": {rollup_config},
              "rollup": {{
                "bridge": "{rollup}",
                "inbox": "{zero}",
                "sequencer-inbox": "{rollup}",
                "deployed-at": {deployed_at},
                "rollup": "{zero}",
                "native-token": "{zero}",
                "upgrade-executor": "{zero}",
                "validator-wallet-creator": "{zero}"
              }}
            }}]"#
    )
}

/// Return the on-chain config for a rollup with a given chain id
pub fn rollup_config(chain_id: u64, chain_owner: Address) -> String {
    let mut cfg = format!(
        r#"{{
            "chainId": {chain_id},
            "homesteadBlock": 0,
            "daoForkBlock": null,
            "daoForkSupport": true,
            "eip150Block": 0,
            "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "eip155Block": 0,
            "eip158Block": 0,
            "byzantiumBlock": 0,
            "constantinopleBlock": 0,
            "petersburgBlock": 0,
            "istanbulBlock": 0,
            "muirGlacierBlock": 0,
            "berlinBlock": 0,
            "londonBlock": 0,
            "clique": {{
            "period": 0,
            "epoch": 0
            }},
            "arbitrum": {{
            "EnableArbOS": true,
            "AllowDebugPrecompiles": false,
            "DataAvailabilityCommittee": false,
            "InitialArbOSVersion": 32,
            "InitialChainOwner": "{chain_owner}",
            "GenesisBlockNum": 0
            }}
        }}"#
    );
    cfg.retain(|c| !c.is_whitespace());
    cfg.shrink_to_fit();
    cfg
}
pub async fn launch_nitro_node(
    chain_id: u64,
    chain_owner: Address,
    mchain_port: u16,
    sequencer_port: u16,
    tag: &str,
) -> Result<(Docker, RootProvider, String)> {
    let port = PortManager::instance().next_port();
    let nitro = Command::new("docker")
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg(format!("offchainlabs/nitro-node:{tag}"))
        .arg(format!("--parent-chain.connection.url=http://localhost:{mchain_port}"))
        .arg("--node.dangerous.disable-blob-reader")
        .arg(format!("--execution.forwarding-target=http://localhost:{sequencer_port}"))
        .arg("--execution.parent-chain-reader.old-header-timeout=1000h")
        .arg("--node.inbox-reader.check-delay=10ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg(format!(
            "--chain.info-json={}",
            rollup_info(&rollup_config(chain_id, chain_owner), "test")
        ))
        .arg("--http.addr=0.0.0.0")
        .arg(format!("--http.port={}", port))
        .arg("--log-level=info")
        .spawn()?;
    let rollup = ProviderBuilder::default().on_http(format!("http://localhost:{}", port).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((Docker(nitro), rollup, format!("http://localhost:{}", port)))
    })
    .await?
}

#[derive(Debug)]
pub struct Components {
    /// Sequencing
    pub sequencing_contract: MetabasedSequencerChainInstance<(), FilledProvider>,
    pub sequencing_provider: FilledProvider,

    /// Settlement
    pub settlement_provider: FilledProvider,
    pub chain_id: u64,
    pub bridge_address: Address,
    pub inbox_address: Address,
    pub assertion_poster_contract_address: Address,

    /// Appchain
    pub appchain_provider: RootProvider,

    // References to keep the processes/tasks alive
    _mchain: (Docker, Option<(Docker, Docker, Docker, Docker)>),
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    _nitro_docker: Docker,
    _translator: Docker,
    _poster: Option<Docker>,
    _sequencer: Docker,
}

/// nitro contract version on the settlement chain used for testing
#[derive(Debug)]
pub enum ContractVersion {
    V213,
    V300,
}

#[derive(Debug, Default)]
struct TranslatorConfig {
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_contract_address: Address,
    mchain_ipc_path: String,
    mchain_auth_ipc_path: String,
    sequencing_rpc_url: String,
    settlement_rpc_url: String,
    metrics_port: u16,
    sequencing_start_block: u64,
    settlement_start_block: u64,
    settlement_delay: u64,
}
#[derive(Debug, Default)]
struct PosterConfig {
    assertion_poster_contract_address: Address,
    settlement_rpc_url: String,
    appchain_rpc_url: String,
    polling_interval: Duration,
    metrics_port: u16,
}
#[derive(Debug, Default)]
struct SequencerConfig {
    sequencing_contract_address: Address,
    settlement_rpc_url: String,
    sequencer_port: u16,
    metrics_port: u16,
}
#[allow(missing_docs)]
#[derive(Debug)]
pub struct ImageTags {
    /// External components
    reth_tag: String,
    nitro_tag: String,

    /// Syndicate components
    poster_tag: Option<String>,
    translator_tag: Option<String>,
    sequencer_tag: Option<String>,
}

impl Default for ImageTags {
    fn default() -> Self {
        Self {
            reth_tag: "2.0.0".to_string(),
            nitro_tag: "v3.4.0-d896e9c-slim".to_string(), // or some other sensible default
            poster_tag: None,
            translator_tag: None,
            sequencer_tag: None,
        }
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct ConfigurationOptions {
    pub sequencing_start_block: u64,
    pub settlement_start_block: u64,
    pub settlement_delay: u64,
}

impl Components {
    pub async fn new(
        pre_loaded: Option<ContractVersion>,
        options: Option<ConfigurationOptions>,
    ) -> Result<Self> {
        let _ = init_test_tracing(Level::INFO);
        info!("Setting tags...");
        let mut tags = ImageTags::default();
        if let Ok(tag) = env::var("NITRO_TAG") {
            tags.nitro_tag = tag;
        }
        if let Ok(tag) = env::var("RETH_TAG") {
            tags.reth_tag = tag;
        }
        if let Ok(tag) = env::var("POSTER_TAG") {
            tags.poster_tag = Some(tag);
        }
        if let Ok(tag) = env::var("TRANSLATOR_TAG") {
            tags.translator_tag = Some(tag);
        }
        if let Ok(tag) = env::var("SEQUENCER_TAG") {
            tags.sequencer_tag = Some(tag);
        }

        info!("Starting components...");
        let mut translator_config: TranslatorConfig = Default::default();
        let mut poster_config: PosterConfig = Default::default();
        let mut sequencer_config: SequencerConfig = Default::default();

        if let Some(opt) = options {
            translator_config.sequencing_start_block = opt.sequencing_start_block;
            translator_config.settlement_start_block = opt.settlement_start_block;
            translator_config.settlement_delay = opt.settlement_delay;
        }
        // Define the addresses of the bridge and inbox contracts depedning on whether we
        // are loading in the full set of Arb contracts or not
        translator_config.arbitrum_bridge_address =
            pre_loaded.as_ref().map_or_else(get_rollup_contract_address, |version| match version {
                ContractVersion::V300 => PRELOAD_BRIDGE_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_BRIDGE_ADDRESS_271,
            });
        translator_config.arbitrum_inbox_address =
            pre_loaded.as_ref().map_or_else(get_rollup_contract_address, |version| match version {
                ContractVersion::V300 => PRELOAD_INBOX_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_INBOX_ADDRESS_271,
            });

        poster_config.assertion_poster_contract_address = pre_loaded.as_ref().map_or_else(
            || Address::ZERO,
            |version| match version {
                ContractVersion::V300 => PRELOAD_POSTER_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_POSTER_ADDRESS_271,
            },
        );

        translator_config.sequencing_contract_address = get_rollup_contract_address();
        sequencer_config.sequencing_contract_address = get_rollup_contract_address();

        info!("Starting reth...");
        let (node, _mchain) = start_reth(MCHAIN_ID, &tags.reth_tag).await?;
        translator_config.mchain_ipc_path = node.ipc;
        translator_config.mchain_auth_ipc_path = node.auth_ipc;

        // Launch mock sequencing chain and deploy contracts
        info!("Starting sequencing chain...");
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(&seq_provider, U256::from(APPCHAIN_CHAIN_ID))
            .send()
            .await?;
        let always_allowed_contract =
            AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
        mine_block(&seq_provider, 0).await?;
        let receipt = always_allowed_contract.get_receipt().await?;
        let always_allowed_module_address = match receipt.contract_address {
            Some(address) => address,
            None => {
                eprintln!("Deployment failed: No contract address found.");
                return Err(eyre!("Deployment failed: No contract address found."));
            }
        };

        // Setup the sequencing contract
        let provider_clone = seq_provider.clone();
        let sequencing_contract =
            MetabasedSequencerChain::new(get_rollup_contract_address(), provider_clone);
        _ = sequencing_contract
            .initialize(seq_provider.default_signer_address(), always_allowed_module_address)
            .send()
            .await?;
        mine_block(&seq_provider, 0).await?;

        // Launch mock settlement chain
        info!("Starting settlement chain...");
        let (set_port, set_anvil, set_provider);
        if let Some(version) = &pre_loaded {
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
        } else {
            // If not use our mock Rollup contract for easier testing
            (set_port, set_anvil, set_provider) = start_anvil(20).await?;
            // Use the mock rollup contract for the test instead of deploying all the nitro rollup
            // contracts
            _ = Rollup::deploy_builder(
                &set_provider,
                U256::from(APPCHAIN_CHAIN_ID),
                rollup_config(APPCHAIN_CHAIN_ID, APPCHAIN_OWNER),
            )
            .nonce(0)
            .send()
            .await?;

            mine_block(&set_provider, 0).await?;
        }

        // Update the RPC URLs in the config
        translator_config.sequencing_rpc_url = format!("http://localhost:{}", seq_port);
        translator_config.settlement_rpc_url = format!("http://localhost:{}", set_port);
        sequencer_config.settlement_rpc_url = format!("http://localhost:{}", set_port);
        poster_config.settlement_rpc_url = format!("http://localhost:{}", set_port);

        // Set up ports
        sequencer_config.sequencer_port = PortManager::instance().next_port();
        sequencer_config.metrics_port = PortManager::instance().next_port();
        translator_config.metrics_port = PortManager::instance().next_port();
        poster_config.metrics_port = PortManager::instance().next_port();

        // Launch components
        info!("Starting sequencer...");
        let sequencer = start_component(
            "metabased-sequencer",
            tags.sequencer_tag,
            get_sequencer_cli_args(&sequencer_config),
        )
        .await?;
        wait_for_service(sequencer_config.metrics_port).await?;
        info!("Starting translator...");
        let translator = start_component(
            "metabased-translator",
            tags.translator_tag,
            get_translator_cli_args(&translator_config),
        )
        .await?;
        wait_for_service(translator_config.metrics_port).await?;
        // Launch the nitro rollup
        info!("Starting nitro node...");
        let (nitro_docker, appchain_provider, nitro_url) = launch_nitro_node(
            APPCHAIN_CHAIN_ID,
            APPCHAIN_OWNER,
            node.http_port,
            sequencer_config.sequencer_port,
            &tags.nitro_tag,
        )
        .await?;
        poster_config.appchain_rpc_url = nitro_url;
        poster_config.polling_interval = Duration::from_secs(1);

        let mut poster = None;
        if pre_loaded.is_some() {
            info!("Starting poster...");
            poster = Some(
                start_component(
                    "metabased-poster",
                    tags.poster_tag,
                    get_poster_cli_args(&poster_config),
                )
                .await?,
            );
            wait_for_service(poster_config.metrics_port).await?;
        }

        // Launch sequencer
        Ok(Self {
            sequencing_contract,
            sequencing_provider: seq_provider,
            settlement_provider: set_provider,
            appchain_provider,
            chain_id: APPCHAIN_CHAIN_ID,
            bridge_address: translator_config.arbitrum_bridge_address,
            inbox_address: translator_config.arbitrum_inbox_address,
            assertion_poster_contract_address: poster_config.assertion_poster_contract_address,

            _mchain,
            _seq_anvil: seq_anvil,
            _set_anvil: set_anvil,
            _nitro_docker: nitro_docker,
            _translator: translator,
            _poster: poster,
            _sequencer: sequencer,
        })
    }

    pub async fn mine_seq_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.sequencing_provider, delay).await?;
        Ok(())
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
}

async fn start_component(
    executable_name: &str,
    tag: Option<String>,
    args: Vec<String>,
) -> Result<Docker> {
    if let Some(tag) = tag {
        Ok(Docker(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("--net=host")
                .arg(format!("ghcr.io/syndicateprotocol/{executable_name}:{tag}"))
                .args(args)
                .spawn()?,
        ))
    } else {
        Ok(Docker(
            Command::new("cargo")
                .current_dir("../")
                .arg("run")
                .arg("--bin")
                .arg(executable_name)
                .arg("--")
                .args(args)
                .spawn()?,
        ))
    }
}

async fn wait_for_service(port: u16) -> Result<()> {
    let client = Client::new();
    loop {
        match client.get(format!("http://localhost:{port}/metrics")).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Metabased-translator is now running.");
                return Ok(());
            }
            _ => {
                println!("Waiting for metabased-translator to start...");
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

fn get_translator_cli_args(config: &TranslatorConfig) -> Vec<String> {
    vec![
        "--mchain-ipc-path".to_string(),
        config.mchain_ipc_path.to_string(),
        "--mchain-auth-ipc-path".to_string(),
        config.mchain_auth_ipc_path.to_string(),
        "--sequencing-contract-address".to_string(),
        config.sequencing_contract_address.to_string(),
        "--arbitrum-bridge-address".to_string(),
        config.arbitrum_bridge_address.to_string(),
        "--arbitrum-inbox-address".to_string(),
        config.arbitrum_inbox_address.to_string(),
        "--sequencing-rpc-url".to_string(),
        config.sequencing_rpc_url.to_string(),
        "--settlement-rpc-url".to_string(),
        config.settlement_rpc_url.to_string(),
        "--metrics-port".to_string(),
        config.metrics_port.to_string(),
        "--target-chain-id".to_string(),
        APPCHAIN_CHAIN_ID.to_string(),
        "--rollup-owner-address".to_string(),
        APPCHAIN_OWNER.to_string(),
        "--sequencing-start-block".to_string(),
        config.sequencing_start_block.to_string(),
        "--settlement-start-block".to_string(),
        config.settlement_start_block.to_string(),
        "--settlement-delay".to_string(),
        config.settlement_delay.to_string(),
    ]
}

fn get_poster_cli_args(config: &PosterConfig) -> Vec<String> {
    vec![
        "--private-key".to_string(),
        DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
        "--appchain-rpc-url".to_string(),
        config.appchain_rpc_url.to_string(),
        "--assertion-poster-contract-address".to_string(),
        config.assertion_poster_contract_address.to_string(),
        "--settlement-rpc-url".to_string(),
        config.settlement_rpc_url.to_string(),
        "--metrics-port".to_string(),
        config.metrics_port.to_string(),
        "--polling-interval".to_string(),
        config.polling_interval.as_secs().to_string() + "s",
    ]
}

fn get_sequencer_cli_args(config: &SequencerConfig) -> Vec<String> {
    vec![
        "--private-key".to_string(),
        DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
        "--chain-contract-address".to_string(),
        config.sequencing_contract_address.to_string(),
        "--chain-rpc-url".to_string(),
        config.settlement_rpc_url.to_string(),
        "--metrics-port".to_string(),
        config.metrics_port.to_string(),
        "--port".to_string(),
        config.sequencer_port.to_string(),
    ]
}
