# IRequirementModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/interfaces/IRequirementModule.sol)

**Inherits:**
[PermissionModule](/src/interfaces/PermissionModule.sol/interface.PermissionModule.md)

Interface for requirement modules


## Functions
### addCheck


```solidity
function addCheck(address _address, bool addToHead) external;
```

### removeCheck


```solidity
function removeCheck(address _address) external;
```

### getAllChecks


```solidity
function getAllChecks() external view returns (address[] memory);
```

