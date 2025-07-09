# AttestationDocVerifier
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/AttestationDocVerifier.sol)

**Inherits:**
[IAttestationDocVerifier](/src/withdrawal/IAttestationDocVerifier.sol/interface.IAttestationDocVerifier.md)


## State Variables
### verifier
The address of the SP1 verifier contract.

*This can either be a specific SP1Verifier for a specific version, or the
SP1VerifierGateway which can be used to verify proofs for any version of SP1.
For the list of supported verifiers on each chain, see:
https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments*


```solidity
address public immutable verifier;
```


### attestationDocVerifierVKey
The verification key for the cert verifier.


```solidity
bytes32 public immutable attestationDocVerifierVKey;
```


### rootCertHash
The expected values for the attestation document.


```solidity
bytes32 public immutable rootCertHash;
```


### pcr0

```solidity
bytes32 public immutable pcr0;
```


### pcr1

```solidity
bytes32 public immutable pcr1;
```


### pcr2

```solidity
bytes32 public immutable pcr2;
```


### pcr8

```solidity
bytes32 public immutable pcr8;
```


### expirationTolerance

```solidity
uint64 public immutable expirationTolerance;
```


## Functions
### constructor


```solidity
constructor(
    address _verifier,
    bytes32 _attestationDocVerifierVKey,
    bytes32 _rootCertHash,
    bytes32 _pcr0,
    bytes32 _pcr1,
    bytes32 _pcr2,
    bytes32 _pcr8,
    uint64 _expirationTolerance
);
```

### verifyAttestationDocProof

The entrypoint for verifying the proof of a certificate.


```solidity
function verifyAttestationDocProof(bytes calldata _publicValues, bytes calldata _proofBytes)
    public
    view
    returns (address);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_publicValues`|`bytes`|The encoded public values.|
|`_proofBytes`|`bytes`|The encoded proof.|


