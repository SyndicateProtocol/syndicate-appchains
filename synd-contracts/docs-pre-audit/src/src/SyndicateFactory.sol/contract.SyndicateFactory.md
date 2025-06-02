# SyndicateFactory
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/SyndicateFactory.sol)

**Inherits:**
AccessControl

Factory contract for creating SyndicateSequencingChain and related contracts
with namespace management and auto-incrementing chain IDs


## State Variables
### MANAGER_ROLE

```solidity
bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
```


### _namespacePrefix

```solidity
uint256 private _namespacePrefix;
```


### _namespaceMultiplier

```solidity
uint256 private _namespaceMultiplier;
```


### _nextAutoChainId

```solidity
uint256 private _nextAutoChainId;
```


### _usedChainIds

```solidity
mapping(uint256 => bool) private _usedChainIds;
```


## Functions
### constructor


```solidity
constructor(address admin);
```

### zeroValuesChainAndTwoAddressesNotAllowed


```solidity
modifier zeroValuesChainAndTwoAddressesNotAllowed(uint256 appchainId, address firstAddrCheck, address secondAddrCheck);
```

### zeroValuesChainAndAddressNotAllowed


```solidity
modifier zeroValuesChainAndAddressNotAllowed(uint256 appchainId, address addrCheck);
```

### validateChainId


```solidity
modifier validateChainId(uint256 appchainId, bool isManuallySpecified);
```

### createSyndicateSequencingChain

Creates a new SyndicateSequencingChain contract with a permission module


```solidity
function createSyndicateSequencingChain(
    uint256 appchainId,
    address admin,
    IRequirementModule permissionModule,
    bytes32 salt
)
    public
    zeroValuesChainAndTwoAddressesNotAllowed(
        appchainId == 0 ? _getNextChainId() : appchainId,
        admin,
        address(permissionModule)
    )
    validateChainId(appchainId == 0 ? _getNextChainId() : appchainId, appchainId != 0)
    returns (address sequencingChain, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`appchainId`|`uint256`|The app chain the contract refers to (0 for auto-increment)|
|`admin`|`address`|The address that will be the admin|
|`permissionModule`|`IRequirementModule`|The address of the permission module|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The address of the newly created SyndicateSequencingChain|
|`actualChainId`|`uint256`|The chain ID that was used (auto-generated or specified)|


### createSyndicateSequencingChainWithRequireAndModule

Creates a SyndicateSequencingChain with RequireAndModule


```solidity
function createSyndicateSequencingChainWithRequireAndModule(address admin, uint256 appchainId, bytes32 salt)
    public
    zeroValuesChainAndAddressNotAllowed(appchainId == 0 ? _getNextChainId() : appchainId, admin)
    returns (address sequencingChain, IRequirementModule permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`appchainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The address of the newly created SyndicateSequencingChain|
|`permissionModule`|`IRequirementModule`|The address of the newly created RequireAndModule|
|`actualChainId`|`uint256`|The chain ID that was used (auto-generated or specified)|


### createSyndicateSequencingChainWithRequireOrModule

Creates a SyndicateSequencingChain with RequireOrModule


```solidity
function createSyndicateSequencingChainWithRequireOrModule(address admin, uint256 appchainId, bytes32 salt)
    public
    zeroValuesChainAndAddressNotAllowed(appchainId == 0 ? _getNextChainId() : appchainId, admin)
    returns (address sequencingChain, IRequirementModule permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`appchainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencingChain`|`address`|The address of the newly created SyndicateSequencingChain|
|`permissionModule`|`IRequirementModule`|The address of the newly created RequireOrModule|
|`actualChainId`|`uint256`|The chain ID that was used (auto-generated or specified)|


### computeSequencingChainAddress

Computes the address of the SyndicateSequencingChain contract


```solidity
function computeSequencingChainAddress(bytes32 salt, uint256 chainId) public view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`salt`|`bytes32`|The salt to use for the deployment|
|`chainId`|`uint256`|The ID of the app chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address of the SyndicateSequencingChain contract|


### getBytecode

Returns the bytecode of the SyndicateSequencingChain contract


```solidity
function getBytecode(uint256 chainId) public pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chainId`|`uint256`|The ID of the app chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|The bytecode of the SyndicateSequencingChain contract|


### _getNextChainId

Get the next available auto-generated chain ID in the namespace


```solidity
function _getNextChainId() internal view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The next available chain ID|


### getNextAutoChainId

Get the current next auto-incremented chain ID


```solidity
function getNextAutoChainId() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The current value of the auto-incrementing chain ID counter|


### isChainIdUsed

Check if a chain ID has already been used


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
|`<none>`|`uint256`|1 if the chain ID has been used, 0 otherwise|


### namespacePrefix

Get the current namespace prefix


```solidity
function namespacePrefix() public view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The current namespace prefix|


### namespaceMultiplier

Get the current namespace multiplier


```solidity
function namespaceMultiplier() public view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The current namespace multiplier|


### updateNamespaceConfig

Update the namespace configuration (manager only)


```solidity
function updateNamespaceConfig(uint256 newPrefix, uint256 newMultiplier) external onlyRole(MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`newPrefix`|`uint256`|The new namespace prefix|
|`newMultiplier`|`uint256`|The new namespace multiplier|


## Events
### SyndicateSequencingChainCreated
Emitted when a new SyndicateSequencingChain is created


```solidity
event SyndicateSequencingChainCreated(
    uint256 indexed appchainId, address indexed SyndicateSequencingChainAddress, address indexed permissionModuleAddress
);
```

### NamespaceConfigUpdated
Emitted when namespace configuration is updated


```solidity
event NamespaceConfigUpdated(
    uint256 oldNamespacePrefix,
    uint256 oldNamespaceMultiplier,
    uint256 newNamespacePrefix,
    uint256 newNamespaceMultiplier
);
```

## Errors
### ZeroAddress

```solidity
error ZeroAddress();
```

### ZeroValue

```solidity
error ZeroValue();
```

### ReservedNamespace

```solidity
error ReservedNamespace();
```

### ChainIdAlreadyExists

```solidity
error ChainIdAlreadyExists();
```

