# SyndicateStorage

[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/backfill/SyndicateStorage.sol)

**Inherits:**
AccessControl

This contract is used to emit events containing App chain block and transaction data

_This contract uses AccessControl for managing permissions_

## State Variables

### appchainId

```solidity
uint256 public immutable appchainId;
```

### indexFromBlock

```solidity
uint256 public indexFromBlock;
```

### MANAGER_ROLE

```solidity
bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");
```

## Functions

### constructor

Constructor that sets up the default admin and manager roles

```solidity
constructor(address admin, address manager, uint256 appchainId_);
```

**Parameters**

| Name          | Type      | Description                                     |
| ------------- | --------- | ----------------------------------------------- |
| `admin`       | `address` | The address that will be the default admin role |
| `manager`     | `address` | The address that will be the manager role       |
| `appchainId_` | `uint256` | The App chain ID                                |

### setIndexFromBlock

Sets the index from block

```solidity
function setIndexFromBlock(uint256 blockNumber) external onlyRole(MANAGER_ROLE);
```

**Parameters**

| Name          | Type      | Description                             |
| ------------- | --------- | --------------------------------------- |
| `blockNumber` | `uint256` | The block number to start indexing from |

### save

Emits a Batch

_The third parameter is a batch: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format_

```solidity
function save(uint256 epochNumber, bytes32, bytes calldata) external onlyRole(MANAGER_ROLE);
```

**Parameters**

| Name          | Type      | Description      |
| ------------- | --------- | ---------------- |
| `epochNumber` | `uint256` | The epoch number |
| `<none>`      | `bytes32` |                  |
| `<none>`      | `bytes`   |                  |

### saveForMany

Emits many Batches

*https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/derivation.md#batch-format*

```solidity
function saveForMany(uint256[] calldata epochNumbers, bytes32[] calldata epochHashes, bytes[] calldata batches)
    external
    onlyRole(MANAGER_ROLE);
```

**Parameters**

| Name           | Type        | Description       |
| -------------- | ----------- | ----------------- |
| `epochNumbers` | `uint256[]` | The epoch numbers |
| `epochHashes`  | `bytes32[]` | The epoch hashes  |
| `batches`      | `bytes[]`   | The batches       |

## Events

### EpochRangeProcessed

Emits a EpochRangeProcessed indicating the range of epochs that have been processed

```solidity
event EpochRangeProcessed(uint256 indexed startEpochNumber, uint256 indexed endEpochNumber);
```

**Parameters**

| Name               | Type      | Description               |
| ------------------ | --------- | ------------------------- |
| `startEpochNumber` | `uint256` | The starting epoch number |
| `endEpochNumber`   | `uint256` | The ending epoch number   |
