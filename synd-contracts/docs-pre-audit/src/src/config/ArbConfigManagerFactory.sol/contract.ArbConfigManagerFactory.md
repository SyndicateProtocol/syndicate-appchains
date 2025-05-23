# ArbConfigManagerFactory
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/config/ArbConfigManagerFactory.sol)

*Factory contract to deploy ArbConfigManager deterministically across chains*


## Functions
### deployArbConfigManager

*Deploy the ArbConfigManager with deterministic address*


```solidity
function deployArbConfigManager(address owner, bytes32 salt) external returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`owner`|`address`|The owner of the ArbConfigManager|
|`salt`|`bytes32`|A unique salt value to deterministically generate the address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address of the deployed ArbConfigManager|


### getBytecode

*Get the bytecode for ArbConfigManager construction*


```solidity
function getBytecode(address owner) public pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`owner`|`address`|The owner address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|The complete contract bytecode|


### getAddress

*Calculate the expected deployment address*


```solidity
function getAddress(bytes memory bytecode, bytes32 salt) public view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bytecode`|`bytes`|The contract bytecode with constructor arguments|
|`salt`|`bytes32`|The unique salt for address generation|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The deterministic address where the contract will be deployed|


### predictDeploymentAddress

*Predict the deployment address before actual deployment*


```solidity
function predictDeploymentAddress(address owner, bytes32 salt) external view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`owner`|`address`|The owner address|
|`salt`|`bytes32`|The unique salt for address generation|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address where the contract will be deployed|


## Events
### ArbConfigManagerDeployed

```solidity
event ArbConfigManagerDeployed(address deployedAddress, address owner);
```

