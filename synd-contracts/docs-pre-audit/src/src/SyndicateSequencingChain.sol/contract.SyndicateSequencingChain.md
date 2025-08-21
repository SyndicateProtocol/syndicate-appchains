# SyndicateSequencingChain
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/SyndicateSequencingChain.sol)

**Inherits:**
[SequencingModuleChecker](/src/SequencingModuleChecker.sol/abstract.SequencingModuleChecker.md)

The core contract for sequencing transactions using a modular permission architecture

*Transaction Lifecycle:
1. Transaction is submitted via processTransaction, processTransactionUncompressed, or processTransactionsBulk methods
2. onlyWhenAllowed modifier checks if the sender and transaction are allowed via SequencingModuleChecker
3. SequencingModuleChecker delegates to the configured permissionRequirementModule (RequireAll/RequireAny)
4. If allowed, a TransactionProcessed event is emitted with the sender and data
5. External systems observe these events to process the transactions on the application chain
This design uses events rather than state changes for scalability and gas efficiency*


## State Variables
### appchainId
The ID of the App chain that this contract is sequencing transactions for.


```solidity
uint256 public immutable appchainId;
```


## Functions
### constructor

Constructs the SyndicateSequencingChain contract.


```solidity
constructor(uint256 _appchainId) SequencingModuleChecker();
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_appchainId`|`uint256`|The ID of the App chain that this contract is sequencing transactions for.|


### processTransaction

Processes a compressed transaction.


```solidity
function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The compressed transaction data.|


### processTransactionUncompressed

Processes multiple uncompressed transactions in bulk.


```solidity
function processTransactionUncompressed(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|An array of transaction data without prepended zero bytes.|


### processTransactionsBulk

Processes multiple transactions in bulk.

*It prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processTransactionsBulk(bytes[] calldata data) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|An array of transaction data.|


### prependZeroByte

Prepends a zero byte to the transaction data

*This helps op-translator identify uncompressed data*


```solidity
function prependZeroByte(bytes calldata _data) public pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_data`|`bytes`|The original transaction data|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|bytes The transaction data with a zero byte prepended|


## Events
### TransactionProcessed
Emitted when a new transaction is processed


```solidity
event TransactionProcessed(address indexed sender, bytes data);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`sender`|`address`|The address that submitted the transaction|
|`data`|`bytes`|The transaction data that was processed|

