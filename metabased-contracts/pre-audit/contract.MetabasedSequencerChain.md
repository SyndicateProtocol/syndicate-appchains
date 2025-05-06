# MetabasedSequencerChain

[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/MetabasedSequencerChain.sol)

**Inherits:**
[SequencingModuleChecker](/src/SequencingModuleChecker.sol/abstract.SequencingModuleChecker.md)

The core contract for sequencing transactions using a modular architecture
to determine who is allowed to sequence.

## State Variables

### appChainId

The ID of the L3 chain that this contract is sequencing transactions for.

```solidity
uint256 public immutable appChainId;
```

## Functions

### constructor

Constructs the MetabasedSequencerChain contract.

```solidity
constructor(uint256 _appChainId) SequencingModuleChecker();
```

**Parameters**

| Name          | Type      | Description                                                               |
| ------------- | --------- | ------------------------------------------------------------------------- |
| `_appChainId` | `uint256` | The ID of the L3 chain that this contract is sequencing transactions for. |

### processTransactionRaw

Processes a single compressed transaction.

```solidity
function processTransactionRaw(bytes calldata data) external onlyWhenAllowed(msg.sender);
```

**Parameters**

| Name   | Type    | Description                      |
| ------ | ------- | -------------------------------- |
| `data` | `bytes` | The compressed transaction data. |

### processTransaction

process transactions

_It prepends a zero byte to the transaction data to signal uncompressed data_

```solidity
function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender);
```

**Parameters**

| Name   | Type    | Description          |
| ------ | ------- | -------------------- |
| `data` | `bytes` | The transaction data |

### processBulkTransactions

Processes multiple transactions in bulk.

_It prepends a zero byte to the transaction data to signal uncompressed data_

```solidity
function processBulkTransactions(bytes[] calldata data) external onlyWhenAllowed(msg.sender);
```

**Parameters**

| Name   | Type      | Description                   |
| ------ | --------- | ----------------------------- |
| `data` | `bytes[]` | An array of transaction data. |

### prependZeroByte

Prepends a zero byte to the transaction data

_This helps op-translator identify uncompressed data_

```solidity
function prependZeroByte(bytes calldata _data) private pure returns (bytes memory);
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
