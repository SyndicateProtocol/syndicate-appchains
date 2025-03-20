# AllowlistSequencingModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/sequencing-modules/AllowlistSequencingModule.sol)

**Inherits:**
[PermissionModule](/src/interfaces/PermissionModule.sol/interface.PermissionModule.md)

*This contract implements an allowlist mechanism to control access to sequencing.*


## State Variables
### admin
The address of the admin who can modify the allowlist.


```solidity
address public admin;
```


### allowlist
Mapping to store allowed addresses.


```solidity
mapping(address user => bool isAllowed) public allowlist;
```


## Functions
### constructor

*Sets the deployer as the initial admin.*


```solidity
constructor(address _admin);
```

### onlyAdmin

*Modifier to check if the caller is the admin.*


```solidity
modifier onlyAdmin();
```

### addToAllowlist

Adds an address to the allowlist.


```solidity
function addToAllowlist(address user) external onlyAdmin;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`user`|`address`|The address to be added to the allowlist.|


### removeFromAllowlist

Removes an address from the allowlist.


```solidity
function removeFromAllowlist(address user) external onlyAdmin;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`user`|`address`|The address to be removed from the allowlist.|


### transferAdmin

Transfers the admin role to a new address.


```solidity
function transferAdmin(address newAdmin) external onlyAdmin;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`newAdmin`|`address`|The address of the new admin. Cannot be address(0).|


### isAllowed

Checks if the caller is allowed.


```solidity
function isAllowed(address proposer) external view override returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if the caller is allowed.|


## Events
### UserAdded

```solidity
event UserAdded(address indexed user);
```

### UserRemoved

```solidity
event UserRemoved(address indexed user);
```

### AdminTransferred

```solidity
event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);
```

## Errors
### NotAdmin

```solidity
error NotAdmin();
```

### AddressNotAllowed

```solidity
error AddressNotAllowed();
```

