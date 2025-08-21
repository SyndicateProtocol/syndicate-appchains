# TestnetSyndTokenCrosschain
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/TestnetSyndTokenCrosschain.sol)

**Inherits:**
[SyndicateTokenCrosschain](/src/token/SyndicateTokenCrosschain.sol/contract.SyndicateTokenCrosschain.md)

Testnet crosschain-compatible Syndicate Protocol token

*Combines TestnetSyndToken functionality with SyndicateTokenCrosschain capabilities
This contract provides all the features of both parent contracts:
- TestnetSyndToken: Flexible minting for testing, no initial supply constraints
- SyndicateTokenCrosschain: Crosschain capabilities, rate limiting, emission budgets
Key Features:
- All TestnetSyndToken functionality (flexible minting for testing)
- All SyndicateTokenCrosschain functionality (crosschain capabilities, rate limiting, emission budgets)
- Secure bridge management and sliding window rate limiting
- Emission budget controls for crosschain minting
- Same contract address across all chains via deterministic deployment*


## State Variables
### MINTER_ROLE
Role for minting tokens during testnet development


```solidity
bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
```


## Functions
### constructor

Initialize the crosschain testnet Syndicate token


```solidity
constructor(address defaultAdmin, address minter) SyndicateTokenCrosschain(defaultAdmin, defaultAdmin);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`defaultAdmin`|`address`|Address that will have default admin privileges|
|`minter`|`address`|Address that will have minter privileges for testnet|


### mint

Mint tokens for testnet purposes

*This function allows flexible minting during testnet development*


```solidity
function mint(address to, uint256 amount) external override onlyRole(MINTER_ROLE);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|Address to mint tokens to|
|`amount`|`uint256`|Amount of tokens to mint|


### name

Returns the name of the token


```solidity
function name() public pure override returns (string memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`string`|The name of the testnet token|


### symbol

Returns the symbol of the token


```solidity
function symbol() public pure override returns (string memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`string`|The symbol of the testnet token|


### supportsInterface

Check if contract supports an interface


```solidity
function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`interfaceId`|`bytes4`|Interface identifier to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|true if interface is supported|


