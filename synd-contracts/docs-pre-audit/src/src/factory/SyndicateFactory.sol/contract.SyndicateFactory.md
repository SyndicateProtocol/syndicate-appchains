# SyndicateFactory
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/factory/SyndicateFactory.sol)

**Inherits:**
AccessControl, Pausable

Factory contract for creating SyndicateSequencingChain contracts

*Uses CREATE2 pattern for deterministic deployments - users deploy permission modules separately*


## State Variables
### MANAGER_ROLE

```solidity
bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
```


### namespacePrefix

```solidity
uint256 public namespacePrefix;
```


### nextAutoChainId

```solidity
uint256 public nextAutoChainId;
```


### usedChainIds

```solidity
mapping(uint256 => bool) public usedChainIds;
```


## Functions
### constructor


```solidity
constructor(address admin);
```

### createSyndicateSequencingChain

Creates a new SyndicateSequencingChain contract


```solidity
function createSyndicateSequencingChain(
    uint256 appchainId,
    address admin,
    IRequirementModule permissionModule,
    bytes32 salt
) external whenNotPaused returns (address sequencingChain, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`appchainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`admin`|`address`|The admin address for the new chain|
|`permissionModule`|`IRequirementModule`|The pre-deployed permission module|
|`salt`|`bytes32`|The salt for CREATE2 deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The deployed sequencing chain address|
|`actualChainId`|`uint256`|The chain ID that was used|


### computeSequencingChainAddress

Computes the address where a sequencing chain will be deployed


```solidity
function computeSequencingChainAddress(bytes32 salt, uint256 chainId) external view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`salt`|`bytes32`|The salt for CREATE2 deployment|
|`chainId`|`uint256`|The chain ID|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The computed address|


### getBytecode

Returns the bytecode for deploying a SyndicateSequencingChain


```solidity
function getBytecode(uint256 chainId) public pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chainId`|`uint256`|The chain ID|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|The bytecode with constructor parameters|


### getNextChainId

Get the next auto-generated chain ID

*Chain ID calculation: concatenates namespacePrefix with nextAutoChainId*

*Example: with namespacePrefix=510 and nextAutoChainId=1, result is 5101*

*Example: with namespacePrefix=510 and nextAutoChainId=1000, result is 5101000*


```solidity
function getNextChainId() public view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The next available chain ID in the namespace|


### isChainIdUsed

Check if a chain ID has been used


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


### markChainIdAsUsed

Manually mark a chain ID as used to reserve it

*Useful for reserving specific chain IDs or marking externally deployed chains*


```solidity
function markChainIdAsUsed(uint256 chainId) external onlyRole(MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chainId`|`uint256`|The chain ID to mark as used|


### updateNamespaceConfig

Update namespace configuration (manager only)


```solidity
function updateNamespaceConfig(uint256 newPrefix) external onlyRole(MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`newPrefix`|`uint256`|The new namespace prefix|


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
### SyndicateSequencingChainCreated
Emitted when a new SyndicateSequencingChain is created


```solidity
event SyndicateSequencingChainCreated(
    uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
);
```

### NamespaceConfigUpdated
Emitted when namespace configuration is updated


```solidity
event NamespaceConfigUpdated(uint256 oldNamespacePrefix, uint256 newNamespacePrefix);
```

### ChainIdManuallyMarked
Emitted when a chain ID is manually marked as used


```solidity
event ChainIdManuallyMarked(uint256 indexed chainId);
```

## Errors
### ZeroAddress

```solidity
error ZeroAddress();
```

### ChainIdAlreadyExists

```solidity
error ChainIdAlreadyExists();
```

