# SequencingModuleChecker
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/SequencingModuleChecker.sol)

**Inherits:**
Ownable, [PermissionModule](/src/interfaces/PermissionModule.sol/interface.PermissionModule.md)

A simplified contract that delegates permission checks to modules


## State Variables
### requirementModule
The requirement module that handles checks


```solidity
PermissionModule public requirementModule;
```


### hasBeenInitialized

```solidity
bool internal hasBeenInitialized = false;
```


## Functions
### constructor

*Constructor function*


```solidity
constructor() Ownable(msg.sender);
```

### initialize

Initializes the contract with a new admin and a requirement module


```solidity
function initialize(address admin, address _requirementModule) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address of the new admin|
|`_requirementModule`|`address`|The address of the requirement module|


### updateRequirementModule

Updates the requirement module


```solidity
function updateRequirementModule(address _newModule) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_newModule`|`address`|The address of the new requirement module|


### onlyWhenAllowed

Checks if an address is allowed to submit batches


```solidity
modifier onlyWhenAllowed(address batchSubmitter);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`batchSubmitter`|`address`|The address to check|


### isAllowed

Implementation of the PermissionModule interface


```solidity
function isAllowed(address proposer) public view virtual override returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`proposer`|`address`|The address to check permissions for|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if the address is allowed|


## Events
### RequirementModuleUpdated

```solidity
event RequirementModuleUpdated(address indexed newModule);
```

## Errors
### InvalidModuleAddress

```solidity
error InvalidModuleAddress();
```

### NotAllowed

```solidity
error NotAllowed(address batchSubmitter);
```

### AlreadyInitialized

```solidity
error AlreadyInitialized();
```

