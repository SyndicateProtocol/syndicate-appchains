// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IAuction} from "../IAuction.sol";

/// @title VickreyAuction
/// @notice A contract for a Vickrey auction. A Vickrey auction is a sealed-bid auction where the highest bidder wins but pays the second-highest bid price.
contract VickreyAuction is IAuction {
    struct Bid {
        bytes32 sealedBid;
        uint256 deposit;
    }

    address public owner;
    string public auctionType;
    bool public auctionActive;
    uint256 public endTime;
    address public highestBidder;
    address public secondHighestBidder;
    uint256 public highestBid;
    uint256 public secondHighestBid;

    mapping(address => Bid) public bids;
    mapping(address => uint256) public refunds;

    error OnlyOwner();
    error AuctionNotActive();
    error InvalidBidDeposit();
    error NoBidFound();
    error InvalidBidReveal();
    error NoFundsToWithdraw();
    error AuctionNotEnded();

    modifier onlyOwner() {
        if (msg.sender != owner) revert OnlyOwner();
        _;
    }

    modifier onlyActive() {
        if (!auctionActive) revert AuctionNotActive();
        _;
    }

    constructor(uint256 _duration) {
        owner = msg.sender;
        auctionType = "Vickrey";
        auctionActive = false;
        endTime = block.timestamp + _duration;
    }

    function initializeAuction() external override onlyOwner {
        auctionActive = true;
    }

    function getAuctionType() external view override returns (string memory) {
        return auctionType;
    }

    function finalizeAuction() external override onlyOwner {
        if (block.timestamp < endTime) revert AuctionNotEnded();
        auctionActive = false;
        if (highestBidder != address(0)) {
            uint256 finalPrice = secondHighestBid;
            payable(owner).transfer(finalPrice);
            refunds[highestBidder] = highestBid - finalPrice;
        }
    }

    function isAuctionActive() external view override returns (bool) {
        return auctionActive;
    }

    function bid(bytes32 _sealedBid) external payable override onlyActive {
        if (msg.value == 0) revert InvalidBidDeposit();
        bids[msg.sender] = Bid(_sealedBid, msg.value);
    }

    function bid() external payable override {
        revert("SealedBidAuction: bid(bytes32) function must be called with a sealed bid");
    }

    function revealBid(uint256 _bid, string memory _salt) external onlyActive {
        Bid memory bidData = bids[msg.sender];
        if (bidData.deposit == 0) revert NoBidFound();

        bytes32 sealedBid = keccak256(abi.encodePacked(_bid, _salt));
        if (sealedBid != bidData.sealedBid) revert InvalidBidReveal();

        if (_bid > highestBid) {
            if (highestBidder != address(0)) {
                secondHighestBidder = highestBidder;
                secondHighestBid = highestBid;
            }
            highestBid = _bid;
            highestBidder = msg.sender;
        } else if (_bid > secondHighestBid) {
            secondHighestBidder = msg.sender;
            secondHighestBid = _bid;
        }

        bids[msg.sender].deposit = 0;
    }

    function withdrawFunds() external override {
        uint256 refund = refunds[msg.sender];
        if (refund == 0) revert NoFundsToWithdraw();
        refunds[msg.sender] = 0;
        payable(msg.sender).transfer(refund);
    }

    function getAuctionWinner() external view override returns (address) {
        return highestBidder;
    }

    function getCurrentPrice() external view override returns (uint256) {
        return highestBid;
    }

    function getAuctionEndTime() external view override returns (uint256) {
        return endTime;
    }
}
