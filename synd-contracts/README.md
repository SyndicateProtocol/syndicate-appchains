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

### Syndicate Mainnet

| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| SyndicateFactoryWrapper       | [0x5328413c1FAf9FDa1dA49607fa35A06e4C2EfCf0](https://synd-mainnet.explorer.alchemy.com/address/0x5328413c1FAf9FDa1dA49607fa35A06e4C2EfCf0) |
| SyndicateFactory              | [0x0620625c3662CbD6a8ca8Eef196ee3b10A8Bd157](https://synd-mainnet.explorer.alchemy.com/address/0x0620625c3662CbD6a8ca8Eef196ee3b10A8Bd157) |
| RequireAndModuleFactory       | [0x3eEb8b1500cbaCbc4A3718D39414C8D191AC906B](https://synd-mainnet.explorer.alchemy.com/address/0x3eEb8b1500cbaCbc4A3718D39414C8D191AC906B) |
| RequireOrModuleFactory        | [0xb133DEC7AB6B736f7401beF19940DF6c15cd78fC](https://synd-mainnet.explorer.alchemy.com/address/0xb133DEC7AB6B736f7401beF19940DF6c15cd78fC) |
| RequireCompositeModuleFactory | [0xf1513660dD199bB4B24249200dE534f232c7794B](https://synd-mainnet.explorer.alchemy.com/address/0xf1513660dD199bB4B24249200dE534f232c7794B) |

### Syndicate Risa

| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| SyndicateFactoryWrapper       | [0x382ECBBcbB9feEd2039c2Dc73061282b0b121345](https://risa-testnet.explorer.alchemy.com/address/0x382ECBBcbB9feEd2039c2Dc73061282b0b121345) |
| SyndicateFactory              | [0x2e44cd104A6b67037b5e6DB662C0E917d1828D9E](https://risa-testnet.explorer.alchemy.com/address/0x2e44cd104A6b67037b5e6DB662C0E917d1828D9E) |
| RequireAndModuleFactory       | [0x60e6Ac9FF8ff09175329EfB3daDa27abDA812aA4](https://risa-testnet.explorer.alchemy.com/address/0x60e6Ac9FF8ff09175329EfB3daDa27abDA812aA4) |
| RequireOrModuleFactory        | [0xfbeD1fB4d03359AbCbDA2a5d7894028C25fb95fA](https://risa-testnet.explorer.alchemy.com/address/0xfbeD1fB4d03359AbCbDA2a5d7894028C25fb95fA) |
| RequireCompositeModuleFactory | [0x0aa1EbFb8C7D035b6ebD9C95b80790C142A3A1E9](https://risa-testnet.explorer.alchemy.com/address/0x0aa1EbFb8C7D035b6ebD9C95b80790C142A3A1E9) |

### Base
| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManagerFactory       | [0x6C75cd3E9218E82Eb667221ac221386D2c145eC7](https://basescan.org/address/0x6C75cd3E9218E82Eb667221ac221386D2c145eC7)                         |
| ArbConfigManager              | [0x65e6D336E311C92D1F19C66CfE68Ec6bE5b4f50B](https://basescan.org/address/0x65e6D336E311C92D1F19C66CfE68Ec6bE5b4f50B)                         |

### Base Sepolia
| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManagerFactory       | [0x5558fC696421d065FEFAe901ACEc9C206a2D5B42](https://sepolia.basescan.org/address/0x5558fC696421d065FEFAe901ACEc9C206a2D5B42)                         |
| ArbConfigManager              | [0xbb53E8736Cc018bb46D0F67A9d2Dbe3C3b306E92](https://sepolia.basescan.org/address/0xbb53E8736Cc018bb46D0F67A9d2Dbe3C3b306E92)                         |

### Ethereum Mainnet

| Contract Name                 | Address                                                                                                                                               |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManagerFactory       | [0x1311AF82Fcf1bb5DD2dba3E763D949A2fCe72f70](https://etherscan.io/address/0x1311AF82Fcf1bb5DD2dba3E763D949A2fCe72f70)                         |
| ArbConfigManager              | [0xec2ba05a9cFFcb86e3225F7A046bA9124419397C](https://etherscan.io/address/0xec2ba05a9cFFcb86e3225F7A046bA9124419397C)                         |
| SyndicateToken                | [0x1bAB804803159aD84b8854581AA53AC72455614E](https://etherscan.io/address/0x1bAB804803159aD84b8854581AA53AC72455614E)                         |

### Ethereum Sepolia

| Contract Name            | Address                                                                                                                        |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| ArbConfigManagerFactory  | [0x7bC5D6B462a10FA91b805351313B9E8E7FC9eEd2](https://sepolia.etherscan.io/address/0x7bC5D6B462a10FA91b805351313B9E8E7FC9eEd2)  |
| ArbConfigManager         | [0xc18feFb2E79Ec35Ca9f3c3e7F1920EC1cad06e8F](https://sepolia.etherscan.io/address/0xc18feFb2E79Ec35Ca9f3c3e7F1920EC1cad06e8F)  |
| TestnetSyndToken         | [0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C](https://sepolia.etherscan.io/address/0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C)  |
| SyndicateToken New       | [0xb80c82702791664f59dF773e8b50dE921fC026fE](https://sepolia.etherscan.io/address/0xb80c82702791664f59dF773e8b50dE921fC026fE)  |
| SyndicateToken Old       | [0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23](https://sepolia.etherscan.io/address/0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23)  |
