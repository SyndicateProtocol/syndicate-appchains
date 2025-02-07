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
        format::{Format, Writer},
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
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Create a temporary buffer.
        let mut buf = String::new();
        {
            // Wrap the temporary buffer in a Writer.
            let buf_writer = Writer::new(&mut buf);
            // Use the inner formatter to write the event.
            self.inner.format_event(ctx, buf_writer, event)?;
        }

        // Now, insert the "chain_id" field into the JSON output.
        // This example assumes the JSON output is an object that ends with `}` (or "}\n").
        if buf.ends_with("}\n") {
            buf.truncate(buf.len() - 2); // Remove "}\n"
            writeln!(writer, "{}, \"chain_id\": {} }}", buf, self.chain_id)
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

    // Create the inner JSON formatter by wrapping Json in the Format adapter.
    let json_formatter = Format::default().json().with_target(true);

    // Wrap the JSON formatter in our custom formatter that appends the chain_id field.
    let chain_formatter = ChainIdJsonFormatter { inner: json_formatter, chain_id };

    // Build the formatting layer using our custom event formatter.
    let fmt_layer = subscriber_fmt::layer().event_format(chain_formatter);

    tracing_subscriber::registry()
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
        .with_max_level(Level::DEBUG)
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
