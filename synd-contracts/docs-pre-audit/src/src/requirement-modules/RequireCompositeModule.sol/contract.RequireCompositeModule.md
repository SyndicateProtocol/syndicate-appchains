# RequireCompositeModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/requirement-modules/RequireCompositeModule.sol)

**Inherits:**
[BaseRequirementModule](/src/requirement-modules/BaseRequirementModule.sol/abstract.BaseRequirementModule.md)

A module that composes both AND and OR logic for flexible permission validation

*Builds upon BaseRequirementModule to add check type functionality*


## State Variables
### checkTypes
Mapping to store check types


```solidity
mapping(address => CheckType) public checkTypes;
```


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


### addPermissionCheckWithType

Adds a permission check with the specified type

*Adds to either the head or the tail of the list*


```solidity
function addPermissionCheckWithType(address _address, CheckType checkType, bool addToHead) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to add|
|`checkType`|`CheckType`|The type of check (AND or OR)|
|`addToHead`|`bool`|True to add to the head of the list, false to add to the tail|


### addPermissionCheck

Overrides the base addPermissionCheck to assign AND type by default


```solidity
function addPermissionCheck(address _address, bool addToHead) public override onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to add|
|`addToHead`|`bool`|True to add to the head of the list, false to add to the tail|


### removePermissionCheck

Overrides the base removePermissionCheck to clean up check type


```solidity
function removePermissionCheck(address _address) public override onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to remove|


### updateCheckType

Updates the check type for an existing permission check


```solidity
function updateCheckType(address _address, CheckType newCheckType) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to update|
|`newCheckType`|`CheckType`|The new check type (AND or OR)|


### getAllPermissionChecksWithTypes

Gets all permission check addresses with their types


```solidity
function getAllPermissionChecksWithTypes()
    external
    view
    returns (address[] memory addresses, CheckType[] memory types);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`addresses`|`address[]`|An array of all permission check addresses|
|`types`|`CheckType[]`|An array of check types corresponding to the addresses|


### isAllowed

Checks if a sender is allowed to submit a transaction using composite logic:
1. All AND checks must pass
2. At least one OR check must pass, if any exist


```solidity
function isAllowed(address msgSender, address txOrigin, bytes calldata data) external view override returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address of the sender to check|
|`txOrigin`|`address`|The address of tx.origin|
|`data`|`bytes`|The calldata to be checked|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if permission requirements are met|


## Events
### PermissionCheckAddedWithType
Emitted when a permission check is added with a specific type


```solidity
event PermissionCheckAddedWithType(address indexed check, CheckType indexed checkType);
```

### CheckTypeUpdated
Emitted when a check type is updated


```solidity
event CheckTypeUpdated(address indexed check, CheckType indexed oldType, CheckType indexed newType);
```

## Errors
### AndCheckFailed
Thrown when an AND permission check fails


```solidity
error AndCheckFailed(address requireAddress, address msgSender);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`requireAddress`|`address`|The address of the check that failed|
|`msgSender`|`address`|The address of the sender|

### AllOrChecksFailed
Thrown when all OR permission checks fail


```solidity
error AllOrChecksFailed(address msgSender);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address of the sender|

## Enums
### CheckType
Types of permission checks


```solidity
enum CheckType {
    AND,
    OR
}
```

