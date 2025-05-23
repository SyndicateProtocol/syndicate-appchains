# ArbSys
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/SyndicateAccumulator.sol)

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


