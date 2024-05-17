// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {CounterWithAuction} from "../src/experiment/CounterWithAuction.sol";
import {SealedBidAuction} from "./../src/experiment/auctions/SealedBidAuction.sol";

contract CounterWithAuctionTest is Test {
    SealedBidAuction sealedBidAuction;
    CounterWithAuction counter;

    address owner = address(0x04);
    address bidder1 = address(0x1);
    address bidder2 = address(0x2);

    function setUp() public {
        vm.startPrank(owner);
        sealedBidAuction = new SealedBidAuction(100);
        counter = new CounterWithAuction(address(sealedBidAuction));

        sealedBidAuction.initializeAuction();

        vm.stopPrank();
    }

    function testIncrementByWinner() public {
        bytes32 sealedBid1 = keccak256(abi.encodePacked(uint256(5), "salt1"));
        bytes32 sealedBid2 = keccak256(abi.encodePacked(uint256(10), "salt2"));

        vm.deal(bidder1, 10 ether);
        vm.deal(bidder2, 10 ether);

        vm.prank(bidder1);
        sealedBidAuction.bid{value: 5 ether}(sealedBid1);

        (bytes32 sealedBid, uint256 deposit) = sealedBidAuction.bids(bidder1);
        console2.logBytes32(sealedBid);
        console2.log(deposit);

        vm.prank(bidder2);
        sealedBidAuction.bid{value: 10 ether}(sealedBid2);

        vm.prank(bidder1);
        sealedBidAuction.revealBid(5, "salt1");

        vm.prank(bidder2);
        sealedBidAuction.revealBid(10, "salt2");

        // Finalize the auction
        vm.warp(block.timestamp + 101); // Fast forward time past auction end time

        vm.prank(owner);
        sealedBidAuction.finalizeAuction();

        console2.logAddress(sealedBidAuction.highestBidder());
        console2.logAddress(sealedBidAuction.getAuctionWinner());

        assertTrue(sealedBidAuction.highestBidder() == bidder2);
        assertTrue(sealedBidAuction.getAuctionWinner() == bidder2);

        // Verify the winner can increment the counter
        vm.prank(bidder2);
        counter.increment();
        assertEq(counter.count(), 1);

        // Verify non-winner cannot increment the counter
        vm.expectRevert(CounterWithAuction.OnlyWinnerError.selector);
        vm.prank(bidder1);
        counter.increment();
    }
}
