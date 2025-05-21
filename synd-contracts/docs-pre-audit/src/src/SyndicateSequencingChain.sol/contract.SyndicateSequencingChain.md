# SyndicateSequencingChain

[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/SyndicateSequencingChain.sol)

**Inherits:**
[SequencingModuleChecker](/src/SequencingModuleChecker.sol/abstract.SequencingModuleChecker.md)

The core contract for sequencing transactions using a modular architecture
to determine the address that is allowed to sequence and whether the calldata is allowed.

## State Variables

### appChainId

The ID of the App chain that this contract is sequencing transactions for.

```solidity
uint256 public immutable appChainId;
```

## Functions

### constructor

Constructs the SyndicateSequencingChain contract.

```solidity
constructor(uint256 _appChainId) SequencingModuleChecker();
```

**Parameters**

| Name          | Type      | Description                                                                |
| ------------- | --------- | -------------------------------------------------------------------------- |
| `_appChainId` | `uint256` | The ID of the App chain that this contract is sequencing transactions for. |

### processTransaction

Processes a single compressed transaction.

```solidity
function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data);
```

**Parameters**

| Name   | Type    | Description                      |
| ------ | ------- | -------------------------------- |
| `data` | `bytes` | The compressed transaction data. |

### processTransactionUncompressed

process transactions

_It prepends a zero byte to the transaction data to signal uncompressed data_

```solidity
function processTransactionUncompressed(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data);
```

**Parameters**

| Name   | Type    | Description          |
| ------ | ------- | -------------------- |
| `data` | `bytes` | The transaction data |

### processTransactionsBulk

Processes multiple transactions in bulk.

_It prepends a zero byte to the transaction data to signal uncompressed data_

```solidity
function processTransactionsBulk(bytes[] calldata data) external;
```

**Parameters**

| Name   | Type      | Description                   |
| ------ | --------- | ----------------------------- |
| `data` | `bytes[]` | An array of transaction data. |

### prependZeroByte

Prepends a zero byte to the transaction data

_This helps op-translator identify uncompressed data_

```solidity
function prependZeroByte(bytes calldata _data) internal pure returns (bytes memory);
```

**Parameters**

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| `_data` | `bytes` | The original transaction data |

**Returns**

| Name     | Type    | Description                |
| -------- | ------- | -------------------------- |
| `<none>` | `bytes` | bytes The transaction data |

## Events

### TransactionProcessed

Emitted when a new transaction is processed.

```solidity
event TransactionProcessed(address indexed sender, bytes data);
```
