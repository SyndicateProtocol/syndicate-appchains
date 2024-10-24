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
| MetabasedFactory        | 0x2f3CC13661A95DD616311a7bddB9cDB4bA67C4d2 |
| MetabasedSequencerChain | 0x2945b20521A86Eb46D89e278B782ccAc26eEab57 |
| AlwaysAllowedModule     | 0x33aB24E0a47A7aAe869755420950A6326e3CB9F3 |
| MetafillerStorage       | 0x745bdeFA5C879ADCf739075bB03FD4ecCd03cE22 |

#### Optimism

| Contract Name           | Address                                    |
| ----------------------- | ------------------------------------------ |
| MetabasedSequencerChain | 0x7DEdabB3Db89310B316bA49D96Aa6517aFC44294 |
| AlwaysAllowedModule     | 0x21e1C6bE46E53693381Cef9BECD3B0A947A590e4 |

#### Metabased Testnet

| Contract Name    | Address                                    |
| ---------------- | ------------------------------------------ |
| MetabasedFactory | 0x9a0Ef1333681b357047282144dc06D7DAA1f76Ba |

Contracts for Testing with Dummy L3 Chain ID `42424242424242424242`
| MetabasedSequencerChain | 0x81221231E4552Ac34c65DCc04EE9869767E65707 |
| AlwaysAllowedModule | 0x471584f0B8e35faEB2a618BD58A62316D8882d63 |
| MetafillerStorage | 0xC329B96c47271426B237bA85dF5504375C5cCB28 |
