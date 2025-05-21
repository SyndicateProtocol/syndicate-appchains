# ISyndicateSequencingChain

[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/interfaces/ISyndicateSequencingChain.sol)

Interface for the SyndicateSequencingChain contract

## Functions

### processTransactionUncompressed

Process a transaction

```solidity
function processTransactionUncompressed(bytes calldata data) external;
```

**Parameters**

| Name   | Type    | Description                     |
| ------ | ------- | ------------------------------- |
| `data` | `bytes` | The transaction data to process |

### processTransaction

Process a raw transaction

```solidity
function processTransaction(bytes calldata data) external;
```

**Parameters**

| Name   | Type    | Description                     |
| ------ | ------- | ------------------------------- |
| `data` | `bytes` | The transaction data to process |

### processTransactionsBulk

Process bulk transactions

```solidity
function processTransactionsBulk(bytes[] calldata data) external;
```

**Parameters**

| Name   | Type      | Description                              |
| ------ | --------- | ---------------------------------------- |
| `data` | `bytes[]` | The array of transaction data to process |
