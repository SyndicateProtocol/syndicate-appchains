# Syndicate Sequencing Chain ExExs

## Description

RETH ExExs are Rust-based applications that interact with the RETH framework to monitor blockchain events and execute actions based on specific transaction criteria. ExExs are designed to be modular and can be easily integrated with the RETH framework to provide additional functionality.

For Syndicate Sequencing Chain, there will exexs for various tasks, including:

- Monitoring of Ethereum blockchain transactions
- Filtering of transactions sent to a specific smart contract address
- Interaction with the smart contract to perform operations
- Provision of real-time logging of blockchain events

### Prerequisites

Ensure you have the following installed on your system:

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Cargo: Comes with Rust installation
- Dotenv: Used for managing environment variables

### Install the dependencies:

```bash
  cargo build
```

### Test

```bash
$ cargo test
```

### Running the Application

To start the application, run:

```sh
cargo run node <RETH-OPTIONS>
```

## Index of ExExs

### ExEx Block Filter ExEx (exex_block_filter)

Block Filter ExEx is a Rust-based application designed to filter blockchain transactions and interact with a smart contract deployed on the Ethereum blockchain. It leverages the `reth_exex` framework to monitor blockchain events and execute actions based on specific transaction criteria.
