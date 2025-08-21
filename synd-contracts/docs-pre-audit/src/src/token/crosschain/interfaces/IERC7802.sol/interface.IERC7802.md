# IERC7802
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/crosschain/interfaces/IERC7802.sol)

**Inherits:**
IERC165

Interface for crosschain token minting and burning.

*See https://eips.ethereum.org/EIPS/eip-7802*

*Used in SuperchainERC20 contract*


## Functions
### crosschainMint

Mints tokens to the recipient (called by authorized bridges)


```solidity
function crosschainMint(address to, uint256 amount) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|The address of the recipient|
|`amount`|`uint256`|The amount of tokens to mint|


### crosschainBurn

Burns tokens from the sender (called by authorized bridges)


```solidity
function crosschainBurn(address from, uint256 amount) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`from`|`address`|The address of the sender|
|`amount`|`uint256`|The amount of tokens to burn|


## Events
### CrosschainMint
Emitted when tokens are minted by a bridge


```solidity
event CrosschainMint(address indexed to, uint256 amount, address indexed bridge);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`to`|`address`|The address of the recipient|
|`amount`|`uint256`|The amount of tokens minted|
|`bridge`|`address`|The address of the bridge that minted the tokens|

### CrosschainBurn
Emitted when tokens are burned by a bridge


```solidity
event CrosschainBurn(address indexed from, uint256 amount, address indexed bridge);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`from`|`address`|The address of the sender|
|`amount`|`uint256`|The amount of tokens burned|
|`bridge`|`address`|The address of the bridge that burned the tokens|

