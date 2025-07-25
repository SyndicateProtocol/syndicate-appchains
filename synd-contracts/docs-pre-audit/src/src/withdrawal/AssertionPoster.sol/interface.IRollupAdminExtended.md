# IRollupAdminExtended
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/AssertionPoster.sol)

**Inherits:**
IRollupAdmin

*Extends IRollupAdmin to include a function for checking if the rollup is paused*


## Functions
### paused

Check if the rollup contract is paused


```solidity
function paused() external view returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the rollup contract is paused|


### setValidatorAfkBlocks


```solidity
function setValidatorAfkBlocks(uint64 newAfkBlocks) external;
```

### setAnyTrustFastConfirmer


```solidity
function setAnyTrustFastConfirmer(address _anyTrustFastConfirmer) external;
```

