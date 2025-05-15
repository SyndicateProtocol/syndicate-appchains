# `Syndicate-Appchains` Monorepo

The monorepo for the Syndicate Appchains stack. Contains the appchain translator, sequencer, smart contracts, RPC node, dev environments, and more!

## Docker

Use the included Dockerfile at repo root to build a Docker image. You should then be able to run the image and pass in the same flags
that you would pass to the Rust binary executable.

Note that this a multi-target Dockerfile, so specify `synd-translator`, `synd-poster`, etc. as the target.

```
docker build --target synd-translator -f Dockerfile . --tag synd-translator

docker run -it -e RUST_LOG=debug synd-translator \ \
  --sequencing-rpc-url <YOUR_URL_HERE> \
  --settlement-rpc-url <YOUR_URL_HERE> \
  --port 8888 \
  ...
  etc.
  ...
```

Run `cargo make print-sample-command` to see all the options for the `synd-translator`.

### Lints
(April 2025)
Here is a terminal alias that will run our CI lint checks locally for ease of development. Note that these are mutating commands and will change your code. If you don't want that, add `--check` flags as necessary

```
alias rt='cargo +nightly fmt --all -- --unstable-features && \
cargo clippy --workspace --all-targets --all-features && \
cargo machete && \
taplo fmt "**/Cargo.toml"'
```
