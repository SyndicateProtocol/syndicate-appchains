# SyndicateToken
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/eb4946a298148d1c686f65f1f0883c9daf2b87fe/src/token/SyndicateToken.sol)

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

