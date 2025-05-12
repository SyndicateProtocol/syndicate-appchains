# IRequirementModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/interfaces/IRequirementModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

Interface for requirement modules with both proposer and calldata validation

*Combines both proposer and calldata permission check capabilities*


## Functions
### addPermissionCheck

Adds a permission check address to the list


```solidity
function addPermissionCheck(address _address, bool addToHead) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to add|
|`addToHead`|`bool`|True to add to the head of the list, false to add to the tail|


### removePermissionCheck

Removes a permission check address from the list


```solidity
function removePermissionCheck(address _address) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_address`|`address`|The address of the check to remove|


### getAllPermissionChecks

Gets all permission check addresses


```solidity
function getAllPermissionChecks() external view returns (address[] memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address[]`|An array of all  check addresses|


