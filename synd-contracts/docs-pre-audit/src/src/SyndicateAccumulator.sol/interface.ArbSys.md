# ArbSys
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/SyndicateAccumulator.sol)

Interface for Arbitrum's ArbSys precompile contract

*Used to get Arbitrum-specific block numbers when running on Arbitrum chains*


## Functions
### arbBlockNumber

Returns the current Arbitrum block number


```solidity
function arbBlockNumber() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The current block number on Arbitrum|


