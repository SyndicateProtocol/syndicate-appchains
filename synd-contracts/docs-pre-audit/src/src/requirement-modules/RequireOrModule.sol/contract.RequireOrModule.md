# RequireOrModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/requirement-modules/RequireOrModule.sol)

**Inherits:**
[BaseRequirementModule](/src/requirement-modules/BaseRequirementModule.sol/abstract.BaseRequirementModule.md)

A module that requires ANY check to pass (OR logic)

*This contract implements permissive permission logic where any module can approve*


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

*Returns true if at least one check passes or if no checks exist (OR logic)*


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
|`<none>`|`bool`|True if the sender passes at least one check, reverts otherwise|


## Errors
### AllOrPermissionChecksFailed
Thrown when all permission checks fail in OR logic


```solidity
error AllOrPermissionChecksFailed(address msgSender, bytes data);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address of the sender|
|`data`|`bytes`|The calldata that was being checked|

