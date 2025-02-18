//! Common tracing module for Metabased Translator
//!
//! This module initializes the tracing configuration for the Metabased Translator, and by extension
//! other crates used in the Metabased Translator as well.

use core::fmt;
use std::{error::Error, fmt::Display};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt as subscriber_fmt,
    fmt::{
        format::{Format, JsonFields, Writer},
        FmtContext, FormatEvent, FormatFields,
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

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

/// A wrapper around a JSON formatter that iterates `extra_fields` and appends them all to every
/// log event.
struct CustomJsonFormatter<F> {
    inner: F,
    /// Extra fields to append as (key, value) pairs.
    extra_fields: Vec<(String, serde_json::Value)>,
}

impl<S, N, F> FormatEvent<S, N> for CustomJsonFormatter<F>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static, // Inner field formatter (e.g. JSON)
    F: FormatEvent<S, N>,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Get the original JSON output
        let mut buf = String::new();
        let buf_writer = Writer::new(&mut buf);
        self.inner.format_event(ctx, buf_writer, event)?;

        // Parse the original JSON
        let mut json: serde_json::Value = serde_json::from_str(&buf).map_err(|_| fmt::Error)?;

        // Add extra fields to the JSON object
        if let serde_json::Value::Object(ref mut map) = json {
            for (key, value) in &self.extra_fields {
                map.insert(key.clone(), value.clone());
            }
        }

        // Write the modified JSON with a newline
        writeln!(writer, "{}", json)?;
        Ok(())
    }
}

/// Initializes a default tracing subscriber with additional fields to include in every log event
pub fn init_tracing_with_extra_fields(
    extra_fields: Vec<(String, serde_json::Value)>,
) -> Result<(), TracingError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let json_formatter = Format::default().json().with_target(true);

    let custom_formatter = CustomJsonFormatter { inner: json_formatter, extra_fields };

    let fmt_layer =
        subscriber_fmt::layer().event_format(custom_formatter).fmt_fields(JsonFields::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
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
    fn test_init_tracing_with_extra_fields() {
        let extra_fields = vec![
            ("chain_id".to_string(), serde_json::json!(555)),
            ("env".to_string(), serde_json::json!("my_computer")),
        ];
        let result = init_tracing_with_extra_fields(extra_fields);
        assert!(result.is_ok());
    }
}
