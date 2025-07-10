# SyndicateFactoryWrapper
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/factory/SyndicateFactoryWrapper.sol)

**Inherits:**
AccessControl, Pausable

Wrapper factory that deploys both permission modules and sequencing chains together

*Combines functionality from individual factories for a complete deployment experience*


## State Variables
### MANAGER_ROLE

```solidity
bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
```


### syndicateFactory

```solidity
SyndicateFactory public immutable syndicateFactory;
```


### requireAndFactory

```solidity
RequireAndModuleFactory public immutable requireAndFactory;
```


### requireOrFactory

```solidity
RequireOrModuleFactory public immutable requireOrFactory;
```


## Functions
### constructor


```solidity
constructor(address admin, address _syndicateFactory, address _requireAndFactory, address _requireOrFactory);
```

### deployCompleteSyndicate

Deploy a complete syndicate with permission module and sequencing chain


```solidity
function deployCompleteSyndicate(
    uint256 appchainId,
    address admin,
    ModuleType moduleType,
    bytes32 moduleSalt,
    bytes32 chainSalt
) public whenNotPaused returns (address sequencingChain, address permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`appchainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`admin`|`address`|The admin address for both contracts|
|`moduleType`|`ModuleType`|The type of permission module to deploy (And or Or)|
|`moduleSalt`|`bytes32`|The salt for the permission module deployment|
|`chainSalt`|`bytes32`|The salt for the sequencing chain deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The deployed sequencing chain address|
|`permissionModule`|`address`|The deployed permission module address|
|`actualChainId`|`uint256`|The chain ID that was used|


### computeCompleteSyndicateAddresses

Compute addresses for a complete syndicate deployment


```solidity
function computeCompleteSyndicateAddresses(
    address admin,
    ModuleType moduleType,
    bytes32 moduleSalt,
    bytes32 chainSalt,
    uint256 chainId
) external view returns (address permissionModuleAddress, address sequencingChainAddress);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The admin address for the module|
|`moduleType`|`ModuleType`|The type of permission module|
|`moduleSalt`|`bytes32`|The salt for the permission module|
|`chainSalt`|`bytes32`|The salt for the sequencing chain|
|`chainId`|`uint256`|The chain ID for the sequencing chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`permissionModuleAddress`|`address`|The computed permission module address|
|`sequencingChainAddress`|`address`|The computed sequencing chain address|


### getNextAutoChainId

Get the next auto-generated chain ID from the syndicate factory


```solidity
function getNextAutoChainId() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The next available chain ID|


### isChainIdUsed

Check if a chain ID has been used in the syndicate factory


```solidity
function isChainIdUsed(uint256 chainId) external view returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chainId`|`uint256`|The chain ID to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|1 if used, 0 if available|


### getNamespacePrefix

Get the namespace prefix from the syndicate factory


```solidity
function getNamespacePrefix() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The current namespace prefix|


### deployWithRequireAndModule

Deploy a syndicate with RequireAndModule


```solidity
function deployWithRequireAndModule(uint256 appchainId, address admin, bytes32 moduleSalt, bytes32 chainSalt)
    external
    whenNotPaused
    returns (address sequencingChain, address permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`appchainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`admin`|`address`|The admin address for both contracts|
|`moduleSalt`|`bytes32`|The salt for the permission module deployment|
|`chainSalt`|`bytes32`|The salt for the sequencing chain deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The deployed sequencing chain address|
|`permissionModule`|`address`|The deployed permission module address|
|`actualChainId`|`uint256`|The chain ID that was used|


### deployWithRequireOrModule

Deploy a syndicate with RequireOrModule


```solidity
function deployWithRequireOrModule(uint256 appchainId, address admin, bytes32 moduleSalt, bytes32 chainSalt)
    external
    whenNotPaused
    returns (address sequencingChain, address permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`appchainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`admin`|`address`|The admin address for both contracts|
|`moduleSalt`|`bytes32`|The salt for the permission module deployment|
|`chainSalt`|`bytes32`|The salt for the sequencing chain deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The deployed sequencing chain address|
|`permissionModule`|`address`|The deployed permission module address|
|`actualChainId`|`uint256`|The chain ID that was used|


### pause

Pause the wrapper factory (admin only)


```solidity
function pause() external onlyRole(DEFAULT_ADMIN_ROLE);
```

### unpause

Unpause the wrapper factory (admin only)


```solidity
function unpause() external onlyRole(DEFAULT_ADMIN_ROLE);
```

## Events
### CompleteSyndicateDeployed
Emitted when a complete syndicate deployment is created


```solidity
event CompleteSyndicateDeployed(
    uint256 indexed chainId,
    address indexed sequencingChain,
    address indexed permissionModule,
    ModuleType moduleType,
    address admin
);
```

## Errors
### ZeroAddress

```solidity
error ZeroAddress();
```

### InvalidModuleType

```solidity
error InvalidModuleType();
```

## Enums
### ModuleType
Types of permission modules supported


```solidity
enum ModuleType {
    RequireAnd,
    RequireOr
}
```

