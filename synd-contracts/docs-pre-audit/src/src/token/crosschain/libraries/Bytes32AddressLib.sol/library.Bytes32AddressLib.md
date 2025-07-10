# Bytes32AddressLib
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/crosschain/libraries/Bytes32AddressLib.sol)

**Author:**
Solmate (https://github.com/Rari-Capital/solmate/blob/main/src/utils/Bytes32AddressLib.sol)

Library for converting between addresses and bytes32 values.


## Functions
### fromLast20Bytes


```solidity
function fromLast20Bytes(bytes32 bytesValue) internal pure returns (address);
```

### fillLast12Bytes


```solidity
function fillLast12Bytes(address addressValue) internal pure returns (bytes32);
```

