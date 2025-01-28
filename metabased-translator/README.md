# Metabased-translator

This is a service that ingests L2 transaction data, organizes the data into slots, transforms it into a unified format to be mined by an Anvil node, then mines it and makes it available to rollup frameworks.

It consists of a `common` crate and 3 component crates: `ingestor`, `slotting` and `block-builder`

#### Rust configuration

Make sure you are using the nightly rust toolkit

```
rustup install nightly
rustup component add rustfmt --toolchain nightly
```

## Useful commands

(From `/metabased-translator` root)

Run all tests:

```
cargo test --all
```

Run unit tests

```
cargo test --lib
```

### Setup for integration tests

Our integration tests pull a docker image from github which requires some authorization setup.

1/ Make sure you have a Personal Access Token (PAT) on github with the `read:packagesDownload` permission

- https://github.com/settings/tokens
- Pro tip: Check under `~/.git-credentials` if you have one generated already

2/ Run the following commands to login to docker with the PAT

```
export CR_PAT="<GITHUB-PAT>"
echo $CR_PAT | docker login ghcr.io -u <GH_USERNAME> --password-stdin
```

`RUST_LOG` can also be `debug` or `trace` for more detail

Run

```
cargo make print-sample-command
```
to see the CLI options needed to run the binary executable, which utilizes the `ingestor`, `slotter`, `block-builder` and `metrics` crates
