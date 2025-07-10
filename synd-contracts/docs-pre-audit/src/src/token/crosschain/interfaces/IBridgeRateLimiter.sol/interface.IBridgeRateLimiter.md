# IBridgeRateLimiter
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/crosschain/interfaces/IBridgeRateLimiter.sol)

Interface for managing bridge rate limits and permissions


## Functions
### setBridgeLimits

Set minting and burning limits for a bridge


```solidity
function setBridgeLimits(address bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`dailyMintLimit`|`uint256`|Maximum tokens that can be minted per day|
|`dailyBurnLimit`|`uint256`|Maximum tokens that can be burned per day|


### setBridgeActive

Set active status for a bridge


```solidity
function setBridgeActive(address bridge, bool isActive) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`isActive`|`bool`|Whether bridge should be active|


### getBridgeConfig

Get bridge configuration


```solidity
function getBridgeConfig(address bridge) external view returns (BridgeConfig memory config);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`config`|`BridgeConfig`|Bridge configuration struct|


### getAvailableMintLimit

Get current available mint limit for a bridge


```solidity
function getAvailableMintLimit(address bridge) external view returns (uint256 available);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`available`|`uint256`|Available mint limit|


### getAvailableBurnLimit

Get current available burn limit for a bridge


```solidity
function getAvailableBurnLimit(address bridge) external view returns (uint256 available);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`available`|`uint256`|Available burn limit|


### isBridgeAuthorized

Check if bridge is authorized and active


```solidity
function isBridgeAuthorized(address bridge) external view returns (bool authorized);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`authorized`|`bool`|True if bridge is authorized and active|


## Events
### BridgeLimitsSet
Emitted when bridge limits are updated


```solidity
event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`dailyMintLimit`|`uint256`|New daily mint limit|
|`dailyBurnLimit`|`uint256`|New daily burn limit|

### BridgeActiveStatusChanged
Emitted when bridge active status changes


```solidity
event BridgeActiveStatusChanged(address indexed bridge, bool isActive);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`isActive`|`bool`|New active status|

## Errors
### UnauthorizedBridge
Thrown when bridge is not authorized


```solidity
error UnauthorizedBridge(address bridge);
```

### InsufficientMintLimit
Thrown when bridge has insufficient mint limit


```solidity
error InsufficientMintLimit(address bridge, uint256 requested, uint256 available);
```

### InsufficientBurnLimit
Thrown when bridge has insufficient burn limit


```solidity
error InsufficientBurnLimit(address bridge, uint256 requested, uint256 available);
```

### BridgeNotActive
Thrown when bridge is not active


```solidity
error BridgeNotActive(address bridge);
```

### CannotAddSelfAsBridge
Thrown when trying to add self as bridge


```solidity
error CannotAddSelfAsBridge();
```

### BridgeMustBeContract
Thrown when bridge address is not a contract


```solidity
error BridgeMustBeContract();
```

### InsufficientEmissionBudget
Thrown when bridge has insufficient emission budget


```solidity
error InsufficientEmissionBudget();
```

### UnreasonableMintLimit
Thrown when mint limit is unreasonably high


```solidity
error UnreasonableMintLimit();
```

### UnreasonableBurnLimit
Thrown when burn limit is unreasonably high


```solidity
error UnreasonableBurnLimit();
```

## Structs
### BridgeConfig
Bridge configuration structure


```solidity
struct BridgeConfig {
    uint256 dailyMintLimit;
    uint256 dailyBurnLimit;
    bool isActive;
}
```

