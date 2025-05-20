//!  The `synd-proposer` polls information from the appchain on a `polling_interval` frequency and
//! posts to the settlement chain

use crate::{config::Config, metrics::ProposerMetrics, types::NitroBlock};
use alloy::{
    network::EthereumWallet,
    providers::{Provider as _, ProviderBuilder, RootProvider, WalletProvider as _},
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
use shared::types::FilledProvider;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, task::JoinHandle};
use tracing::{error, info};

#[derive(Debug)]
struct Proposer {
    appchain_provider: RootProvider,
    polling_interval: Duration,
    assertion_poster: AssertionPosterInstance<(), FilledProvider>,
    metrics: ProposerMetrics,
}

/// Starts the Proposer loop
pub async fn run(config: Config, metrics: ProposerMetrics) -> Result<()> {
    let appchain_provider =
        ProviderBuilder::default().connect(config.appchain_rpc_url.as_str()).await?;
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let settlement_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(config.settlement_rpc_url.as_str())
        .await?;

    let assertion_poster =
        AssertionPoster::new(config.assertion_poster_contract_address, settlement_provider);

    let proposer = Arc::new(Proposer {
        appchain_provider,
        polling_interval: config.polling_interval,
        assertion_poster,
        metrics,
    });

    // Clone for both tasks
    let proposer_polling = Arc::clone(&proposer);
    let proposer_http = Arc::clone(&proposer);

    // Start polling loop
    let polling_task: JoinHandle<Result<()>> =
        tokio::spawn(async move { proposer_polling.main_loop().await });

    // Start HTTP server with /post endpoint // TODO(SEQ-948)
    let app = Router::new().route("/post", post(post_assertion_handler)).with_state(proposer_http);
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

async fn post_assertion_handler(State(proposer): State<Arc<Proposer>>) -> Response {
    match proposer.fetch_and_post().await {
        Ok(_) => (StatusCode::OK, "Assertion posted successfully").into_response(),
        Err(err) => {
            error!("Handler error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to post assertion: {:?}", err))
                .into_response()
        }
    }
}

impl Proposer {
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
        // TODO (SEQ-948)
        match self.fetch_block().await {
            Ok(block) => {
                if let Err(err) = self.post_assertion(block).await {
                    error!("Failed to post assertion: {:?}", err); // TODO (SEQ-948)
                }
            }
            Err(err) => {
                error!("Failed to fetch block: {:?}", err);
            }
        }
        Ok(())
    }
}
