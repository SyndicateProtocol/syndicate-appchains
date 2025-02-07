//! Common tracing module for Metabased Translator
//!
//! This module initializes the tracing configuration for the Metabased Translator, and by extension
//! other crates used in the Metabased Translator as well.

use core::fmt;
use std::{collections::BTreeMap, error::Error, fmt::Display};
use tracing::{instrument::WithSubscriber, Event, Level, Subscriber};
use tracing_subscriber::{
    filter::FilterExt,
    fmt as subscriber_fmt,
    fmt::{FormatEvent, FormatFields},
    layer::{Context, SubscriberExt},
    util::SubscriberInitExt,
    EnvFilter, Layer,
};
// TODO(SEQ-515): Reconsider location, put me in `bin` ?

#[allow(missing_docs)]
#[derive(Debug)]
pub enum TracingError {
    SubscriberInit(String),
}

impl Display for TracingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubscriberInit(msg) => {
                write!(f, "Failed to initialize subscriber: {}", msg)
            }
        }
    }
}

impl Error for TracingError {}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ChainIdLayer {
    chain_id: u64,
}

#[allow(missing_docs)]
impl ChainIdLayer {
    pub fn new(chain_id: u64) -> Self {
        Self { chain_id }
    }
}

struct JsonVisitor<'a> {
    fields: &'a mut BTreeMap<String, serde_json::Value>,
    chain_id: u64,
}

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.fields.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.fields.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.fields.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.fields.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.fields.insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(&mut self, field: &tracing::field::Field, value: &(dyn Error + 'static)) {
        self.fields.insert(field.name().to_string(), serde_json::json!(value.to_string()));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(field.name().to_string(), serde_json::json!(format!("{:?}", value)));
    }
}

impl<S> Layer<S> for ChainIdLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut fields = BTreeMap::new();

        // Add chain_id first
        fields.insert("chain_id".to_string(), serde_json::json!(self.chain_id));

        // Then visit all other fields
        let mut visitor = JsonVisitor { fields: &mut fields, chain_id: self.chain_id };
        event.record(&mut visitor);

        // Let the standard JSON formatter handle the rest
        // Output the event in JSON
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "level": format!("{:?}", event.metadata().level()),
            "fields": fields,
        });
        // #[allow(clippy::expect_used)]
        println!("{}", output);
    }
}

/// TODO docs
pub fn init_tracing_with_chain(chain_id: u64) -> Result<(), TracingError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let chain_id_layer = ChainIdLayer::new(chain_id);

    let fmt_layer = subscriber_fmt::layer().json().with_target(true);

    // subscriber_fmt()
    //     .json()
    //     .with_target(true)
    //     .with_env_filter(env_filter)
    //     .with(chain_id_layer)
    //     .try_init()
    //     .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
        .with(chain_id_layer)
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;

    Ok(())
}

/// Initializes a tracing subscriber
pub fn init_tracing() -> Result<(), TracingError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    subscriber_fmt()
        .json()
        .with_target(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;

    Ok(())
}

/// Initializes a tracing subscriber for testing purposes
pub fn init_test_tracing(level: Level) -> Result<(), TracingError> {
    subscriber_fmt()
        .with_env_filter(EnvFilter::new(level.to_string()))
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tracing() {
        let result = init_tracing();
        assert!(result.is_ok());
    }
}
