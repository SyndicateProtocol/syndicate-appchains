# IOptimismBridge
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/bridges/OptimismBridgeProxy.sol)

Interface for the Optimism L1StandardBridge contract

*This interface defines the function used to deposit ERC20 tokens to Optimism L2*


## Functions
### depositERC20To

Deposit an ERC20 token to a recipient on L2


```solidity
function depositERC20To(
    address _l1Token,
    address _l2Token,
    address _to,
    uint256 _amount,
    uint32 _l2Gas,
    bytes calldata _data
) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_l1Token`|`address`|Address of the L1 token being deposited|
|`_l2Token`|`address`|Address of the corresponding L2 token|
|`_to`|`address`|Address of the recipient on L2|
|`_amount`|`uint256`|Amount of tokens to deposit|
|`_l2Gas`|`uint32`|Gas limit for the L2 transaction|
|`_data`|`bytes`|Optional data to pass to the L2 contract|


