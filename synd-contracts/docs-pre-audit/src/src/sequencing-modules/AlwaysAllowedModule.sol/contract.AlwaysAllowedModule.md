# AlwaysAllowedModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/sequencing-modules/AlwaysAllowedModule.sol)

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


