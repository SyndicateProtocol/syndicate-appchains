# AssertionPoster
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/withdrawal/AssertionPoster.sol)

**Inherits:**
Ownable

Supports both legacy (v2) and new (v3) Nitro contracts

*Facilitates posting assertions to an Arbitrum rollup chain*


## State Variables
### self

```solidity
address private immutable self;
```


### rollup

```solidity
IRollup private immutable rollup;
```


### executor

```solidity
IUpgradeExecutor private immutable executor;
```


### legacy

```solidity
bool private immutable legacy;
```


### seqBatchAcc

```solidity
bytes32 private immutable seqBatchAcc;
```


### state

```solidity
GlobalState private state;
```


### nodeNum

```solidity
uint64 private nodeNum;
```


### currentInboxSize

```solidity
uint64 private currentInboxSize;
```


### assertionHash

```solidity
bytes32 private assertionHash;
```


### prevAssertionHash

```solidity
bytes32 private prevAssertionHash;
```


### data

```solidity
ConfigData private data;
```


## Functions
### constructor

Constructs the AssertionPoster contract


```solidity
constructor(IRollup rollup_) Ownable(msg.sender);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`rollup_`|`IRollup`|Address of the rollup contract|


### configure

Configures the contract by configuring the rollup's permissions

*Must be called via upgrade executor delegatecall*


```solidity
function configure() external;
```

### postAssertion

Posts a new assertion to the rollup


```solidity
function postAssertion(bytes32 blockHash, bytes32 sendRoot) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`blockHash`|`bytes32`|Hash of the block|
|`sendRoot`|`bytes32`|Root of the sends|


### _configureLegacy

Configures the contract for legacy rollup (v2)

*Sets up executor role and pauses regular assertion creation*


```solidity
function _configureLegacy() private;
```

### _configureNew

Configures the contract for new rollup (v3)

*Sets up validators, posts initial batch if needed*


```solidity
function _configureNew() private;
```

### _postInitialBatch

Posts an initial batch to the sequencer inbox

*Temporarily sets executor as batch poster if needed*


```solidity
function _postInitialBatch() private;
```

### _postLegacyAssertion

Posts an assertion using the legacy rollup protocol (v2)


```solidity
function _postLegacyAssertion(bytes32 blockHash, bytes32 sendRoot) private;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`blockHash`|`bytes32`|Hash of the block|
|`sendRoot`|`bytes32`|Root of the sends|


### _postNewAssertion

Posts an assertion using the new rollup protocol (v3)


```solidity
function _postNewAssertion(bytes32 blockHash, bytes32 sendRoot) private;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`blockHash`|`bytes32`|Hash of the block|
|`sendRoot`|`bytes32`|Root of the sends|


### _updateConfigData

Updates the config data with latest values from the rollup


```solidity
function _updateConfigData() private;
```

