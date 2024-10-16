// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {BasedSequencerChain, RequireListManager} from "src/BasedSequencerChain.sol";
import {SealedBidAuctionSequencingModule} from "src/sequencing-modules/SealedBidAuctionSequencingModule.sol";
import {EMPTY_PARENT_HASH} from "../utils/Constants.sol";

contract BasedSequencerChainIntegrationTest is Test {
    BasedSequencerChain public basedSequencerChain;
    SealedBidAuctionSequencingModule public sealedBidAuctionSequencingModule;
    address public admin;
    address public treasury;
    address public bidder;
    address public nonWinningBidder;
    address public anotherBidder;

    uint256 bidAmount = 1 ether;
    uint256 auctionDuration = 1 days;

    function setUp() public {
        vm.createSelectFork("syndicate_frame");

        admin = makeAddr("admin");
        treasury = makeAddr("treasury");
        bidder = makeAddr("bidder");
        nonWinningBidder = makeAddr("nonWinningBidder");
        anotherBidder = makeAddr("anotherBidder");

        // Give ether funds to bidders
        vm.deal(bidder, 10 ether);
        vm.deal(nonWinningBidder, 10 ether);
        vm.deal(anotherBidder, 10 ether);

        // Deploy SealedBidAuctionSequencingModule
        sealedBidAuctionSequencingModule = new SealedBidAuctionSequencingModule(auctionDuration, treasury);

        // Deploy BasedSequencerChain
        basedSequencerChain = new BasedSequencerChain();

        // Add the module to the requireAllList of BasedSequencerChain
        basedSequencerChain.addRequireAllCheck(address(sealedBidAuctionSequencingModule), true);
    }

    function testSequenceNextBatch() public {
        vm.startPrank(bidder);
        // Place a bid in the auction module
        bytes32 sealedBid = keccak256(abi.encodePacked(uint256(bidAmount), "secret"));
        sealedBidAuctionSequencingModule.bid{value: bidAmount}(sealedBid);

        // Reveal the bid
        sealedBidAuctionSequencingModule.revealBid(bidAmount, "secret");

        vm.warp(block.timestamp + auctionDuration + 1);
        sealedBidAuctionSequencingModule.finalizeAuction();

        // Prepare user-provided batch
        bytes[] memory transactions = new bytes[](1);
        transactions[0] = abi.encode("test transaction");

        BasedSequencerChain.UserProvidedBatch memory userBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: transactions
        });

        vm.stopPrank();

        // Simulate setting the parent hash in the batch
        vm.store(
            address(basedSequencerChain),
            keccak256(abi.encode(basedSequencerChain.calculateCurrentEpochNumber(), uint256(1))),
            bytes32(uint256(userBatch.non_empty_parent_hash))
        );

        // Bidder sequence the next batch
        vm.prank(bidder);
        basedSequencerChain.sequenceNextBatch(userBatch);

        // Verify the batch is sequenced correctly
        (bytes32 parent_hash,,, bytes32 epoch_hash) =
            basedSequencerChain.batches(basedSequencerChain.calculateCurrentEpochNumber());
        assertEq(parent_hash, userBatch.non_empty_parent_hash, "Parent hash should match");
        assertTrue(basedSequencerChain.checkParentHash(epoch_hash), "Parent hash should match");
    }

    function testSequenceNextBatchWithoutWinningBid() public {
        vm.startPrank(bidder);
        // Place a bid in the auction module
        bytes32 sealedBidWinningBid = keccak256(abi.encodePacked(uint256(bidAmount), "secret"));
        sealedBidAuctionSequencingModule.bid{value: bidAmount}(sealedBidWinningBid);

        // Reveal the bid
        sealedBidAuctionSequencingModule.revealBid(bidAmount, "secret");
        vm.stopPrank();

        vm.startPrank(nonWinningBidder);
        // Place a bid in the auction module
        bytes32 sealedBid = keccak256(abi.encodePacked(uint256(bidAmount / 2), "secret"));
        sealedBidAuctionSequencingModule.bid{value: bidAmount / 2}(sealedBid);

        // Reveal the bid
        sealedBidAuctionSequencingModule.revealBid(bidAmount / 2, "secret");

        vm.warp(block.timestamp + auctionDuration + 1);
        sealedBidAuctionSequencingModule.finalizeAuction();

        // Prepare user-provided batch
        bytes[] memory transactions = new bytes[](1);
        transactions[0] = abi.encode("test transaction");

        BasedSequencerChain.UserProvidedBatch memory userBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: transactions
        });

        vm.stopPrank();

        // Simulate setting the parent hash in the batch
        vm.store(
            address(basedSequencerChain),
            keccak256(abi.encode(basedSequencerChain.calculateCurrentEpochNumber(), uint256(1))),
            bytes32(uint256(userBatch.non_empty_parent_hash))
        );

        // Non-winning bidder attempts to sequence the next batch
        bytes memory errorMessage = abi.encodeWithSelector(
            RequireListManager.RequireAllCheckFailed.selector,
            address(sealedBidAuctionSequencingModule),
            nonWinningBidder
        );
        vm.expectRevert(errorMessage);
        vm.prank(nonWinningBidder);
        basedSequencerChain.sequenceNextBatch(userBatch);
    }

    function testMultipleBidsAndSequenceNextBatch() public {
        // Place a higher bid from another bidder
        vm.startPrank(anotherBidder);
        bytes32 anotherSealedBid = keccak256(abi.encodePacked(uint256(bidAmount * 2), "anotherSecret"));
        sealedBidAuctionSequencingModule.bid{value: bidAmount * 2}(anotherSealedBid);

        // Reveal the higher bid
        sealedBidAuctionSequencingModule.revealBid(bidAmount * 2, "anotherSecret");

        // Place a bid from the original bidder
        vm.startPrank(bidder);
        bytes32 sealedBid = keccak256(abi.encodePacked(uint256(bidAmount), "secret"));
        sealedBidAuctionSequencingModule.bid{value: bidAmount}(sealedBid);

        // Reveal the bid
        sealedBidAuctionSequencingModule.revealBid(bidAmount, "secret");

        vm.warp(block.timestamp + auctionDuration + 1);
        sealedBidAuctionSequencingModule.finalizeAuction();

        // Prepare user-provided batch
        bytes[] memory transactions = new bytes[](1);
        transactions[0] = abi.encode("test transaction");

        BasedSequencerChain.UserProvidedBatch memory userBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: EMPTY_PARENT_HASH,
            transaction_list: transactions
        });

        vm.stopPrank();

        // Simulate setting the parent hash in the batch
        vm.store(
            address(basedSequencerChain),
            keccak256(abi.encode(basedSequencerChain.calculateCurrentEpochNumber(), uint256(1))),
            bytes32(uint256(userBatch.non_empty_parent_hash))
        );

        // The highest bidder (anotherBidder) sequences the next batch
        vm.prank(anotherBidder);
        basedSequencerChain.sequenceNextBatch(userBatch);

        // Verify the batch is sequenced correctly
        (bytes32 parent_hash,,, bytes32 epoch_hash) =
            basedSequencerChain.batches(basedSequencerChain.calculateCurrentEpochNumber());
        assertEq(parent_hash, userBatch.non_empty_parent_hash, "Parent hash should match");
        assertTrue(basedSequencerChain.checkParentHash(epoch_hash), "Parent hash should match");
    }
}
