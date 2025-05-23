# NotInitializedModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/f93e91004eb8d04d84acd3b9cb0e8f7e6abfa528/src/sequencing-modules/NotInitializedModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A module that returns false for all addresses


## Functions
### isAllowed

Returns false for all addresses


```solidity
function isAllowed(address, address, bytes calldata) external pure override returns (bool);
```

