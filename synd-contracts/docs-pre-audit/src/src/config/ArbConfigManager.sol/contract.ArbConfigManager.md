# ArbConfigManager
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/config/ArbConfigManager.sol)

**Inherits:**
Ownable

*Manages deployment of ArbChainConfig contracts using CREATE2 for deterministic addresses
Uses the Beacon Proxy pattern for upgradeable implementations*


## State Variables
### beacon

```solidity
UpgradeableBeacon public immutable beacon;
```


### deployedConfigs

```solidity
mapping(uint256 chainId => address deployedProxyAddress) public deployedConfigs;
```


## Functions
### constructor

*Constructor to deploy the implementation contract and beacon*


```solidity
constructor(address _owner) Ownable(_owner);
```

### createArbChainConfig

*Create a new ArbChainConfig contract for a specific chainId*


```solidity
function createArbChainConfig(
    address owner,
    uint256 chainId,
    uint256 sequencingChainId,
    address arbitrumBridgeAddress,
    address arbitrumInboxAddress,
    bool arbitrumIgnoreDelayedMessages,
    uint256 settlementDelay,
    uint256 settlementStartBlock,
    address sequencingContractAddress,
    uint256 sequencingStartBlock,
    address initialAppchainOwner,
    string memory sequencingChainRpcUrl,
    string memory appchainBlockExplorerUrl,
    address[] memory allowedSettlementAddresses
) external onlyOwner returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`owner`|`address`|The address of the contract owner|
|`chainId`|`uint256`|The chain ID|
|`sequencingChainId`|`uint256`|The ID of the sequencing chain|
|`arbitrumBridgeAddress`|`address`|Address of the Arbitrum bridge|
|`arbitrumInboxAddress`|`address`|Address of the Arbitrum inbox|
|`arbitrumIgnoreDelayedMessages`|`bool`|Whether to ignore delayed messages|
|`settlementDelay`|`uint256`|Delay for settlement|
|`settlementStartBlock`|`uint256`|Starting block for settlement|
|`sequencingContractAddress`|`address`|Address of the sequencing contract|
|`sequencingStartBlock`|`uint256`|Starting block for sequencing|
|`initialAppchainOwner`|`address`|Initial appchain owner|
|`sequencingChainRpcUrl`|`string`|Default RPC URL for the sequencing chain|
|`appchainBlockExplorerUrl`|`string`|URL for the appchain block explorer|
|`allowedSettlementAddresses`|`address[]`|Array of addresses allowed for settlement|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|address The address of the deployed ArbChainConfig contract|


### getArbChainConfigAddress

*Get the deterministic address for a chain config given a chainId*


```solidity
function getArbChainConfigAddress(uint256 chainId) public view returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`chainId`|`uint256`|The chain ID|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The deterministic address where the config would be deployed|


### upgradeImplementation

This function can only be called by the beacon owner, which is transferred
to the deployer in the constructor

*Upgrade the implementation for all proxies by updating the beacon*


```solidity
function upgradeImplementation(address newImplementation) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`newImplementation`|`address`|The address of the new implementation|


## Events
### ArbChainConfigCreated

```solidity
event ArbChainConfigCreated(uint256 indexed chainId, address configAddress);
```

### ImplementationUpgraded

```solidity
event ImplementationUpgraded(address newImplementation);
```

