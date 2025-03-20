# MetabasedSequencerChain
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/MetabasedSequencerChain.sol)

**Inherits:**
[SequencingModuleChecker](/src/SequencingModuleChecker.sol/abstract.SequencingModuleChecker.md)

The core contract for sequencing transactions using a modular architecture
to determine who is allowed to sequence.


## State Variables
### l3ChainId
The ID of the L3 chain that this contract is sequencing transactions for.


```solidity
uint256 public immutable l3ChainId;
```


## Functions
### constructor

Constructs the MetabasedSequencerChain contract.


```solidity
constructor(uint256 _l3ChainId) SequencingModuleChecker();
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_l3ChainId`|`uint256`|The ID of the L3 chain that this contract is sequencing transactions for.|


### processTransactionRaw

Processes a single compressed transaction.


```solidity
function processTransactionRaw(bytes calldata data) external onlyWhenAllowed(msg.sender);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The compressed transaction data.|


### processTransaction

process transactions

*It prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data|


### processBulkTransactions

Processes multiple transactions in bulk.

*It prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processBulkTransactions(bytes[] calldata data) external onlyWhenAllowed(msg.sender);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|An array of transaction data.|


### prependZeroByte

Prepends a zero byte to the transaction data

*This helps op-translator identify uncompressed data*


```solidity
function prependZeroByte(bytes calldata _data) private pure returns (bytes memory);
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

