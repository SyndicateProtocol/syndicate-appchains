# PublicValuesStruct

[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/AttestationDocVerifier.sol)

```solidity
struct PublicValuesStruct {
    bytes32 root_cert_hash;
    uint64 validity_window_start;
    uint64 validity_window_end;
    bytes32 pcr_0;
    bytes32 pcr_1;
    bytes32 pcr_2;
    address tee_signing_key;
}
```
