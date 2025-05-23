# SyndicateAccumulator
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/SyndicateAccumulator.sol)

A contract that maintains a cryptographic accumulator of processed transactions

*The accumulator creates a tamper-evident chain of transaction hashes that includes:
- Previous accumulator state
- Transaction sender
- Block number (Arbitrum-aware)
- Block timestamp
- Transaction data hash
This provides a verifiable history of all processed transactions for off-chain verification*


## State Variables
### isArbChain
Whether this contract is deployed on an Arbitrum chain

*Determined at construction time and affects block number retrieval*


```solidity
bool public immutable isArbChain;
```


### ACCUMULATOR_STORAGE_LOCATION
Storage slot for the accumulator state

*keccak256("syndicate.accumulator") - prevents storage collisions*


```solidity
bytes32 public constant ACCUMULATOR_STORAGE_LOCATION =
    0x847fe1a0bfd701c2dbb0b62670ad8712eed4c0ff4d2c6c0917f4c8d260ed0b90;
```


## Functions
### constructor

Constructs the SyndicateAccumulator contract

*Detects if running on Arbitrum by checking for the ArbSys precompile
and validates it's working correctly if present*


```solidity
constructor();
```

### _getAccumulator

Retrieves the accumulator storage reference

*Uses inline assembly to access the custom storage slot*


```solidity
function _getAccumulator() private pure returns (Accumulator storage $);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`$`|`Accumulator`|Reference to the Accumulator storage struct|


### accumulator

Returns the current accumulator hash value

*Public view function to read the current state of the accumulator*


```solidity
function accumulator() public view returns (bytes32);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|The current accumulator hash as bytes32|


### _transactionProcessed

Processes a transaction and updates the accumulator

*Internal function that:
1. Gets the appropriate block number (Arbitrum-aware)
2. Creates a hash of transaction metadata
3. Updates the accumulator with chained hash
4. Emits a TransactionProcessed event*


```solidity
function _transactionProcessed(bytes memory data) internal;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`data`|`bytes`|The transaction data to be processed and accumulated|


## Events
### TransactionProcessed
Emitted when a new transaction is processed and added to the accumulator


```solidity
event TransactionProcessed(address indexed sender, bytes data);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`sender`|`address`|The address that submitted the transaction|
|`data`|`bytes`|The transaction data that was processed|

## Structs
### Accumulator
Storage structure for the accumulator state

*Uses a custom storage slot to avoid storage collisions in inheritance*


```solidity
struct Accumulator {
    bytes32 acc;
}
```

