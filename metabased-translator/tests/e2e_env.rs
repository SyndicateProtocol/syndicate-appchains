use alloy::node_bindings::{Anvil, AnvilInstance};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::Http;
use alloy_provider::RootProvider;
use reqwest::Url;

#[derive(Clone, Debug)]
pub struct TestEnvConfig {
    pub rollup_type: RollupType,
    pub settlement_rpc: Option<String>,
    pub sequencing_rpc: Option<String>,
}

impl TestEnvConfig {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            rollup_type: RollupType::from_env()?,
            settlement_rpc: std::env::var("SETTLEMENT_CHAIN_RPC_URL").ok(),
            sequencing_rpc: std::env::var("SEQUENCING_CHAIN_RPC_URL").ok(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RollupType {
    Optimism,
    Arbitrum,
}

impl RollupType {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        match std::env::var("ROLLUP_TYPE").as_deref() {
            Ok("OP") => Ok(RollupType::Optimism),
            Ok("ARB") => Ok(RollupType::Arbitrum),
            Ok(other) => {
                Err(format!("Invalid ROLLUP_TYPE: {}. Must be 'OP' or 'ARB'", other).into())
            }
            Err(_) => Err("ROLLUP_TYPE environment variable not set".into()),
        }
    }
}

pub struct TestEnv {
    pub rollup_type: RollupType,
    pub settlement_chain: RootProvider<Http<reqwest::Client>>,
    pub sequencing_chain: RootProvider<Http<reqwest::Client>>,
    pub l3: RootProvider<Http<reqwest::Client>>,
    settlement_anvil: Option<AnvilInstance>,
    sequencing_anvil: Option<AnvilInstance>,
}

impl TestEnv {
    pub async fn new(config: TestEnvConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let (settlement_url, settlement_anvil) =
            Self::get_or_create_rpc(config.settlement_rpc).await?;
        let (sequencing_url, sequencing_anvil) =
            Self::get_or_create_rpc(config.sequencing_rpc).await?;

        let settlement_chain = ProviderBuilder::new().on_http(settlement_url);
        let sequencing_chain = ProviderBuilder::new().on_http(sequencing_url);

        // TODO use something real here... below is just a placeholder
        let l3 = ProviderBuilder::new().on_http(Url::parse("https://l3.metabased.xyz")?);

        let env = Self {
            rollup_type: config.rollup_type,
            settlement_anvil,
            sequencing_anvil,
            settlement_chain,
            sequencing_chain,
            l3,
        };

        // Deploy contracts right after environment creation
        env.deploy_contracts().await?;

        Ok(env)
    }

    async fn get_or_create_rpc(
        maybe_url: Option<String>,
    ) -> Result<(Url, Option<AnvilInstance>), Box<dyn std::error::Error>> {
        match maybe_url {
            Some(url) => Ok((Url::parse(&url)?, None)),
            None => {
                let anvil = Anvil::new().spawn();
                Ok((anvil.endpoint_url(), Some(anvil)))
            }
        }
    }

    pub async fn deploy_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Deploy to settlement chain
        self.deploy_settlement_contracts().await?;

        // Deploy to sequencing chain
        self.deploy_sequencing_contracts().await?;

        Ok(())
    }

    async fn deploy_settlement_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Deploy settlement chain specific contracts
        Ok(())
    }

    async fn deploy_sequencing_contracts(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO
        Ok(())
    }

    // async fn sequence_tx(&self, tx: Transaction) -> Result<(), Box<dyn std::error::Error>> {
    //     // TODO
    //     Ok(())
    // }

    // Helper to check if all connections are alive
    async fn check_connections(&self) -> Result<(), Box<dyn std::error::Error>> {
        for provider in [&self.settlement_chain, &self.sequencing_chain] {
            provider.get_block_number().await?;
        }
        Ok(())
    }
}
