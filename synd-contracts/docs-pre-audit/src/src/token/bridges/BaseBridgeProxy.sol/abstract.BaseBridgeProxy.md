# BaseBridgeProxy
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/bridges/BaseBridgeProxy.sol)

**Inherits:**
[IBridgeProxy](/src/token/interfaces/IBridgeProxy.sol/interface.IBridgeProxy.md), AccessControl, ReentrancyGuard

**Author:**
Syndicate Protocol

Base implementation contract for all bridge proxy implementations

*This abstract contract provides common security features, access controls,
and safety mechanisms that all bridge implementations should have.
Features:
- Role-based access control with separate admin and caller roles
- Reentrancy protection for all bridge operations
- Daily transfer limits with automatic reset
- Maximum single transfer limits
- Emergency pause functionality
- Safe token handling with automatic approval management
- Token recovery mechanisms for stuck funds*

**Note:**
security-contact: security@syndicate.io


## State Variables
### BRIDGE_ADMIN_ROLE
Role for bridge administration (configuration, pause/unpause)


```solidity
bytes32 public constant BRIDGE_ADMIN_ROLE = keccak256("BRIDGE_ADMIN_ROLE");
```


### BRIDGE_CALLER_ROLE
Role for executing bridge operations (typically the SyndicateToken contract)


```solidity
bytes32 public constant BRIDGE_CALLER_ROLE = keccak256("BRIDGE_CALLER_ROLE");
```


### bridgeTarget
The actual bridge contract address that this proxy calls


```solidity
address public bridgeTarget;
```


### bridgeActive
Whether this bridge is currently active and accepting transactions


```solidity
bool public bridgeActive;
```


### maxSingleTransfer
Maximum amount that can be transferred in a single transaction


```solidity
uint256 public maxSingleTransfer;
```


### dailyLimit
Maximum amount that can be transferred in a 24-hour period


```solidity
uint256 public dailyLimit;
```


### dailyUsed
Amount already transferred today


```solidity
uint256 public dailyUsed;
```


### lastResetDay
The day number when daily limits were last reset (block.timestamp / 1 days)


```solidity
uint256 public lastResetDay;
```


### bridgeName
Human-readable name of this bridge


```solidity
string public bridgeName;
```


## Functions
### onlyActiveBridge

*Ensures the bridge is active before allowing operations*


```solidity
modifier onlyActiveBridge();
```

### onlyAuthorizedCaller

*Ensures only authorized callers can execute bridge operations*


```solidity
modifier onlyAuthorizedCaller();
```

### constructor

Initialize the base bridge proxy


```solidity
constructor(
    address admin,
    address caller,
    string memory _bridgeName,
    address _bridgeTarget,
    uint256 _maxSingleTransfer,
    uint256 _dailyLimit
);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|Address that will have admin privileges|
|`caller`|`address`|Address that will be able to call executeBridge (typically SyndicateToken)|
|`_bridgeName`|`string`|Human-readable name for this bridge|
|`_bridgeTarget`|`address`|The actual bridge contract address|
|`_maxSingleTransfer`|`uint256`|Maximum amount per single transaction|
|`_dailyLimit`|`uint256`|Maximum amount per 24-hour period|


### executeBridge

Execute a bridge operation with comprehensive safety checks

*This function implements the IBridgeProxy interface and provides common
security checks before delegating to bridge-specific implementation*


```solidity
function executeBridge(address token, uint256 amount, bytes calldata dynamicData)
    external
    virtual
    override
    nonReentrant
    onlyActiveBridge
    onlyAuthorizedCaller;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The token to bridge|
|`amount`|`uint256`|The amount to bridge|
|`dynamicData`|`bytes`|Bridge-specific data|


### _executeBridgeCall

Abstract function that must be implemented by specific bridge contracts

*This function contains the actual bridge-specific logic for each implementation*


```solidity
function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal virtual;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The token to bridge|
|`amount`|`uint256`|The amount to bridge|
|`dynamicData`|`bytes`|Bridge-specific parameters|


### _updateDailyLimits

Update daily transfer limits, resetting if a new day has started

*This function is called before each transfer to ensure limits are current*


```solidity
function _updateDailyLimits() internal;
```

### setBridgeTarget

Set the bridge target contract address

*Only callable by bridge admin*


```solidity
function setBridgeTarget(address target) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`target`|`address`|The new bridge target address|


### setBridgeActive

Set the bridge active status

*Use this to pause/unpause the bridge in emergencies*


```solidity
function setBridgeActive(bool active) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`active`|`bool`|Whether the bridge should be active|


### setDailyLimit

Set the daily transfer limit

*This limit resets every 24 hours*


```solidity
function setDailyLimit(uint256 limit) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`limit`|`uint256`|The new daily limit in token units|


### setMaxSingleTransfer

Set the maximum single transfer amount

*This prevents any single transaction from being too large*


```solidity
function setMaxSingleTransfer(uint256 amount) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`amount`|`uint256`|The new maximum single transfer amount|


### getBridgeInfo

Get information about this bridge proxy

*Implementation of IBridgeProxy interface*


```solidity
function getBridgeInfo() external view override returns (string memory name, address target, bool active);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`name`|`string`|Human-readable bridge name|
|`target`|`address`|The bridge contract address|
|`active`|`bool`|Whether the bridge is currently active|


### getDailyUsage

Get current daily usage statistics


```solidity
function getDailyUsage() external view returns (uint256 used, uint256 limit, uint256 remaining);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`used`|`uint256`|Amount used today|
|`limit`|`uint256`|Daily limit|
|`remaining`|`uint256`|Amount remaining for today|


### recoverTokens

Emergency function to recover stuck tokens

*Only callable by DEFAULT_ADMIN_ROLE in case tokens get stuck in the contract*


```solidity
function recoverTokens(address token, uint256 amount, address to) external onlyRole(DEFAULT_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The token to recover|
|`amount`|`uint256`|The amount to recover|
|`to`|`address`|The address to send the recovered tokens to|


## Events
### BridgeTargetUpdated
Emitted when the bridge target address is updated


```solidity
event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`oldTarget`|`address`|The previous bridge target address|
|`newTarget`|`address`|The new bridge target address|

### BridgeStatusUpdated
Emitted when the bridge active status is changed


```solidity
event BridgeStatusUpdated(bool active);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`active`|`bool`|The new active status|

### BridgeExecuted
Emitted when a bridge operation is successfully executed


```solidity
event BridgeExecuted(address indexed token, uint256 amount, address indexed target);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The token that was bridged|
|`amount`|`uint256`|The amount that was bridged|
|`target`|`address`|The bridge contract that was called|

### DailyLimitUpdated
Emitted when the daily limit is updated


```solidity
event DailyLimitUpdated(uint256 oldLimit, uint256 newLimit);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`oldLimit`|`uint256`|The previous daily limit|
|`newLimit`|`uint256`|The new daily limit|

### DailyLimitReset
Emitted when daily limits are reset


```solidity
event DailyLimitReset(uint256 day, uint256 previousUsed);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`day`|`uint256`|The new day number|
|`previousUsed`|`uint256`|The amount used in the previous day|

## Errors
### ZeroAddress
Thrown when a zero address is provided where a valid address is required


```solidity
error ZeroAddress();
```

### ZeroAmount
Thrown when a zero amount is provided for a bridge operation


```solidity
error ZeroAmount();
```

### BridgeCallFailed
Thrown when a bridge call fails with a specific reason


```solidity
error BridgeCallFailed(string reason);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`reason`|`string`|The failure reason returned by the bridge|

### BridgeNotActive
Thrown when trying to use a bridge that is currently inactive


```solidity
error BridgeNotActive();
```

### UnauthorizedCaller
Thrown when an unauthorized address tries to call a restricted function


```solidity
error UnauthorizedCaller();
```

### ExcessiveAmount
Thrown when a transfer amount exceeds the configured limits


```solidity
error ExcessiveAmount();
```

