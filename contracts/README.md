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
| MetabasedSequencerChain | 0x73Ba7D784d13Ec0070a4Ea6F49Ff57dc007Bb48d |
| AlwaysAllowedModule     | 0x0324b4d8F786e11206F82e016DD4480de2332cF3 |

#### Optimism

| Contract Name           | Address                                    |
| ----------------------- | ------------------------------------------ |
| MetabasedSequencerChain | 0x7DEdabB3Db89310B316bA49D96Aa6517aFC44294 |
| AlwaysAllowedModule     | 0x21e1C6bE46E53693381Cef9BECD3B0A947A590e4 |
