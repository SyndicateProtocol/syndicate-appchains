# OptimismBridgeProxy
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/bridges/OptimismBridgeProxy.sol)

**Inherits:**
[BaseBridgeProxy](/src/token/bridges/BaseBridgeProxy.sol/abstract.BaseBridgeProxy.md)

**Author:**
Syndicate Protocol

Bridge proxy implementation for Optimism (OP Mainnet) and OP Stack chains

*This contract implements the bridge logic for sending tokens from Ethereum L1
to Optimism L2 using the official Optimism bridge infrastructure.
Key Features:
- Integrates with Optimism's L1StandardBridge contract
- Supports configurable L2 token addresses and recipients
- Allows dynamic gas limit configuration for L2 transactions
- Inherits all security features from BaseBridgeProxy
Usage:
The SyndicateToken contract calls executeBridge() with optional dynamicData
containing (recipient, gasLimit) parameters. If no dynamicData is provided,
default values are used.*

**Note:**
security-contact: security@syndicate.io


## State Variables
### l2Token
Address of the corresponding token on Optimism L2


```solidity
address public l2Token;
```


### recipient
Default recipient address on L2 (usually the emissions distributor)


```solidity
address public recipient;
```


### l2Gas
Default gas limit for L2 transactions


```solidity
uint32 public l2Gas;
```


## Functions
### constructor

Initialize the Optimism bridge proxy


```solidity
constructor(
    address admin,
    address caller,
    address _bridgeTarget,
    uint256 _maxSingleTransfer,
    uint256 _dailyLimit,
    address _l2Token,
    address _recipient,
    uint32 _l2Gas
) BaseBridgeProxy(admin, caller, "Optimism Bridge", _bridgeTarget, _maxSingleTransfer, _dailyLimit);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|Address that will have admin privileges|
|`caller`|`address`|Address that can execute bridge operations (SyndicateToken)|
|`_bridgeTarget`|`address`|Address of the Optimism L1StandardBridge contract|
|`_maxSingleTransfer`|`uint256`|Maximum amount per single transaction|
|`_dailyLimit`|`uint256`|Maximum amount per 24-hour period|
|`_l2Token`|`address`|Address of the token on Optimism L2|
|`_recipient`|`address`|Default recipient address on L2|
|`_l2Gas`|`uint32`|Default gas limit for L2 transactions|


### _executeBridgeCall

Execute the Optimism bridge call

*This function implements the bridge-specific logic for Optimism.
It approves the bridge contract to spend tokens and calls depositERC20To.
Dynamic Data Format:
- If provided: abi.encode(address recipient, uint32 gasLimit)
- If empty: uses default recipient and l2Gas values*


```solidity
function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The L1 token address to bridge|
|`amount`|`uint256`|The amount of tokens to bridge|
|`dynamicData`|`bytes`|Optional ABI-encoded (recipient, gasLimit) parameters|


### setOptimismConfig

Update Optimism-specific configuration

*Only callable by bridge admin. Allows updating L2 token address,
default recipient, and default gas limit.*


```solidity
function setOptimismConfig(address _l2Token, address _recipient, uint32 _l2Gas) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_l2Token`|`address`|Address of the token on Optimism L2|
|`_recipient`|`address`|Default recipient address on L2|
|`_l2Gas`|`uint32`|Default gas limit for L2 transactions|


### getOptimismConfig

Get current Optimism configuration


```solidity
function getOptimismConfig() external view returns (address l2TokenAddr, address recipientAddr, uint32 gasLimit);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`l2TokenAddr`|`address`|Address of the L2 token|
|`recipientAddr`|`address`|Default recipient address|
|`gasLimit`|`uint32`|Default L2 gas limit|


## Events
### OptimismConfigUpdated
Emitted when Optimism-specific configuration is updated


```solidity
event OptimismConfigUpdated(address l2Token, address recipient, uint32 l2Gas);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`l2Token`|`address`|The new L2 token address|
|`recipient`|`address`|The new default recipient address|
|`l2Gas`|`uint32`|The new default L2 gas limit|

