# ITeeKeyManager
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/ITeeKeyManager.sol)


## Functions
### isKeyValid

Checks if a public key is considered a valid TEE key.


```solidity
function isKeyValid(address publicKey) external view returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`publicKey`|`address`|The public key to check.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the key is valid, false otherwise.|


