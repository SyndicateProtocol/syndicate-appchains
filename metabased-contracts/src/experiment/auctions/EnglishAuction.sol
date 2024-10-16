// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {IAuction} from "../IAuction.sol";

/// @title EnglishAuction
/// @notice An English auction contract. An English auction is an open ascending price auction where bidders openly bid against each other, with each subsequent bid required to be higher than the previous bid.
contract EnglishAuction is IAuction {
    address public owner;
    string public auctionType;
    bool public auctionActive;
    uint256 public endTime;
    address public highestBidder;
    uint256 public highestBid;

    mapping(address => uint256) public bids;
    mapping(address => uint256) public refunds;

    error OnlyOwner();
    error AuctionNotActive();
    error InvalidBidAmount();
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
        auctionType = "English";
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
            payable(owner).transfer(highestBid);
        }
    }

    function isAuctionActive() external view override returns (bool) {
        return auctionActive;
    }

    function bid() external payable override onlyActive {
        if (msg.value <= highestBid) revert InvalidBidAmount();

        if (highestBidder != address(0)) {
            refunds[highestBidder] += highestBid;
        }

        highestBid = msg.value;
        highestBidder = msg.sender;
    }

     function bid(bytes32) external payable override {
        revert("SealedBidAuction: bid() function must be called with a bid");
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
