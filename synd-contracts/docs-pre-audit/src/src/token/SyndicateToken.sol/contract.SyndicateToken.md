# SyndicateToken
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/token/SyndicateToken.sol)

**Inherits:**
ERC20, AccessControl, ERC20Permit


## State Variables
### MINTER_ROLE

```solidity
bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
```


## Functions
### constructor


```solidity
constructor(address defaultAdmin, address minter) ERC20("Syndicate", "SYND") ERC20Permit("Syndicate");
```

### mint


```solidity
function mint(address to, uint256 amount) public onlyRole(MINTER_ROLE);
```

