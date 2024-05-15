// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {StructuredLinkedList} from "./LinkedList/StructuredLinkedList.sol";
import {SafeTransferLib} from "./SafeTransferLib/SafeTransferLib.sol";

contract BasedSequencerChain {
    // We can adjust this as needed. Ideally epochs are as fast as possible to
    // allow a wide variety of based sequencers to participate.
    // NOTE: The faster the epoch length, the less time block builders have to
    // put together proposed blocks. We'll want to find an ideal tradeoff here.
    // NOTE: For every chain, we'll want it to have an epoch length that is the
    // same (ideally) or shorter to enable cross-chain atomicity. If the epoch
    // length is shorter, then the chain with a shorter epoch length can handle
    // conversions to block numbers of a longer epoch length. For simplicity, we
    // don't deal with epoch length conversions in this version and assume that
    // all epoch lengths are the same.
    // NOTE: If we go below one second, the epoch math gets complicated since
    // Ethereum block timestamps are UNIX timestamps (i.e. seconds-based). One
    // second is likely the lowest we can go without adding significant
    // complexity, but that's OK since we also need to have our system be
    // tolerant of network latency
    uint256 public constant EPOCH_LENGTH = 1; // 1 second.

    // We use an initial epoch number rather than starting the chain at block 1.
    // This is because we want to keep block numbers consistent across L3s on
    // the same L2. This lets us unlock cross-chain atomicity, which is very
    // important and greatly enabled by consistent block numbers.
    uint256 public immutable INITIAL_EPOCH_NUMBER;

    // The max bid list size is set to 5. This logic will eventually be
    // modularized, so take it as more of a reference example than the final
    // implementation.
    uint256 public constant MAX_BID_LIST_SIZE = 5;

    // This value is necessary so that we can keep track of the previous parent
    // hashes. Otherwise, the parent hash would be the empty block.
    uint256 public lastNonEmptyEpochNumber;

    // Based on the Optimism batch structure
    struct Batch {
        // parent_hash is a potential DOS vector, since a based sequencer could
        // maliciously spam it and prevent other transactions from being
        // included. There are two options here:
        // 1. We can require that all epochs are reserved in advance by a based
        // sequencer. i.e. Every single epoch is put out for a bid, and epochs
        // are strictly time-based. This way, only one based sequencer can write
        // at a time.
        // 2. We can allow any based sequencer to submit at any time, but
        // require that the based sequencer specifies all of the state that
        // their block touches. This would allow batches to be submitted in
        // parallel as long as they touch different state. However, touching the
        // same state could still be a DOS vector. In addition, not all state is
        // necessarily known upfront. I'd lean against this approach given the
        // implementation complexity that it adds.
        bytes32 parent_hash;
        // This is equivalent to a block number. Maybe we want to rename it for
        // clarity.
        // TODO: Consider cutting this since it's implicitly available in the
        // `batches` mapping.
        uint256 epoch_number;
        // This is the hash of the transaction_list. This also will be
        // referenced by the next block as the parent_hash.
        bytes32 epoch_hash;
        // We will autofill this based on when the batch is submitted. This will
        // not be provided by the based sequencer.
        // TODO: Should the timestamp of the submitted batch be used by the
        // based sequencer?
        uint256 timestamp;
        // The ist of transactions in this batch.
        bytes[] transaction_list; // EIP-2718 encoded transactions
    }

    // Should we consider renaming this to "blocks"? Still not sure how much we
    // should keep the Optimism internal naming structure, versus using more
    // externally-legible naming around blocks.
    // NOTE: If an epochNumber has no corresponding batch, we should treat
    // it as an empty block. Empty blocks occur when no blocks are submitted for
    // a given epoch. Because we treat null data as an empty block, this is
    // straightforward.
    mapping(uint256 epochNumber => Batch batch) public batches;

    // These are proposed batches that have not yet been finalized. This is also
    // utilized by the second-choice, third-choice, etc bidders in a batch
    mapping(uint256 epochNumber => mapping(address batchProposer => Batch batch)) proposedBatches;

    // Mapping of epoch numbers to the top 5 bids for that epoch
    // The top 5 bids are represented as a linked list. This allows us to easily
    // add a new top bid by creating a new head and dropping the tail,
    // preventing us from incurring gas costs on every item to adjust the bid
    // order.
    mapping(uint256 epochNumber => StructuredLinkedList.List bidsForEpoch) bids;

    error BidMustBeHighest();
    error CannotBidZero();

    constructor() {
        // NOTE: We explicitly do not start the epoch number at 0. The reason
        // why is because we want to keep block numbers consistent across L3s on
        // the same L2. This lets us unlock cross-chain atomicity, which is very
        // important and greatly enabled by consistent block numbers.
        INITIAL_EPOCH_NUMBER = calculateCurrentEpochNumber();
    }

    // A based sequencer must reserve a batch prior to being able to add a batch
    // Note that based sequencers can pre-queue future batches based on the
    // epoch number
    // Note that the based sequencer can overwrite their proposed batch any time
    // before finalization. This allows them to react to e.g. receiving better
    // bids from block builders prior to finalization.
    // QUESTION: What action should we take if a based sequencer does not submit
    // to a batch that it's reserved?
    // One option is to have the batch be missed. This would be an empty block.
    // Maybe this is OK, but it reduces chain throughput so I'm not a fan of it.
    // Another option here is to allow the batch to be submitted by anyone. This
    // would be simple to implement, but is suboptimal since it leaves money on
    // the table.
    // A third option here is to have the batch go to the second highest bidder,
    // and then the third highest bidder, etc. This could continue until some
    // kind of expiration period, in which case anyone can submit a batch.
    // One way to make finding alternative bidders easier is to have
    // second-choice, third-choice, etc. bidders also be able to call
    // addBatch(). This way, if the top bidder doesn't submit, the second bidder
    // can finalize the batch.
    // ANSWER: We will go with the third option. This allows us to maximize
    // throughput, since blocks won't be missed if the top bidder does not
    // submit.
    function proposeBatch(uint256 epochNumber, Batch calldata batch) public {}

    // This function is only necessary if we have alternative bidders available.
    // If only the top bidder were able to submit, this function wouldn't be
    // needed and the epoch would be missed instead.
    // In this case, the top bidder has the batch reserved for X milliseconds, and
    // then after Y milliseconds the second bidder can add, and then after Z milliseconds
    // the third bidder can add, etc.
    // This will only ever finalize the next batch, so there is no need to specify the epochNumber
    function finalizeNextBatch() public {
        // NOTE: We will need to reject batches that don't have the correct
        // non-empty parent hash, since they may be created by nodes that are
        // missing data.
        // If a based sequencer reserves a range of epochs, they could submit
        // multiple proposals, since they know the parent hash of the prior
        // epoch. Finalization would then work properly.
    }

    // Convenience function to add and finalize a batch in the same step. This is useful for two reasons:
    // 1. It avoids revealing your batch until it's able to be included.
    // 2. It guarantees that once you add a batch, it will also be finalized.
    // This is desirable because you don't have to worry about network issues
    // between adding and finalizing batches.
    function proposeAndFinalizeNextBatch(Batch calldata batch) public {}

    // QUESTION: Should bids be immutable? i.e. Once a bid is submitted, it
    // cannot be removed. It can only be replaced by a higher bid.
    // I am in favor of this approach, since adding spoof bids could be
    // disruptive (you don't actually know what the price would be if bids could
    // be removed)
    // This also means that we can automatically allow the top bidder to
    // finalize a batch, and then the next bidder, etc
    // Bids are only accepted before the epoch starts. Once the epoch starts,
    // further bidding is not allowed.
    // ANSWER: Yes, bids should be immutable. This prevents auction manipulation
    // and allows us to go to second-choice, third-choice, etc. bidders if the
    // top bidder does not submit a batch.
    // QUESTION: Should epoch bidding be relative or absolute? i.e. Should
    // epochs be bid on based on time since the last epoch, or based on an
    // absolute timestamp.
    // Time since the last epoch is likely desirable here. It makes it harder to
    // forecast when a future epoch number will be hit, but it's much more
    // resilient to chain downtime or inactivity since you never have to worry
    // about skipping epoch numbers
    // ANSWER: We'll offer both relative and absolute bidding via bidEpoch() and
    // bidNextEpoch()
    function bidEpoch(uint256 epochNumber) public payable {
        if (msg.value == 0) {
            revert CannotBidZero();
        }

        StructuredLinkedList.List storage bidsForEpoch = bids[epochNumber];
        uint256 bidAmount = msg.value;
        address bidder = msg.sender;

        // If no bids exist, we can add the bid as the head
        if (StructuredLinkedList.sizeOf(bidsForEpoch) == 0) {
            StructuredLinkedList.pushFront(bidsForEpoch, bidAmount, bidder);
        } else if (StructuredLinkedList.sizeOf(bidsForEpoch) < MAX_BID_LIST_SIZE) {
            // If there are fewer than 5 bids, we can add the bid as the head.
            // It must be the new highest bid
            if (bidAmount <= StructuredLinkedList.getHead(bidsForEpoch)) {
                revert BidMustBeHighest();
            }
            StructuredLinkedList.pushFront(bidsForEpoch, bidAmount, bidder);
        } else {
            if (bidAmount <= StructuredLinkedList.getHead(bidsForEpoch)) {
                revert BidMustBeHighest();
            }
            // If there are 5 or more bids, we need to add the bid as the head
            // while also removing the tail
            StructuredLinkedList.pushFront(bidsForEpoch, bidAmount, bidder);
            (uint256 refundedBidAmount, address refundedBidAddress) = StructuredLinkedList.popBack(bidsForEpoch);

            // We should refund the bidder who was popped off the back
            SafeTransferLib.safeTransferETH(refundedBidAddress, refundedBidAmount);
        }
    }

    // A convenience function for the based sequencer to bid on the next epoch.
    // A user can pass in 0 to bid on the current epoch.
    // NOTE: Having the ability to bid via relative values is particularly
    // useful for MEV. Relying only on the chain for these relative values
    // (rather than needing to pass data in) helps ensure that bids are always
    // focused on the current epoch
    function bidNextEpoch(uint256 epochNumberFromCurrentEpoch) public payable {
        bidEpoch(calculateCurrentEpochNumber() + epochNumberFromCurrentEpoch);
    }

    // Calculate the epoch number based on a given timestamp
    function calculateEpochNumber(uint256 timestamp) public pure returns (uint256) {
        // This is a simple calculation that divides the timestamp by the epoch
        // length. This will give us the epoch number.
        return timestamp / EPOCH_LENGTH;
    }

    // Convenience function to calculate the current epoch number based on the
    // current block timestamp.
    function calculateCurrentEpochNumber() public view returns (uint256) {
        return calculateEpochNumber(block.timestamp);
    }
}
