# AtomicSequencer
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/cced719ff6d4998b665e130eebebe54b39f5cf15/src/atomic-sequencer/AtomicSequencer.sol)

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

