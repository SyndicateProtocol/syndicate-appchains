//! Test fixtures and setup utilities for benchmarks

/// Create a temporary directory for testing
pub fn setup_temp_dir() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("bench_test_{}", rand::random::<u64>()))
}

/// Cleanup test directory
pub fn cleanup_test_dir(dir_path: &std::path::Path) {
    if dir_path.exists() {
        std::fs::remove_dir_all(dir_path).ok();
    }
}

/// Load scenarios for different benchmark intensities
#[derive(Debug, Clone, Copy)]
pub enum LoadScenario {
    Light,   // Low transaction volume
    Medium,  // Moderate transaction volume
    Heavy,   // High transaction volume
    Extreme, // Very high transaction volume
}

impl LoadScenario {
    pub const fn transaction_count(self) -> usize {
        match self {
            Self::Light => 10,
            Self::Medium => 100,
            Self::Heavy => 1000,
            Self::Extreme => 10000,
        }
    }

    pub const fn batch_size(self) -> usize {
        match self {
            Self::Light => 5,
            Self::Medium => 25,
            Self::Heavy => 100,
            Self::Extreme => 500,
        }
    }
}
