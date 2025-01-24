# Metabased-translator

This is a service that ingests L2 transaction data, transforms it into a unified format to be mined by an Anvil node, then mines it and makes it available to rollup frameworks. 

It consists of 2 crates: `data-ingestor` and `block-builder`

## Useful commands
(From `/metabased-translator` root)

Run all tests:
```
cargo test --all
```

Run `block-builder` with logging and CLI args:
```
RUST_LOG=info cargo run -p block-builder -- --port 9000 --chain-id 12345 --genesis-timestamp 1712500000
```
`RUST_LOG` can also be `debug` or `trace` for more detail


Run
```
make print-sample-command
```
to see the CLI options needed to run the binary executable, which utilizes the `ingestor`, `slotter`, and `block-builder` crates