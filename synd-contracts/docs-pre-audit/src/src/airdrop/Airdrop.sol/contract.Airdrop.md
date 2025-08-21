# Airdrop
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/airdrop/Airdrop.sol)

Turbo gas optimized bulk transfers of ERC20

*Inspired by the author Harrison (@PopPunkOnChain) contract GasliteDrop at https://github.com/PopPunkLLC/GasliteDrop/blob/main/contracts/src/GasliteDrop.sol*


## Functions
### airdropERC20

Airdrop ERC20 tokens to a list of addresses


```solidity
function airdropERC20(address _token, address[] calldata _addresses, uint256[] calldata _amounts, uint256 _totalAmount)
    external
    payable;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_token`|`address`|The address of the ERC20 contract|
|`_addresses`|`address[]`|The addresses to airdrop to|
|`_amounts`|`uint256[]`|The amounts to airdrop|
|`_totalAmount`|`uint256`|The total amount to airdrop|


## Errors
### ArrayLengthMismatch
Thrown when arrays length mismatch


```solidity
error ArrayLengthMismatch();
```

### TransferFailed
Thrown when token transfer fails


```solidity
error TransferFailed();
```

