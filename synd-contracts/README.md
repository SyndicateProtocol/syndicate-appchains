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

### Syndicate Network

| Contract Name                 | Address                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| SyndicateFactoryWrapper       | [0x6f970AD6AD70acb9990CDb499fc70FbcE81aACD7](https://synd-mainnet.explorer.alchemy.com/address/0x6f970AD6AD70acb9990CDb499fc70FbcE81aACD7) |
| SyndicateFactory              | [0x453348A58443B2Ede5c62480C47cf91c750BA41f](https://synd-mainnet.explorer.alchemy.com/address/0x453348A58443B2Ede5c62480C47cf91c750BA41f) |
| RequireAndModuleFactory       | [0xD3A9d539fe3Fb2838fF73C652EB94dAE050d873a](https://synd-mainnet.explorer.alchemy.com/address/0xD3A9d539fe3Fb2838fF73C652EB94dAE050d873a) |
| RequireOrModuleFactory        | [0x715595259b492Dbc3ad9298F66E9F917eeD6d013](https://synd-mainnet.explorer.alchemy.com/address/0x715595259b492Dbc3ad9298F66E9F917eeD6d013) |
| RequireCompositeModuleFactory | [0xDC57Da2B7C11C4CEc933910EfD4c0479cbB24434](https://synd-mainnet.explorer.alchemy.com/address/0xDC57Da2B7C11C4CEc933910EfD4c0479cbB24434) |

### Mainnet

| Contract Name                 | Address                                                                                                               |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManagerFactory       | [0x52e02554e63801ec66d2D72fdD2c679D18a117dE](https://etherscan.io/address/0x52e02554e63801ec66d2D72fdD2c679D18a117dE) |
| ArbConfigManager              | [0x9a525431620B2B3C808684BFFE6738307921f19d](https://etherscan.io/address/0x9a525431620B2B3C808684BFFE6738307921f19d) |
| AttestationDocVerifier        | [0xC9E070caa1C8F231D5f3fa265AAb61A6A1fD0466](https://etherscan.io/address/0xC9E070caa1C8F231D5f3fa265AAb61A6A1fD0466) |
| TeeKeyManager                 | [0xFB7a527Af1C80dEA95b174FBfa2662B36006b90e](https://etherscan.io/address/0xFB7a527Af1C80dEA95b174FBfa2662B36006b90e) |

### Base

| Contract Name                 | Address                                                                                                               |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| ArbConfigManagerFactory       | [0x1e05910C82027f2E0e4733cba5ba41f528cAefE1](https://basescan.org/address/0x1e05910C82027f2E0e4733cba5ba41f528cAefE1) |
| ArbConfigManager              | [0xE37CfB1520F5F33F829d5f990D7AA3C563AbBF18](https://basescan.org/address/0xE37CfB1520F5F33F829d5f990D7AA3C563AbBF18) |
| AttestationDocVerifier        | [0xd55483B303B8a9b1327e954043f19E18692FA293](https://basescan.org/address/0xd55483B303B8a9b1327e954043f19E18692FA293) |
| TeeKeyManager                 | [0xC12BB72B44286b3EAF8A065402A102C3719C8cA1](https://basescan.org/address/0xC12BB72B44286b3EAF8A065402A102C3719C8cA1) |

### Base Sepolia

| Contract Name                 | Address                                                                                                                        |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| ArbConfigManagerFactory       | [0xcf733FD99A9E7052F126C01bf033159775bEcF0B](https://sepolia.basescan.org/address/0xcf733FD99A9E7052F126C01bf033159775bEcF0B)  |
| ArbConfigManager              | [0x647e0da79BBEF0e3B076a59cfB3a42429273CE52](https://sepolia.basescan.org/address/0x647e0da79BBEF0e3B076a59cfB3a42429273CE52)  |
| AttestationDocVerifier        | [0xC1A11c77AAfCd663945ea234345ffc6EF53dC12c](https://sepolia.basescan.org/address/0xC1A11c77AAfCd663945ea234345ffc6EF53dC12c)  |
| TeeKeyManager                 | [0x9bFA7C84fF3816a2Ca9F72865036e8998385147b](https://sepolia.basescan.org/address/0x9bFA7C84fF3816a2Ca9F72865036e8998385147b)  |

### Risa

| Contract Name                 | Address                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| SyndicateFactoryWrapper       | [0x99d2b60FD2dBdF08CDA18165D20f4E713768b07E](https://risa-testnet.explorer.alchemy.com/address/0x99d2b60FD2dBdF08CDA18165D20f4E713768b07E) |
| SyndicateFactory              | [0x33aB24E0a47A7aAe869755420950A6326e3CB9F3](https://risa-testnet.explorer.alchemy.com/address/0x33aB24E0a47A7aAe869755420950A6326e3CB9F3) |
| RequireAndModuleFactory       | [0x2f3CC13661A95DD616311a7bddB9cDB4bA67C4d2](https://risa-testnet.explorer.alchemy.com/address/0x2f3CC13661A95DD616311a7bddB9cDB4bA67C4d2) |
| RequireOrModuleFactory        | [0x61fC28cf640235d560bd89350033cAee9642F8E5](https://risa-testnet.explorer.alchemy.com/address/0x61fC28cf640235d560bd89350033cAee9642F8E5) |
| RequireCompositeModuleFactory | [0xD1e50D5A203E6485e97E3bC8A951b49aaFC28603](https://risa-testnet.explorer.alchemy.com/address/0xD1e50D5A203E6485e97E3bC8A951b49aaFC28603) |


### ETH Holesky

| Contract Name                | Address                                                                                                                       |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| SynGasToken holSYND (Devnet) | [0x19aaf160dA8985c54bb97adAF9304B5aC7890421](https://holesky.etherscan.io/address/0x19aaf160dA8985c54bb97adAF9304B5aC7890421) |
| SynGasToken SYND (Testnet)   | [0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba](https://holesky.etherscan.io/address/0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba) |

### ETH Sepolia

| Contract Name            | Address                                                                                                                        |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| ArbConfigManagerFactory  | [0x8ad717D4719b57A3DA853F7d422284859b00b10e](https://sepolia.etherscan.io/address/0x8ad717D4719b57A3DA853F7d422284859b00b10e)  |
| ArbConfigManager         | [0xf7a9732b8e149Da788C11D3D1AAd029B732A8118](https://sepolia.etherscan.io/address/0xf7a9732b8e149Da788C11D3D1AAd029B732A8118)  |
| AttestationDocVerifier   | [0x860b4eF98ff5A777aEAb715ce06d7fD8c8C1cC25](https://sepolia.etherscan.io/address/0x860b4eF98ff5A777aEAb715ce06d7fD8c8C1cC25)  |
| TestnetSyndToken         | [0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C](https://sepolia.etherscan.io/address/0x55f6e82a8BF5736d46837246DcBEAf7e61b3c27C)  |
| SyndicateToken New       | [0xb80c82702791664f59dF773e8b50dE921fC026fE](https://sepolia.etherscan.io/address/0xb80c82702791664f59dF773e8b50dE921fC026fE)  |
| SyndicateToken Old       | [0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23](https://sepolia.etherscan.io/address/0xC89095a650BB50336e1C7A8ffD4dD4bce2456e23)  |
