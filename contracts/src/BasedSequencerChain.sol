// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {AddressStructuredLinkedList} from "./LinkedList/AddressStructuredLinkedList.sol";
import {IsAllowed} from "./interfaces/IsAllowed.sol";

// This is the core contract for sequencing chains
// It uses a modular architecture to determine who is allowed to sequence,
// allowing for rapid experimentation.
// While this contract is immutable, it could always be swapped at specific
// block numbers via client upgrades
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

    // This is the data that the user will provide when they want to submit a
    // batch. The rest of the batch struct will be auto-filled
    struct UserProvidedBatch {
        // The parent hash of the last non-empty block. We'll get the epoch
        // number automatically, so users don't need to pass it in
        bytes32 non_empty_parent_hash;
        // The user-generated batch is a list of transactions that the user
        // wants to submit. This is a list of transactions that the user wants
        // to submit.
        bytes[] transaction_list; // EIP-2718 encoded transactions
    }

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
        uint256 parent_epoch_number;
        // This is equivalent to a block number. Maybe we want to rename it for
        // clarity.
        // TODO: Consider cutting this since it's implicitly available in the
        // `batches` mapping.
        uint256 epoch_number;
        // This is the hash of the transaction_list. This also will be
        // referenced by the next block as the parent_hash.
        bytes32 epoch_hash;
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

    // A list of isAllowed checks that must pass before a batch can be sequenced
    // For requireAll checks, all checks must pass for the batch to be sequenced
    // This will fail early upon the first check that fails.
    AddressStructuredLinkedList.List public requireAllList;
    // For requireAny checks, at least one check must pass for the batch to be
    // sequenced. This will succeed early upon the first check that passes.
    AddressStructuredLinkedList.List public requireAnyList;

    // The admin address is the only address that can add or remove from the
    // requireAllList and requireAnyList
    address public admin;

    error ParentHashDoesNotMatch(bytes32 expectedParentHash, bytes32 actualParentHash);
    error RequireAllCheckFailed(address requireAllAddress, address batchSubmitter);
    // There's no requireAnyAddress specified here, since this error means that
    // all requireAny checks failed
    error RequireAnyCheckFailed(address batchSubmitter);

    constructor() {
        // NOTE: We explicitly do not start the epoch number at 0. The reason
        // why is because we want to keep block numbers consistent across L3s on
        // the same L2. This lets us unlock cross-chain atomicity, which is very
        // important and greatly enabled by consistent block numbers.
        INITIAL_EPOCH_NUMBER = calculateCurrentEpochNumber();
        admin = msg.sender;
    }

    modifier onlyAdmin() {
        require(msg.sender == admin, "Only admin can perform this action");
        _;
    }

    // This will only ever sequence the next batch, so there is no need to specify the epochNumber
    function sequenceNextBatch(UserProvidedBatch calldata userProvidedBatch) public {
        // We will need to reject batches that don't have the correct
        // non-empty parent hash, since they may be created by nodes that are
        // missing data.
        // (As an aside, this is also why we set one second epochs! We want to
        // ensure that network latency isn't a reason for missing a parent hash,
        // and one second is a good target for it)
        if (!checkParentHash(userProvidedBatch.non_empty_parent_hash)) {
            revert ParentHashDoesNotMatch(
                batches[lastNonEmptyEpochNumber].epoch_hash, userProvidedBatch.non_empty_parent_hash
            );
        }

        // Run requireAll checks
        requireAllAllowed(msg.sender);

        // Run requireAny checks
        requireAnyAllowed(msg.sender);

        // Run requireAny checks
        requireAnyAllowed(msg.sender);

        Batch memory batch = Batch({
            parent_hash: userProvidedBatch.non_empty_parent_hash,
            parent_epoch_number: lastNonEmptyEpochNumber,
            epoch_number: calculateCurrentEpochNumber(),
            epoch_hash: keccak256(abi.encode(userProvidedBatch.transaction_list)),
            transaction_list: userProvidedBatch.transaction_list
        });

        lastNonEmptyEpochNumber = batch.epoch_number;
    }

    // While this requires a revert, someone can always check the list item by
    // item if they want to see whether they'll pass or fail a specific check.
    // It's not terribly useful to know which address is the first one you'll
    // fail, compared to being able to check each address individually
    function requireAllAllowed(address batchSubmitter) public view {
        address requireAllAddress = AddressStructuredLinkedList.getHead(requireAllList);
        bool requireAllNextNodeExists;
        address requireAllNextNodeAddress;

        (requireAllNextNodeExists, requireAllNextNodeAddress) =
            AddressStructuredLinkedList.getNextNode(requireAllList, requireAllAddress);

        if (requireAllAddress != address(0)) {
            // isAllowed check for head node
            if (!IsAllowed(requireAllAddress).isAllowed()) {
                revert RequireAllCheckFailed(requireAllAddress, batchSubmitter);
            }

            // isAllowed check for all subsequent nodes
            while (requireAllNextNodeExists) {
                // isAllowed check for node
                if (!IsAllowed(requireAllAddress).isAllowed()) {
                    revert RequireAllCheckFailed(requireAllAddress, batchSubmitter);
                }

                (requireAllNextNodeExists, requireAllAddress) =
                    AddressStructuredLinkedList.getNextNode(requireAllList, requireAllAddress);
            }
        }
    }

    function requireAnyAllowed(address batchSubmitter) public view {
        address requireAnyAddress = AddressStructuredLinkedList.getHead(requireAnyList);
        bool requireAnyNextNodeExists;
        address requireAnyNextNodeAddress;

        (requireAnyNextNodeExists, requireAnyNextNodeAddress) =
            AddressStructuredLinkedList.getNextNode(requireAnyList, requireAnyAddress);

        if (requireAnyAddress != address(0)) {
            // isAllowed check for head node
            if (IsAllowed(requireAnyAddress).isAllowed()) {
                return;
            }

            // isAllowed check for all subsequent nodes
            while (requireAnyNextNodeExists) {
                // isAllowed check for node
                if (IsAllowed(requireAnyAddress).isAllowed()) {
                    return;
                }

                (requireAnyNextNodeExists, requireAnyAddress) =
                    AddressStructuredLinkedList.getNextNode(requireAnyList, requireAnyAddress);
            }
        }

        // If we reach this point, then no requireAny checks passed
        revert RequireAnyCheckFailed(batchSubmitter);
    }

    /// @notice Pass in true to get all requireAll checks, false to get all
    /// requireAny checks
    /// @dev This function isn't used within the contract logic itself. It's
    /// primarily for nodes to check the sequencing criteria for a given chain
    function getAllRequirements(bool requireAll) public view returns (address[] memory) {
        AddressStructuredLinkedList.List storage allowList = requireAll ? requireAllList : requireAnyList;
        address allowAddress = AddressStructuredLinkedList.getHead(allowList);
        bool allowNextNodeExists;
        address allowNextNodeAddress;

        (allowNextNodeExists, allowNextNodeAddress) = AddressStructuredLinkedList.getNextNode(allowList, allowAddress);

        address[] memory allChecks = new address[](AddressStructuredLinkedList.sizeOf(allowList));

        if (allowAddress != address(0)) {
            allChecks[0] = allowAddress;

            uint256 i = 1;
            while (allowNextNodeExists) {
                allChecks[i] = allowNextNodeAddress;

                (allowNextNodeExists, allowNextNodeAddress) =
                    AddressStructuredLinkedList.getNextNode(allowList, allowNextNodeAddress);
                i++;
            }
        }

        return allChecks;
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

    // Check the parent hash against the last non-empty epoch number
    function checkParentHash(bytes32 parentHash) public view returns (bool) {
        return parentHash == batches[lastNonEmptyEpochNumber].epoch_hash;
    }

    /*//////////////////////////////////////////////////////////////
                            ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    // Add an address to the requireAllList
    function addRequireAllCheck(address _address) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            !AddressStructuredLinkedList.nodeExists(requireAllList, _address),
            "Address already exists in requireAllList"
        );
        AddressStructuredLinkedList.pushBack(requireAllList, _address);
    }

    // Remove an address from the requireAllList
    function removeRequireAllCheck(address _address) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            AddressStructuredLinkedList.nodeExists(requireAllList, _address), "Address does not exist in requireAllList"
        );
        AddressStructuredLinkedList.remove(requireAllList, _address);
    }

    // Add an address to the requireAnyList
    function addRequireAnyCheck(address _address) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            !AddressStructuredLinkedList.nodeExists(requireAnyList, _address),
            "Address already exists in requireAnyList"
        );
        AddressStructuredLinkedList.pushBack(requireAnyList, _address);
    }

    // Remove an address from the requireAnyList
    function removeRequireAnyCheck(address _address) public onlyAdmin {
        require(_address != address(0), "Invalid address");
        require(
            AddressStructuredLinkedList.nodeExists(requireAnyList, _address), "Address does not exist in requireAnyList"
        );
        AddressStructuredLinkedList.remove(requireAnyList, _address);
    }
}
