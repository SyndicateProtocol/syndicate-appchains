# NotInitializedModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/cced719ff6d4998b665e130eebebe54b39f5cf15/src/sequencing-modules/NotInitializedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A module that returns false for all addresses


## Functions
### isAllowed

Returns false for all addresses


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```

