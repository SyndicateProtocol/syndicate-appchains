// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {BasedSequencerChain} from "src/BasedSequencerChain.sol";
import {SealedBidAuctionSequencingModule} from "src/sequencing-modules/SealedBidAuctionSequencingModule.sol";
import {EMPTY_PARENT_HASH} from "../utils/Constants.sol";

contract BasedSequencerChainIntegrationTest is Test {
    BasedSequencerChain public basedSequencerChain;
    SealedBidAuctionSequencingModule public sealedBidAuctionSequencingModule;
    address public admin;
    address public treasury;
    address public bidder;

    uint256 bidAmount = 1 ether;
    uint256 auctionDuration = 1 days;

    function setUp() public {
        string memory syndicateFrameRPC = vm.envString("SYNDICATE_FRAME_RPC_URL");
        vm.createSelectFork(syndicateFrameRPC);

        admin = makeAddr("admin");
        treasury = makeAddr("treasury");
        bidder = makeAddr("bidder");

        // give ether funds to bidder1
        vm.deal(bidder, 10 ether);

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
}
