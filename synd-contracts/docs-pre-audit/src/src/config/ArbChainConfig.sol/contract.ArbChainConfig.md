# ArbChainConfig
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/7027a63d41514909f85c2d3245a5d979fd2c367a/src/config/ArbChainConfig.sol)

**Inherits:**
Initializable

*Configuration contract for settlement chain parameters*


## State Variables
### owner

```solidity
address public owner;
```


### INITIAL_APPCHAIN_OWNER

```solidity
address public INITIAL_APPCHAIN_OWNER;
```


### ARBITRUM_BRIDGE_ADDRESS

```solidity
address public ARBITRUM_BRIDGE_ADDRESS;
```


### ARBITRUM_INBOX_ADDRESS

```solidity
address public ARBITRUM_INBOX_ADDRESS;
```


### SEQUENCING_CONTRACT_ADDRESS

```solidity
address public SEQUENCING_CONTRACT_ADDRESS;
```


### ARBITRUM_IGNORE_DELAYED_MESSAGES

```solidity
bool public ARBITRUM_IGNORE_DELAYED_MESSAGES;
```


### CHAIN_ID

```solidity
uint256 public CHAIN_ID;
```


### SEQUENCING_CHAIN_ID

```solidity
uint256 public SEQUENCING_CHAIN_ID;
```


### SETTLEMENT_DELAY

```solidity
uint256 public SETTLEMENT_DELAY;
```


### SETTLEMENT_START_BLOCK

```solidity
uint256 public SETTLEMENT_START_BLOCK;
```


### SEQUENCING_START_BLOCK

```solidity
uint256 public SEQUENCING_START_BLOCK;
```


### ALLOWED_SETTLEMENT_ADDRESSES

```solidity
address[] public ALLOWED_SETTLEMENT_ADDRESSES;
```


### DEFAULT_SEQUENCING_CHAIN_RPC_URL

```solidity
string public DEFAULT_SEQUENCING_CHAIN_RPC_URL;
```


### APPCHAIN_BLOCK_EXPLORER_URL

```solidity
string public APPCHAIN_BLOCK_EXPLORER_URL;
```


## Functions
### constructor

*Constructor for the implementation contract
This is only used when deploying the implementation contract
It will not be called when deploying proxies*


```solidity
constructor();
```

### initialize

*Initializer function - replaces constructor for proxy pattern*


```solidity
function initialize(
    address _owner,
    uint256 chainId,
    uint256 sequencingChainId,
    address arbitrumBridgeAddress,
    address arbitrumInboxAddress,
    bool arbitrumIgnoreDelayedMessages,
    uint256 settlementDelay,
    uint256 settlementStartBlock,
    address sequencingContractAddress,
    uint256 sequencingStartBlock,
    address initialAppchainOwner,
    string memory sequencingChainRpcUrl,
    string memory appchainBlockExplorerUrl,
    address[] memory allowedSettlementAddresses
) public initializer;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_owner`|`address`|The address of the contract owner|
|`chainId`|`uint256`|The chain ID|
|`sequencingChainId`|`uint256`|The ID of the sequencing chain|
|`arbitrumBridgeAddress`|`address`|Address of the Arbitrum bridge|
|`arbitrumInboxAddress`|`address`|Address of the Arbitrum inbox|
|`arbitrumIgnoreDelayedMessages`|`bool`|Whether to ignore delayed messages|
|`settlementDelay`|`uint256`|Delay for settlement|
|`settlementStartBlock`|`uint256`|Starting block for settlement|
|`sequencingContractAddress`|`address`|Address of the sequencing contract|
|`sequencingStartBlock`|`uint256`|Starting block for sequencing|
|`initialAppchainOwner`|`address`|Initial appchain owner|
|`sequencingChainRpcUrl`|`string`|Default RPC URL for the sequencing chain|
|`appchainBlockExplorerUrl`|`string`|URL for the appchain block explorer|
|`allowedSettlementAddresses`|`address[]`|Array of addresses allowed for settlement|


### onlyOwner

*Modifier to check if the caller is the owner*


```solidity
modifier onlyOwner();
```

### updateDefaultSequencingChainRpcUrl

*Update DEFAULT_SEQUENCING_CHAIN_RPC_URL*


```solidity
function updateDefaultSequencingChainRpcUrl(string calldata newRpcUrl) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`newRpcUrl`|`string`|The new RPC URL for sequencing chain|


### updateAppchainBlockExplorerUrl

*Update APPCHAIN_BLOCK_EXPLORER_URL*


```solidity
function updateAppchainBlockExplorerUrl(string calldata newUrl) external onlyOwner;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`newUrl`|`string`|The new URL for the appchain block explorer|


### transferOwnership

*Transfers ownership of the contract to a new account (`newOwner`).
Can only be called by the current owner.*


```solidity
function transferOwnership(address newOwner) public virtual onlyOwner;
```

### getAllowedSettlementAddresses

*Get the allowed settlement addresses*


```solidity
function getAllowedSettlementAddresses() public view returns (address[] memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address[]`|The allowed settlement addresses|


### _transferOwnership

*Transfers ownership of the contract to a new account (`newOwner`).
Internal function without access restriction.*


```solidity
function _transferOwnership(address newOwner) internal virtual;
```

## Events
### DefaultSequencingChainRpcUrlUpdated

```solidity
event DefaultSequencingChainRpcUrlUpdated(string newRpcUrl);
```

### AppchainBlockExplorerUrlUpdated

```solidity
event AppchainBlockExplorerUrlUpdated(string newUrl);
```

### OwnershipTransferred

```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```

