//! Shared retry utilities with exponential backoff functionality

use std::time::Duration;
use tokio::time;

/// Configuration for exponential backoff retry behavior
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    /// Initial delay duration (default: 100ms)
    pub initial_delay: Duration,
    /// Maximum delay duration (default: 30 seconds)
    pub max_delay: Duration,
    /// Multiplier for each retry (default: 2.0)
    pub multiplier: f64,
    /// Maximum jitter percentage (default: 0.1 for 10%)
    pub jitter: f64,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: 0.1,
        }
    }
}

impl ExponentialBackoff {
    /// Create a new exponential backoff configuration with custom parameters
    pub const fn new(
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
        jitter: f64,
    ) -> Self {
        Self { initial_delay, max_delay, multiplier, jitter }
    }

    /// Calculate the delay for a given attempt number (0-based)
    ///
    /// # Arguments
    /// * `attempt` - The attempt number (0 for first retry, 1 for second, etc.)
    ///
    /// # Returns
    /// The calculated delay duration with jitter applied
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let base_delay =
            self.initial_delay.as_millis() as f64 * self.multiplier.powi(attempt as i32);
        let capped_delay = base_delay.min(self.max_delay.as_millis() as f64);

        // Apply jitter: random value between (1-jitter) and (1+jitter)
        let jitter_factor = if self.jitter > 0.0 {
            use std::{
                collections::hash_map::DefaultHasher,
                hash::{Hash, Hasher},
            };

            // Use current time and attempt as seed for consistent but pseudo-random jitter
            let mut hasher = DefaultHasher::new();
            (
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64,
                attempt,
            )
                .hash(&mut hasher);
            let hash = hasher.finish();
            let normalized = (hash % 10000) as f64 / 10000.0; // 0.0 to 1.0
            self.jitter.mul_add(2.0f64.mul_add(normalized, -1.0), 1.0) // jitter around 1.0
        } else {
            1.0
        };

        Duration::from_millis((capped_delay * jitter_factor) as u64)
    }

    /// Sleep with exponential backoff for the given attempt
    ///
    /// # Arguments
    /// * `attempt` - The attempt number (0 for first retry, 1 for second, etc.)
    pub async fn sleep(&self, attempt: u32) {
        let delay = self.delay_for_attempt(attempt);
        time::sleep(delay).await;
    }
}

/// Trait for classifying errors as recoverable or unrecoverable
pub trait ErrorClassification {
    /// Returns true if this error is recoverable and retry should be attempted
    fn is_recoverable(&self) -> bool;

    /// Returns true if this error is unrecoverable and the process should exit
    fn is_unrecoverable(&self) -> bool {
        !self.is_recoverable()
    }
}

/// Actions to take when handling an error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorAction {
    /// Retry the operation with exponential backoff
    Retry,
    /// Exit the process immediately (unrecoverable error)
    Exit,
}

/// Determine the appropriate action for an error
///
/// # Arguments
/// * `error` - The error to classify
///
/// # Returns
/// The action that should be taken for this error
pub fn classify_error<E: ErrorClassification>(error: &E) -> ErrorAction {
    if error.is_recoverable() {
        ErrorAction::Retry
    } else {
        ErrorAction::Exit
    }
}

/// Handle an error with appropriate retry or exit logic
///
/// This is a convenience function that classifies an error and takes the appropriate action.
/// For recoverable errors, it will sleep with exponential backoff.
/// For unrecoverable errors, it will log a fatal error and exit the process.
///
/// # Arguments
/// * `error` - The error to handle
/// * `attempt` - The current attempt number (for backoff calculation)
/// * `service_name` - Name of the service (for logging)
///
/// # Returns
/// `Some(new_attempt)` if the error is recoverable and caller should retry
/// `None` if the error is unrecoverable (this function will exit the process)
pub async fn handle_error_with_backoff<E: ErrorClassification + std::fmt::Display>(
    error: &E,
    attempt: u32,
    service_name: &str,
) -> Option<u32> {
    match classify_error(error) {
        ErrorAction::Retry => {
            tracing::error!("{} recoverable error: {}", service_name, error);
            exponential_backoff_sleep(attempt).await;
            Some(attempt + 1)
        }
        ErrorAction::Exit => {
            tracing::error!("{} unrecoverable error: {}", service_name, error);
            tracing::error!("{} exiting due to unrecoverable error", service_name);
            std::process::exit(1);
        }
    }
}

/// Sleep with default exponential backoff parameters
///
/// This is a convenience function for the most common use case.
///
/// # Arguments
/// * `attempt` - The attempt number (0 for first retry, 1 for second, etc.)
///
/// # Example
/// ```rust
/// use shared::retry::exponential_backoff_sleep;
///
/// async fn retry_example() {
///     let mut attempt = 0;
///     loop {
///         match try_operation().await {
///             Ok(result) => return result,
///             Err(e) => {
///                 tracing::error!("Operation failed: {}", e);
///                 exponential_backoff_sleep(attempt).await;
///                 attempt += 1;
///             }
///         }
///     }
/// }
/// # async fn try_operation() -> Result<(), &'static str> { Err("test") }
/// ```
pub async fn exponential_backoff_sleep(attempt: u32) {
    ExponentialBackoff::default().sleep(attempt).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct RecoverableError;

    impl std::fmt::Display for RecoverableError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "recoverable error")
        }
    }

    impl ErrorClassification for RecoverableError {
        fn is_recoverable(&self) -> bool {
            true
        }
    }

    #[derive(Debug)]
    struct UnrecoverableError;

    impl std::fmt::Display for UnrecoverableError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unrecoverable error")
        }
    }

    impl ErrorClassification for UnrecoverableError {
        fn is_recoverable(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_error_classification() {
        let recoverable = RecoverableError;
        let unrecoverable = UnrecoverableError;

        assert_eq!(classify_error(&recoverable), ErrorAction::Retry);
        assert_eq!(classify_error(&unrecoverable), ErrorAction::Exit);

        assert!(recoverable.is_recoverable());
        assert!(!recoverable.is_unrecoverable());

        assert!(!unrecoverable.is_recoverable());
        assert!(unrecoverable.is_unrecoverable());
    }

    #[test]
    fn test_default_backoff_delays() {
        let backoff = ExponentialBackoff::default();

        // Test that delays increase exponentially (approximately)
        let delay0 = backoff.delay_for_attempt(0);
        let delay1 = backoff.delay_for_attempt(1);
        let delay2 = backoff.delay_for_attempt(2);

        assert!(delay0 >= Duration::from_millis(90)); // ~100ms with jitter
        assert!(delay0 <= Duration::from_millis(110));

        assert!(delay1 >= Duration::from_millis(180)); // ~200ms with jitter
        assert!(delay1 <= Duration::from_millis(220));

        assert!(delay2 >= Duration::from_millis(360)); // ~400ms with jitter
        assert!(delay2 <= Duration::from_millis(440));
    }

    #[test]
    fn test_max_delay_cap() {
        let backoff = ExponentialBackoff::new(
            Duration::from_secs(1),
            Duration::from_secs(5),
            2.0,
            0.0, // No jitter for predictable testing
        );

        let delay_high = backoff.delay_for_attempt(10); // Should be capped
        assert_eq!(delay_high, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_exponential_backoff_sleep() {
        let start = std::time::Instant::now();
        exponential_backoff_sleep(0).await;
        let elapsed = start.elapsed();

        // Should be approximately 100ms (with some tolerance for jitter and timing)
        assert!(elapsed >= Duration::from_millis(80));
        assert!(elapsed <= Duration::from_millis(150));
    }
}
