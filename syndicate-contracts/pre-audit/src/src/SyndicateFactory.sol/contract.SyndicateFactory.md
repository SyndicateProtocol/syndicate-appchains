# SyndicateFactory
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/SyndicateFactory.sol)

**Inherits:**
AccessControl

Factory contract for creating SyndicateSequencerChain and related contracts
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
modifier zeroValuesChainAndTwoAddressesNotAllowed(uint256 appChainId, address firstAddrCheck, address secondAddrCheck);
```

### zeroValuesChainAndAddressNotAllowed


```solidity
modifier zeroValuesChainAndAddressNotAllowed(uint256 appChainId, address addrCheck);
```

### validateChainId


```solidity
modifier validateChainId(uint256 appChainId, bool isManuallySpecified);
```

### createSyndicateSequencerChain

Creates a new SyndicateSequencerChain contract with a permission module


```solidity
function createSyndicateSequencerChain(
    uint256 appChainId,
    address admin,
    IRequirementModule permissionModule,
    bytes32 salt
)
    public
    zeroValuesChainAndTwoAddressesNotAllowed(
        appChainId == 0 ? _getNextChainId() : appChainId,
        admin,
        address(permissionModule)
    )
    validateChainId(appChainId == 0 ? _getNextChainId() : appChainId, appChainId != 0)
    returns (address sequencerChain, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`appChainId`|`uint256`|the app chain the contract refers to (0 for auto-increment)|
|`admin`|`address`|The address that will be the admin|
|`permissionModule`|`IRequirementModule`|The address of the permission module|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencerChain`|`address`|The address of the newly created SyndicateSequencerChain|
|`actualChainId`|`uint256`|The chain ID that was used (auto-generated or specified)|


### createSyndicateSequencerChainWithRequireAllModule

Creates SyndicateSequencerChain with RequireAllModule


```solidity
function createSyndicateSequencerChainWithRequireAllModule(address admin, uint256 appChainId, bytes32 salt)
    public
    zeroValuesChainAndAddressNotAllowed(appChainId == 0 ? _getNextChainId() : appChainId, admin)
    returns (address sequencerChain, IRequirementModule permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`appChainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencerChain`|`address`|The address of the newly created SyndicateSequencerChain|
|`permissionModule`|`IRequirementModule`|The address of the newly created RequireAllModule|
|`actualChainId`|`uint256`|The chain ID that was used (auto-generated or specified)|


### createSyndicateSequencerChainWithRequireAnyModule

Creates SyndicateSequencerChain with RequireAnyModule


```solidity
function createSyndicateSequencerChainWithRequireAnyModule(address admin, uint256 appChainId, bytes32 salt)
    public
    zeroValuesChainAndAddressNotAllowed(appChainId == 0 ? _getNextChainId() : appChainId, admin)
    returns (address sequencerChain, IRequirementModule permissionModule, uint256 actualChainId);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`appChainId`|`uint256`|The app chain ID (0 for auto-increment)|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencerChain`|`address`|The address of the newly created SyndicateSequencerChain|
|`permissionModule`|`IRequirementModule`|The address of the newly created RequireAnyModule|
|`actualChainId`|`uint256`|The chain ID that was used (auto-generated or specified)|


### computeSequencerChainAddress

Computes the address of the SyndicateSequencerChain contract


```solidity
function computeSequencerChainAddress(bytes32 salt, uint256 chainId) public view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`salt`|`bytes32`|The salt to use for the deployment|
|`chainId`|`uint256`|The ID of the app chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address of the SyndicateSequencerChain contract|


### getBytecode

Returns the bytecode of the SyndicateSequencerChain contract


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
|`<none>`|`bytes`|The bytecode of the SyndicateSequencerChain contract|


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
### SyndicateSequencerChainCreated
Emitted when a new SyndicateSequencerChain is created


```solidity
event SyndicateSequencerChainCreated(
    uint256 indexed appChainId, address indexed syndicateSequencerChainAddress, address indexed permissionModuleAddress
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

