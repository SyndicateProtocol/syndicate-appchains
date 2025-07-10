# AtomicSequencerImplementation
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/atomic-sequencer/AtomicSequencerImplementation.sol)

Implementation contract containing the logic for atomic sequencing


## Functions
### processTransactionsAtomically

Processes transactions on multiple Syndicate chains atomically.


```solidity
function processTransactionsAtomically(
    SyndicateSequencingChain[] calldata chains,
    bytes[] calldata transactions,
    bool[] calldata isRaw
) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chains`|`SyndicateSequencingChain[]`|Array of Syndicate chains|
|`transactions`|`bytes[]`|Array of transactions corresponding to each chain|
|`isRaw`|`bool[]`|Array indicating whether each transaction should use raw processing|


### processTransactionsBulkAtomically

Processes bulk transactions on multiple Syndicate chains atomically. Only used with encoded transactions.


```solidity
function processTransactionsBulkAtomically(SyndicateSequencingChain[] calldata chains, bytes[][] calldata transactions)
    external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chains`|`SyndicateSequencingChain[]`|Array of Syndicate chains|
|`transactions`|`bytes[][]`|Array of transaction arrays corresponding to each chain|


## Errors
### InputLengthMismatchError
*Thrown when input array lengths don't match or are invalid*


```solidity
error InputLengthMismatchError();
```

