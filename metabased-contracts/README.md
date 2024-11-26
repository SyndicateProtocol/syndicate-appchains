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
| MetabasedFactory        | 0xD1e50D5A203E6485e97E3bC8A951b49aaFC28603 |
| MetabasedSequencerChain | 0xEFfCeaf782d14D7271B30C68e9667cD3B4218553 |
| AlwaysAllowedModule     | 0x33aB24E0a47A7aAe869755420950A6326e3CB9F3 |
| MetafillerStorage       | 0x745bdeFA5C879ADCf739075bB03FD4ecCd03cE22 |

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
| MetabasedFactory                | 0x19aaf160dA8985c54bb97adAF9304B5aC7890421 |
| AlwaysAllowedModule             | 0x87F4DE1886298255c5fce2adF15977fE44F48f68 |
| MetabasedSequencerChain (Frame) | 0x56b2cd4fb8d6D6486b95Ff2DF5cDC30FE526FFaf |
| MetabasedSequencerChain (Ham)   | 0x0e6A93458fEdaEFaAcb106f08441058c8E0b2b0F |
| MetafillerStorage (Frame)       | 0x0D26034350193af5Fb3e7c206f3462A8fD25F289 |
| MetafillerStorage (Ham)         | 0xE3c1d5335203066EB8c3212a6bda57Ff33133f54 |
| RequiredAllModule (Ham)         | 0xAfeA8F68921242A90ae9e35f4DDF0d3769dE3150 |
| RequiredAllModule (Frame)       | 0x7Bc475096B936Ad04Cfc544FB56aC54B3661beE6 |
