# AtomicSequencer
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/atomic-sequencer/AtomicSequencer.sol)

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

*This is a virtual function that should be overridden so it returns the address to which the fallback function
and {_fallback} should delegate.*


```solidity
function _implementation() internal view override returns (address);
```

