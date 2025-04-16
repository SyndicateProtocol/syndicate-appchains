# RequireAnyModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/564ccf6a3d85fe3c184cae4f9cbab9ecfb6401c6/src/requirement-modules/RequireAnyModule.sol)

**Inherits:**
[IRequirementModule](/src/interfaces/IRequirementModule.sol/interface.IRequirementModule.md), Ownable

A module that requires at least one check to pass


## State Variables
### checks

```solidity
AddressStructuredLinkedList.List private checks;
```


## Functions
### constructor


```solidity
constructor(address admin) Ownable(admin);
```

### isAllowed


```solidity
function isAllowed(address proposer) external view override returns (bool);
```

### addCheck


```solidity
function addCheck(address _address, bool addToHead) external override onlyOwner;
```

### removeCheck


```solidity
function removeCheck(address _address) external override onlyOwner;
```

### getAllChecks


```solidity
function getAllChecks() external view override returns (address[] memory);
```

## Events
### CheckAdded

```solidity
event CheckAdded(address indexed check);
```

### CheckRemoved

```solidity
event CheckRemoved(address indexed check);
```

## Errors
### CheckFailed

```solidity
error CheckFailed(address batchSubmitter);
```

### InvalidAddress

```solidity
error InvalidAddress();
```

### AddressAlreadyExists

```solidity
error AddressAlreadyExists();
```

### AddressDoesNotExist

```solidity
error AddressDoesNotExist();
```

