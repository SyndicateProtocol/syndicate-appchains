//! Module to store unique Redis key formats in order to improve visibility and avoid collisions

/// Redis key to retrieve Wallet Nonce values
pub mod wallet_nonce {
    use alloy::primitives::{Address, ChainId};

    /// Unique prefix of Wallet Nonce key for Redis String retrieval
    pub const WALLET_NONCE_KEY_PREFIX: &str = "maestro:wallet-nonce";

    /// Generates a Redis String key for a specific chain and wallet address. Note that such a
    /// combination is unique per chain
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to create the key for
    /// * `wallet_address` - The wallet address to create the key for
    ///
    /// # Returns
    /// A string in the format `synd-maestro:wallet-nonce:{chain_id}_{wallet_address}`
    pub fn chain_wallet_nonce_key(
        chain_id: ChainId,
        wallet_address: Address,
    ) -> ChainWalletNonceKey {
        let chain_wallet_composite_key = format!("{}_{}", chain_id, wallet_address);
        format!("{}:{}", WALLET_NONCE_KEY_PREFIX, chain_wallet_composite_key)
    }

    /// Key produced by `chain_wallet_nonce_key()` above
    pub type ChainWalletNonceKey = String;
}

/// Redis keys to retrieve Waiting Transaction values
pub mod waiting_txn {
    use alloy::primitives::{Address, ChainId};

    /// Unique prefix of Waiting Gap Txns key for Redis String retrieval
    pub const WAITING_GAP_KEY_PREFIX: &str = "maestro:waiting-gap-txns";

    /// Generates a Redis String key for a specific chain, wallet, and nonce, thereby corresponding
    /// to a unique transaction.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to create the key for
    /// * `wallet_address` - The wallet address to create the key for
    /// * `nonce` - The transaction nonce
    ///
    /// # Returns
    /// A string in the format `synd-maestro:waiting-gap-txns:{chain_id}_{wallet_address}_{nonce}`
    pub fn waiting_gap_txns_key(chain_id: ChainId, wallet_address: Address, nonce: u64) -> String {
        let waiting_gap_composite_key = format!("{}_{}_{}", chain_id, wallet_address, nonce);
        format!("{}:{}", WAITING_GAP_KEY_PREFIX, waiting_gap_composite_key)
    }
}
