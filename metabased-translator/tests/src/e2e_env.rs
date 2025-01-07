use alloy::providers::ProviderBuilder;
use alloy::transports::http::Http;
use alloy_provider::{Provider, RootProvider};
use eyre::Error;
use reqwest::Url;

// NOTE: all these env vars should be in place when running tests
const ENV_SETTLEMENT_CHAIN_RPC_URL: &str = "SETTLEMENT_CHAIN_RPC_URL";
const ENV_SEQUENCING_CHAIN_RPC_URL: &str = "SEQUENCING_CHAIN_RPC_URL";
const ENV_META_BASED_CHAIN_RPC_URL: &str = "META_BASED_CHAIN_RPC_URL";
const ENV_ROLLUP_TYPE: &str = "ROLLUP_TYPE";

#[derive(Clone, Debug)]
pub struct TestEnvConfig {
    pub rollup_type: RollupType,
    pub settlement_rpc: Url,
    pub sequencing_rpc: Url,
    pub l3_rpc: Url,
}

impl TestEnvConfig {
    pub fn from_env() -> Result<Self, Error> {
        // TODO TODO (SEQ-435)
        Ok(Self {
            rollup_type: RollupType::from_env()?,
            settlement_rpc: Url::parse(&std::env::var(ENV_SETTLEMENT_CHAIN_RPC_URL)?)?,
            sequencing_rpc: Url::parse(&std::env::var(ENV_SEQUENCING_CHAIN_RPC_URL)?)?,
            l3_rpc: Url::parse(&std::env::var(ENV_META_BASED_CHAIN_RPC_URL)?)?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RollupType {
    Optimism,
    Arbitrum,
}

impl RollupType {
    pub fn from_env() -> Result<Self, Error> {
        match std::env::var(ENV_ROLLUP_TYPE).as_deref() {
            Ok("OP") => Ok(RollupType::Optimism),
            Ok("ARB") => Ok(RollupType::Arbitrum),
            Ok(other) => Err(eyre::eyre!(
                "Invalid ROLLUP_TYPE: {}. Must be 'OP' or 'ARB'",
                other
            )),
            Err(_) => Err(eyre::eyre!("ROLLUP_TYPE environment variable not set")),
        }
    }
}

pub struct TestEnv {
    pub config: TestEnvConfig,
    pub settlement_chain: RootProvider<Http<reqwest::Client>>,
    pub sequencing_chain: RootProvider<Http<reqwest::Client>>,
    pub l3: RootProvider<Http<reqwest::Client>>,
}

impl TestEnv {
    pub async fn new() -> Result<Self, Error> {
        let config = TestEnvConfig::from_env()?;

        let settlement_chain = ProviderBuilder::new().on_http(config.settlement_rpc.clone());
        let sequencing_chain = ProviderBuilder::new().on_http(config.sequencing_rpc.clone());
        let l3 = ProviderBuilder::new().on_http(config.l3_rpc.clone());

        let env = Self {
            config,
            settlement_chain,
            sequencing_chain,
            l3,
        };

        env.check_connections().await?;

        Ok(env)
    }

    // async fn sequence_tx(&self, tx: Transaction) -> Result<(), Box<dyn std::error::Error>> {
    //     // TODO
    //     Ok(())
    // }

    // Helper to check if all connections are alive
    async fn check_connections(&self) -> Result<(), Error> {
        for provider in [&self.settlement_chain, &self.sequencing_chain, &self.l3] {
            provider.get_block_number().await?;
        }
        Ok(())
    }
}
