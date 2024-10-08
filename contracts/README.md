## Base Sequencer Chain Contracts

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

#### Syndicate Frame Chain

| Contract Name             | Address                                    |
| ------------------------- | ------------------------------------------ |
| BasedSequencerChain       | 0x8430FDed8bb66c6EA2f1f966E2abF9D481eEF418 |
| AllowlistSequencingModule | 0xA3d1304Afff72a8aD77F7c6A7B0c18d63629062d |

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
| MetabasedSequencerChain | 0xcFc46cEBB3eAEc9b5776e3FDe5879125B8BBA05d |
| AlwaysAllowedModule     | 0x471584f0B8e35faEB2a618BD58A62316D8882d63 |
| L3BackfillData          | 0xA67d13E3227B68bFc96DB9f5AFe7197ca32F0033 |
| L3BackfillStorage       | 0x10a54C8afdF6795f74fE3FE8DAaD339f35A1Dc49 |
| L3BackfillMapper        | 0x1194b2A8fFf08f6518AF7d66B072938E35C71706 |
