# Create a Syndicate Appchain

This repo contains scripts to create a new Syndicate Appchain. It handles the configuration and deployment of the required contracts to both the sequencing chain and the settlement chain.

## Deploy Foundation

Deploy foundational dependencies onto the settlement & sequencing chains required by the Appchain node to run.

### Foundation Background

This section will deploy the following.

Settlement Chain:

- [Nitro Core Contracts](https://github.com/OffchainLabs/nitro-contracts)
- `ArbChainConfig`

Sequencing Chain:

- `RequireAndModule`
- `AllowlistSequencingModule`
- `SyndicateSequencingChain`

> [!IMPORTANT]
> This is the default configuration for an Appchain. If the chain you are configuring has a custom SyndicateSequencingChain you may require a different contract to be deployed here. These chains are configured on a case-by-case basis, but deploying the default contracts does not hurt!

### Install Foundation dependencies

```bash
bun install
```

### Run Foundation Script

To deploy the contracts required by the Appchain initially, make sure you have created a `foundation.config.json`. An example can be found in `foundation.config.json`.

```bash
bun foundation
```

This will deploy the required contracts to both the sequencing chain and the settlement chain and output the results to the console as well as save the results to the `out/<chain_name>/*.json` files.

### Foundation Secret

> NOTE: An EOA is created for the batch sequencer during this process & you will be prompted at the end to optionally fund it from the deployer.

Please save the interm-owner private key & sequencer private keys in a secure location

```txt
Interim Owner
addr: <ADDRESS_HERE>
pk: <PRIVATE_KEY_HERE>

Sequencer
addr: <ADDRESS_HERE>
pk: <PRIVATE_KEY_HERE>
```

## Deploy Features

Deploy additional contracts to the settlement chain & appchain that depend on the Appchain node.

> [!IMPORTANT]
> The Appchain RPC URL must be available for this step! DO NOT CONTINUE unless the appchain RPC is working!

### Features Background

This section will deploy the following.

Settlement Chain:

- [Nitro Token Bridge](https://github.com/OffchainLabs/token-bridge-contracts): Allows users to bridge non-native tokens in & out of the Appchain
- `TeeKeyManager`: Required for withdraws
- `AssertionPoster`: Required for withdraws
- `TeeModule`: Required for withdraws

Appchain:

- [`Multicall3`](https://github.com/mds1/multicall3/blob/main/src/Multicall3.sol): Utility contract for aggregating results from multiple function calls

### Install Features dependencies

```bash
bun install
```

### Run Features Script

Create a `features.config.json` based on `features.config.json.example` for the Appchain you are finishing setting up.

```bash
bun features
```

Once the bridge has been successfully created all required contract address will be output to `out/<chain_name>/*.json`. Please save these values.

> NOTE: This process can take some time (~5-10 minutes) as there are checks the retryable tickets used to communicate between the settlement and appchain are successful.

### Features Secret

> NOTE: An EOA is created for the proposer during this process & you will be prompted at the end to optionally fund it from the deployer.

Please save the proposer wallet details in a secure location:

```txt
Proposer
addr: <ADDRESS_HERE>
pk: <PRIVATE_KEY_HERE>
```
