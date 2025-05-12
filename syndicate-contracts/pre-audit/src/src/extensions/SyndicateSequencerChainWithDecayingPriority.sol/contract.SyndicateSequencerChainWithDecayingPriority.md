# SyndicateSequencerChainWithDecayingPriority
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/extensions/SyndicateSequencerChainWithDecayingPriority.sol)

**Inherits:**
[SyndicateSequencerChain](/src/SyndicateSequencerChain.sol/contract.SyndicateSequencerChain.md)

This is an example on how to extend the SyndicateSequencerChain.
It is not necessarily a feature implemented in Syndicate Sequencer.
It only serves as example and does not purport that this will be added in the future.


## State Variables
### PRIORITY_DECAY_RATE
The constant rate at which priority decays (10 units per second)


```solidity
uint256 public constant PRIORITY_DECAY_RATE = 10;
```


## Functions
### constructor

Constructs the SyndicateSequencerChainWithDecayingPriority contract.


```solidity
constructor(uint256 _appChainId) SyndicateSequencerChain(_appChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_appChainId`|`uint256`|The ID of the App chain that this contract is sequencing transactions for.|


### processTransactionRaw

Processes a single compressed transaction with priority.


```solidity
function processTransactionRaw(bytes calldata data, uint256 priority)
    external
    onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The compressed transaction data.|
|`priority`|`uint256`|The initial priority of the transaction.|


### processTransaction

Processes a single transaction with priority, prepending a zero byte.

*Prepends a zero byte to the transaction data to signal uncompressed data*


```solidity
function processTransaction(bytes calldata data, uint256 priority)
    external
    onlyWhenAllowed(msg.sender, tx.origin, data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data|
|`priority`|`uint256`|The initial priority of the transaction|


### processBulkTransactions

Processes multiple transactions in bulk with individual priorities.

*Prepends a zero byte to each transaction data to signal uncompressed data*


```solidity
function processBulkTransactions(bytes[] calldata data, uint256[] calldata priorities) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes[]`|An array of transaction data.|
|`priorities`|`uint256[]`|An array of priorities for the transactions.|


### calculateEffectivePriority

Calculate the effective priority after decay based on time elapsed

*This is just a read function for calculation. It is not used anywhere on purpose.*


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

