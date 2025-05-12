# SequencingModuleChecker
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/cced719ff6d4998b665e130eebebe54b39f5cf15/src/SequencingModuleChecker.sol)

**Inherits:**
Ownable, [IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A simplified contract that delegates permission checks to modules


## State Variables
### permissionRequirementModule
The requirement module that handles checks


```solidity
IPermissionModule public permissionRequirementModule;
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
function initialize(address admin, address _permissionRequirementModule) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address of the new admin|
|`_permissionRequirementModule`|`address`|The address of the RequireAll or RequireAny module|


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

Modifier to checks if an address is allowed to submit txs based on the sender, origin and data


```solidity
modifier onlyWhenAllowed(address proposer, address originator, bytes calldata data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`proposer`|`address`|The address to check|
|`originator`|`address`|The address of tx.origin. Useful to know the sender originator in wrapper contracts|
|`data`|`bytes`|The calldata to check|


### isAllowed

Checks if both the proposer and calldata are allowed


```solidity
function isAllowed(address proposer, address originator, bytes calldata data) public view returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`proposer`|`address`|The address to check|
|`originator`|`address`|The address of tx.origin.|
|`data`|`bytes`|The calldata to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if both the proposer and calldata are allowed|


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

### TransactionOrProposerNotAllowed

```solidity
error TransactionOrProposerNotAllowed();
```

### AlreadyInitialized

```solidity
error AlreadyInitialized();
```

