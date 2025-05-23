# AtomicSequencer
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/atomic-sequencer/AtomicSequencer.sol)

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

