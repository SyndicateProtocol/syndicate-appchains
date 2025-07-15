# AlwaysAllowedModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/sequencing-modules/AlwaysAllowedModule.sol)

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


