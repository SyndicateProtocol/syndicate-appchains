# IPermissionModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/interfaces/IPermissionModule.sol)


## Functions
### isAllowed

Checks if the caller is allowed.


```solidity
function isAllowed(address proposer, address originator, bytes calldata data) external view returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`proposer`|`address`|The address of proposed to be checked.|
|`originator`|`address`|The address of tx.origin. Useful to know the sender originator in wrapper contracts|
|`data`|`bytes`|The calldata to be checked.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if the caller is allowed.|


