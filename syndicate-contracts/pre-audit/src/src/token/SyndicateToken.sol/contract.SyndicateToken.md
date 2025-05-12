# SyndicateToken
[Git Source](https://github.com/SyndicateProtocol/metabased-rollup/blob/4da316517677819af5853c256a98505484d835fa/src/token/SyndicateToken.sol)

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

