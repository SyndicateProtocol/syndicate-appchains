# DummyKeyManager
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/testing-purposes/DummyKeyManager.sol)

**Inherits:**
[ITeeKeyManager](/src/withdrawal/ITeeKeyManager.sol/interface.ITeeKeyManager.md)


## State Variables
### validKeys

```solidity
EnumerableSet.AddressSet internal validKeys;
```


## Functions
### isKeyValid

Checks if a public key is considered a valid TEE key.


```solidity
function isKeyValid(address publicKey) external view override returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`publicKey`|`address`|The public key to check.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the key is valid (i.e., marked as valid in `keyValidity`), false otherwise.|


### addKey


```solidity
function addKey(address publicKey) external;
```

### revokeAllKeys


```solidity
function revokeAllKeys() public;
```

## Events
### KeyAdded

```solidity
event KeyAdded(address indexed key);
```

### KeysRevoked

```solidity
event KeysRevoked();
```

