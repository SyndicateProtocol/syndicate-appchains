# ISyndicateSequencerChain
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/interfaces/ISyndicateSequencerChain.sol)

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


