# NotInitializedModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/sequencing-modules/NotInitializedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A module that returns false for all addresses


## Functions
### isAllowed

Returns false for all addresses


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```

