# AtomicSequencerImplementation
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/atomic-sequencer/AtomicSequencerImplementation.sol)

Implementation contract containing the logic for atomic sequencing


## Functions
### processTransactionsAtomically

Processes transactions on multiple Metabased chains atomically.


```solidity
function processTransactionsAtomically(
    MetabasedSequencerChain[] calldata chains,
    bytes[] calldata transactions,
    bool[] calldata isRaw
) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chains`|`MetabasedSequencerChain[]`|Array of Metabased chains|
|`transactions`|`bytes[]`|Array of transactions corresponding to each chain|
|`isRaw`|`bool[]`|Array indicating whether each transaction should use raw processing|


### processBulkTransactionsAtomically

Processes bulk transactions on multiple Metabased chains atomically. Only used with encoded transactions.


```solidity
function processBulkTransactionsAtomically(MetabasedSequencerChain[] calldata chains, bytes[][] calldata transactions)
    external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chains`|`MetabasedSequencerChain[]`|Array of Metabased chains|
|`transactions`|`bytes[][]`|Array of transaction arrays corresponding to each chain|


## Errors
### InputLengthMismatchError
*Thrown when input array lengths don't match or are invalid*


```solidity
error InputLengthMismatchError();
```

