# NotInitializedModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/63941b4c3f2f1cd214f76245ed2d624869358aba/src/sequencing-modules/NotInitializedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A module that returns false for all addresses


## Functions
### isAllowed

Returns false for all addresses


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```

