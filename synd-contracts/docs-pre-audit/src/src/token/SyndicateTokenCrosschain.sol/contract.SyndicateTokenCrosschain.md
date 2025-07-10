# SyndicateTokenCrosschain
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/SyndicateTokenCrosschain.sol)

**Inherits:**
[SyndicateToken](/src/token/SyndicateToken.sol/contract.SyndicateToken.md), [IERC7802](/src/token/crosschain/interfaces/IERC7802.sol/interface.IERC7802.md), [IBridgeRateLimiter](/src/token/crosschain/interfaces/IBridgeRateLimiter.sol/interface.IBridgeRateLimiter.md)

Crosschain-compatible Syndicate Protocol token

*Extends the existing SyndicateToken with crosschain capabilities including:
- ERC7802 compatibility for SuperChain and modern bridges
- Bridge rate limiting and access controls
- Same contract address across all chains via deterministic deployment
- Full compatibility with existing emission scheduler and governance
Key Features:
- All existing SyndicateToken functionality (governance, permits, emissions, etc.)
- Native crosschain mint/burn functions for authorized bridges
- Rate limiting per bridge to prevent abuse
- Emergency controls for bridge management
- ERC165 interface detection for bridge compatibility*


## State Variables
### BRIDGE_MANAGER_ROLE
Role for managing bridge configurations


```solidity
bytes32 public constant BRIDGE_MANAGER_ROLE = keccak256("BRIDGE_MANAGER_ROLE");
```


### EMISSION_BUDGET_MANAGER_ROLE
Role for allocating emission budgets to bridges (typically emission scheduler)


```solidity
bytes32 public constant EMISSION_BUDGET_MANAGER_ROLE = keccak256("EMISSION_BUDGET_MANAGER_ROLE");
```


### bridgeConfigs
Mapping of bridge address to bridge configuration


```solidity
mapping(address => BridgeConfig) public bridgeConfigs;
```


### bridges
Set of all configured bridges for enumeration and O(1) operations


```solidity
EnumerableSet.AddressSet private bridges;
```


### bridgeEmissionBudgets
Mapping of bridge address to emission budget allocated


```solidity
mapping(address => uint256) public bridgeEmissionBudgets;
```


### hourlyMintUsage
Mapping of bridge -> hour -> mint usage for sliding window rate limiting


```solidity
mapping(address => mapping(uint256 => uint256)) public hourlyMintUsage;
```


### hourlyBurnUsage
Mapping of bridge -> hour -> burn usage for sliding window rate limiting


```solidity
mapping(address => mapping(uint256 => uint256)) public hourlyBurnUsage;
```


## Functions
### constructor

Initialize the crosschain Syndicate token


```solidity
constructor(address defaultAdmin, address syndTreasuryAddress) SyndicateToken(defaultAdmin, syndTreasuryAddress);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`defaultAdmin`|`address`|Address that will have default admin privileges|
|`syndTreasuryAddress`|`address`|Address to receive the initial token mint|


### crosschainMint

Mints tokens to the recipient (called by authorized bridges)


```solidity
function crosschainMint(address to, uint256 amount) external override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|The address of the recipient|
|`amount`|`uint256`|The amount of tokens to mint|


### crosschainBurn

Burns tokens from the sender (called by authorized bridges)


```solidity
function crosschainBurn(address from, uint256 amount) external override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`from`|`address`|The address of the sender|
|`amount`|`uint256`|The amount of tokens to burn|


### setBridgeLimits

Set minting and burning limits for a bridge


```solidity
function setBridgeLimits(address bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit)
    external
    override
    onlyRole(BRIDGE_MANAGER_ROLE);
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
function setBridgeActive(address bridge, bool isActive) external override onlyRole(BRIDGE_MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`isActive`|`bool`|Whether bridge should be active|


### removeBridge

Remove a bridge from the system

*Only callable by bridge manager role*


```solidity
function removeBridge(address bridge) external onlyRole(BRIDGE_MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address to remove|


### allocateEmissionBudget

Allocate emission budget to a bridge

*Only callable by emission budget manager role (typically emission scheduler)*


```solidity
function allocateEmissionBudget(address bridge, uint256 amount) external onlyRole(EMISSION_BUDGET_MANAGER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address to allocate budget to|
|`amount`|`uint256`|Amount of emission budget to allocate|


### _validateAndUseMintLimit

Validate bridge authorization and consume mint limit using sliding window


```solidity
function _validateAndUseMintLimit(address bridge, uint256 amount) internal;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`amount`|`uint256`|Amount to mint|


### _validateAndUseBurnLimit

Validate bridge authorization and consume burn limit using sliding window


```solidity
function _validateAndUseBurnLimit(address bridge, uint256 amount) internal;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`amount`|`uint256`|Amount to burn|


### _validateAndConsumeEmissionBudget

Validate bridge emission budget and consume it


```solidity
function _validateAndConsumeEmissionBudget(address bridge, uint256 amount) internal;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`amount`|`uint256`|Amount to consume from emission budget|


### getBridgeConfig

Get bridge configuration


```solidity
function getBridgeConfig(address bridge) external view override returns (BridgeConfig memory config);
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
function getAvailableMintLimit(address bridge) external view override returns (uint256 available);
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
function getAvailableBurnLimit(address bridge) external view override returns (uint256 available);
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
function isBridgeAuthorized(address bridge) external view override returns (bool authorized);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`authorized`|`bool`|True if bridge is authorized and active|


### getBridgeCount

Get total number of configured bridges


```solidity
function getBridgeCount() external view returns (uint256 count);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`count`|`uint256`|Number of bridges|


### getBridgeAtIndex

Get bridge address at index


```solidity
function getBridgeAtIndex(uint256 index) external view returns (address bridge);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`index`|`uint256`|Index in bridges set|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|


### getAllBridges

Get all configured bridges


```solidity
function getAllBridges() external view returns (address[] memory allBridges);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`allBridges`|`address[]`|Array of bridge addresses|


### getEmissionBudget

Get emission budget for a bridge


```solidity
function getEmissionBudget(address bridge) external view returns (uint256 budget);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`budget`|`uint256`|Available emission budget for the bridge|


### supportsInterface

Check if contract supports an interface


```solidity
function supportsInterface(bytes4 interfaceId) public view virtual override(AccessControl, IERC165) returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`interfaceId`|`bytes4`|Interface identifier to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|true if interface is supported|


## Events
### BridgeAdded
Emitted when a bridge is added to the system


```solidity
event BridgeAdded(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`dailyMintLimit`|`uint256`|Initial daily mint limit|
|`dailyBurnLimit`|`uint256`|Initial daily burn limit|

### BridgeRemoved
Emitted when a bridge is removed from the system


```solidity
event BridgeRemoved(address indexed bridge);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|

### EmissionBudgetAllocated
Emitted when emission budget is allocated to a bridge


```solidity
event EmissionBudgetAllocated(address indexed bridge, uint256 amount);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`amount`|`uint256`|Amount of emission budget allocated|

### EmissionBudgetConsumed
Emitted when emission budget is consumed by a bridge


```solidity
event EmissionBudgetConsumed(address indexed bridge, uint256 amount);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bridge`|`address`|Bridge address|
|`amount`|`uint256`|Amount of emission budget consumed|

