# ISyndicateSequencerChain
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/interfaces/ISyndicateSequencerChain.sol)

Interface for the SyndicateSequencerChain contract


## Functions
### processTransaction

Process a transaction


```solidity
function processTransaction(bytes calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data to process|


### processTransactionRaw

Process a raw transaction


```solidity
function processTransactionRaw(bytes calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data to process|


### processBulkTransactions

Process bulk transactions


```solidity
function processBulkTransactions(bytes[] calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|The array of transaction data to process|


