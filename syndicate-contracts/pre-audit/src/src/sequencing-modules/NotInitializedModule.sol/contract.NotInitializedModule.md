# NotInitializedModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/sequencing-modules/NotInitializedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A module that returns false for all addresses


## Functions
### isAllowed

Returns false for all addresses


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```

