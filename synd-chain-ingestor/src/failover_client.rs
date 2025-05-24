use crate::eth_client::EthClient;
use alloy::{
    eips::BlockNumberOrTag,
    pubsub::Subscription,
    rpc::types::{Filter, Header},
    transports::{RpcError, TransportErrorKind},
};
use shared::types::Receipt;
use std::time::Duration;
use tracing::{debug, error, warn};

#[derive(Debug, Clone)]
pub struct FailoverEthClient {
    clients: Vec<EthClient>,
    current_index: std::sync::Arc<tokio::sync::RwLock<usize>>,
    failover_wait_ms: u64,
}

impl FailoverEthClient {
    pub async fn new(urls: Vec<String>, timeout: Duration, log_timeout: Duration, channel_size: usize, failover_wait_ms: u64) -> Self {
        let mut clients = Vec::new();
        
        for url in urls {
            let client = EthClient::new(&url, timeout, log_timeout, channel_size).await;
            clients.push(client);
        }
        
        if clients.is_empty() {
            panic!("No RPC URLs provided for failover client");
        }
        
        Self {
            clients,
            current_index: std::sync::Arc::new(tokio::sync::RwLock::new(0)),
            failover_wait_ms,
        }
    }

    async fn get_current_client(&self) -> EthClient {
        let index = *self.current_index.read().await;
        self.clients[index].clone()
    }

    async fn switch_to_next_client(&self) -> bool {
        let mut current_index = self.current_index.write().await;
        let old_index = *current_index;
        *current_index = (*current_index + 1) % self.clients.len();
        
        if *current_index != old_index {
            warn!(old_index=%old_index, new_index=%*current_index, "Switching to backup RPC client");
            true
        } else {
            false
        }
    }

    pub async fn get_block_header(&self, block_identifier: BlockNumberOrTag) -> Header {
        let mut attempts = 0;
        let max_attempts = self.clients.len();

        loop {
            let client = self.get_current_client().await;
            let start_time = std::time::Instant::now();
            
            match tokio::time::timeout(Duration::from_secs(10), client.get_block_header(block_identifier)).await {
                Ok(result) => return result,
                Err(_) => {
                    attempts += 1;
                    error!("get_block_header timed out, attempt {}/{}", attempts, max_attempts);
                    
                    if attempts >= max_attempts {
                        panic!("All RPC clients failed for get_block_header");
                    }
                    
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    if elapsed < self.failover_wait_ms {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            self.failover_wait_ms - elapsed
                        )).await;
                    }
                    
                    self.switch_to_next_client().await;
                }
            }
        }
    }

    pub async fn get_block_receipts(&self, number: u64) -> Vec<Receipt> {
        let mut attempts = 0;
        let max_attempts = self.clients.len();

        loop {
            let client = self.get_current_client().await;
            let start_time = std::time::Instant::now();
            
            match tokio::time::timeout(Duration::from_secs(10), client.get_block_receipts(number)).await {
                Ok(result) => return result,
                Err(_) => {
                    attempts += 1;
                    error!("get_block_receipts timed out, attempt {}/{}", attempts, max_attempts);
                    
                    if attempts >= max_attempts {
                        panic!("All RPC clients failed for get_block_receipts");
                    }
                    
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    if elapsed < self.failover_wait_ms {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            self.failover_wait_ms - elapsed
                        )).await;
                    }
                    
                    self.switch_to_next_client().await;
                }
            }
        }
    }

    pub async fn subscribe_blocks(&self) -> Subscription<Header> {
        self.get_current_client().await.subscribe_blocks().await
    }

    pub async fn get_chain_id(&self) -> u64 {
        let mut attempts = 0;
        let max_attempts = self.clients.len();

        loop {
            let client = self.get_current_client().await;
            let start_time = std::time::Instant::now();
            
            match tokio::time::timeout(Duration::from_secs(10), client.get_chain_id()).await {
                Ok(result) => return result,
                Err(_) => {
                    attempts += 1;
                    error!("get_chain_id timed out, attempt {}/{}", attempts, max_attempts);
                    
                    if attempts >= max_attempts {
                        panic!("All RPC clients failed for get_chain_id");
                    }
                    
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    if elapsed < self.failover_wait_ms {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            self.failover_wait_ms - elapsed
                        )).await;
                    }
                    
                    self.switch_to_next_client().await;
                }
            }
        }
    }

    pub async fn get_logs(&self, filter: &Filter) -> Result<Vec<alloy::rpc::types::Log>, RpcError<TransportErrorKind>> {
        let mut attempts = 0;
        let max_attempts = self.clients.len();

        loop {
            let client = self.get_current_client().await;
            let start_time = std::time::Instant::now();
            
            match tokio::time::timeout(Duration::from_secs(30), client.get_logs(filter)).await {
                Ok(Ok(result)) => return Ok(result),
                Ok(Err(e)) => {
                    if let RpcError::ErrorResp(_) = e {
                        return Err(e);
                    }
                    
                    attempts += 1;
                    error!("get_logs failed with error: {}, attempt {}/{}", e, attempts, max_attempts);
                    
                    if attempts >= max_attempts {
                        error!("All RPC clients failed for get_logs");
                        return Err(e);
                    }
                    
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    if elapsed < self.failover_wait_ms {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            self.failover_wait_ms - elapsed
                        )).await;
                    }
                    
                    self.switch_to_next_client().await;
                },
                Err(_) => {
                    attempts += 1;
                    error!("get_logs timed out, attempt {}/{}", attempts, max_attempts);
                    
                    if attempts >= max_attempts {
                        return Err(TransportErrorKind::Custom("All RPC clients timed out for get_logs".into()).into());
                    }
                    
                    let elapsed = start_time.elapsed().as_millis() as u64;
                    if elapsed < self.failover_wait_ms {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            self.failover_wait_ms - elapsed
                        )).await;
                    }
                    
                    self.switch_to_next_client().await;
                }
            }
        }
    }
}
