use alloy_provider::{ProviderBuilder, RootProvider, WsConnect};
use alloy_pubsub::PubSubFrontend;
use std::error::Error;

pub struct SettlementChain {
    client: RootProvider<PubSubFrontend>, // Adjusted type to match the returned value
}

impl SettlementChain {
    pub async fn new(rpc_url: &str) -> Result<Self, Box<dyn Error>> {
        let connect = WsConnect::new(rpc_url);
        // Configure the WebSocket connection
        let client = ProviderBuilder::new()
            .on_ws(connect) // Use the WebSocket URL
            .await?; // Await the provider building process

        Ok(Self { client })
    }
}
