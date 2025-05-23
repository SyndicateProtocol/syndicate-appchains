# SyndicateToken
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/token/SyndicateToken.sol)

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

