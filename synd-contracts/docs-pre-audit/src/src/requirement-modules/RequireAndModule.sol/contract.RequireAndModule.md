# RequireAndModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/requirement-modules/RequireAndModule.sol)

**Inherits:**
[BaseRequirementModule](/src/requirement-modules/BaseRequirementModule.sol/abstract.BaseRequirementModule.md)

A module that requires ALL checks to pass (AND logic)

*This contract implements strict permission logic where every module must approve*


## Functions
### constructor

Initializes the contract with an admin address


```solidity
constructor(address admin) BaseRequirementModule(admin);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address of the admin who can add/remove checks|


### isAllowed

Checks if a sender is allowed to submit a transaction

*Runs through all permission checks in the linked list - ALL must pass (AND logic)*


```solidity
function isAllowed(address msgSender, address txOrigin, bytes calldata data) external view override returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address of the sender to check|
|`txOrigin`|`address`|The address of tx.origin. Useful to know the sender originator in wrapper contracts|
|`data`|`bytes`|The calldata to be checked|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the sender passes all checks, reverts otherwise|


## Errors
### AndPermissionCheckFailed
Thrown when a permission check fails in AND logic


```solidity
error AndPermissionCheckFailed(address requireAddress, address msgSender, bytes data);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`requireAddress`|`address`|The address of the check that failed|
|`msgSender`|`address`|The address of the sender|
|`data`|`bytes`|The calldata that was being checked|

