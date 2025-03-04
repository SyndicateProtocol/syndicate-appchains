# MetabasedFactory
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/MetabasedFactory.sol)

Factory contract for creating MetabasedSequencerChain and related contracts


## Functions
### zeroValuesNotAllowed


```solidity
modifier zeroValuesNotAllowed(uint256 l3ChainId, address firstAddrCheck, address secondAddrCheck);
```

### createMetabasedSequencerChain

Creates a new MetabasedSequencerChain contract with a permission module


```solidity
function createMetabasedSequencerChain(
    uint256 l3ChainId,
    address admin,
    IRequirementModule permissionModule,
    bytes32 salt
) public zeroValuesNotAllowed(l3ChainId, admin, address(permissionModule)) returns (address sequencerChain);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`l3ChainId`|`uint256`|the l3 chain the contract refers to|
|`admin`|`address`|The address that will be the admin|
|`permissionModule`|`IRequirementModule`|The address of the permission module|
|`salt`|`bytes32`|The salt to use for the deployment, this should be the l3ChainId if it has not been previously used|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencerChain`|`address`|The address of the newly created MetabasedSequencerChain|


### createMetafillerStorage

Creates a new MetafillerStorage contract


```solidity
function createMetafillerStorage(address admin, address manager, uint256 l3ChainId)
    public
    zeroValuesNotAllowed(l3ChainId, admin, manager)
    returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`manager`|`address`|The address that will be the manager role|
|`l3ChainId`|`uint256`|The L3 chain ID|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address of the newly created MetafillerStorage contract|


### createAllContractsWithRequireAllModule

Creates all contracts: MetabasedSequencerChain, RequireAllModule, and MetafillerStorage


```solidity
function createAllContractsWithRequireAllModule(address admin, address manager, uint256 l3ChainId, bytes32 salt)
    public
    zeroValuesNotAllowed(l3ChainId, admin, manager)
    returns (address sequencerChain, address metafillerStorage, IRequirementModule permissionModule);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`manager`|`address`|The address that will be the manager role for MetafillerStorage|
|`l3ChainId`|`uint256`|The L3 chain ID|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencerChain`|`address`|The address of the newly created MetabasedSequencerChain|
|`metafillerStorage`|`address`|The address of the newly created MetafillerStorage|
|`permissionModule`|`IRequirementModule`|The address of the newly created RequireAllModule|


### createAllContractsWithRequireAnyModule

Creates all contracts: MetabasedSequencerChain, RequireAnyModule, and MetafillerStorage


```solidity
function createAllContractsWithRequireAnyModule(address admin, address manager, uint256 l3ChainId, bytes32 salt)
    public
    zeroValuesNotAllowed(l3ChainId, admin, manager)
    returns (address sequencerChain, address metafillerStorage, IRequirementModule permissionModule);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|The address that will be the default admin role|
|`manager`|`address`|The address that will be the manager role for MetafillerStorage|
|`l3ChainId`|`uint256`|The L3 chain ID|
|`salt`|`bytes32`|The salt to use for the deployment|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`sequencerChain`|`address`|The address of the newly created MetabasedSequencerChain|
|`metafillerStorage`|`address`|The address of the newly created MetafillerStorage|
|`permissionModule`|`IRequirementModule`|The address of the newly created RequireAnyModule|


### computeSequencerChainAddress

Computes the address of the MetabasedSequencerChain contract


```solidity
function computeSequencerChainAddress(bytes32 salt, uint256 chainId) public view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`salt`|`bytes32`|The salt to use for the deployment|
|`chainId`|`uint256`|The ID of the L3 chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address of the MetabasedSequencerChain contract|


### getBytecode

Returns the bytecode of the MetabasedSequencerChain contract


```solidity
function getBytecode(uint256 chainId) public pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chainId`|`uint256`|The ID of the L3 chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|The bytecode of the MetabasedSequencerChain contract|


## Events
### MetabasedSequencerChainCreated
Emitted when a new MetabasedSequencerChain is created


```solidity
event MetabasedSequencerChainCreated(
    uint256 indexed l3ChainId, address indexed metabasedSequencerChainAddress, address indexed permissionModuleAddress
);
```

### MetafillerStorageCreated
Emitted when a new MetafillerStorage is created


```solidity
event MetafillerStorageCreated(uint256 indexed l3ChainId, address indexed metafillerStorageAddress);
```

### AllContractsCreated
Emitted when all contracts are created at once


```solidity
event AllContractsCreated(
    uint256 l3ChainId,
    address indexed sequencerChainAddress,
    address indexed metafillerStorageAddress,
    address indexed permissionModuleAddress
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

