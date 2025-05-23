# ISyndicateSequencingChain
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/interfaces/ISyndicateSequencingChain.sol)

Interface for the SyndicateSequencingChain contract


## Functions
### processTransactionUncompressed

Processes an uncompressed transaction.

*It prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processTransactionUncompressed(bytes calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|An array of transaction data without prepended zero bytes.|


### processTransaction


```solidity
function processTransaction(bytes calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The compressed transaction data.|


### processTransactionsBulk

Process bulk transactions


```solidity
function processTransactionsBulk(bytes[] calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|The array of transaction data to process|


