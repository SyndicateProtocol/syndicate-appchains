# TeeKeyManager
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/TeeKeyManager.sol)

**Inherits:**
[ITeeKeyManager](/src/withdrawal/ITeeKeyManager.sol/interface.ITeeKeyManager.md), Ownable

Manages TEE program hashes and their associated public keys.
Allows anyone to add a key, by providing a proof for a valid attestation document.
to check if a given key is valid (i.e., associated with an active program).


## State Variables
### attestationDocVerifier

```solidity
IAttestationDocVerifier public attestationDocVerifier;
```


### validKeys

```solidity
EnumerableSet.AddressSet internal validKeys;
```


## Functions
### constructor


```solidity
constructor(IAttestationDocVerifier _attestationDocVerifier) Ownable(msg.sender);
```

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

Adds a public key to the valid keys mapping.


```solidity
function addKey(bytes calldata _publicValues, bytes calldata _proofBytes) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_publicValues`|`bytes`|The encoded public values.|
|`_proofBytes`|`bytes`|The encoded proof.|


### revokeAllKeys

Revokes all keys.


```solidity
function revokeAllKeys() public onlyOwner;
```

### updateAttestationDocVerifier

Updates the attestation doc verifier.


```solidity
function updateAttestationDocVerifier(IAttestationDocVerifier _attestationDocVerifier) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_attestationDocVerifier`|`IAttestationDocVerifier`|The new attestation doc verifier.|


## Events
### KeyAdded

```solidity
event KeyAdded(address indexed key);
```

### KeysRevoked

```solidity
event KeysRevoked();
```

