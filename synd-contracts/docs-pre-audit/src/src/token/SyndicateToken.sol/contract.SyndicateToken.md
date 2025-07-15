# SyndicateToken

[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/token/SyndicateToken.sol)

**Inherits:**
ERC20, AccessControl, ERC20Permit, ERC20Votes

**Author:**
Syndicate Protocol

The main Syndicate Protocol token

\*This contract implements the core SYND token deployed on Ethereum L1.
It provides basic ERC20 functionality with governance capabilities.
Emissions are handled by a separate SyndicateTokenEmissionScheduler contract.
Key Features:

- ERC20 token with 1B total supply (900M initial + 100M emissions)
- Voting functionality via ERC20Votes for governance
- Permit functionality for gasless approvals
- Controlled minting for emissions (only by emission scheduler)
- Time-based transfer restrictions for airdrop tokens
- Airdrop manager controls for emergency actions during lock period
- Comprehensive access controls
  Supply Distribution:
- 900M tokens (90%): Initial mint to foundation
- 100M tokens (10%): Available for emissions via emission scheduler\*

**Note:**
security-contact: security@syndicate.io

## State Variables

### EMISSION_MINTER_ROLE

Role for minting emission tokens (typically the emission scheduler)

```solidity
bytes32 public constant EMISSION_MINTER_ROLE = keccak256("EMISSION_MINTER_ROLE");
```

### AIRDROP_MANAGER_ROLE

Role for managing airdrop operations (transfer, burn during lock period)

```solidity
bytes32 public constant AIRDROP_MANAGER_ROLE = keccak256("AIRDROP_MANAGER_ROLE");
```

### TOTAL_SUPPLY

Total token supply: 1 billion tokens

```solidity
uint256 public constant TOTAL_SUPPLY = 1_000_000_000 * 10 ** 18;
```

### INITIAL_MINT_SUPPLY

Initial mint to foundation: 900 million tokens (90%)

```solidity
uint256 public constant INITIAL_MINT_SUPPLY = 900_000_000 * 10 ** 18;
```

### MAX_LOCK_DURATION

Maximum lock duration: 90 days

```solidity
uint256 public constant MAX_LOCK_DURATION = 90 days;
```

### unlockTimestamp

Timestamp when tokens become transferable (0 = no restrictions)

```solidity
uint256 public unlockTimestamp;
```

### maxLockTimestamp

Maximum timestamp until which tokens can be locked

```solidity
uint256 public immutable maxLockTimestamp;
```

## Functions

### constructor

Initialize the Syndicate token contract

```solidity
constructor(address defaultAdmin, address syndTreasuryAddress)
    ERC20("Syndicate", "SYND")
    ERC20Permit("Syndicate")
    ERC20Votes();
```

**Parameters**

| Name                  | Type      | Description                                     |
| --------------------- | --------- | ----------------------------------------------- |
| `defaultAdmin`        | `address` | Address that will have default admin privileges |
| `syndTreasuryAddress` | `address` | Address to receive the initial token mint       |

### mint

Mint tokens

_This function can only be called by addresses with EMISSION_MINTER_ROLE.
It is designed to be called by the SyndicateTokenEmissionScheduler contract._

```solidity
function mint(address to, uint256 amount) external virtual onlyRole(EMISSION_MINTER_ROLE);
```

**Parameters**

| Name     | Type      | Description                   |
| -------- | --------- | ----------------------------- |
| `to`     | `address` | The address to mint tokens to |
| `amount` | `uint256` | The amount of tokens to mint  |

### burn

Burn tokens from caller's balance

```solidity
function burn(uint256 amount) external;
```

**Parameters**

| Name     | Type      | Description              |
| -------- | --------- | ------------------------ |
| `amount` | `uint256` | Amount of tokens to burn |

### burnFrom

Burn tokens from a specific address (only during lock period)

_Only callable by addresses with AIRDROP_MANAGER_ROLE and only during lock period_

```solidity
function burnFrom(address from, uint256 amount) external onlyRole(AIRDROP_MANAGER_ROLE);
```

**Parameters**

| Name     | Type      | Description                 |
| -------- | --------- | --------------------------- |
| `from`   | `address` | Address to burn tokens from |
| `amount` | `uint256` | Amount of tokens to burn    |

### setUnlockTimestamp

Set the unlock timestamp for transfer restrictions

_Only callable by addresses with DEFAULT_ADMIN_ROLE_

```solidity
function setUnlockTimestamp(uint256 newUnlockTimestamp) external onlyRole(DEFAULT_ADMIN_ROLE);
```

**Parameters**

| Name                 | Type      | Description                                                          |
| -------------------- | --------- | -------------------------------------------------------------------- |
| `newUnlockTimestamp` | `uint256` | New timestamp when transfers become allowed (must be > 0 and future) |

### transfersLocked

Check if transfers are currently locked

```solidity
function transfersLocked() public view returns (bool);
```

**Returns**

| Name     | Type   | Description                                   |
| -------- | ------ | --------------------------------------------- |
| `<none>` | `bool` | True if transfers are locked, false otherwise |

### getRemainingLockTime

Get remaining lock time in seconds

```solidity
function getRemainingLockTime() external view returns (uint256);
```

**Returns**

| Name     | Type      | Description                                            |
| -------- | --------- | ------------------------------------------------------ |
| `<none>` | `uint256` | Seconds remaining until unlock (0 if already unlocked) |

### getVotingPower

Get voting power of an account at current block

```solidity
function getVotingPower(address account) external view returns (uint256);
```

**Parameters**

| Name      | Type      | Description          |
| --------- | --------- | -------------------- |
| `account` | `address` | The account to check |

**Returns**

| Name     | Type      | Description              |
| -------- | --------- | ------------------------ |
| `<none>` | `uint256` | The current voting power |

### getPastVotingPower

Get past voting power of an account at a specific block

```solidity
function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256);
```

**Parameters**

| Name          | Type      | Description               |
| ------------- | --------- | ------------------------- |
| `account`     | `address` | The account to check      |
| `blockNumber` | `uint256` | The block number to check |

**Returns**

| Name     | Type      | Description                    |
| -------- | --------- | ------------------------------ |
| `<none>` | `uint256` | The voting power at that block |

### getCurrentTotalSupply

Get total supply at current block

```solidity
function getCurrentTotalSupply() external view returns (uint256);
```

**Returns**

| Name     | Type      | Description          |
| -------- | --------- | -------------------- |
| `<none>` | `uint256` | Current total supply |

### \_update

_Override required by Solidity for multiple inheritance_

_Implements transfer restrictions based on lock timestamp and roles_

```solidity
function _update(address from, address to, uint256 value) internal virtual override(ERC20, ERC20Votes);
```

### nonces

_Override required by Solidity for multiple inheritance_

```solidity
function nonces(address owner) public view virtual override(ERC20Permit, Nonces) returns (uint256);
```

## Events

### UnlockTimestampUpdated

Emitted when unlock timestamp is updated

```solidity
event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);
```

**Parameters**

| Name           | Type      | Description                        |
| -------------- | --------- | ---------------------------------- |
| `oldTimestamp` | `uint256` | Previous unlock timestamp          |
| `newTimestamp` | `uint256` | New unlock timestamp               |
| `updatedBy`    | `address` | Address that updated the timestamp |

### TokensBurnedByManager

Emitted when tokens are burned by airdrop manager

```solidity
event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);
```

**Parameters**

| Name     | Type      | Description                     |
| -------- | --------- | ------------------------------- |
| `from`   | `address` | Address tokens were burned from |
| `amount` | `uint256` | Amount of tokens burned         |
| `burner` | `address` | Address that performed the burn |

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

### ExceedsTotalSupply

Thrown when trying to mint more than the total supply allows

```solidity
error ExceedsTotalSupply();
```

### TransfersLocked

Thrown when transfers are locked and caller is not authorized

```solidity
error TransfersLocked();
```

### UnlockTimestampTooLate

Thrown when trying to set unlock timestamp beyond maximum allowed

```solidity
error UnlockTimestampTooLate();
```

### UnlockTimestampInPast

Thrown when trying to set unlock timestamp in the past

```solidity
error UnlockTimestampInPast();
```

### BurnOnlyDuringLockPeriod

Thrown when trying to burn tokens outside of lock period

```solidity
error BurnOnlyDuringLockPeriod();
```
