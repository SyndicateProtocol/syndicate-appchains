# RequireOrModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/requirement-modules/RequireOrModule.sol)

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
### CheckFailed
Thrown when all permission checks fail


```solidity
error CheckFailed(address msgSender);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address of the sender|

