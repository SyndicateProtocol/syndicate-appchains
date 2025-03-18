//! The `nonce` module provides a nonce manager that caches nonces for a wallet and can be reset.

use alloy::{
    network::Network,
    primitives::Address,
    providers::{fillers::NonceManager, Provider},
    transports::TransportResult,
};
use dashmap::DashMap;
use futures::lock::Mutex;
use jsonrpsee::core::async_trait;
use std::sync::Arc;

/// A nonce manager that is able to reset its nonces.
#[derive(Clone, Debug, Default)]
pub struct CachedResettingNonceManager {
    nonces: Arc<DashMap<Address, Arc<Mutex<u64>>>>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl NonceManager for CachedResettingNonceManager {
    async fn get_next_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        P: Provider<N>,
        N: Network,
    {
        // Use `u64::MAX` as a sentinel value to indicate that the nonce has not been fetched yet.
        const NONE: u64 = u64::MAX;

        // Locks dashmap internally for a short duration to clone the `Arc`.
        // We also don't want to hold the dashmap lock through the await point below.
        let nonce = {
            let rm = self.nonces.entry(address).or_insert_with(|| Arc::new(Mutex::new(NONE)));
            Arc::clone(rm.value())
        };

        let mut nonce = nonce.lock().await;
        let new_nonce = if *nonce == NONE {
            // Initialize the nonce if we haven't seen this account before.
            provider.get_transaction_count(address).await?
        } else {
            *nonce + 1
        };
        *nonce = new_nonce;
        Ok(new_nonce)
    }
}

impl CachedResettingNonceManager {
    /// Resets all nonces
    pub fn clear_all_nonces(&self) {
        self.nonces.clear();
    }
}
