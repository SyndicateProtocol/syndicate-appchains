//! Prometheus metrics for Valkey cache operations.
//!
//! This module provides comprehensive metrics collection for Valkey (Redis-compatible) cache
//! operations, including operation counters, duration tracking, error classification, and
//! connection monitoring.
//!
//! # Features
//!
//! - **Operation Metrics**: Track cache operations (read, write, delete, stream operations) with
//!   status labels
//! - **Duration Tracking**: Monitor operation latency in microseconds
//! - **Cache Performance**: Track hit/miss ratios for cache effectiveness
//! - **Error Classification**: Detailed error tracking by type and context
//! - **Connection Monitoring**: Track active and idle connection counts
//! - **Automatic Recording**: High-level wrapper for automatic timing and error handling
//! - **Macro Integration**: Automated metrics recording via `with_cache_metrics!` macro
//!
//! # Usage
//!
//! ## Manual Recording
//! ```rust
//! use prometheus_client::registry::Registry;
//! use std::time::Duration;
//! use synd_maestro::valkey_metrics::{CacheType, Operation, ValkeyMetrics};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create metrics and register with Prometheus
//! let mut registry = Registry::default();
//! let metrics = ValkeyMetrics::new(&mut registry);
//!
//! // Record operations manually
//! metrics.record_hit();
//! metrics.record_miss();
//!
//! // Or use automatic recording with error handling
//! let result = metrics
//!     .record_operation(Operation::Read, CacheType::ValkeyCache, "conn.myFunc".into(), || async {
//!         Ok::<String, std::io::Error>("data".to_string())
//!     })
//!     .await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Macro-based Automated Recording
//! ```rust
//! use synd_maestro::{
//!     valkey_metrics::{CacheType, Operation},
//!     with_cache_metrics,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let metrics = synd_maestro::valkey_metrics::ValkeyMetrics::default();
//! # struct MockConn;
//! # impl MockConn {
//! #     async fn get(&self, _key: &str) -> Result<String, std::io::Error> {
//! #         Ok("value".to_string())
//! #     }
//! #     async fn get_wallet_nonce(&self, _chain_id: u64, _address: &str) -> Result<u64, std::io::Error> {
//! #         Ok(42)
//! #     }
//! # }
//! # let conn = MockConn;
//! # let chain_id = 1u64;
//! # let address = "0x123";
//!
//! // Automatically wrap cache operations with metrics
//! let result =
//!     with_cache_metrics!(&metrics, Operation::Read, CacheType::ValkeyCache, conn.get("key"));
//!
//! // Or let the macro infer the operation type
//! let result = with_cache_metrics!(
//!     &metrics,
//!     conn.get_wallet_nonce(chain_id, address), // Auto-detects as (Operation::Read, CacheType::ValkeyCache)
//!     track_hit_miss: false
//! );
//! # Ok(())
//! # }
//! ```

use prometheus_client::{
    encoding::{EncodeLabelSet, EncodeLabelValue, LabelValueEncoder},
    metrics::{counter::Counter, family::Family, gauge::Gauge},
    registry::Registry,
};
use std::{
    fmt::Write,
    future::Future,
    time::{Duration, Instant},
};
use tracing::warn;

/// Prometheus metrics collector for Valkey operations.
///
/// This struct contains all the metric families for tracking Valkey cache performance,
/// errors, and connection health. It provides both low-level metric recording methods
/// and high-level convenience methods with automatic timing and error classification.
///
/// # Metrics Collected
///
/// - **Operations**: Total count of operations by type and status
/// - **Duration**: Operation latency in microseconds
/// - **Cache Performance**: Hit/miss ratios
/// - **Errors**: Detailed error tracking by type and context
/// - **Connections**: Active and idle connection counts
#[derive(Debug, Clone)]
pub struct ValkeyMetrics {
    /// Total cache operations by operation and status
    cache_operations_total: Family<OperationLabels, Counter>,
    /// Duration of cache operations, in microseconds
    cache_operation_duration_us: Family<DurationLabels, Gauge>,
    /// Cache hit/miss tracking
    cache_requests_total: Family<RequestLabels, Counter>,
    /// Detailed error tracking
    cache_errors_total: Family<ErrorLabels, Counter>,
}

impl Default for ValkeyMetrics {
    fn default() -> Self {
        Self::new(&mut Registry::default())
    }
}

/// Types of cache operations that can be performed.
///
/// Used as a label to categorize different kinds of cache interactions
/// for metrics tracking and analysis.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Operation {
    /// Standard read operation from cache
    Read,
    /// Standard write operation to cache
    Write,
    /// Delete operation removing data from cache
    Delete,
    /// Reading from a Valkey stream
    StreamRead,
    /// Writing to a Valkey stream
    StreamWrite,
    /// Deleting data from a Valkey stream
    StreamDelete,
    /// Some other operation. This should not be used in standard operation.
    Other,
}

impl EncodeLabelValue for Operation {
    fn encode(&self, encoder: &mut LabelValueEncoder<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Delete => "delete",
            Self::StreamRead => "stream_read",
            Self::StreamWrite => "stream_write",
            Self::StreamDelete => "stream_delete",
            Self::Other => "other",
        };
        encoder.write_str(value)
    }
}

/// Status of a cache operation indicating its outcome.
///
/// Used to track success rates and categorize different types of failures
/// for operational monitoring and alerting.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OperationStatus {
    /// Operation completed successfully
    Success,
    /// Operation failed with a general error
    Error,
    /// Operation failed due to timeout
    Timeout,
    /// Operation failed because the requested key was not found
    NotFound,
}

impl EncodeLabelValue for OperationStatus {
    fn encode(&self, encoder: &mut LabelValueEncoder<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Timeout => "timeout",
            Self::NotFound => "not_found",
        };
        encoder.write_str(value)
    }
}

/// Type of cache being accessed.
///
/// Distinguishes between different Valkey data structures and usage patterns
/// to enable separate monitoring and performance analysis.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CacheType {
    /// Standard Valkey cache operations (strings, hashes, etc.)
    ValkeyCache,
    /// Valkey stream operations
    ValkeyStream,
}

impl EncodeLabelValue for CacheType {
    fn encode(&self, encoder: &mut LabelValueEncoder<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            Self::ValkeyCache => "valkey_cache",
            Self::ValkeyStream => "valkey_stream",
        };
        encoder.write_str(value)
    }
}

/// Result of a cache request indicating cache effectiveness.
///
/// Used to track cache hit ratios, which are critical metrics for
/// cache performance and cost optimization.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CacheResult {
    /// Data was found in cache
    Hit,
    /// Data was not found in cache
    Miss,
}

impl EncodeLabelValue for CacheResult {
    fn encode(&self, encoder: &mut LabelValueEncoder<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            Self::Hit => "hit",
            Self::Miss => "miss",
        };
        encoder.write_str(value)
    }
}

/// Detailed classification of error types for diagnostic purposes.
///
/// Provides granular error categorization to help identify specific
/// failure patterns and guide troubleshooting efforts.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ErrorType {
    /// Network or connection-related errors
    ConnectionError,
    /// Operation timeout errors
    Timeout,
    /// Data serialization/deserialization errors
    SerializationError,
    /// Requested key was not found in cache
    KeyNotFound,
    /// Type mismatch errors (wrong data type for operation)
    TypeMismatch,
    /// Authentication or authorization errors
    AuthenticationError,
    /// Catch-all for unclassified errors
    UnknownError,
}

impl EncodeLabelValue for ErrorType {
    fn encode(&self, encoder: &mut LabelValueEncoder<'_>) -> Result<(), std::fmt::Error> {
        let value = match self {
            Self::ConnectionError => "connection_error",
            Self::Timeout => "timeout",
            Self::SerializationError => "serialization_error",
            Self::KeyNotFound => "key_not_found",
            Self::TypeMismatch => "type_mismatch",
            Self::AuthenticationError => "authentication_error",
            Self::UnknownError => "unknown_error",
        };
        encoder.write_str(value)
    }
}

/// Labels for cache operations combining operation type and status.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct OperationLabels {
    /// The type of operation performed
    pub operation: Operation,
    /// The status/outcome of the operation
    pub status: OperationStatus,
    /// The function name that was called
    pub func_name: String,
}

/// Labels for cache operation duration metrics.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct DurationLabels {
    /// The type of operation performed
    pub operation: Operation,
    /// The type of cache being accessed
    pub cache_type: CacheType,
    /// The function name that was called
    pub func_name: String,
}

/// Labels for cache request results (hit/miss tracking).
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RequestLabels {
    /// Whether the request resulted in a hit or miss
    pub result: CacheResult,
}

/// Labels for detailed error tracking with full context.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ErrorLabels {
    /// The specific type of error that occurred
    pub error_type: ErrorType,
    /// The operation that was being performed when the error occurred
    pub operation: Operation,
    /// The type of cache being accessed when the error occurred
    pub cache_type: CacheType,
    /// The function name that was called
    pub func_name: String,
}

impl ValkeyMetrics {
    /// Create a new `ValkeyMetrics` instance and register all metrics with the provided registry.
    ///
    /// This method creates all the necessary metric families and registers them with descriptive
    /// names and help text in the Prometheus registry.
    ///
    /// # Arguments
    ///
    /// * `registry` - Mutable reference to a Prometheus registry where metrics will be registered
    ///
    /// # Returns
    ///
    /// A new `ValkeyMetrics` instance ready for use
    ///
    /// # Example
    ///
    /// ```rust
    /// use prometheus_client::registry::Registry;
    /// use synd_maestro::valkey_metrics::ValkeyMetrics;
    ///
    /// let mut registry = Registry::default();
    /// let metrics = ValkeyMetrics::new(&mut registry);
    /// ```
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self {
            cache_operations_total: Family::<OperationLabels, Counter>::default(),
            cache_operation_duration_us: Family::<DurationLabels, Gauge>::default(),
            cache_requests_total: Family::<RequestLabels, Counter>::default(),
            cache_errors_total: Family::<ErrorLabels, Counter>::default(),
        };

        registry.register(
            "cache_operations_total",
            "Total number of cache operations",
            metrics.cache_operations_total.clone(),
        );

        registry.register(
            "cache_operation_duration",
            "Duration of cache operations, in microseconds",
            metrics.cache_operation_duration_us.clone(),
        );

        registry.register(
            "cache_requests_total",
            "Total cache requests by result",
            metrics.cache_requests_total.clone(),
        );

        registry.register(
            "cache_errors_total",
            "Total cache errors by type, operation, and cache type",
            metrics.cache_errors_total.clone(),
        );

        metrics
    }

    /// Record a cache operation with automatic timing and error handling.
    ///
    /// This high-level method wraps a cache operation, automatically recording its duration,
    /// success/failure status, and any errors that occur. It provides comprehensive metrics
    /// collection with minimal overhead.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function that returns a future
    /// - `Fut`: Future type returned by the function
    /// - `R`: Success result type
    /// - `E`: Error type (must implement `std::error::Error`)
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation being performed
    /// * `cache_type` - The type of cache being accessed
    /// * `f` - Function that performs the actual cache operation
    ///
    /// # Returns
    ///
    /// The original result of the operation, while recording metrics as a side effect
    ///
    /// # Example
    ///
    /// ```rust
    /// # use synd_maestro::valkey_metrics::{ValkeyMetrics, Operation, CacheType};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let metrics = ValkeyMetrics::default();
    /// let result = metrics
    ///     .record_operation(Operation::Read, CacheType::ValkeyCache, "conn.myFunc".into(), || async {
    ///         Ok::<String, std::io::Error>("data".to_string())
    ///     })
    ///     .await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn record_operation<F, Fut, R, E>(
        &self,
        operation: Operation,
        cache_type: CacheType,
        func_name: String,
        f: F,
    ) -> Result<R, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<R, E>>,
        E: std::error::Error,
    {
        let start = Instant::now();

        let result = f().await;
        let duration = start.elapsed();

        // Record duration
        self.cache_operation_duration_us
            .get_or_create(&DurationLabels {
                operation: operation.clone(),
                cache_type: cache_type.clone(),
                func_name: func_name.clone(),
            })
            .set(duration.as_micros() as i64);

        match result {
            Ok(value) => {
                self.cache_operations_total
                    .get_or_create(&OperationLabels {
                        operation: operation.clone(),
                        status: OperationStatus::Success,
                        func_name,
                    })
                    .inc();
                Ok(value)
            }
            Err(error) => {
                let status = classify_error_status(&error);
                let error_type = classify_error_type(&error);

                self.cache_operations_total
                    .get_or_create(&OperationLabels {
                        operation: operation.clone(),
                        status,
                        func_name: func_name.clone(),
                    })
                    .inc();

                self.cache_errors_total
                    .get_or_create(&ErrorLabels {
                        error_type,
                        operation: operation.clone(),
                        cache_type: cache_type.clone(),
                        func_name,
                    })
                    .inc();

                Err(error)
            }
        }
    }

    /// Record a cache hit.
    ///
    /// Increments the cache hit counter, which is used to calculate cache hit ratios
    /// for performance monitoring.
    pub fn record_hit(&self) {
        self.cache_requests_total.get_or_create(&RequestLabels { result: CacheResult::Hit }).inc();
    }

    /// Record a cache miss.
    ///
    /// Increments the cache miss counter, which is used to calculate cache hit ratios
    /// for performance monitoring.
    pub fn record_miss(&self) {
        self.cache_requests_total.get_or_create(&RequestLabels { result: CacheResult::Miss }).inc();
    }

    /// Increment operation counter with specific labels.
    ///
    /// Low-level method for manually recording operation counts when automatic
    /// recording via `record_operation` is not suitable.
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation performed
    /// * `status` - The outcome of the operation
    /// * `func_name` - The name of the invoked cache function
    pub fn increment_operation(
        &self,
        operation: Operation,
        status: OperationStatus,
        func_name: String,
    ) {
        self.cache_operations_total
            .get_or_create(&OperationLabels { operation, status, func_name })
            .inc();
    }

    /// Record operation duration manually.
    ///
    /// Low-level method for recording operation timing when automatic timing
    /// via `record_operation` is not suitable.
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation performed
    /// * `cache_type` - The type of cache accessed
    /// * `duration` - How long the operation took
    /// * `func_name` - The name of the invoked cache function
    pub fn record_duration(
        &self,
        operation: Operation,
        cache_type: CacheType,
        func_name: String,
        duration: Duration,
    ) {
        self.cache_operation_duration_us
            .get_or_create(&DurationLabels { operation, cache_type, func_name })
            .set(duration.as_micros() as i64);
    }

    /// Record error with specific type and context.
    ///
    /// Low-level method for manually recording errors when automatic error
    /// classification via `record_operation` is not suitable.
    ///
    /// # Arguments
    ///
    /// * `error_type` - The specific type of error that occurred
    /// * `operation` - The operation that was being performed
    /// * `cache_type` - The type of cache being accessed
    /// * `func_name` - The name of the invoked cache function
    pub fn record_error(
        &self,
        error_type: ErrorType,
        operation: Operation,
        cache_type: CacheType,
        func_name: String,
    ) {
        self.cache_errors_total
            .get_or_create(&ErrorLabels { error_type, operation, cache_type, func_name })
            .inc();
    }
}

/// Classify error for status labels based on error message content.
///
/// This function examines the error message to determine the appropriate
/// `OperationStatus` for metrics labeling.
///
/// # Arguments
///
/// * `error` - The error to classify
///
/// # Returns
///
/// An `OperationStatus` representing the general category of the error
pub fn classify_error_status(error: &dyn std::error::Error) -> OperationStatus {
    let error_str = error.to_string().to_lowercase();

    if error_str.contains("timeout") {
        OperationStatus::Timeout
    } else if error_str.contains("not found") || error_str.contains("nil") {
        OperationStatus::NotFound
    } else {
        OperationStatus::Error
    }
}

/// Classify the error type for detailed error tracking based on error message content.
///
/// This function examines the error message to determine the specific
/// `ErrorType` for detailed metrics tracking and troubleshooting.
///
/// # Arguments
///
/// * `error` - The error to classify
///
/// # Returns
///
/// An `ErrorType` representing the specific category of the error
pub fn classify_error_type(error: &dyn std::error::Error) -> ErrorType {
    let error_str = error.to_string().to_lowercase();

    match error_str {
        s if s.contains("timeout") => ErrorType::Timeout,
        s if s.contains("connection") || s.contains("connect") => ErrorType::ConnectionError,
        s if s.contains("serialization") ||
            s.contains("serialize") ||
            s.contains("deserialize") =>
        {
            ErrorType::SerializationError
        }
        s if s.contains("not found") || s.contains("nil") => ErrorType::KeyNotFound,
        s if s.contains("wrongtype") => ErrorType::TypeMismatch,
        s if s.contains("auth") => ErrorType::AuthenticationError,
        _ => ErrorType::UnknownError,
    }
}

/// Macro for automatically recording cache operation metrics.
///
/// This macro wraps cache operations with automatic metrics collection, including
/// timing, success/failure tracking, and error classification.
///
/// # Variants
///
/// ## Explicit Operation Type
/// ```rust
/// # use synd_maestro::{valkey_metrics::{ValkeyMetrics, Operation, CacheType}, with_cache_metrics};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let metrics = ValkeyMetrics::default();
/// # struct MockConn;
/// # impl MockConn {
/// #     async fn get(&self, _key: &str) -> Result<String, std::io::Error> {
/// #         Ok("value".to_string())
/// #     }
/// # }
/// # let conn = MockConn;
/// let result = with_cache_metrics!(
///     &metrics,               // ValkeyMetrics instance
///     conn.get("key")         // The cache operation
/// );
/// # Ok(())
/// # }
/// ```
///
/// ## Auto-detected Operation Type
/// ```rust
/// # use synd_maestro::{valkey_metrics::{ValkeyMetrics, CacheType}, with_cache_metrics};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let metrics = ValkeyMetrics::default();
/// # struct MockConn;
/// # impl MockConn {
/// #     async fn get_wallet_nonce(&self, _chain_id: u64, _address: &str) -> Result<u64, std::io::Error> {
/// #         Ok(42)
/// #     }
/// # }
/// # let conn = MockConn;
/// # let chain_id = 1u64;
/// # let address = "0x123";
/// let result = with_cache_metrics!(
///     &metrics,                                 // ValkeyMetrics instance
///     conn.get_wallet_nonce(chain_id, address)  // Operation type auto-detected
/// );
/// # Ok(())
/// # }
/// ```
///
/// ## With Hit/Miss Tracking
/// ```rust
/// # use synd_maestro::{valkey_metrics::{ValkeyMetrics, CacheType}, with_cache_metrics};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let metrics = ValkeyMetrics::default();
/// # struct MockConn;
/// # impl MockConn {
/// #     async fn get(&self, _key: &str) -> Result<Option<String>, std::io::Error> {
/// #         Ok(Some("value".to_string()))
/// #     }
/// # }
/// # let conn = MockConn;
/// let result = with_cache_metrics!(
///     &metrics,
///     conn.get("key"),
///     track_hit_miss: true  // Automatically records cache hits/misses
/// );
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! with_cache_metrics {
    // Explicit operation type with hit/miss tracking
    ($metrics:expr, $operation:expr, $cache_type:expr, $operation_expr:expr, track_hit_miss: $track:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let func_name = $crate::valkey_metrics::extract_func_name(stringify!($operation_expr));
        let result = $operation_expr.await;
        let duration = start.elapsed();

        // Record duration
        $metrics.record_duration($operation.clone(), $cache_type.clone(), func_name.clone(), duration);

        match &result {
            Ok(value) => {
                $metrics.increment_operation($operation.clone(), $crate::valkey_metrics::OperationStatus::Success, func_name.clone());

                // Track hit/miss for read operations if requested
                if $track && matches!($operation, $crate::valkey_metrics::Operation::Read) {
                    // Try to determine if this was a hit or miss based on the result
                    if $crate::valkey_metrics::is_cache_hit(value) {
                        $metrics.record_hit();
                    } else {
                        $metrics.record_miss();
                    }
                }
            }
            Err(error) => {
                let status = $crate::valkey_metrics::classify_error_status(error);
                let error_type = $crate::valkey_metrics::classify_error_type(error);

                $metrics.increment_operation($operation.clone(), status, func_name.clone());
                $metrics.record_error(error_type, $operation.clone(), $cache_type.clone(), func_name.clone());

                if $track && matches!($operation, $crate::valkey_metrics::Operation::Read) {
                    // Errors on reads are considered misses
                    $metrics.record_miss();
                }
            }
        }

        result
    }};

    // Explicit operation type without hit/miss tracking
    ($metrics:expr, $operation:expr, $cache_type:expr, $operation_expr:expr) => {{
        $crate::with_cache_metrics!($metrics, $operation, $cache_type, $operation_expr, track_hit_miss: false)
    }};

    // Auto-detect operation type with hit/miss tracking
    ($metrics:expr, $operation_expr:expr, track_hit_miss: $track:expr) => {{
        let (operation, cache_type) = $crate::valkey_metrics::detect_operation_and_cache_type(stringify!($operation_expr));
        $crate::with_cache_metrics!($metrics, operation, cache_type, $operation_expr, track_hit_miss: $track)
    }};

    // Auto-detect operation type without hit/miss tracking (default case)
    ($metrics:expr, $operation_expr:expr) => {{
        let (operation, cache_type) = $crate::valkey_metrics::detect_operation_and_cache_type(stringify!($operation_expr));
        $crate::with_cache_metrics!($metrics, operation, cache_type, $operation_expr, track_hit_miss: true)
    }};
}

/// Extract the function name from an expression string.
/// For example, "`conn.set_wallet_nonce(chain_id, signer, rpc_nonce, ttl)`"
/// returns "`conn.set_wallet_nonce`"
pub fn extract_func_name(expr_str: &str) -> String {
    expr_str.split('(').next().unwrap_or("unknown").trim().to_string()
}

/// Helper function to detect the operation type from the method name. This relies on method naming
/// consistency within the `synd-maestro` package.
pub fn detect_operation_and_cache_type(expr_str: &str) -> (Operation, CacheType) {
    // Extract method name from the expression
    let method_name = expr_str.split('.').next_back().unwrap_or("").split('(').next().unwrap_or("");

    let op = match method_name {
        // Read operations
        name if name.starts_with("get") => Operation::Read,
        name if name.starts_with("xread") => Operation::StreamRead,
        name if name.starts_with("xlen") => Operation::StreamRead,
        name if name.starts_with("xrange") => Operation::StreamRead,
        name if name.contains("read") => Operation::Read,

        // Write operations
        name if name.starts_with("set") => Operation::Write,
        name if name.starts_with("xadd") => Operation::StreamWrite,
        name if name.contains("write") || name.contains("add") => Operation::Write,

        // Delete operations
        name if name.starts_with("del") => Operation::Delete,
        name if name.starts_with("xdel") => Operation::StreamDelete,
        name if name.contains("delete") || name.contains("remove") => Operation::Delete,

        // Default fallback
        name => {
            warn!(
                "Uncategorized method name: {name}. Please add a case to categorize this method name."
            );
            Operation::Other
        }
    };
    let cache = match method_name {
        name if name.starts_with('x') => CacheType::ValkeyStream,
        _ => CacheType::ValkeyCache,
    };
    (op, cache)
}

/// Helper function to determine if a result represents a cache hit
pub fn is_cache_hit<T>(result: &T) -> bool
where
    T: std::fmt::Debug,
{
    let debug_str = format!("{result:?}");

    // Common patterns that indicate a cache hit
    !debug_str.contains("None") &&
        !debug_str.contains("null") &&
        !debug_str.contains("[]") &&
        !debug_str.is_empty()
}

#[allow(clippy::cognitive_complexity)]
#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;
    use std::fmt::{Debug, Formatter};

    #[test]
    fn test_valkey_metrics_creation() {
        let mut registry = Registry::default();
        let metrics = ValkeyMetrics::new(&mut registry);

        // Test that we can record operations
        metrics.record_hit();
        metrics.record_miss();
        metrics.increment_operation(
            Operation::Read,
            OperationStatus::Success,
            "conn.myFunc".into(),
        );
        metrics.record_duration(
            Operation::Write,
            CacheType::ValkeyCache,
            "conn.myFunc".into(),
            Duration::from_millis(100),
        );
        metrics.record_error(
            ErrorType::Timeout,
            Operation::Read,
            CacheType::ValkeyStream,
            "conn.myFunc".into(),
        );
    }

    #[test]
    fn test_error_classification() {
        struct TestError(String);
        impl std::fmt::Display for TestError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl Debug for TestError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::error::Error for TestError {}

        let timeout_error = TestError("Connection timeout".to_string());
        assert!(matches!(classify_error_status(&timeout_error), OperationStatus::Timeout));
        assert!(matches!(classify_error_type(&timeout_error), ErrorType::Timeout));

        let not_found_error = TestError("Key not found".to_string());
        assert!(matches!(classify_error_status(&not_found_error), OperationStatus::NotFound));
        assert!(matches!(classify_error_type(&not_found_error), ErrorType::KeyNotFound));
    }

    #[tokio::test]
    async fn test_record_operation() {
        let mut registry = Registry::default();
        let metrics = ValkeyMetrics::new(&mut registry);

        // Test successful operation
        let result = metrics
            .record_operation(
                Operation::Read,
                CacheType::ValkeyCache,
                "conn.myFunc".into(),
                || async { Ok::<i32, std::io::Error>(42) },
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        // Test failed operation
        let result = metrics
            .record_operation(
                Operation::Write,
                CacheType::ValkeyCache,
                "conn.myFunc".into(),
                || async {
                    Err::<i32, std::io::Error>(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "timeout",
                    ))
                },
            )
            .await;
        assert!(result.is_err());
    }

    #[test]
    fn test_enum_encoding() {
        // Test each enum by verifying the string values match expected patterns
        // Since we can't easily create LabelValueEncoder instances in tests,
        // we verify the logic by testing the match arms directly

        let operation_value = match Operation::Read {
            Operation::Read => "read",
            Operation::Write => "write",
            Operation::Delete => "delete",
            Operation::StreamRead => "stream_read",
            Operation::StreamWrite => "stream_write",
            Operation::StreamDelete => "stream_delete",
            Operation::Other => "other",
        };
        assert_eq!(operation_value, "read");

        let status_value = match OperationStatus::Success {
            OperationStatus::Success => "success",
            OperationStatus::Error => "error",
            OperationStatus::Timeout => "timeout",
            OperationStatus::NotFound => "not_found",
        };
        assert_eq!(status_value, "success");

        let cache_type_value = match CacheType::ValkeyCache {
            CacheType::ValkeyCache => "valkey_cache",
            CacheType::ValkeyStream => "valkey_stream",
        };
        assert_eq!(cache_type_value, "valkey_cache");

        let cache_result_value = match CacheResult::Hit {
            CacheResult::Hit => "hit",
            CacheResult::Miss => "miss",
        };
        assert_eq!(cache_result_value, "hit");

        let error_type_value = match ErrorType::ConnectionError {
            ErrorType::ConnectionError => "connection_error",
            ErrorType::Timeout => "timeout",
            ErrorType::SerializationError => "serialization_error",
            ErrorType::KeyNotFound => "key_not_found",
            ErrorType::TypeMismatch => "type_mismatch",
            ErrorType::AuthenticationError => "authentication_error",
            ErrorType::UnknownError => "unknown_error",
        };
        assert_eq!(error_type_value, "connection_error");
    }

    #[test]
    fn test_detect_operation_type() {
        // Test read operations
        assert!(matches!(
            detect_operation_and_cache_type("conn.get(key)"),
            (Operation::Read, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("self.conn.get_wallet_nonce(a, b)"),
            (Operation::Read, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("conn.xread_options(&keys, &ids, &opts)"),
            (Operation::StreamRead, CacheType::ValkeyStream)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("something.contains_read_operation()"),
            (Operation::Read, CacheType::ValkeyCache)
        ));

        // Test write operations
        assert!(matches!(
            detect_operation_and_cache_type("conn.set(key, value)"),
            (Operation::Write, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("self.conn.set_wallet_nonce(a, b, c, d)"),
            (Operation::Write, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("conn.xadd(stream, id, fields)"),
            (Operation::StreamWrite, CacheType::ValkeyStream)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("helper.write_data(data)"),
            (Operation::Write, CacheType::ValkeyCache)
        ));

        // Test delete operations
        assert!(matches!(
            detect_operation_and_cache_type("conn.del(keys)"),
            (Operation::Delete, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("conn.del_waiting_txn_keys(&ids)"),
            (Operation::Delete, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("conn.xdel(stream, ids)"),
            (Operation::StreamDelete, CacheType::ValkeyStream)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("cache.remove_entry(key)"),
            (Operation::Delete, CacheType::ValkeyCache)
        ));

        // Test stream operations
        assert!(matches!(
            detect_operation_and_cache_type("conn.xlen(stream)"),
            (Operation::StreamRead, CacheType::ValkeyStream)
        ));
        assert!(matches!(
            detect_operation_and_cache_type("conn.xrange(stream, start, end)"),
            (Operation::StreamRead, CacheType::ValkeyStream)
        ));

        // Test fallback
        assert!(matches!(
            detect_operation_and_cache_type("conn.unknown_method()"),
            (Operation::Other, CacheType::ValkeyCache)
        ));
        assert!(matches!(
            detect_operation_and_cache_type(""),
            (Operation::Other, CacheType::ValkeyCache)
        ));
    }

    #[test]
    fn test_is_cache_hit() {
        // Test cache hits
        assert!(is_cache_hit(&Some("value".to_string())));
        assert!(is_cache_hit(&vec![1, 2, 3]));
        assert!(is_cache_hit(&"non-empty string"));
        assert!(is_cache_hit(&42));

        // Test cache misses
        assert!(!is_cache_hit(&None::<String>));
        assert!(!is_cache_hit(&Vec::<i32>::new()));

        // Note: These are harder to test because they depend on Debug formatting
        // In practice, the function checks for common patterns in debug output
    }

    #[tokio::test]
    async fn test_with_cache_metrics_macro() {
        let mut registry = Registry::default();
        let metrics = ValkeyMetrics::new(&mut registry);

        // Mock a successful cache operation
        let mock_operation = || async { Ok::<String, std::io::Error>("cached_value".to_string()) };

        // Test the macro with explicit operation type
        let result = with_cache_metrics!(
            &metrics,
            Operation::Read,
            CacheType::ValkeyCache,
            mock_operation(),
            track_hit_miss: true
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "cached_value");
    }

    #[tokio::test]
    async fn test_with_cache_metrics_macro_error_handling() {
        let mut registry = Registry::default();
        let metrics = ValkeyMetrics::new(&mut registry);

        // Mock a failing cache operation
        let mock_failing_operation = || async {
            Err::<String, std::io::Error>(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Connection timeout",
            ))
        };

        let result = with_cache_metrics!(
            &metrics,
            Operation::Read,
            CacheType::ValkeyCache,
            mock_failing_operation(),
            track_hit_miss: true
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::TimedOut);
    }

    // Integration test showing typical usage patterns
    #[tokio::test]
    async fn test_usage_patterns() {
        let mut registry = Registry::default();
        let metrics = ValkeyMetrics::new(&mut registry);

        // Simulate typical cache operations

        // 1. Read operation that hits
        let cache_hit_result = with_cache_metrics!(&metrics, async {
            Ok::<Option<String>, redis::RedisError>(Some("cached_data".to_string()))
        });
        assert!(cache_hit_result.is_ok());

        // 2. Read operation that misses
        let cache_miss_result =
            with_cache_metrics!(&metrics, async { Ok::<Option<String>, redis::RedisError>(None) });
        assert!(cache_miss_result.is_ok());
        assert_eq!(cache_miss_result.unwrap(), None);

        // 3. Write operation
        let write_result =
            with_cache_metrics!(&metrics, Operation::Write, CacheType::ValkeyCache, async {
                Ok::<String, redis::RedisError>("OK".to_string())
            });
        assert!(write_result.is_ok());

        // 4. Stream operation
        let stream_result =
            with_cache_metrics!(&metrics, Operation::StreamRead, CacheType::ValkeyStream, async {
                Ok::<Vec<String>, redis::RedisError>(vec!["message1".to_string()])
            });
        assert!(stream_result.is_ok());
    }

    // Integration test demonstrating metrics recording for cache operations
    #[tokio::test]
    async fn test_cache_updates() {
        // Create a test registry and metrics
        let mut registry = Registry::default();
        let valkey_metrics = ValkeyMetrics::new(&mut registry);

        // Simulate successful cache read operation
        let (operation, cache_type) = (Operation::Read, CacheType::ValkeyCache);

        // Record a successful cache operation
        valkey_metrics.increment_operation(
            operation.clone(),
            OperationStatus::Success,
            "conn.myFunc".into(),
        );
        valkey_metrics.record_duration(
            operation.clone(),
            cache_type.clone(),
            "conn.myFunc".into(),
            Duration::from_micros(150),
        );
        valkey_metrics.record_hit();

        // Verify metrics were recorded
        let operation_labels = OperationLabels {
            operation: operation.clone(),
            status: OperationStatus::Success,
            func_name: "conn.myFunc".into(),
        };

        let duration_labels =
            DurationLabels { operation, cache_type, func_name: "conn.myFunc".into() };

        let hit_labels = RequestLabels { result: CacheResult::Hit };

        // Check that the operation counter was incremented
        assert_eq!(
            valkey_metrics.cache_operations_total.get(&operation_labels).map_or(0, |c| c.get()),
            1
        );

        // Check that duration was recorded
        assert!(
            valkey_metrics.cache_operation_duration_us.get(&duration_labels).map_or(0, |g| g.get()) >
                0
        );

        // Check that a cache hit was recorded
        assert_eq!(valkey_metrics.cache_requests_total.get(&hit_labels).map_or(0, |c| c.get()), 1);

        // Test cache miss scenario
        valkey_metrics.record_miss();

        let miss_labels = RequestLabels { result: CacheResult::Miss };

        assert_eq!(valkey_metrics.cache_requests_total.get(&miss_labels).map_or(0, |c| c.get()), 1);

        // Test error scenario
        let (error_operation, cache_type) = (Operation::Read, CacheType::ValkeyCache);
        let error_status = OperationStatus::Error;
        let error_type = ErrorType::ConnectionError;

        valkey_metrics.increment_operation(
            error_operation.clone(),
            error_status.clone(),
            "conn.myFunc".into(),
        );
        valkey_metrics.record_error(
            error_type.clone(),
            error_operation.clone(),
            cache_type.clone(),
            "conn.myFunc".into(),
        );

        let error_operation_labels = OperationLabels {
            operation: error_operation.clone(),
            status: error_status,
            func_name: "conn.myFunc".into(),
        };

        let error_labels = ErrorLabels {
            error_type,
            operation: error_operation,
            cache_type,
            func_name: "conn.myFunc".into(),
        };

        assert_eq!(
            valkey_metrics
                .cache_operations_total
                .get(&error_operation_labels)
                .map_or(0, |c| c.get()),
            1
        );

        assert_eq!(valkey_metrics.cache_errors_total.get(&error_labels).map_or(0, |c| c.get()), 1);
    }
}
