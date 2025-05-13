# AlwaysAllowedModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/sequencing-modules/AlwaysAllowedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

*Module used for testing and experimentations. It allows any proposer to send batch data.*


## Functions
### isAllowed

caller is allowed.


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if the caller is allowed.|


