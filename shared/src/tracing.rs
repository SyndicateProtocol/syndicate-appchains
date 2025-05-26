//! The `tracing` module contains code for setting up logs, tracing and metrics

use std::collections::HashMap;

use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::{
    ExporterBuildError, MetricExporter as OtlpMetricExporter, SpanExporter as OtlpSpanExporter,
};
use opentelemetry_sdk::{
    metrics::{MeterProviderBuilder, SdkMeterProvider, Temporality},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use opentelemetry_stdout::SpanExporter as StdoutSpanExporter;
use thiserror::Error;
use tracing::Span;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{
    filter::{FromEnvError, LevelFilter, Targets},
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
    EnvFilter,
};

pub use opentelemetry::global as otel_global;
pub use opentelemetry::propagation::TextMapPropagator;
pub use opentelemetry::trace::Status as TraceStatus;
pub use opentelemetry_sdk::propagation::TraceContextPropagator;
pub use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Percentage of traces to capture when debugging locally.
pub const DEBUG_SAMPLING_RATE: f64 = 1.0;

/// Percentage of traces to capture from a deployed service.
pub const SAMPLING_RATE: f64 = 0.1;

/// Configuration for the tracing system
#[derive(Debug, Clone)]
pub struct ServiceTracingConfig {
    /// Name of the service
    service_name: String,
    /// Version of the service
    version: String,
    /// Deployment environment (e.g., "development", "production")
    env: String,
    /// Whether to sample all logs and print to stdout
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

// Construct MeterProvider for MetricsLayer
fn init_meter_provider(config: &ServiceTracingConfig) -> Result<SdkMeterProvider, Error> {
    let otlp_exporter = OtlpMetricExporter::builder()
        .with_http()
        .with_temporality(Temporality::default())
        .build()
        .map_err(Error::SpanExporter)?;

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(Resource::from(config.clone()))
        .with_periodic_exporter(otlp_exporter)
        .build();

    opentelemetry::global::set_meter_provider(meter_provider.clone());

    Ok(meter_provider)
}

// Construct `TracerProvider` for `OpenTelemetryLayer`
fn init_tracer_provider(config: &ServiceTracingConfig) -> Result<SdkTracerProvider, Error> {
    let otlp_exporter =
        OtlpSpanExporter::builder().with_http().build().map_err(Error::SpanExporter)?;

    let sampling_ratio = if config.debug { DEBUG_SAMPLING_RATE } else { SAMPLING_RATE };
    let sampler = Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(sampling_ratio)));

    let provider = SdkTracerProvider::builder()
        .with_sampler(sampler)
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
    // let meter_provider = init_meter_provider(&config)?;

    let tracer = tracer_provider.tracer("tracing-opentelemetry");
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let env_filter =
        EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env()?;

    tracing_subscriber::registry()
        // some sane defaults
        .with(
            Targets::new()
                // TODO: will this conflict with env_filter? is printing DEBUG logs possible?
                .with_default(LevelFilter::INFO)
                // disable spammy and unconnected jsonrpsee_server `connection` spans
                .with_target("jsonrpsee_server", LevelFilter::OFF),
        )
        // logging to stdout
        .with(
            tracing_subscriber::fmt::layer()
                // output in JSON format
                .json()
                // include codepath origin of log
                .with_target(true)
                .with_test_writer(),
        )
        .with(env_filter)
        // OpenTelemetry tracing + metrics layers
        // .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        .try_init()
        .map_err(|e| Error::DefaultLoggerInit(e.to_string()))?;

    Ok(OtelGuard {
        tracer_provider,
        // meter_provider
    })
}

/// A shorthand to set up a subscriber for tests,
/// bypassing tracing/metrics initialization.
pub fn setup_test_logging() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_target(true)
        .with_test_writer()
        .try_init()
        .map_err(|e| Error::DefaultLoggerInit(e.to_string()))
}

/// Extract the tracing context from the request headers
/// and set it as the parent context for the current span.
pub fn current_traceparent() -> Option<String> {
    let mut carrier = HashMap::new();
    otel_global::get_text_map_propagator(|propagator| propagator.inject(&mut carrier));
    carrier.get("traceparent").cloned()
}

/// Extract the tracing context from the request headers
/// and set it as the parent context for the current span.
pub fn extract_tracing_context(headers: &HashMap<String, String>) -> Option<()> {
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
    // meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        // Can't handle errors in Drop, just log them
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("failed to shutdown OpenTelemetry tracing: {err:?}");
        }
        // if let Err(err) = self.meter_provider.shutdown() {
        //     eprintln!("failed to shutdown OpenTelemetry metrics: {err:?}");
        // }
    }
}

/// Errors relating to the logger
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    /// error parsing `RUST_LOG` variable
    #[error("unable to get env filter from RUST_LOG env variable: {0}")]
    EnvFilter(#[from] FromEnvError),
    /// error initializing the default logger
    #[error("unable to initialize default logger - did you call this more than once?: {0} ")]
    DefaultLoggerInit(String),
    /// error building the span exporter
    #[error("unable to build span exporter: {0}")]
    SpanExporter(#[from] ExporterBuildError),
}
