# ISyndicateSequencerChain
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/interfaces/ISyndicateSequencerChain.sol)

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


