# SyndicateSequencingChainWithDecayingPriority
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/extensions/SyndicateSequencingChainWithDecayingPriority.sol)

**Inherits:**
[SyndicateSequencingChain](/src/SyndicateSequencingChain.sol/contract.SyndicateSequencingChain.md)

Extension of SyndicateSequencingChain that implements a decaying priority mechanism for transactions.
This is only an example implementation. Not for production use.


## State Variables
### PRIORITY_DECAY_RATE
The constant rate at which priority decays (10 units per second)


```solidity
uint256 public constant PRIORITY_DECAY_RATE = 10;
```


## Functions
### constructor

Constructs the SyndicateSequencingChainWithDecayingPriority contract.


```solidity
constructor(uint256 _appchainId) SyndicateSequencingChain(_appchainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_appchainId`|`uint256`|The ID of the App chain that this contract is sequencing transactions for.|


### processTransaction

Processes a single compressed transaction with priority.


```solidity
function processTransaction(bytes calldata data, uint256 priority)
    external
    onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The compressed transaction data.|
|`priority`|`uint256`|The initial priority of the transaction.|


### processTransactionUncompressed

Processes an uncompressed transaction with a zero byte prepended.


```solidity
function processTransactionUncompressed(bytes calldata data, uint256 priority)
    external
    onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data without the prepended zero byte.|
|`priority`|`uint256`|The initial priority of the transaction|


### processTransactionsBulk

Processes multiple uncompressed transactions in bulk.


```solidity
function processTransactionsBulk(bytes[] calldata data, uint256[] calldata priorities) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|An array of transaction data without prepended zero bytes.|
|`priorities`|`uint256[]`|An array of priorities for the transactions.|


### calculateEffectivePriority

Calculate the effective priority after decay based on time elapsed

*This is just a read function for calculation. It is not used internally by the contract.*


```solidity
function calculateEffectivePriority(uint256 originalPriority, uint256 submittedTimestamp, uint256 currentTimestamp)
    public
    pure
    returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`originalPriority`|`uint256`|The original priority of the transaction|
|`submittedTimestamp`|`uint256`|The timestamp when the transaction was submitted|
|`currentTimestamp`|`uint256`|The current timestamp to calculate the decay against|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The effective priority after applying the decay formula|


## Events
### TransactionProcessed
Emitted when a new transaction is processed with priority and timestamp


```solidity
event TransactionProcessed(address indexed sender, bytes data, uint256 originalPriority, uint256 timestamp);
```

