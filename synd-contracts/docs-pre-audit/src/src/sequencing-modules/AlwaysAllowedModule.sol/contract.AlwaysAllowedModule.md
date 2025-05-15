# AlwaysAllowedModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/sequencing-modules/AlwaysAllowedModule.sol)

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


