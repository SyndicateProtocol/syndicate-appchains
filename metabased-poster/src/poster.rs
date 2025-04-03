//!  The `poster` polls information from the appchain on a `polling_interval` frequency and posts to
//! the settlement chain

use crate::{config::Config, metrics::PosterMetrics, types::NitroBlock};
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
};
use contract_bindings::arbitrum::assertionposter::AssertionPoster::{
    self, AssertionPosterInstance,
};
use eyre::{eyre, Result};
use std::{str::FromStr, time::Duration};
use tracing::{error, info};

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

#[derive(Debug)]
struct Poster {
    appchain_provider: RootProvider,
    polling_interval: Duration,
    assertion_poster: AssertionPosterInstance<(), FilledProvider>,
    metrics: PosterMetrics,
    settlement_provider: FilledProvider,
    wallet_address: Address,
}

/// Starts the poster loop
pub async fn run(config: Config, metrics: PosterMetrics) -> Result<()> {
    let appchain_provider = ProviderBuilder::default().on_http(config.appchain_rpc_url.clone());
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let wallet_address = signer.address();
    let settlement_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .on_http(config.settlement_rpc_url.clone());

    let assertion_poster =
        AssertionPoster::new(config.assertion_poster_contract_address, settlement_provider.clone());

    let poster = Poster {
        appchain_provider,
        polling_interval: config.polling_interval,
        assertion_poster,
        metrics,
        settlement_provider,
        wallet_address,
    };
    poster.main_loop().await
}

impl Poster {
    async fn main_loop(self) -> Result<()> {
        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match self.fetch_block().await {
                        Ok(block) => {
                            if let Err(err) = self.post_assertion(block).await {
                                error!("Failed to post assertion: {:?}", err);
                            }
                        }
                        Err(err) => {
                            error!("Failed to fetch block: {:?}", err);
                        }
                    }

                }
            }
        }
    }

    async fn fetch_block(&self) -> Result<NitroBlock> {
        self.appchain_provider
            .raw_request("eth_getBlockByNumber".into(), ("latest", false))
            .await
            .map_err(|err| eyre!("eth_getBlockByNumber request failed: {:?}", err))
    }

    async fn post_assertion(&self, block: NitroBlock) -> Result<()> {
        let _ = self.assertion_poster.postAssertion(block.hash, block.send_root).send().await?;
        self.metrics.record_last_block_posted(block.number.to());
        info!("Assertion submitted for block: {:?}", block);

        // Check and record wallet balance
        match self.settlement_provider.get_balance(self.wallet_address).await {
            Ok(balance) => {
                self.metrics.record_wallet_balance(balance.to::<u128>(), self.wallet_address);
            }
            Err(e) => {
                error!(
                    ?self.wallet_address,
                    "Error getting wallet balance: {}",
                    e
                );
                self.metrics.record_wallet_balance_error(self.wallet_address);
            }
        }

        Ok(())
    }
}
