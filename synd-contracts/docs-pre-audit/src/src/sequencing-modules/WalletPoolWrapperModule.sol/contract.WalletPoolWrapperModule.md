# WalletPoolWrapperModule

[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/sequencing-modules/WalletPoolWrapperModule.sol)

**Inherits:**
[AllowlistSequencingModule](/src/sequencing-modules/AllowlistSequencingModule.sol/contract.AllowlistSequencingModule.md)

_This contract is a wrapper for TC wallet, inheriting from the AllowlistSequencingModule._

## Functions

### constructor

_Constructor that sets the admin address._

```solidity
constructor(address _admin) AllowlistSequencingModule(_admin);
```

**Parameters**

| Name     | Type      | Description                                            |
| -------- | --------- | ------------------------------------------------------ |
| `_admin` | `address` | The address of the admin who can modify the allowlist. |

### onlyAllowed

_Modifier to check if the caller is allowed to process transactions._

```solidity
modifier onlyAllowed();
```

### SyndicateSequencingChainNotZero

_Modifier to check if the syndicate sequencer chain address is not zero._

```solidity
modifier SyndicateSequencingChainNotZero(address _SyndicateSequencingChain);
```

**Parameters**

| Name                        | Type      | Description                           |
| --------------------------- | --------- | ------------------------------------- |
| `_SyndicateSequencingChain` | `address` | The syndicate sequencer chain address |

### processTransactionUncompressed

_Function to process a transaction._

```solidity
function processTransactionUncompressed(address _SyndicateSequencingChain, bytes calldata data)
    external
    onlyAllowed
    SyndicateSequencingChainNotZero(_SyndicateSequencingChain);
```

**Parameters**

| Name                        | Type      | Description                           |
| --------------------------- | --------- | ------------------------------------- |
| `_SyndicateSequencingChain` | `address` | The syndicate sequencer chain address |
| `data`                      | `bytes`   | The transaction data to process.      |

### processTransaction

_Function to process a raw transaction._

```solidity
function processTransaction(address _SyndicateSequencingChain, bytes calldata data)
    external
    onlyAllowed
    SyndicateSequencingChainNotZero(_SyndicateSequencingChain);
```

**Parameters**

| Name                        | Type      | Description                           |
| --------------------------- | --------- | ------------------------------------- |
| `_SyndicateSequencingChain` | `address` | The syndicate sequencer chain address |
| `data`                      | `bytes`   | The transaction data to process.      |

### processTransactionsBulk

_Function to process bulk transactions._

```solidity
function processTransactionsBulk(address _SyndicateSequencingChain, bytes[] calldata data)
    external
    onlyAllowed
    SyndicateSequencingChainNotZero(_SyndicateSequencingChain);
```

**Parameters**

| Name                        | Type      | Description                               |
| --------------------------- | --------- | ----------------------------------------- |
| `_SyndicateSequencingChain` | `address` | The syndicate sequencer chain address     |
| `data`                      | `bytes[]` | The array of transaction data to process. |

## Events

### WalletPoolWrapperTransactionSent

```solidity
event WalletPoolWrapperTransactionSent(address indexed from, address indexed SyndicateSequencingChain);
```

### WalletPoolWrapperBulkTransactionsSent

```solidity
event WalletPoolWrapperBulkTransactionsSent(
    address indexed from, address indexed SyndicateSequencingChain, uint256 count
);
```

## Errors

### ZeroSequencerAddressNotAllowed

```solidity
error ZeroSequencerAddressNotAllowed();
```
