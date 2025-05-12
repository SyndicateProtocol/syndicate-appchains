# TokenBalanceSequencingModule
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/sequencing-modules/TokenBalanceSequencingModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

*This contract allows sequencing based on the caller's token balance.*

*Useful in case Syndicate releases a token and wants to allow only token holders to sequence.*


## State Variables
### tokenAddress
The address of the ERC20 token contract.


```solidity
address public immutable tokenAddress;
```


### minimumBalance
The minimum token balance required to be allowed.


```solidity
uint256 public immutable minimumBalance;
```


## Functions
### constructor

*Sets the token address and minimum balance required.*


```solidity
constructor(address _tokenAddress, uint256 _minimumBalance);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_tokenAddress`|`address`|The address of the ERC20 token contract.|
|`_minimumBalance`|`uint256`|The minimum token balance required to be allowed.|


### isAllowed

Checks if the caller is allowed based on their token balance.


```solidity
function isAllowed(address proposer, address, bytes calldata) external view override returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|bool indicating if the caller is allowed.|


