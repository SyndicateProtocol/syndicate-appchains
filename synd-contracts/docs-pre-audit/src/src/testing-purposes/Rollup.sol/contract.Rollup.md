# Rollup
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/testing-purposes/Rollup.sol)

This is a contract solely for testing purposes. Not to be used in production.


## State Variables
### maxDataSize

```solidity
uint64 public constant maxDataSize = 117964;
```


### delayBlocks

```solidity
uint64 public constant delayBlocks = 7200;
```


### futureBlocks

```solidity
uint64 public constant futureBlocks = 12;
```


### delaySeconds

```solidity
uint64 public constant delaySeconds = 86400;
```


### futureSeconds

```solidity
uint64 public constant futureSeconds = 3600;
```


### seqBlockNumber

```solidity
uint64 public seqBlockNumber = 0;
```


### seqBlockHash

```solidity
uint256 public seqBlockHash = 0;
```


### setBlockNumber

```solidity
uint64 public setBlockNumber = 0;
```


### setBlockHash

```solidity
uint256 public setBlockHash = 0;
```


### owner

```solidity
address public owner;
```


### delayedInboxAccs

```solidity
bytes32[] public delayedInboxAccs;
```


### sequencerInboxAccs

```solidity
bytes32[] public sequencerInboxAccs;
```


### totalDelayedMessagesRead

```solidity
uint256 public totalDelayedMessagesRead;
```


### INITIALIZATION_MSG_TYPE

```solidity
uint8 public constant INITIALIZATION_MSG_TYPE = 11;
```


### L1MessageType_ethDeposit

```solidity
uint8 public constant L1MessageType_ethDeposit = 12;
```


### HEADER_LENGTH

```solidity
uint256 internal constant HEADER_LENGTH = 40;
```


## Functions
### constructor


```solidity
constructor(uint256 chainId, string memory chainConfig, address owner_);
```

### delayedMessageCount


```solidity
function delayedMessageCount() external view returns (uint256);
```

### sequencerMessageCount


```solidity
function sequencerMessageCount() external view returns (uint256);
```

### batchCount


```solidity
function batchCount() external view returns (uint256);
```

### inboxAccs


```solidity
function inboxAccs(uint256 index) external view returns (bytes32);
```

### getSourceChainsProcessedBlocks


```solidity
function getSourceChainsProcessedBlocks()
    external
    view
    returns (uint64 _seqBlockNumber, uint256 _seqBlockHash, uint64 _setBlockNumber, uint256 _setBlockHash);
```

### postBatch


```solidity
function postBatch(
    bytes memory data,
    uint64 _seqBlockNumber,
    uint256 _seqBlockHash,
    uint64 _setBlockNumber,
    uint256 _setBlockHash
) public;
```

### depositEth


```solidity
function depositEth(address src, address dest, uint256 value) external;
```

### deliverMessage


```solidity
function deliverMessage(uint8 kind, address sender, bytes memory messageData) public;
```

### formCallDataHash


```solidity
function formCallDataHash(bytes memory data, uint256 afterDelayedMessagesRead)
    internal
    view
    returns (bytes32, IBridge.TimeBounds memory);
```

## Errors
### DataTooLarge
*Provided data was too large*


```solidity
error DataTooLarge(uint256 dataLength, uint256 maxDataLength);
```

**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`dataLength`|`uint256`|The length of the data that is too large|
|`maxDataLength`|`uint256`|The max length the data can be|

