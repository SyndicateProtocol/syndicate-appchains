# Metabased-translator

This is a service that ingests L2 transaction data, transforms it into a unified format to be mined by an Anvil node, then mines it and makes it available to rollup frameworks. 

It consists of 2 crates: `data-ingestor` and `block-builder`

## Useful commands
(From root)
Run all tests:
```
cargo test --all
```

Run `block-builder` with logging:
```
RUST_LOG=info cargo run -p block-builder
```
`RUST_LOG` can also be `debug` or `trace` for more detail