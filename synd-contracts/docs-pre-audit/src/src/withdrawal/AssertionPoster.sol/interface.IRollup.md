# IRollup
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/AssertionPoster.sol)

**Inherits:**
IRollupCore, IOwnable

*Extends IRollupCore and IOwnable to define additional functions needed for assertion management*


## Functions
### forceCreateNode

Forces the creation of a new node in the rollup chain


```solidity
function forceCreateNode(
    uint64 prevNode,
    uint256 prevNodeInboxMaxCount,
    Assertion memory assertion,
    bytes32 expectedNodeHash
) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`prevNode`|`uint64`|Index of the previous node|
|`prevNodeInboxMaxCount`|`uint256`|Max inbox count for the previous node|
|`assertion`|`Assertion`|The assertion data|
|`expectedNodeHash`|`bytes32`|Expected hash of the new node|


### forceConfirmNode

Forces confirmation of a node in the rollup chain


```solidity
function forceConfirmNode(uint64 nodeNum, bytes32 blockHash, bytes32 sendRoot) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`nodeNum`|`uint64`|Index of the node to confirm|
|`blockHash`|`bytes32`|Hash of the block|
|`sendRoot`|`bytes32`|Root of the sends|


### computeAssertionHash

Computes the hash of an assertion


```solidity
function computeAssertionHash(bytes32 prevAssertionHash, AssertionState calldata state, bytes32 inboxAcc)
    external
    pure
    returns (bytes32);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`prevAssertionHash`|`bytes32`|The hash of the assertion's parent|
|`state`|`AssertionState`|The execution state for the assertion|
|`inboxAcc`|`bytes32`|The inbox batch accumulator|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|bytes32 The computed assertion hash|


### fastConfirmNewAssertion

Allows immediate creation and confirmation of an assertion by the anyTrustFastConfirmer

*Only intended for AnyTrust chains with sufficient signatures*


```solidity
function fastConfirmNewAssertion(AssertionInputs calldata assertion, bytes32 expectedAssertionHash) external;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`assertion`|`AssertionInputs`|The assertion inputs|
|`expectedAssertionHash`|`bytes32`|Expected hash of the assertion|


### genesisAssertionHash


```solidity
function genesisAssertionHash() external pure returns (bytes32);
```

### getValidators


```solidity
function getValidators() external view returns (address[] memory);
```

