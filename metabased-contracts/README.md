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
| MetabasedSequencerChain | 0x0E3E7d53c6451D62CE9f86201743587419Dc88Be |
| AlwaysAllowedModule     | 0x33aB24E0a47A7aAe869755420950A6326e3CB9F3 |

#### Optimism

| Contract Name           | Address                                    |
| ----------------------- | ------------------------------------------ |
| MetabasedSequencerChain | 0x7DEdabB3Db89310B316bA49D96Aa6517aFC44294 |
| AlwaysAllowedModule     | 0x21e1C6bE46E53693381Cef9BECD3B0A947A590e4 |

#### Metabased Testnet

| Contract Name           | Address                                    |
| ----------------------- | ------------------------------------------ |
| MetabasedFactory        | 0x2f3CC13661A95DD616311a7bddB9cDB4bA67C4d2 |

Contracts for Testing with Dummy L3 Chain ID `42424242424242424242`
| MetabasedSequencerChain | 0x2945b20521A86Eb46D89e278B782ccAc26eEab57 |
| AlwaysAllowedModule     | 0x471584f0B8e35faEB2a618BD58A62316D8882d63 |
| MetafillerStorage       | 0x745bdeFA5C879ADCf739075bB03FD4ecCd03cE22 |
