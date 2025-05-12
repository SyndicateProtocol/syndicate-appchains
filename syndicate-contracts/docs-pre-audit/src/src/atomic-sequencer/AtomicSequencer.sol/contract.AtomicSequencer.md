# AtomicSequencer
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/atomic-sequencer/AtomicSequencer.sol)

**Inherits:**
Proxy

A wrapper contract that sequences transactions on two SyndicateChain contracts atomically.
Uses DELEGATECALL to maintain the original msg.sender context.


## State Variables
### implementation
The implementation contract containing the sequencing logic


```solidity
address public immutable implementation;
```


## Functions
### constructor


```solidity
constructor();
```

### _implementation

*This is a virtual function that should be overridden so it returns the address to which the fallback
function and {_fallback} should delegate.*


```solidity
function _implementation() internal view override returns (address);
```

