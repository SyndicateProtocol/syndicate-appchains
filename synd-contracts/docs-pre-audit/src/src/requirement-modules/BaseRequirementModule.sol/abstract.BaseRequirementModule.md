# BaseRequirementModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/requirement-modules/BaseRequirementModule.sol)

**Inherits:**
[IRequirementModule](/src/interfaces/IRequirementModule.sol/interface.IRequirementModule.md), Ownable

Base abstract contract for requirement modules that share common functionality

*Contains shared code for RequireAndModule and RequireOrModule*


## State Variables
### permissionChecks
List of permission checks addresses


```solidity
AddressStructuredLinkedList.List internal permissionChecks;
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


### addPermissionCheck

Adds permission check address to the list

*Can add to either the head or the tail of the list*


```solidity
function addPermissionCheck(address _address, bool addToHead) public virtual override onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to add|
|`addToHead`|`bool`|True to add to the head of the list, false to add to the tail|


### removePermissionCheck

Removes permission check address from the list


```solidity
function removePermissionCheck(address _address) public virtual override onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to remove|


### getAllPermissionChecks

Gets all permission check addresses


```solidity
function getAllPermissionChecks() external view virtual override returns (address[] memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address[]`|An array of all permission check addresses|


### isAllowed

Checks if a sender is allowed to submit a transaction

*Must be implemented by derived contracts (AND or OR logic)*


```solidity
function isAllowed(address msgSender, address txOrigin, bytes calldata data)
    external
    view
    virtual
    override
    returns (bool);
```

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

