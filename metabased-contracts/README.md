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
| MetabasedFactory                | 0xc75954B9B4Bb4B80883Cf645744612138b7e4870 |
| AlwaysAllowedModule             | 0x471584f0B8e35faEB2a618BD58A62316D8882d63 |
| MetabasedSequencerChain (Frame) | 0x2d45AA77567618C031b8C8AFD3296ED724793B01 |
| MetabsedSequencerChain (Ham)    | 0x869eE769d6ab3221894Cc555792e8c6467953bE1 |
| MetafillerStorage (Frame)       | 0x04a1D138d681De93412581673a993eD7878EaBC4 |
| MetafillerStorage (Ham)         | 0x037A5BC4DB3C5c4Be90d8C3D7eaD6B2B231b9EFb |
