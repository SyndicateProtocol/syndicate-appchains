//! The `tracing` module contains code for setting up logs, tracing and metrics

use http::Extensions;
// Re-exports for usability without requiring additional dependencies
pub use opentelemetry::{
    global as otel_global,
    propagation::TextMapPropagator,
    trace::{SpanKind, Status as TraceStatus, TraceContextExt},
};
use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::{ExporterBuildError, SpanExporter as OtlpSpanExporter};
pub use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use opentelemetry_stdout::SpanExporter as StdoutSpanExporter;
use std::collections::HashMap;
use thiserror::Error;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetryLayer;
pub use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{
    filter, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter,
};

/// Configuration for the tracing system
#[derive(Debug, Clone)]
pub struct ServiceTracingConfig {
    /// Name of the service
    service_name: String,
    /// Version of the service
    version: String,
    /// Deployment environment (e.g., "development", "production")
    env: String,
    /// Whether to print all logs to standard output
    debug: bool,
}

impl ServiceTracingConfig {
    /// Create a new `ServiceTracingConfig` instance, with `env`
    /// from `OTEL_DEPLOYMENT_ENVIRONMENT_NAME` env var.
    pub fn from_env(service_name: &str, version: &str) -> Self {
        let env = std::env::var("OTEL_DEPLOYMENT_ENVIRONMENT_NAME")
            .unwrap_or_else(|_| "development".to_string());
        let debug = std::env::var("OTEL_DEBUG").is_ok();

        Self { service_name: service_name.to_string(), version: version.to_string(), env, debug }
    }
}

impl From<ServiceTracingConfig> for Resource {
    fn from(config: ServiceTracingConfig) -> Self {
        Self::builder()
            .with_service_name(config.service_name)
            .with_schema_url(
                [
                    KeyValue::new(SERVICE_VERSION, config.version),
                    KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, config.env),
                ],
                SCHEMA_URL,
            )
            .build()
    }
}

// Construct `TracerProvider` for `OpenTelemetryLayer`
fn init_tracer_provider(config: &ServiceTracingConfig) -> Result<SdkTracerProvider, Error> {
    let otlp_exporter =
        OtlpSpanExporter::builder().with_http().build().map_err(Error::SpanExporter)?;

    let provider = SdkTracerProvider::builder()
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::from(config.clone()))
        .with_batch_exporter(otlp_exporter);
    let provider = if config.debug {
        provider.with_simple_exporter(StdoutSpanExporter::default())
    } else {
        provider
    }
    .build();

    opentelemetry::global::set_tracer_provider(provider.clone());

    Ok(provider)
}

/// Initialize tracing-subscriber and return [`OtelGuard`]
/// for OpenTelemetry-related termination processing.
pub fn setup_global_tracing(config: ServiceTracingConfig) -> Result<OtelGuard, Error> {
    let tracer_provider = init_tracer_provider(&config)?;

    let tracer = tracer_provider.tracer("tracing-opentelemetry");
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let env_filter = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .from_env()?
        // disable spammy and unconnected jsonrpsee_server `connection` spans
        .add_directive("jsonrpsee_server=off".parse()?);

    let disable_json = std::env::var("RUST_LOG_DISABLE_JSON").is_ok();
    let disable_telemetry = std::env::var("RUST_LOG_DISABLE_TELEMETRY").is_ok();

    let fmt_layer = tracing_subscriber::fmt::layer()
        // include codepath origin of log
        .with_target(true)
        .with_test_writer();

    let tracing_subscriber = tracing_subscriber::registry().with(env_filter);

    match (disable_telemetry, disable_json) {
        (true, true) => tracing_subscriber.with(fmt_layer).try_init(),
        (true, false) => tracing_subscriber.with(fmt_layer.json()).try_init(),
        (false, true) => tracing_subscriber.with(OpenTelemetryLayer::new(tracer)).try_init(),
        (false, false) => tracing_subscriber
            .with(fmt_layer.json())
            .with(OpenTelemetryLayer::new(tracer))
            .try_init(),
    }
    .map_err(|e| Error::DefaultLoggerInit(e.to_string()))?;

    Ok(OtelGuard {
        tracer_provider,
        // meter_provider
    })
}

/// A shorthand to set up a subscriber for tests,
/// bypassing tracing/metrics initialization.
pub fn setup_global_logging() -> Result<(), Error> {
    let env_filter =
        EnvFilter::builder().with_default_directive(filter::LevelFilter::INFO.into()).from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_test_writer()
        .try_init()
        .map_err(|e| Error::DefaultLoggerInit(e.to_string()))
}

/// Extract the current tracing context from the global OpenTelemetry propagator.
pub fn current_traceparent() -> Option<String> {
    let context = Span::current().context();
    let mut carrier = HashMap::new();
    otel_global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&context, &mut carrier)
    });
    carrier.get("traceparent").cloned()
}

/// Extract the tracing context from the request headers
/// and set it as the parent context for the current span.
pub fn extract_tracing_context(extensions: &Extensions) -> Option<()> {
    let fallback_map = HashMap::<String, String>::new();
    let headers = extensions.get::<HashMap<_, _>>().unwrap_or(&fallback_map);
    let traceparent = headers.get("traceparent")?;
    let mut carrier = HashMap::new();
    // TODO(SEQ-973): '-03' sent by universal-relay is incompatible with Rust TraceContextPropagator
    carrier.insert("traceparent".to_string(), traceparent.replace("-03", "-01"));
    let parent_context =
        otel_global::get_text_map_propagator(|propagator| propagator.extract(&carrier));
    Span::current().set_parent(parent_context);
    Some(())
}

/// A guard that ensures the OpenTelemetry SDK is sending spans/metrics
/// while it is held, and properly shut down when dropped.
#[derive(Debug)]
pub struct OtelGuard {
    tracer_provider: SdkTracerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        // Can't handle errors in Drop, just log them
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("failed to shutdown OpenTelemetry tracing: {err:?}");
        }
    }
}

/// Errors relating to the logger
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    /// error parsing `RUST_LOG` variable
    #[error("unable to get env filter from RUST_LOG env variable: {0}")]
    EnvFilter(#[from] filter::FromEnvError),
    /// error parsing filtering directive
    #[error("failed to parse filtering directive: {0}")]
    ParseFilter(#[from] filter::ParseError),
    /// error initializing the default logger
    #[error("unable to initialize default logger - did you call this more than once?: {0} ")]
    DefaultLoggerInit(String),
    /// error building the span exporter
    #[error("unable to build span exporter: {0}")]
    SpanExporter(#[from] ExporterBuildError),
}
