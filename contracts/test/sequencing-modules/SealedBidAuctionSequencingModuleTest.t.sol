// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {SealedBidAuctionSequencingModule} from "src/sequencing-modules/SealedBidAuctionSequencingModule.sol";

contract SealedBidAuctionSequencingModuleTest is Test {
    SealedBidAuctionSequencingModule auction;
    address treasury = address(0x1);
    address bidder1 = address(0x2);
    address bidder2 = address(0x3);
    uint256 auctionDuration = 1 days;
    uint256 bidAmount = 1 ether;

    function setUp() public {
        auction = new SealedBidAuctionSequencingModule(auctionDuration, treasury);

        // give ether funds to bidder1 and bidder2
        vm.deal(bidder1, 10 ether);
        vm.deal(bidder2, 10 ether);
    }

    function testBidAndReveal() public {
        bytes32 sealedBid1 = keccak256(abi.encodePacked(uint256(bidAmount), "salt1"));
        bytes32 sealedBid2 = keccak256(abi.encodePacked(uint256(bidAmount + 1 ether), "salt2"));

        // bids
        vm.prank(bidder1);
        auction.bid{value: bidAmount}(sealedBid1);

        vm.prank(bidder2);
        auction.bid{value: bidAmount + 1 ether}(sealedBid2);

        // reveal bids
        vm.prank(bidder1);
        auction.revealBid(bidAmount, "salt1");

        vm.prank(bidder2);
        auction.revealBid(bidAmount + 1 ether, "salt2");

        vm.warp(block.timestamp + auctionDuration + 1);
        auction.finalizeAuction();

        assertEq(auction.highestBidder(), bidder2);
        assertEq(auction.getCurrentPrice(), bidAmount + 1 ether);
    }

    function testWithdrawFunds() public {
        bytes32 sealedBid1 = keccak256(abi.encodePacked(uint256(bidAmount), "salt1"));
        bytes32 sealedBid2 = keccak256(abi.encodePacked(uint256(bidAmount + 1 ether), "salt2"));

        uint256 initialBalance = address(bidder1).balance;

        vm.prank(bidder1);
        auction.bid{value: bidAmount}(sealedBid1);

        vm.prank(bidder2);
        auction.bid{value: bidAmount + 1 ether}(sealedBid2);

        vm.prank(bidder2);
        auction.revealBid(bidAmount + 1 ether, "salt2");

        vm.startPrank(bidder1);
        auction.revealBid(bidAmount, "salt1");
        auction.withdrawFunds();
        vm.stopPrank();

        vm.warp(block.timestamp + auctionDuration + 1);
        auction.finalizeAuction();

        // bidder1 should have withdrawn their funds
        assertEq(address(bidder1).balance, initialBalance);
    }

    function testFinalizeAuction() public {
        bytes32 sealedBid1 = keccak256(abi.encodePacked(uint256(bidAmount), "salt1"));

        vm.startPrank(bidder1);
        auction.bid{value: bidAmount}(sealedBid1);
        auction.revealBid(bidAmount, "salt1");
        vm.stopPrank();

        vm.warp(block.timestamp + auctionDuration + 1);
        auction.finalizeAuction();

        console.log("treasury balance", address(treasury).balance);
        console.log("auction balance", address(auction).balance);

        assertEq(address(treasury).balance, bidAmount);
    }

    function testInvalidBidDeposit() public {
        bytes32 sealedBid = keccak256(abi.encodePacked(uint256(bidAmount), "salt1"));

        vm.startPrank(bidder1);
        vm.expectRevert(SealedBidAuctionSequencingModule.InvalidBidDeposit.selector);
        auction.bid(sealedBid);
        vm.stopPrank();
    }

    function testInvalidBidReveal() public {
        bytes32 sealedBid = keccak256(abi.encodePacked(uint256(bidAmount), "salt1"));

        vm.startPrank(bidder1);
        auction.bid{value: bidAmount}(sealedBid);
        vm.stopPrank();

        vm.startPrank(bidder1);
        vm.expectRevert(SealedBidAuctionSequencingModule.InvalidBidReveal.selector);
        auction.revealBid(bidAmount + 1 ether, "wrongSalt");
        vm.stopPrank();
    }
}
