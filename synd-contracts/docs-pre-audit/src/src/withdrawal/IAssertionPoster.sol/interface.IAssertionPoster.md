# IAssertionPoster
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/IAssertionPoster.sol)


## Functions
### postAssertion

Posts a new assertion to the rollup


```solidity
function postAssertion(bytes32 blockHash, bytes32 sendRoot) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`blockHash`|`bytes32`|Hash of the block|
|`sendRoot`|`bytes32`|Root of the sends|


