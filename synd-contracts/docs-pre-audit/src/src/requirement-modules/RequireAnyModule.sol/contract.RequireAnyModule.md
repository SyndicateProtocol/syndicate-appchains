# RequireAnyModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/df30b030435a593e97b9e072bc9adc687b8fa1c4/src/requirement-modules/RequireAnyModule.sol)

**Inherits:**
[IRequirementModule](/src/interfaces/IRequirementModule.sol/interface.IRequirementModule.md), Ownable

A module that requires at least one check to pass for either proposers, originators, or calldata

*This contract maintains a linked lists of check addresses*


## State Variables
### permissionChecks
List of permission checks addresses


```solidity
AddressStructuredLinkedList.List private permissionChecks;
```


## Functions
### constructor

Initializes the contract with an admin address


```solidity
constructor(address admin) Ownable(admin);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address of the admin who can add/remove checks|


### isAllowed

Checks if a proposer is allowed to submit a transaction

*Returns true if at least one check passes or if no checks exist*


```solidity
function isAllowed(address proposer, address originator, bytes calldata data) external view override returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`proposer`|`address`|The address of the proposer to check|
|`originator`|`address`|The address of tx.origin. Useful to know the sender originator in wrapper contracts|
|`data`|`bytes`|The calldata to be checked|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the proposer passes at least one check, reverts otherwise|


### addPermissionCheck

Adds permission check address to the list

*Can add to either the head or the tail of the list*


```solidity
function addPermissionCheck(address _address, bool addToHead) external override onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to add|
|`addToHead`|`bool`|True to add to the head of the list, false to add to the tail|


### removePermissionCheck

Removes permission check address from the list


```solidity
function removePermissionCheck(address _address) external override onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to remove|


### getAllPermissionChecks

Gets all permission check addresses


```solidity
function getAllPermissionChecks() external view override returns (address[] memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address[]`|An array of all proposer check addresses|


## Events
### PermissionCheckAdded
Emitted when a permission check is added


```solidity
event PermissionCheckAdded(address indexed check);
```

### PermissionCheckRemoved
Emitted when a permission check is removed


```solidity
event PermissionCheckRemoved(address indexed check);
```

## Errors
### CheckFailed
Thrown when all permission checks fail


```solidity
error CheckFailed(address proposer);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`proposer`|`address`|The address of the proposer|

### InvalidAddress
Thrown when an invalid address is provided


```solidity
error InvalidAddress();
```

### AddressAlreadyExists
Thrown when attempting to add an address that already exists


```solidity
error AddressAlreadyExists();
```

### AddressDoesNotExist
Thrown when attempting to remove an address that doesn't exist


```solidity
error AddressDoesNotExist();
```

