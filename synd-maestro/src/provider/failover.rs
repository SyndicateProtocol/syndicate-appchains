use alloy::{
    network::Ethereum,
    primitives::ChainId,
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::{RpcError, TransportErrorKind},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub struct FailoverProvider {
    providers: Vec<RootProvider<Ethereum>>,
    current_index: Arc<RwLock<usize>>,
    chain_id: ChainId,
    failover_wait_ms: u64,
}

impl FailoverProvider {
    pub async fn new(
        urls: Vec<String>,
        chain_id: ChainId,
        failover_wait_ms: u64,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut providers = Vec::new();

        for url in urls {
            match ProviderBuilder::default().connect(&url).await {
                Ok(provider) => match provider.get_chain_id().await {
                    Ok(resp_chain_id) if resp_chain_id == chain_id => {
                        providers.push(provider);
                    }
                    Ok(resp_chain_id) => {
                        error!(%url, expected=%chain_id, got=%resp_chain_id, "Chain ID mismatch for RPC provider");
                    }
                    Err(e) => {
                        error!(%url, %e, "Failed to verify chain ID for RPC provider");
                    }
                },
                Err(e) => {
                    error!(%url, %e, "Failed to connect to RPC provider");
                }
            }
        }

        if providers.is_empty() {
            return Err("No working RPC providers found".into());
        }

        Ok(Self { providers, current_index: Arc::new(RwLock::new(0)), chain_id, failover_wait_ms })
    }

    async fn get_current_provider(&self) -> RootProvider<Ethereum> {
        let index = *self.current_index.read().await;
        self.providers[index].clone()
    }

    async fn should_failover(&self, error: &RpcError<TransportErrorKind>) -> bool {
        match error {
            RpcError::Transport(transport_err) => transport_err.recoverable(),
            RpcError::ErrorResp(_) => false,
            _ => false,
        }
    }

    async fn switch_to_next_provider(&self) -> bool {
        let mut current_index = self.current_index.write().await;
        let old_index = *current_index;
        *current_index = (*current_index + 1) % self.providers.len();

        if *current_index != old_index {
            warn!(
                chain_id=%self.chain_id,
                old_index=%old_index,
                new_index=%*current_index,
                "Switching to backup RPC provider"
            );
            true
        } else {
            false
        }
    }

    pub async fn execute_with_failover<F, T>(
        &self,
        operation: F,
    ) -> Result<T, RpcError<TransportErrorKind>>
    where
        F: Fn(
            RootProvider<Ethereum>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<T, RpcError<TransportErrorKind>>> + Send>,
        >,
        T: Send,
    {
        let max_attempts = self.providers.len();
        let mut attempts = 0;

        loop {
            let provider = self.get_current_provider().await;
            let start_time = std::time::Instant::now();

            match operation(provider).await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    attempts += 1;

                    if attempts >= max_attempts {
                        error!(%error, chain_id=%self.chain_id, "All RPC providers failed");
                        return Err(error);
                    }

                    if self.should_failover(&error).await {
                        debug!(%error, chain_id=%self.chain_id, "RPC error detected, attempting failover");

                        let elapsed = start_time.elapsed().as_millis() as u64;
                        if elapsed < self.failover_wait_ms {
                            tokio::time::sleep(std::time::Duration::from_millis(
                                self.failover_wait_ms - elapsed,
                            ))
                            .await;
                        }

                        if !self.switch_to_next_provider().await {
                            error!(%error, chain_id=%self.chain_id, "No more providers to failover to");
                            return Err(error);
                        }
                    } else {
                        error!(%error, chain_id=%self.chain_id, "Non-recoverable RPC error");
                        return Err(error);
                    }
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl Provider<Ethereum> for FailoverProvider {
    fn root(&self) -> &RootProvider<Ethereum> {
        &self.providers[*self.current_index.blocking_read()]
    }

    async fn get_chain_id(&self) -> Result<ChainId, RpcError<TransportErrorKind>> {
        self.execute_with_failover(|provider| {
            Box::pin(async move { provider.get_chain_id().await })
        })
        .await
    }

    async fn get_transaction_count(
        &self,
        address: alloy::primitives::Address,
    ) -> Result<u64, RpcError<TransportErrorKind>> {
        self.execute_with_failover(|provider| {
            Box::pin(async move { provider.get_transaction_count(address).await })
        })
        .await
    }

    async fn get_balance(
        &self,
        address: alloy::primitives::Address,
    ) -> Result<alloy::primitives::U256, RpcError<TransportErrorKind>> {
        self.execute_with_failover(|provider| {
            Box::pin(async move { provider.get_balance(address).await })
        })
        .await
    }

    async fn get_transaction_receipt(
        &self,
        hash: alloy::primitives::B256,
    ) -> Result<Option<alloy::rpc::types::TransactionReceipt>, RpcError<TransportErrorKind>> {
        self.execute_with_failover(|provider| {
            Box::pin(async move { provider.get_transaction_receipt(hash).await })
        })
        .await
    }

    async fn send_raw_transaction(
        &self,
        tx: alloy::primitives::Bytes,
    ) -> Result<alloy::primitives::TxHash, RpcError<TransportErrorKind>> {
        self.execute_with_failover(|provider| {
            Box::pin(async move {
                let result = provider.send_raw_transaction(&tx).await?;
                Ok(*result)
            })
        })
        .await
    }
}
