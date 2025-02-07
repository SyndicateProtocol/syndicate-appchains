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
    fmt::{
        format::{Format, Writer},
        FmtContext, FormatEvent, FormatFields, Formatter,
    },
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

impl<'a> JsonVisitor<'a> {
    pub fn new(fields: &'a mut BTreeMap<String, serde_json::Value>, chain_id: u64) -> Self {
        let visitor = Self { fields, chain_id };
        visitor.fields.insert("chain_id".to_string(), serde_json::json!(chain_id));
        visitor
    }
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
        struct ChainIdVisitor(u64);

        impl tracing::field::Visit for ChainIdVisitor {
            fn record_u64(&mut self, field: &tracing::field::Field, _: u64) {
                // We only want to add our chain_id value
                if field.name() == "chain_id" {
                    let x = self.0;
                    self.record_debug(field, &x)
                }
            }

            fn record_debug(&mut self, field: &tracing::field::Field, _: &dyn fmt::Debug) {
                // Implementation needed for the trait but we don't use it
                if field.name() == "chain_id" {
                    let x = self.0;
                    self.record_debug(field, &x)
                }
            }
        }

        let mut visitor = ChainIdVisitor(self.chain_id);
        event.record(&mut visitor);
    }
}

// impl<S> Layer<S> for ChainIdLayer
// where
//     S: Subscriber,
// {
//     fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
//         let mut fields = BTreeMap::new();
//
//         // Add chain_id first
//         fields.insert("chain_id".to_string(), serde_json::json!(self.chain_id));
//
//         // Then visit all other fields
//         // let mut visitor = JsonVisitor { fields: &mut fields, chain_id: self.chain_id };
//         let mut visitor = JsonVisitor::new(&mut fields, self.chain_id);
//         event.record(&mut visitor);
//
//         // Let the standard JSON formatter handle the rest
//         // Output the event in JSON
//         // let output = serde_json::json!({
//         //     "target": event.metadata().target(),
//         //     "name": event.metadata().name(),
//         //     "level": format!("{:?}", event.metadata().level()),
//         //     "fields": fields,
//         // });
//         // #[allow(clippy::expect_used)]
//         // println!("{}", output);
//     }
//     // fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
//     //     // Just add the chain_id to the event
//     //     let mut fields = BTreeMap::new();
//     //     fields.insert("chain_id".to_string(), serde_json::json!(self.chain_id));
//     //
//     //     // Then visit all other fields
//     //     let mut visitor = JsonVisitor { fields: &mut fields, chain_id: self.chain_id };
//     //     event.record(&mut visitor);
//     // }
// }

/// A wrapper around a JSON formatter that appends `chain_id` to every event.
struct ChainIdJsonFormatter<F> {
    inner: F,
    chain_id: u64,
}

impl<S, N, F> FormatEvent<S, N> for ChainIdJsonFormatter<F>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    // N is the inner field formatter (for JSON, for example)
    N: for<'a> FormatFields<'a> + 'static,
    F: FormatEvent<S, N>,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        /// Create a temporary buffer.
        let mut buf = String::new();
        {
            // Wrap the temporary buffer in a Writer.
            let mut buf_writer = Writer::new(&mut buf);
            // Use the inner formatter to write the event.
            self.inner.format_event(ctx, buf_writer, event)?;
        }

        // Now, insert the "chain_id" field into the JSON output.
        // This example assumes the JSON output is an object that ends with `}` (or "}\n").
        if buf.ends_with("}\n") {
            buf.truncate(buf.len() - 2); // Remove "}\n"
            write!(writer, "{}, \"chain_id\": {} }}\n", buf, self.chain_id)
        } else if buf.ends_with('}') {
            buf.pop(); // Remove the final '}'
            write!(writer, "{}, \"chain_id\": {} }}", buf, self.chain_id)
        } else {
            // Fallback: just write the original output.
            writer.write_str(&buf)
        }
    }
}

/// TODO docs
pub fn init_tracing_with_chain(chain_id: u64) -> Result<(), TracingError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // but it shows how you might use additional layers).
    let chain_id_layer = ChainIdLayer::new(chain_id);

    // Create the inner JSON formatter by wrapping Json in the Format adapter.
    let json_formatter = Format::default().json().with_target(true);

    // Wrap the JSON formatter in our custom formatter that appends the chain_id field.
    let chain_formatter = ChainIdJsonFormatter { inner: json_formatter, chain_id };

    // Build the formatting layer using our custom event formatter.
    let fmt_layer = subscriber_fmt::layer().event_format(chain_formatter);

    // subscriber_fmt()
    //     .json()
    //     .with_target(true)
    //     .with_env_filter(env_filter)
    //     .with(chain_id_layer)
    //     .try_init()
    //     .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;

    tracing_subscriber::registry()
        // .with(chain_id_layer)
        .with(fmt_layer)
        .with(env_filter)
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
