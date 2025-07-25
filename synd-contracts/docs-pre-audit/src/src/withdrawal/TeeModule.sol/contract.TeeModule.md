# TeeModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/TeeModule.sol)

**Inherits:**
Ownable, ReentrancyGuard


## State Variables
### poster

```solidity
IAssertionPoster public immutable poster;
```


### bridge

```solidity
IBridge public immutable bridge;
```


### l1BlockOrBridge

```solidity
address public immutable l1BlockOrBridge;
```


### isL1Chain

```solidity
bool public immutable isL1Chain;
```


### teeKeyManager

```solidity
ITeeKeyManager public immutable teeKeyManager;
```


### teeTrustedInput

```solidity
TeeTrustedInput public teeTrustedInput;
```


### pendingAssertions

```solidity
PendingAssertion[] public pendingAssertions;
```


### teeHackCount

```solidity
uint256 public teeHackCount;
```


### challengeWindowEnd

```solidity
uint64 public challengeWindowEnd;
```


### challengeWindowDuration

```solidity
uint64 public challengeWindowDuration;
```


## Functions
### receive


```solidity
receive() external payable;
```

### constructor

Constructs the TeeModule contract


```solidity
constructor(
    IAssertionPoster poster_,
    IBridge bridge_,
    bytes32 configHash_,
    bytes32 appStartBlockHash_,
    bytes32 seqStartBlockHash_,
    bytes32 l1StartBatchAcc_,
    address l1BlockOrBridge_,
    bool isL1Chain_,
    uint64 challengeWindowDuration_,
    ITeeKeyManager teeKeyManager_
);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`poster_`|`IAssertionPoster`|Address of the assertion poster contract|
|`bridge_`|`IBridge`|Settlement chain address of the appchain `Bridge` contract|
|`configHash_`|`bytes32`|Hash of the configuration data passed to the TEE|
|`appStartBlockHash_`|`bytes32`|The starting block hash of the appchain|
|`seqStartBlockHash_`|`bytes32`|The starting block hash of the sequencing chain|
|`l1StartBatchAcc_`|`bytes32`|The sequencing chain start batch accumulator|
|`l1BlockOrBridge_`|`address`|Address of the l1 block contract - 0x4200000000000000000000000000000000000015 for bedrock rollups - or the l1 <-> sequencing chain bridge if the settlement chain is the same as the l1 chain.|
|`isL1Chain_`|`bool`|True if l1BlockOrBridge is the bridge address and false if it is the l1 block contract address instead|
|`challengeWindowDuration_`|`uint64`|The duration of the challenge window in seconds|
|`teeKeyManager_`|`ITeeKeyManager`|The address of the TEE key manager contract Note that the AssertionPoster must be owned by the TeeModule for closing the challenge window to work properly|


### closeChallengeWindow


```solidity
function closeChallengeWindow() public nonReentrant;
```

### submitAssertion


```solidity
function submitAssertion(PendingAssertion calldata assertion, bytes calldata signature, address rewardAddr)
    external
    nonReentrant;
```

### resolveChallenge


```solidity
function resolveChallenge(PendingAssertion calldata assertion) external onlyOwner nonReentrant;
```

