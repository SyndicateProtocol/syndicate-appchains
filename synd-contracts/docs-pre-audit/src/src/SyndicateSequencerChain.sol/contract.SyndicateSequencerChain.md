# SyndicateSequencerChain
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/SyndicateSequencerChain.sol)

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

Constructs the SyndicateSequencerChain contract.


```solidity
constructor(uint256 _appChainId) SequencingModuleChecker();
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_appChainId`|`uint256`|The ID of the App chain that this contract is sequencing transactions for.|


### processTransactionRaw

Processes a single compressed transaction.


```solidity
function processTransactionRaw(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The compressed transaction data.|


### processTransaction

process transactions

*It prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data|


### processBulkTransactions

Processes multiple transactions in bulk.

*It prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processBulkTransactions(bytes[] calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|An array of transaction data.|


### prependZeroByte

Prepends a zero byte to the transaction data

*This helps op-translator identify uncompressed data*


```solidity
function prependZeroByte(bytes calldata _data) internal pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_data`|`bytes`|The original transaction data|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|bytes The transaction data|


## Events
### TransactionProcessed
Emitted when a new transaction is processed.


```solidity
event TransactionProcessed(address indexed sender, bytes data);
```

