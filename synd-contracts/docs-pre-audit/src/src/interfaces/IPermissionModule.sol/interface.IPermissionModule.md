# IPermissionModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/interfaces/IPermissionModule.sol)


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


