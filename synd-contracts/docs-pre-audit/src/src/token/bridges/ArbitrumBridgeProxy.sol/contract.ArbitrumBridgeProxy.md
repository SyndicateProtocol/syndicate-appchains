# ArbitrumBridgeProxy
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/bridges/ArbitrumBridgeProxy.sol)

**Inherits:**
[BaseBridgeProxy](/src/token/bridges/BaseBridgeProxy.sol/abstract.BaseBridgeProxy.md)

**Author:**
Syndicate Protocol

Bridge proxy implementation for Arbitrum One and Arbitrum Nova

*This contract implements the bridge logic for sending tokens from Ethereum L1
to Arbitrum L2 using the official Arbitrum bridge infrastructure.
Key Features:
- Integrates with Arbitrum's L1GatewayRouter contract
- Supports configurable gas parameters for L2 transactions
- Handles ETH payments for L2 gas fees
- Allows dynamic recipient and gas configuration
- Inherits all security features from BaseBridgeProxy
Usage:
The SyndicateToken contract calls executeBridge() with optional dynamicData
containing (recipient, maxGas, gasPriceBid) parameters. If no dynamicData is provided,
default values are used. ETH must be sent with the transaction to pay for L2 gas.*

**Note:**
security-contact: security@syndicate.io


## State Variables
### recipient
Default recipient address on L2 (usually the emissions distributor)


```solidity
address public recipient;
```


### maxGas
Default maximum gas for L2 transactions


```solidity
uint256 public maxGas;
```


### gasPriceBid
Default gas price bid for L2 transactions


```solidity
uint256 public gasPriceBid;
```


### maxSubmissionCost
Maximum submission cost for Arbitrum L2 transactions


```solidity
uint256 public maxSubmissionCost;
```


## Functions
### constructor

Initialize the Arbitrum bridge proxy


```solidity
constructor(
    address admin,
    address caller,
    address _bridgeTarget,
    uint256 _maxSingleTransfer,
    uint256 _dailyLimit,
    address _recipient,
    uint256 _maxGas,
    uint256 _gasPriceBid
) BaseBridgeProxy(admin, caller, "Arbitrum Bridge", _bridgeTarget, _maxSingleTransfer, _dailyLimit);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`admin`|`address`|Address that will have admin privileges|
|`caller`|`address`|Address that can execute bridge operations (SyndicateToken)|
|`_bridgeTarget`|`address`|Address of the Arbitrum L1GatewayRouter contract|
|`_maxSingleTransfer`|`uint256`|Maximum amount per single transaction|
|`_dailyLimit`|`uint256`|Maximum amount per 24-hour period|
|`_recipient`|`address`|Default recipient address on L2|
|`_maxGas`|`uint256`|Default maximum gas for L2 transactions|
|`_gasPriceBid`|`uint256`|Default gas price bid for L2 transactions|


### _executeBridgeCall

Execute the Arbitrum bridge call

*This function implements the bridge-specific logic for Arbitrum.
It approves the bridge contract to spend tokens and calls outboundTransferCustomRefund.
ETH value is calculated as maxGas * gasPriceBid to pay for L2 execution.
Dynamic Data Format:
- If provided: abi.encode(address recipient, uint256 maxGas, uint256 gasPriceBid)
- If empty: uses default recipient, maxGas, and gasPriceBid values*


```solidity
function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`token`|`address`|The L1 token address to bridge|
|`amount`|`uint256`|The amount of tokens to bridge|
|`dynamicData`|`bytes`|Optional ABI-encoded (recipient, maxGas, gasPriceBid) parameters|


### setArbitrumConfig

Update Arbitrum-specific configuration

*Only callable by bridge admin. Allows updating default recipient
and gas parameters for L2 transactions.*


```solidity
function setArbitrumConfig(address _recipient, uint256 _maxGas, uint256 _gasPriceBid)
    external
    onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_recipient`|`address`|Default recipient address on L2|
|`_maxGas`|`uint256`|Default maximum gas for L2 transactions|
|`_gasPriceBid`|`uint256`|Default gas price bid for L2 transactions|


### setMaxSubmissionCost

Update the maximum submission cost for Arbitrum L2 transactions

*Only callable by bridge admin*


```solidity
function setMaxSubmissionCost(uint256 _maxSubmissionCost) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_maxSubmissionCost`|`uint256`|New maximum submission cost value|


### getArbitrumConfig

Get current Arbitrum configuration


```solidity
function getArbitrumConfig()
    external
    view
    returns (address recipientAddr, uint256 maxGasLimit, uint256 gasPriceBidAmount);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`recipientAddr`|`address`|Default recipient address|
|`maxGasLimit`|`uint256`|Default maximum gas|
|`gasPriceBidAmount`|`uint256`|Default gas price bid|


### calculateEthValue

Calculate the ETH value needed for a bridge operation


```solidity
function calculateEthValue(uint256 _maxGas, uint256 _gasPriceBid) external pure returns (uint256 ethValue);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_maxGas`|`uint256`|Maximum gas for the L2 transaction|
|`_gasPriceBid`|`uint256`|Gas price bid for the L2 transaction|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`ethValue`|`uint256`|The amount of ETH needed|


### withdrawEth

Withdraw ETH from the contract

*Only callable by bridge admin. Allows removal of excess ETH that may
accumulate from gas refunds or over-funding of the contract.*


```solidity
function withdrawEth(address payable to, uint256 amount) external onlyRole(BRIDGE_ADMIN_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address payable`|Address to send the ETH to|
|`amount`|`uint256`|Amount of ETH to withdraw (in wei)|


### receive

Allow contract to receive ETH for gas payments and refunds

*The contract needs to accept ETH for:
1. Paying L2 gas fees during bridge operations
2. Receiving refunds from Arbitrum bridge if gas is overestimated*


```solidity
receive() external payable;
```

## Events
### ArbitrumConfigUpdated
Emitted when Arbitrum-specific configuration is updated


```solidity
event ArbitrumConfigUpdated(address recipient, uint256 maxGas, uint256 gasPriceBid);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`recipient`|`address`|The new default recipient address|
|`maxGas`|`uint256`|The new default maximum gas|
|`gasPriceBid`|`uint256`|The new default gas price bid|

### EthWithdrawn
Emitted when ETH is withdrawn from the contract


```solidity
event EthWithdrawn(address indexed to, uint256 amount);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|Address that received the ETH|
|`amount`|`uint256`|Amount of ETH withdrawn|

