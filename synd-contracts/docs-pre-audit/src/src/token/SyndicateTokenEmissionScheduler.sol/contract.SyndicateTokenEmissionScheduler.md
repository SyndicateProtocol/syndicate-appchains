# SyndicateTokenEmissionScheduler
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/SyndicateTokenEmissionScheduler.sol)

**Inherits:**
AccessControl, Pausable, ReentrancyGuard

**Author:**
Syndicate Protocol

Manages the emission schedule and distribution of SYND tokens

*This contract handles all emissions logic and calls the SyndicateToken contract
to mint tokens according to a predefined schedule. It includes automated
emissions distribution to L2 chains via configurable bridge proxies.
Key Features:
- Automated emissions over 48 epochs (~4 years) with decay schedule
- Modular bridge architecture supporting multiple bridge providers
- Comprehensive access controls and emergency mechanisms
- Emissions tracking and validation against total supply limits
Emissions Schedule:
- 48 epochs of 30 days each (~4 years total)
- Decreasing amounts per epoch in 8 periods of 6 epochs each
- Automated distribution via configurable bridge proxy*

**Note:**
security-contact: security@syndicate.io


## State Variables
### EMISSIONS_MANAGER_ROLE
Role for managing emissions (start, pause, resume)


```solidity
bytes32 public constant EMISSIONS_MANAGER_ROLE = keccak256("EMISSIONS_MANAGER_ROLE");
```


### PAUSER_ROLE
Role for emergency pausing functionality


```solidity
bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
```


### BRIDGE_MANAGER_ROLE
Role for managing bridge configuration


```solidity
bytes32 public constant BRIDGE_MANAGER_ROLE = keccak256("BRIDGE_MANAGER_ROLE");
```


### EPOCH_DURATION
Duration of each emission epoch: 30 days


```solidity
uint256 public constant EPOCH_DURATION = 30 days;
```


### TOTAL_EPOCHS
Total number of emission epochs: 48 (~4 years)


```solidity
uint256 public constant TOTAL_EPOCHS = 48;
```


### EMISSION_BUFFER_TIME
Buffer time before scheduled emission to prevent MEV/timing issues


```solidity
uint256 public constant EMISSION_BUFFER_TIME = 1 hours;
```


### syndicateToken
The SyndicateToken contract that will receive minted emissions


```solidity
ISyndicateTokenMintable public immutable syndicateToken;
```


### emissionSchedule
Emission amounts for each epoch (fixed schedule with decay)


```solidity
uint256[TOTAL_EPOCHS] public emissionSchedule;
```


### emissionsStartTime
Timestamp when emissions were started


```solidity
uint256 public emissionsStartTime;
```


### currentEpoch
Current emission epoch (0-based)


```solidity
uint256 public currentEpoch;
```


### totalEmissionsMinted
Total amount of emissions minted so far


```solidity
uint256 public totalEmissionsMinted;
```


### bridgeProxy
Bridge proxy contract for cross-chain emissions distribution


```solidity
IBridgeProxy public bridgeProxy;
```


### bridgeData
Bridge-specific configuration data


```solidity
bytes public bridgeData;
```


## Functions
### constructor

Initialize the emission scheduler contract


```solidity
constructor(address _syndicateToken, address defaultAdmin, address emissionsManager, address pauser);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_syndicateToken`|`address`|Address of the SyndicateToken contract|
|`defaultAdmin`|`address`|Address that will have default admin privileges|
|`emissionsManager`|`address`|Address that can manage emissions|
|`pauser`|`address`|Address that can pause the contract in emergencies|


### startEmissions

Start the emissions process

*Can only be called once. Requires bridge proxy to be configured.
Only callable by addresses with EMISSIONS_MANAGER_ROLE.*


```solidity
function startEmissions() external onlyRole(EMISSIONS_MANAGER_ROLE);
```

### mintEmission

Mint emission tokens and bridge them to L2

*This function can only be called by the emissions manager.
It mints tokens based on the current epoch schedule and bridges them
using the configured bridge proxy. Includes comprehensive safety checks.*


```solidity
function mintEmission() external whenNotPaused nonReentrant onlyRole(EMISSIONS_MANAGER_ROLE);
```

### setBridgeProxy

Set the bridge proxy for emissions distribution

*Only callable by addresses with BRIDGE_MANAGER_ROLE*


```solidity
function setBridgeProxy(IBridgeProxy _bridgeProxy) external onlyRole(BRIDGE_MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_bridgeProxy`|`IBridgeProxy`|The new bridge proxy contract implementing IBridgeProxy|


### setBridgeData

Set bridge configuration data

*This data is passed to the bridge proxy during execution.
Format depends on the specific bridge implementation.
Only callable by addresses with BRIDGE_MANAGER_ROLE.*


```solidity
function setBridgeData(bytes calldata _bridgeData) external onlyRole(BRIDGE_MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_bridgeData`|`bytes`|Encoded bridge configuration data|


### getBridgeConfiguration

Get current bridge configuration


```solidity
function getBridgeConfiguration() external view returns (IBridgeProxy proxy, bytes memory data);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`proxy`|`IBridgeProxy`|The current bridge proxy contract|
|`data`|`bytes`|The current bridge configuration data|


### getEmissionSchedule

Get the complete emission schedule


```solidity
function getEmissionSchedule() external view returns (uint256[TOTAL_EPOCHS] memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256[TOTAL_EPOCHS]`|The array of emission amounts for each epoch|


### getCurrentEpochInfo

Get comprehensive information about the current epoch


```solidity
function getCurrentEpochInfo()
    external
    view
    returns (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`epoch`|`uint256`|Current epoch number|
|`nextEmissionTime`|`uint256`|Timestamp when next emission can be minted|
|`nextEmissionAmount`|`uint256`|Amount of tokens in the next emission|
|`canMintEmission`|`bool`|Whether emission can currently be minted|


### emissionsEnded

Check if all emissions have been completed


```solidity
function emissionsEnded() public view returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if all 48 epochs have been minted|


### emissionsStarted

Check if emissions have been started


```solidity
function emissionsStarted() public view returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if startEmissions() has been called|


### getNextEmissionTime

Get the timestamp when the next emission can be minted


```solidity
function getNextEmissionTime() public view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|Timestamp of the next emission opportunity|


### pause

Pause the entire contract in emergency situations

*This pauses all emissions operations. Only callable by PAUSER_ROLE.*


```solidity
function pause() external onlyRole(PAUSER_ROLE);
```

### unpause

Unpause the contract

*Only callable by DEFAULT_ADMIN_ROLE to ensure careful consideration.*


```solidity
function unpause() external onlyRole(DEFAULT_ADMIN_ROLE);
```

### _initializeEmissionSchedule

Initialize the emission schedule with predefined decay pattern

*Creates a decreasing emission schedule over 8 periods of 6 epochs each.
Each period has a lower emission rate than the previous one.*


```solidity
function _initializeEmissionSchedule() private;
```

## Events
### EmissionsStarted
Emitted when emissions are started


```solidity
event EmissionsStarted(uint256 startTime);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`startTime`|`uint256`|The timestamp when emissions began|

### EmissionMinted
Emitted when emission tokens are minted and bridged


```solidity
event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`epoch`|`uint256`|The epoch number that was minted|
|`amount`|`uint256`|The amount of tokens minted|
|`destination`|`address`|The bridge proxy that received the tokens|

### BridgeProxyUpdated
Emitted when the bridge proxy is updated


```solidity
event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`oldProxy`|`address`|The previous bridge proxy address|
|`newProxy`|`address`|The new bridge proxy address|

### BridgeDataUpdated
Emitted when bridge configuration data is updated


```solidity
event BridgeDataUpdated(bytes oldData, bytes newData);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`oldData`|`bytes`|The previous bridge data|
|`newData`|`bytes`|The new bridge data|

## Errors
### ZeroAddress
Thrown when an address is zero


```solidity
error ZeroAddress();
```

### EmissionsAlreadyStarted
Thrown when trying to start emissions that have already been started


```solidity
error EmissionsAlreadyStarted();
```

### EmissionsNotStarted
Thrown when trying to perform emissions operations before they're started


```solidity
error EmissionsNotStarted();
```

### AllEmissionsCompleted
Thrown when trying to mint emissions after all epochs are completed


```solidity
error AllEmissionsCompleted();
```

### BridgeNotConfigured
Thrown when trying to start emissions without a configured bridge


```solidity
error BridgeNotConfigured();
```

### EmissionTooEarly
Thrown when trying to mint emissions too early (before buffer time)


```solidity
error EmissionTooEarly();
```

