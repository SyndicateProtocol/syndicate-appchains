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

### Deploy

Look at Makefile for more details.

Example:

```shell
$ make deploy-based-sequencerchain-frame
```

### Deployed Contracts

#### Base Sepolia

| Contract Name           | Address                                    |
| ----------------------- | ------------------------------------------ |
| MetabasedFactory        | 0xb80c82702791664f59dF773e8b50dE921fC026fE |
| MetabasedSequencerChain | 0xb1567B5DFa038e4F279d3b585D4D45b8bDD2263D |
| AlwaysAllowedModule     | 0x33aB24E0a47A7aAe869755420950A6326e3CB9F3 |
| MetafillerStorage       | 0x91C77f7857EcD4Edd1d5242e38345E42Ad1212E4 |
| RequiredAllModule       | 0xaE938C7D5b69106BaA2316BC007D0f30e6239826 |

#### Metabased Testnet (Private Devnet)

| Contract Name                             | Address                                    |
| ----------------------------------------- | ------------------------------------------ |
| MetabasedFactory                          | 0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba |
| MetabasedSequencerChain (With dummy data) | 0x5b6bB750222ADeE5B6031702a08D239f97F3b063 |
| AlwaysAllowedModule                       | 0x471584f0B8e35faEB2a618BD58A62316D8882d63 |
| MetafillerStorage                         | 0xC329B96c47271426B237bA85dF5504375C5cCB28 |

### Metabased Stratos

| Contract Name                   | Address                                    |
| ------------------------------- | ------------------------------------------ |
| MetabasedFactory                | 0xe2aBFBD5357D28425638bad0849f57ea87417D1b |
| AlwaysAllowedModule             | 0x87F4DE1886298255c5fce2adF15977fE44F48f68 |
| MetabasedSequencerChain (Frame) | 0x56b2cd4fb8d6D6486b95Ff2DF5cDC30FE526FFaf |
| MetabasedSequencerChain (Ham)   | 0x0e6A93458fEdaEFaAcb106f08441058c8E0b2b0F |
| MetafillerStorage (Frame)       | 0x428F74D12fB57dFc7b2979Dd0679813daB023406 |
| MetafillerStorage (Ham)         | 0xfCF224BfF63658524d3Ed8c277c20e06488B59A8 |
| RequiredAllModule (Ham)         | 0xAfeA8F68921242A90ae9e35f4DDF0d3769dE3150 |
| RequiredAllModule (Frame)       | 0x7Bc475096B936Ad04Cfc544FB56aC54B3661beE6 |

#### ETH Sepolia

| Contract Name | Address                                    |
| ------------- | ------------------------------------------ |
| SynGasToken   | 0x8c8861c1bBd3a47deC0cfc5dc82e4B5E88810BfE |
