# IPermissionModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/interfaces/IPermissionModule.sol)


## Functions
### isAllowed

Checks if a transaction sender is allowed.


```solidity
function isAllowed(address msgSender, address txOrigin, bytes calldata data) external view returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`msgSender`|`address`|The address that called the function (msg.sender).|
|`txOrigin`|`address`|The address that initiated the transaction (tx.origin).|
|`data`|`bytes`|The calldata to be checked.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if the transaction is allowed.|


