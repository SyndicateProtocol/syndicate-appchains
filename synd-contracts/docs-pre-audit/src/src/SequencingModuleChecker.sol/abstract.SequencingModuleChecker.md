# SequencingModuleChecker
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/SequencingModuleChecker.sol)

**Inherits:**
Ownable, [IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A contract that delegates permission checks to modular permission systems

*This separation of concerns allows for flexible permission systems:
1. The SequencingModuleChecker manages the core permission interface
2. The permissionRequirementModule (typically RequireAndModule or RequireOrModule) handles the actual permission logic
3. This design allows for complex permission structures (AND/OR logic) that can be upgraded over time
4. The initialization pattern allows for proper setup of the permission system after deployment*


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

*This two-step initialization process allows for proper setup of the contract:
1. First deployed with a temporary admin (deployer)
2. Then initialized with the permanent admin and requirement module
3. Once initialized, it cannot be re-initialized (immutable pattern)*


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

Modifier to check if an address is allowed to submit txs based on the sender, origin and data


```solidity
modifier onlyWhenAllowed(address msgSender, address txOrigin, bytes calldata data);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address calling the function (msg.sender)|
|`txOrigin`|`address`|The address that initiated the transaction (tx.origin)|
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

### TransactionOrSenderNotAllowed

```solidity
error TransactionOrSenderNotAllowed();
```

### AlreadyInitialized

```solidity
error AlreadyInitialized();
```

