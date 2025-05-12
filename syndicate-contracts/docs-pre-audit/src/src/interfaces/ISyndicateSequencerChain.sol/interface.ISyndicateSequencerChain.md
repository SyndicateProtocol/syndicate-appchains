# ISyndicateSequencerChain
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/cced719ff6d4998b665e130eebebe54b39f5cf15/src/interfaces/ISyndicateSequencerChain.sol)

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


