# TestnetSyndToken
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/TestnetSyndToken.sol)

**Inherits:**
ERC20, AccessControl, ERC20Permit, ERC20Votes

Testnet version of Syndicate Token with testing utilities

*This contract is for testnet deployments. It is ERC20Votes and ERC20Permit compatible.*


## State Variables
### MINTER_ROLE

```solidity
bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
```


## Functions
### constructor


```solidity
constructor(address defaultAdmin, address minter)
    ERC20("Testnet Syndicate", "TestnetSYND")
    ERC20Permit("Testnet Syndicate")
    ERC20Votes();
```

### mint

Mint function for testing (uses MINTER_ROLE)


```solidity
function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|The address to mint tokens to|
|`amount`|`uint256`|The amount of tokens to mint|


### getVotingPower

Get voting power of an account at current block


```solidity
function getVotingPower(address account) external view returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`account`|`address`|The account to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The current voting power|


### getPastVotingPower

Get past voting power of an account at a specific block


```solidity
function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`account`|`address`|The account to check|
|`blockNumber`|`uint256`|The block number to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The voting power at that block|


### getCurrentTotalSupply

Get total supply at current block


```solidity
function getCurrentTotalSupply() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|Current total supply|


### _update

*Override required by Solidity for multiple inheritance*


```solidity
function _update(address from, address to, uint256 value) internal virtual override(ERC20, ERC20Votes);
```

### nonces

*Override required by Solidity for multiple inheritance*


```solidity
function nonces(address owner) public view virtual override(ERC20Permit, Nonces) returns (uint256);
```

## Errors
### ZeroAddress
Thrown when an address is zero


```solidity
error ZeroAddress();
```

### ZeroAmount
Thrown when an amount is zero


```solidity
error ZeroAmount();
```

