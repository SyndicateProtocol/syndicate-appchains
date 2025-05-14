# NotInitializedModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/df30b030435a593e97b9e072bc9adc687b8fa1c4/src/sequencing-modules/NotInitializedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A module that returns false for all addresses


## Functions
### isAllowed

Returns false for all addresses


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```

