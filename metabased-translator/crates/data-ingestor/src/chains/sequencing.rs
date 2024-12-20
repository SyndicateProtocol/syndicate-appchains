use alloy_provider::{ProviderBuilder, RootProvider};
use alloy_transport_http::{Http, Client};
use reqwest::Url;
use std::error::Error;

pub struct SequencingChain {
    client: RootProvider<Http<Client>>,
}

impl SequencingChain {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let url = Url::parse(url)?;
        
        let client = ProviderBuilder::new()
            .on_http(url);
        Ok(Self { client })
    }
}