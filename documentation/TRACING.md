# Tracing

Services in this repository support [OpenTelemetry](https://opentelemetry.io/) tracing via [`tracing-opentelemetry`](https://github.com/tokio-rs/tracing-opentelemetry) crate.

## Adding tracing to your service

Use the code from the `shared::tracing` module in your `main` function to start OpenTelemetry trace collection and exporting:

```rust
// my-service/src/main.rs
use shared::tracing::{setup_global_tracing, ServiceTracingConfig};

#[tokio::main]
async fn main() -> Result<()> {
  // holding _guard in your main allows for
  // proper shutdown of OTel when main finishes
  let _guard = setup_global_tracing(ServiceTracingConfig::from_env(
    // ServiceTracingConfig takes a `name` and `version` of your service,
    // the simplest way is to grab it from Cargo itself
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
  ))?;
  // main continues. while _guard is in scope,
  // all instrumented functions will be traced
}
```

## Basic usage - tracing a function

To add tracing to a function:

1. Import `tracing::instrument` macro
2. Annotate the function with `#[instrument]`
3. **Skip arguments** that you don't want to log by using `skip(...)`
    - Example: `#[instrument(skip(raw_tx))]`
4. Add `err` to **trace errors**
    - Example: `#[instrument(err)]`
5. **Add fields** with `fields(...)` as needed
    - Example: `fields(tx_hash = %...))`
        - use `%` to format the field value, use `?` to use the `Debug` representation
6. Each function call will create a new span in the trace (one level of nesting in the waterfall view)

**Example:**

```rust
use tracing::instrument;

#[instrument(
  skip(raw_tx),
  err,
  fields(
    tx_hash = %format!("0x{}", hex::encode(keccak256(&raw_tx)))
  )
)]
pub fn handle_transaction(raw_tx: Bytes, chain_id: u64) -> Result<()> {
  todo!("your code here");
  enqueue_transaction(raw_tx, chain_id)
}
```

## Advanced Features

### Manual span creation

Useful if you want to add a span corresponding to a specific operation that is not a function call. NB: consider refactoring to a function if possible.

In this example, `info_span!` macro creates a new span named `my_manual_span`, with level `INFO` (the default level used by `#[instrument]`, which makes it visually compatible with traced functions).

The `entered()` method is called to enter the span context, which will be active for the duration of the block (e.g., the span will be closed when `_guard` goes out of scope).

```rust
use tracing::{info_span};

// ...
{
    let _guard = info_span!("my_manual_span").entered();
    // traced code goes here
}
// continues outside of my_manual_span
```

## Environment Variables

The tracing code can be configured using the following environment variables:

| Variable Name               | Description               | Example Value                                      |
|-----------------------------|---------------------------|----------------------------------------------------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | Exporter endpoint URL      | `http://localhost:4317`, `https://api.honeycomb.io` |
| `OTEL_DEBUG`               | Enable debug logging for OpenTelemetry | `true`                                             |

Set these in your deployment environment, or use a tool like `direnv` to test tracing locally.

## References

- [OpenTelemetry docs](https://opentelemetry.io/docs/)
- `tracing::instrument` macro documentation: [tracing docs.rs](https://docs.rs/tracing/latest/tracing/attr.instrument.html) 

## Troubleshooting

- Ensure the exporter endpoint is reachable.
- Check that `OTEL_EXPORTER_OTLP_ENDPOINT` is set to the correct URL.
- Ensure that the service is calling `setup_global_tracing` in its `main` function, and the resulting `_guard` is kept in scope until the service shuts down.
- Set `OTEL_DEBUG` to `true` to enable debug logging for OpenTelemetry, which can help diagnose issues with tracing.