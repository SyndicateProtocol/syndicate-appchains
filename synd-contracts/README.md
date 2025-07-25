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

## Using Static Analyzer for the contracts

Lib used [Aderyn](https://github.com/Cyfrin/aderyn)

- Installation

```bash
cargo install aderyn
```

- Run the static analysis

```bash
aderyn [Option] [Path]
```

Example:

```bash
aderyn -s src/MyContract.sol
```

See List of options:

run `aderyn --help`

### Deploy

Look at Makefile for more details.

#### Setup

Import dev private key to cast, this will ask for private key and a password

```bash
cast wallet import deployer --interactive
```

add the public address of the wallet in `.env` file

```bash
DEV_PUB_ADDRESS=xxxx
```

Example:

```shell
$ make deploy-syndicate-factory
```

### Deployed Contracts

### Base Sepolia
| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManagerFactory       | [0xff82F81c66F144cAb9160A4900f7D0D92C94dd97](https://sepolia.basescan.org/address/0xff82F81c66F144cAb9160A4900f7D0D92C94dd97)                         |
| ArbConfigManager              | [0xbD6b5264de6d3e65ce6e4Cf3E0071E30D01b509C](https://sepolia.basescan.org/address/0xbD6b5264de6d3e65ce6e4Cf3E0071E30D01b509C)                         |

### Risa

| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| SyndicateFactoryWrapper       | [0x99d2b60FD2dBdF08CDA18165D20f4E713768b07E](https://risa-testnet.explorer.alchemy.com/address/0x99d2b60FD2dBdF08CDA18165D20f4E713768b07E) |
| SyndicateFactory              | [0x33aB24E0a47A7aAe869755420950A6326e3CB9F3](https://risa-testnet.explorer.alchemy.com/address/0x33aB24E0a47A7aAe869755420950A6326e3CB9F3) |
| RequireAndModuleFactory       | [0x2f3CC13661A95DD616311a7bddB9cDB4bA67C4d2](https://risa-testnet.explorer.alchemy.com/address/0x2f3CC13661A95DD616311a7bddB9cDB4bA67C4d2) |
| RequireOrModuleFactory        | [0x61fC28cf640235d560bd89350033cAee9642F8E5](https://risa-testnet.explorer.alchemy.com/address/0x61fC28cf640235d560bd89350033cAee9642F8E5) |
| RequireCompositeModuleFactory | [0xD1e50D5A203E6485e97E3bC8A951b49aaFC28603](https://risa-testnet.explorer.alchemy.com/address/0xD1e50D5A203E6485e97E3bC8A951b49aaFC28603) |

### Risa Devnet

| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| SyndicateFactory              | [0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba](https://explorer-1205614516323464.devnet.alchemy.com/address/0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba) |
| RequireAndModuleFactory       | [0x19aaf160dA8985c54bb97adAF9304B5aC7890421](https://explorer-1205614516323464.devnet.alchemy.com/address/0x19aaf160dA8985c54bb97adAF9304B5aC7890421) |
| RequireOrModuleFactory        | [0xcFc46cEBB3eAEc9b5776e3FDe5879125B8BBA05d](https://explorer-1205614516323464.devnet.alchemy.com/address/0xcFc46cEBB3eAEc9b5776e3FDe5879125B8BBA05d) |
| RequireCompositeModuleFactory | [0x471584f0B8e35faEB2a618BD58A62316D8882d63](https://explorer-1205614516323464.devnet.alchemy.com/address/0x471584f0B8e35faEB2a618BD58A62316D8882d63) |

### Syndicate Exo

| Contract Name           | Address                                                                                                                                                                         |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SyndicateFactory        | TODO: deploy new `SyndicateFactory` [0x60D834808d4C90a5A0D66fcFd44292FcAa4537fe](https://syndicate-exo.explorer.alchemy.com/address/0x60D834808d4C90a5A0D66fcFd44292FcAa4537fe) |
| WalletPoolWrapperModule | [0x9d9E8B09C1f7d9cC1Cdd4a843e695fD580a390E8](https://syndicate-exo.explorer.alchemy.com/address/0x9d9E8B09C1f7d9cC1Cdd4a843e695fD580a390E8)                                     |

### ETH Holesky

| Contract Name                | Address                                                                                                                       |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| SynGasToken holSYND (Devnet) | [0x19aaf160dA8985c54bb97adAF9304B5aC7890421](https://holesky.etherscan.io/address/0x19aaf160dA8985c54bb97adAF9304B5aC7890421) |
| SynGasToken SYND (Testnet)   | [0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba](https://holesky.etherscan.io/address/0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba) |

### ETH Sepolia

| Contract Name      | Address                                                                                                                       |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| TestnetSyndToken   | [0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C](https://sepolia.etherscan.io/address/0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C) |
| SyndicateToken New | [0xb80c82702791664f59dF773e8b50dE921fC026fE](https://sepolia.etherscan.io/address/0xb80c82702791664f59dF773e8b50dE921fC026fE) |
| SyndicateToken Old | [0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23](https://sepolia.etherscan.io/address/0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23) |
