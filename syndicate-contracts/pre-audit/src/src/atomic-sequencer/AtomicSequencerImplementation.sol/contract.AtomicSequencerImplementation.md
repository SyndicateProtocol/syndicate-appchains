# AtomicSequencerImplementation
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/atomic-sequencer/AtomicSequencerImplementation.sol)

Implementation contract containing the logic for atomic sequencing


## Functions
### processTransactionsAtomically

Processes transactions on multiple Syndicate chains atomically.


```solidity
function processTransactionsAtomically(
    SyndicateSequencerChain[] calldata chains,
    bytes[] calldata transactions,
    bool[] calldata isRaw
) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chains`|`SyndicateSequencerChain[]`|Array of Syndicate chains|
|`transactions`|`bytes[]`|Array of transactions corresponding to each chain|
|`isRaw`|`bool[]`|Array indicating whether each transaction should use raw processing|


### processBulkTransactionsAtomically

Processes bulk transactions on multiple Syndicate chains atomically. Only used with encoded transactions.


```solidity
function processBulkTransactionsAtomically(SyndicateSequencerChain[] calldata chains, bytes[][] calldata transactions)
    external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chains`|`SyndicateSequencerChain[]`|Array of Syndicate chains|
|`transactions`|`bytes[][]`|Array of transaction arrays corresponding to each chain|


## Errors
### InputLengthMismatchError
*Thrown when input array lengths don't match or are invalid*


```solidity
error InputLengthMismatchError();
```

