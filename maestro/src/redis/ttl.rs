//! Module for all Redis time-to-live (TTL) values for different Redis String keys

/// Time-to-live (TTL) for wallet nonce keys
pub mod wallet_nonce {
    /// Time-to-live (TTL) for Wallet Nonce keys, in seconds
    pub const WALLET_NONCE_TTL_SEC: u64 = 3;
}

/// Time-to-live (TTL) for waiting transaction keys
pub mod waiting_txn {
    /// Time-to-live (TTL) for Waiting Gap Txns keys, in seconds
    pub const WAITING_TXN_TTL_SEC: u64 = 15 * 60; // 15min
}
