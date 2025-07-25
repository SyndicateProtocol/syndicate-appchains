# RequireCompositeModuleFactory
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/factory/PermissionModuleFactories.sol)

**Inherits:**
AccessControl, Pausable

Factory for deploying RequireCompositeModule contracts using CREATE2 pattern


## State Variables
### MANAGER_ROLE

```solidity
bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
```


## Functions
### constructor


```solidity
constructor(address admin);
```

### createRequireCompositeModule

Deploy a new RequireCompositeModule using CREATE2


```solidity
function createRequireCompositeModule(address admin, bytes32 salt) external whenNotPaused returns (address module);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The admin address for the module|
|`salt`|`bytes32`|The salt for deterministic deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`module`|`address`|The deployed module address|


### computeModuleAddress

Compute the address where a module will be deployed


```solidity
function computeModuleAddress(address admin, bytes32 salt) external view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The admin address for the module|
|`salt`|`bytes32`|The salt for deterministic deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The computed address|


### pause

Pause the factory (admin only)


```solidity
function pause() external onlyRole(DEFAULT_ADMIN_ROLE);
```

### unpause

Unpause the factory (admin only)


```solidity
function unpause() external onlyRole(DEFAULT_ADMIN_ROLE);
```

## Events
### RequireCompositeModuleCreated
Emitted when a new RequireCompositeModule is created


```solidity
event RequireCompositeModuleCreated(address indexed module, address indexed admin);
```

## Errors
### ZeroAddress

```solidity
error ZeroAddress();
```

