# IArbitrumBridge
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/bridges/ArbitrumBridgeProxy.sol)

Interface for the Arbitrum L1GatewayRouter contract

*This interface defines the function used to transfer ERC20 tokens to Arbitrum L2*


## Functions
### outboundTransferCustomRefund

Transfer tokens to L2 with custom refund address


```solidity
function outboundTransferCustomRefund(
    address _token,
    address _refundTo,
    address _to,
    uint256 _amount,
    uint256 _maxGas,
    uint256 _gasPriceBid,
    bytes calldata _data
) external payable returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_token`|`address`|Address of the L1 token being transferred|
|`_refundTo`|`address`|Address to refund excess gas fees on L1|
|`_to`|`address`|Address of the recipient on L2|
|`_amount`|`uint256`|Amount of tokens to transfer|
|`_maxGas`|`uint256`|Maximum gas to use for the L2 transaction|
|`_gasPriceBid`|`uint256`|Gas price bid for the L2 transaction|
|`_data`|`bytes`|Optional data to pass to the L2 contract|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|Unique identifier for the transfer|


### getGateway


```solidity
function getGateway(address) external view returns (address);
```

