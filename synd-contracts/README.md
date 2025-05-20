## Base Sequencer Chain Contracts

### install

Ensure Foundry is installed.

```shell
$ forge install
```

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

#### Run coverage test:

```shell
forge coverage --ir-minimum --no-match-coverage "(script|test)"
```

or

```shell
make test-coverage
```

## Certora Verification

### Setup

1. Install Certora CLI ([installation guide](https://docs.certora.com/en/latest/docs/user-guide/install.html))

   ```bash
   # Recommended for macOS
   brew install pipx
   pipx install certora-cli
   ```

2. Get your Certora key from [certora.com/signup](https://www.certora.com/signup)

3. Add key to your environment
   ```bash
   echo 'export CERTORAKEY=your_key_here' >> ~/.zshrc  # or ~/.bashrc
   source ~/.zshrc  # or ~/.bashrc
   ```

### Running Specs

```bash
certoraRun certora/conf/SyndicateSequencingChain.conf
```

### Troubleshooting

If CERTORAKEY isn't recognized, check with `echo $CERTORAKEY` or set it manually:

```bash
export CERTORAKEY=your_key_here
```

### Documentation

Generate documentation for the Solidity contracts:

```shell
$ forge doc
```

This command generates markdown documentation for all Solidity source files in the project. By default, it:

- Outputs documentation to the `docs/` directory in the project root
- Generates markdown files for each contract, interface, and library
- Includes function signatures, parameters, return values, events, and errors
- Does not build or serve the documentation (requires additional flags)

#### Options:

- `--out <PATH>`: Specify a custom output directory for the documentation
- `--build`: Build the documentation into an mdbook
- `--serve`: Serve the documentation locally
- `--port <PORT>`: Specify the port for serving documentation (requires `--serve`)
- `--hostname <HOSTNAME>`: Specify the hostname for serving documentation (requires `--serve`)

#### Examples:

Generate and build documentation:

```shell
$ forge doc --build
```

Generate documentation with a custom output directory:

```shell
$ forge doc --out ./custom-docs
```

Generate, build, and serve documentation locally:

```shell
$ forge doc --build --serve --port 3000
```

The documentation includes details about contracts, functions, events, and errors, similar to the files in the `pre-audit/` directory.

### Deploy

Look at Makefile for more details.

Example:

```shell
$ make deploy-syndicate-factory
```

### Deployed Contracts

### Syndicate Exo

| Contract Name           | Address                                                                                                                                                        |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SyndicateFactory        | `SyndicateFactory` [0x60D834808d4C90a5A0D66fcFd44292FcAa4537fe](https://syndicate-exo.explorer.alchemy.com/address/0x60D834808d4C90a5A0D66fcFd44292FcAa4537fe) |
| WalletPoolWrapperModule | [0x9d9E8B09C1f7d9cC1Cdd4a843e695fD580a390E8](https://syndicate-exo.explorer.alchemy.com/address/0x9d9E8B09C1f7d9cC1Cdd4a843e695fD580a390E8)                    |

### Base Sepolia

| Contract Name    | Address                                                                                                                       |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManager | [0xCaf9F341401cD3e72Fc49081E498Ef0F86055b67](https://sepolia.etherscan.io/address/0xCaf9F341401cD3e72Fc49081E498Ef0F86055b67) |

### ETH Holesky

| Contract Name                | Address                                                                                                                       |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| SynGasToken holSYND (Devnet) | [0x19aaf160dA8985c54bb97adAF9304B5aC7890421](https://holesky.etherscan.io/address/0x19aaf160dA8985c54bb97adAF9304B5aC7890421) |
| SynGasToken SYND (Testnet)   | [0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba](https://holesky.etherscan.io/address/0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba) |

### ETH Sepolia

| Contract Name  | Address                                                                                                                       |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| SyndicateToken | [0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23](https://sepolia.etherscan.io/address/0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23) |
