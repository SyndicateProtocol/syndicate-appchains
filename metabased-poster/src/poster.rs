//!  The `poster` polls information from the appchain on a `polling_interval` frequency and posts to
//! the settlement chain

use crate::{config::Config, metrics::PosterMetrics, types::NitroBlock};
use alloy::{
    network::EthereumWallet,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider as _, ProviderBuilder, RootProvider, WalletProvider as _,
    },
    signers::local::PrivateKeySigner,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve, Router,
};
use contract_bindings::arbitrum::assertionposter::AssertionPoster::{
    self, AssertionPosterInstance,
};
use eyre::{eyre, Result};
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, task::JoinHandle};
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
}

/// Starts the poster loop
pub async fn run(config: Config, metrics: PosterMetrics) -> Result<()> {
    let appchain_provider = ProviderBuilder::default().on_http(config.appchain_rpc_url.clone());
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let settlement_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .on_http(config.settlement_rpc_url.clone());

    let assertion_poster =
        AssertionPoster::new(config.assertion_poster_contract_address, settlement_provider);

    let poster = Arc::new(Poster {
        appchain_provider,
        polling_interval: config.polling_interval,
        assertion_poster,
        metrics,
    });

    // Clone for both tasks
    let poster_polling = Arc::clone(&poster);
    let poster_http = Arc::clone(&poster);

    // Start polling loop
    let polling_task: JoinHandle<Result<()>> =
        tokio::spawn(async move { poster_polling.main_loop().await });

    // Start HTTP server with /post endpoint
    let app = Router::new().route("/post", post(post_assertion_handler)).with_state(poster_http);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    let server_task = tokio::spawn(async move {
        if let Err(err) = serve(listener, app).await {
            eprintln!("HTTP server error: {:?}", err);
        }
    });

    // Wait for both tasks
    let _ = tokio::try_join!(polling_task, server_task)?;
    Ok(())
}

async fn post_assertion_handler(State(poster): State<Arc<Poster>>) -> Response {
    match poster.fetch_and_post().await {
        Ok(_) => (StatusCode::OK, "Assertion posted successfully").into_response(),
        Err(err) => {
            error!("Handler error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to post assertion: {:?}", err))
                .into_response()
        }
    }
}

impl Poster {
    async fn main_loop(&self) -> Result<()> {
        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            interval.tick().await;
            self.fetch_and_post().await?;
        }
    }

    async fn fetch_block(&self) -> Result<NitroBlock> {
        self.appchain_provider
            .raw_request("eth_getBlockByNumber".into(), ("latest", false))
            .await
            .map_err(|err| eyre!("eth_getBlockByNumber request failed: {:?}", err))
    }

    async fn record_wallet_balance(&self) -> Result<()> {
        let provider = self.assertion_poster.provider();
        let wallet_address = provider.default_signer_address();

        let balance = provider.get_balance(wallet_address).await?;
        self.metrics.record_wallet_balance(balance.to());
        Ok(())
    }

    async fn post_assertion(&self, block: NitroBlock) -> Result<()> {
        self.record_wallet_balance().await?;

        let _ = self.assertion_poster.postAssertion(block.hash, block.send_root).send().await?;
        self.metrics.record_last_block_posted(block.number.to());

        info!("Assertion submitted for block: {:?}", block);
        Ok(())
    }

    async fn fetch_and_post(&self) -> Result<()> {
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
        Ok(())
    }
}
