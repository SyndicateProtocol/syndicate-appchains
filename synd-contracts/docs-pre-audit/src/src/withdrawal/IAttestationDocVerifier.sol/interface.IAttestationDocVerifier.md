# IAttestationDocVerifier
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/IAttestationDocVerifier.sol)


## Functions
### verifyAttestationDocProof

Verifies the attestation document proof.


```solidity
function verifyAttestationDocProof(bytes calldata _publicValues, bytes calldata _proofBytes)
    external
    view
    returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_publicValues`|`bytes`|The encoded public values.|
|`_proofBytes`|`bytes`|The encoded proof.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The public key hold by the TEE.|


