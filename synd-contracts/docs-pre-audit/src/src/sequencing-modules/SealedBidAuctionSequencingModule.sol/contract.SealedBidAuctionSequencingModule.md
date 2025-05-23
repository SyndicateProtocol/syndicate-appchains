# SealedBidAuctionSequencingModule
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/sequencing-modules/SealedBidAuctionSequencingModule.sol)

**Inherits:**
[IPermissionModule](/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)

A sealed bid auction sequencing module contract where all bidders simultaneously submit sealed bids so that no bidder knows how much the other auction participants have bid.


## State Variables
### treasury
The address to receive the highest bid of the auction.


```solidity
address public treasury;
```


### auctionType
The type of auction.


```solidity
string public auctionType;
```


### auctionActive
Indicates if the auction is active.


```solidity
bool public auctionActive;
```


### endTime
The end time of the auction.


```solidity
uint256 public endTime;
```


### highestBidder
The address of the highest bidder.


```solidity
address public highestBidder;
```


### highestBid
The highest bid amount, initialized to 0 by design as no bids exist at contract creation.


```solidity
uint256 public highestBid;
```


### bids
Mapping to store bids of each participant.


```solidity
mapping(address => Bid) public bids;
```


### refunds
Mapping to store refunds for bidders who didn't win.


```solidity
mapping(address => uint256) public refunds;
```


## Functions
### onlyActive


```solidity
modifier onlyActive();
```

### constructor

Constructor to initialize the auction.


```solidity
constructor(uint256 _duration, address _treasury);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_duration`|`uint256`|Duration of the auction in seconds.|
|`_treasury`|`address`|The address to receive the auction highest bid.|


### getAuctionType

Returns the type of the auction.


```solidity
function getAuctionType() external view returns (string memory);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`string`|The auction type as a string.|


### finalizeAuction

Finalizes the auction, transferring the highest bid to the treasury.


```solidity
function finalizeAuction() external;
```

### isAuctionActive

Checks if the auction is active.


```solidity
function isAuctionActive() external view returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|Boolean indicating if the auction is active.|


### bid

Places a bid with a sealed bid hash.


```solidity
function bid(bytes32 _sealedBid) external payable onlyActive;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_sealedBid`|`bytes32`|The hash of the bid and salt.|


### revealBid

Reveals the actual bid and salt, checking it against the sealed bid.


```solidity
function revealBid(uint256 _bid, string memory _salt) external onlyActive;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_bid`|`uint256`|The actual bid amount.|
|`_salt`|`string`|The salt used to hash the bid.|


### withdrawFunds

Withdraws the funds for a non-winning bid.


```solidity
function withdrawFunds() external;
```

### getAuctionWinner

Returns the address of the auction winner.


```solidity
function getAuctionWinner() external view returns (address);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|The address of the highest bidder.|


### getCurrentPrice

Returns the current highest bid.


```solidity
function getCurrentPrice() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The highest bid amount.|


### getAuctionEndTime

Returns the end time of the auction.


```solidity
function getAuctionEndTime() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The auction end time as a timestamp.|


### isAllowed

Checks if the caller is allowed to sequence.


```solidity
function isAllowed(address proposer, address, bytes calldata) external view override returns (bool);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|Boolean indicating if the caller is the highest bidder.|


## Events
### BidRevealed

```solidity
event BidRevealed(address indexed bidder, uint256 bid, bool isHighestBid);
```

## Errors
### AddressNotAllowed

```solidity
error AddressNotAllowed();
```

### AuctionNotActive

```solidity
error AuctionNotActive();
```

### InvalidBidDeposit

```solidity
error InvalidBidDeposit();
```

### NoBidFound

```solidity
error NoBidFound();
```

### InvalidBidReveal

```solidity
error InvalidBidReveal();
```

### NoFundsToWithdraw

```solidity
error NoFundsToWithdraw();
```

### AuctionNotEnded

```solidity
error AuctionNotEnded();
```

### TransactionFailed

```solidity
error TransactionFailed();
```

### BidExceedsDeposit

```solidity
error BidExceedsDeposit();
```

### InvalidDuration

```solidity
error InvalidDuration();
```

## Structs
### Bid

```solidity
struct Bid {
    bytes32 sealedBid;
    uint256 deposit;
}
```

