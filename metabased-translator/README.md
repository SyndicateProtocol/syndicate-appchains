# Metabased-translator

This is a service that ingests L2 transaction data, organizes the data into slots, transforms it into a unified format to be mined by an Anvil node, then mines it and makes it available to rollup frameworks.

It consists of a `common` crate and 3 component crates: `ingestor`, `slotter` and `block-builder`

#### Metachain configuration

The metachain has chain id 84532 and a genesis timestamp of 1712500000.

The rollup contract is deployed to 0x5FbDB2315678afecb367f032d93F642f64180aa3 on the first metachain block
with the following on-chain configuration:
{
   "chainId": 13331370,
   "homesteadBlock": 0,
   "daoForkBlock": null,
   "daoForkSupport": true,
   "eip150Block": 0,
   "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
   "eip155Block": 0,
   "eip158Block": 0,
   "byzantiumBlock": 0,
   "constantinopleBlock": 0,
   "petersburgBlock": 0,
   "istanbulBlock": 0,
   "muirGlacierBlock": 0,
   "berlinBlock": 0,
   "londonBlock": 0,
   "clique": {
      "period": 0,
      "epoch": 0
   },
   "arbitrum": {
      "EnableArbOS": true,
      "AllowDebugPrecompiles": false,
      "DataAvailabilityCommittee": false,
      "InitialArbOSVersion": 10,
      "InitialChainOwner": "0x0000000000000000000000000000000000000000",
      "GenesisBlockNum": 0
   }
}

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

1. Make sure you have a Personal Access Token (PAT) on github with the `read:packagesDownload` permission
   - https://github.com/settings/tokens
   - Pro tip: Check under `~/.git-credentials` if you have one generated already

2. Run the following commands to login to docker with the PAT

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
