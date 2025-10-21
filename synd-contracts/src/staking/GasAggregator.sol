// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {ISyndicateSequencingChain} from "../interfaces/ISyndicateSequencingChain.sol";
import {GasCounter} from "./GasCounter.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {EpochTracker} from "./EpochTracker.sol";

interface ISyndicateProxy {
    // This function exists in both the new proxy and legacy implementation contract
    function tokensUsedPerEpoch(uint256 epoch) external view returns (uint256);
}

/// @notice Storage struct for GasAggregator using ERC-7201 namespaced storage pattern
/// @dev This struct is used to store the aggregated epoch data hash.
/// @dev Using ERC-7201 ensures storage slots don't conflict during upgrades.
/// @custom:storage-location erc7201:syndicate.storage.GasAggregator
struct GasAggregatorStorage {
    /// @dev Stores the final hash for each completed epoch.
    mapping(uint256 => bytes32) aggregatedEpochDataHash;
}

/**
 * @title GasAggregator
 * @notice Aggregates gas usage data from appchains
 * @dev This contract manages the collection and aggregation of gas usage data from multiple appchains.
 *      It supports both automatic aggregation (for small numbers of appchains) and off-chain aggregation
 *      (for larger numbers of appchains) with a challenge mechanism for data integrity.
 * @dev Inherits from Ownable for admin functions
 */
contract GasAggregator is Ownable(msg.sender), Pausable, EpochTracker {
    using EnumerableSet for EnumerableSet.UintSet;

    /*//////////////////////////////////////////////////////////////
                            CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Version of the GasAggregator contract (updatable during upgrades)
    /// @dev Semantic version string to track contract upgrades and compatibility
    uint256 public constant VERSION = 1_000_000; // 1.0.0 (major * 1_000_000 + minor * 1_000 + patch)

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Factory contract address to determine create2 addresses
    address public factory;

    /// @notice Proxy bytecode hash to determine the create2 addresses
    bytes32 public syndicateProxyBytecodeHash;

    /// @notice Maximum number of appchains to submit gas data for.
    /// This is also the max chunk size.
    uint256 public maxAppchainsToQuery;

    /// @notice Fee required to add a chain to the gas tracking registry
    /// @dev Exists as a spam-preventing measure. Paid in SYND.
    uint256 public addChainFee;

    /// @notice Registry of chains that are currently tracked for gas usage
    EnumerableSet.UintSet internal _appchains;

    /// @notice Current epoch being processed for aggregation
    /// @dev Tracks which epoch is pending aggregation
    uint256 public currentEpoch;

    /// @notice Hash of the pending data for the current epoch
    /// @dev Stores the hash of (appchainIDs, tokens) for verification
    bytes32 public pendingDataHash;

    // Current appchain start index in case the list of appchains is too big
    uint256 public currentAggregateIndex;

    /// @notice appchain contract addresses mapping
    mapping(uint256 chainId => address) public appchainContract;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Error thrown when a zero address is provided
    /// @dev Prevents invalid contract addresses
    error ZeroAddress();

    /// @notice Error thrown when chain id is zero
    error ZeroChainId();

    /// @notice Error thrown when data hash is invalid
    /// @dev Ensures data integrity
    error InvalidDataHash();
    error ChainAlreadyTracked(uint256 chainId);
    error InvalidFee(uint256 required, uint256 provided);
    error ChainNotFound(uint256 chainId);
    error FactoryAlreadySet();
    error NoChainsAdded();

    /// @notice Error thrown when attempting to aggregate an epoch that hasn't ended
    /// @dev Prevents aggregation of current or future epochs
    error EpochNotOver(uint256 epoch, uint256 currentEpoch);

    /*//////////////////////////////////////////////////////////////
                              EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a chain is successfully added to the gas tracking registry
    /// @param chainId The chain ID that was added
    /// @param chainContract The contract address of the sequencing chain
    /// @param addedBy The address that paid the fee to add this chain
    event ChainAdded(uint256 indexed epoch, uint256 indexed chainId, address chainContract, address indexed addedBy);

    /// @notice Emitted when a chain is removed from the gas tracking registry
    /// @param chainId The chain ID that was removed
    event ChainRemoved(uint256 indexed epoch, uint256 indexed chainId);

    /// @notice Emitted when the fee for adding chains is updated
    /// @param oldFee The previous fee amount
    /// @param newFee The new fee amount
    event AddChainFeeUpdated(uint256 oldFee, uint256 newFee);

    /// @notice Emitted when the aggregation is pending
    /// @param epoch The epoch that is pending
    /// @param remainingChains The number of chains remaining to be aggregated
    event AggregationPending(uint256 indexed epoch, uint256 remainingChains);

    /// @notice Emitted when the aggregation is complete
    /// @param epoch The epoch that is complete
    /// @param chainIds The chain IDs that were aggregated
    /// @param tokens The tokens that were aggregated
    event AggregatedTokens(uint256 indexed epoch, uint256[] chainIds, uint256[] tokens);

    /// @notice Emitted when the maximum number of appchains to query is updated
    /// @param epoch The epoch that the update is for
    /// @param maxAppchainsToQuery The new maximum number of appchains to query
    event UpdateMaxAppchainsToQuery(uint256 indexed epoch, uint256 maxAppchainsToQuery);

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Constructor for the GasAggregator contract
    /// @dev The epoch is the first epoch to start from
    /// @dev The addChainFee is the fee to add a chain (setting it to 0 will default to 5 ether)
    /// @dev The maxAppchainsToQuery is the maximum number of appchains to query (setting it to 0 will default to 100)
    /// @param _epoch The epoch to start from
    /// @param _addChainFee The fee to add a chain
    /// @param _maxAppchainsToQuery The maximum number of appchains to query
    constructor(uint256 _epoch, uint256 _addChainFee, uint256 _maxAppchainsToQuery) {
        require(_epoch != 0);
        currentEpoch = _epoch;
        addChainFee = _addChainFee;
        if (addChainFee == 0) {
            addChainFee = 5 ether;
        }
        maxAppchainsToQuery = _maxAppchainsToQuery;
        if (maxAppchainsToQuery == 0) {
            maxAppchainsToQuery = 100;
        }
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds a chain to the gas tracking registry
    /// @dev Anyone can call this function by paying the required fee. The chain must exist at the
    ///      deterministic address and not be banned. Successfully added chains will participate
    ///      in gas aggregation and emissions distribution. The owner does not pay any fees.
    /// @param chainId The chain ID to add to the tracking registry
    function addChain(uint256 chainId) external payable whenNotPaused {
        if (msg.sender != owner()) {
            require(msg.value == addChainFee, InvalidFee(addChainFee, msg.value));
        } else {
            require(msg.value == 0, InvalidFee(0, msg.value));
        }
        require(chainId > 0, ZeroChainId());
        require(_appchains.add(chainId), ChainAlreadyTracked(chainId));
        address chainContract = Create2.computeAddress(bytes32(chainId), syndicateProxyBytecodeHash, factory);
        require(chainContract.code.length > 0, ChainNotFound(chainId));
        appchainContract[chainId] = chainContract;
        emit ChainAdded(currentEpoch, chainId, chainContract, msg.sender);
    }

    /**
     * @notice Triggers automatic aggregation of appchain gas usage data
     * @dev May need to be called multiple times if the number of appchains is large
     * Pause contract while aggregating. Unpause when finished.
     */
    function aggregateTokens(uint256[] calldata prevChainIds, uint256[] calldata prevTokens) external {
        if (currentAggregateIndex == 0) {
            // If this is the first time we are aggregating, pause the contract till we are done
            _pause();
        } else {
            _requirePaused();
            require(pendingDataHash == keccak256(abi.encode(prevChainIds, prevTokens)), InvalidDataHash());
        }

        uint256[] memory chainIds;
        uint256[] memory tokens;
        // Simulate the next chunk of data and update the current aggregate index
        (currentAggregateIndex, chainIds, tokens) =
            simulateAggregateTokens(currentAggregateIndex, prevChainIds, prevTokens);

        pendingDataHash = keccak256(abi.encode(chainIds, tokens));
        // If we are not done aggregating, return
        if (currentAggregateIndex != 0) {
            emit AggregationPending(currentEpoch, _appchains.length() - currentAggregateIndex);
            return;
        }

        _getGasAggregatorStorage().aggregatedEpochDataHash[currentEpoch] = pendingDataHash;
        pendingDataHash = 0;
        emit AggregatedTokens(currentEpoch, chainIds, tokens);
        currentEpoch++;
        _unpause();
    }

    function simulateAggregateTokens(
        uint256 aggregateIndex,
        uint256[] calldata prevChainIds,
        uint256[] calldata prevTokens
    ) public view returns (uint256 nextAggregateIndex, uint256[] memory chainIds, uint256[] memory tokens) {
        uint256 epoch = getCurrentEpoch();
        require(epoch > currentEpoch, EpochNotOver(currentEpoch, epoch));

        uint256 appchainToQuery = _appchains.length() - aggregateIndex;
        require(appchainToQuery > 0, NoChainsAdded());
        if (maxAppchainsToQuery < appchainToQuery) {
            appchainToQuery = maxAppchainsToQuery;
            nextAggregateIndex = aggregateIndex + appchainToQuery;
        }

        uint256 chainIdCount = prevChainIds.length;
        require(chainIdCount == prevTokens.length, InvalidDataHash());
        if (aggregateIndex == 0) {
            // If this is the first time we are aggregating, the arrays of previous data should be empty
            require(chainIdCount == 0, InvalidDataHash());
        }

        chainIds = new uint256[](chainIdCount + appchainToQuery);
        tokens = new uint256[](chainIdCount + appchainToQuery);

        for (uint256 i = 0; i < chainIdCount; i++) {
            chainIds[i] = prevChainIds[i];
            tokens[i] = prevTokens[i];
        }

        for (uint256 i = 0; i < appchainToQuery; i++) {
            uint256 chainId = _appchains.at(aggregateIndex + i);
            uint256 gasUsed = ISyndicateProxy(appchainContract[chainId]).tokensUsedPerEpoch(currentEpoch);
            // ignore appchains with no gas usage in the epoch
            if (gasUsed > 0) {
                chainIds[chainIdCount] = chainId;
                tokens[chainIdCount] = gasUsed;
                chainIdCount++;
            }
        }

        // truncate arrays
        assembly {
            mstore(chainIds, chainIdCount)
            mstore(tokens, chainIdCount)
        }

        // select the top n appchains
        if (chainIdCount > maxAppchainsToQuery) {
            GasAggregatorUtils.select(chainIds, tokens, maxAppchainsToQuery);
        }
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the total number of chains currently being tracked for gas usage
    /// @return The number of chains in the tracking registry
    function getTrackedChainCount() external view returns (uint256) {
        return _appchains.length();
    }

    /// @notice Get all chain IDs currently being tracked for gas usage
    function getTrackedChainIds() external view returns (uint256[] memory chainIDs) {
        bytes32[] memory ids = _appchains._inner._values;
        assembly {
            chainIDs := ids
        }
    }

    /// @notice Get a chain ID currently being tracked for gas usage
    function getTrackedChainId(uint256 index) external view returns (uint256) {
        return _appchains.at(index);
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Adds a chain to the gas tracking registry with a chainContract override.
    /// This legacy function is deactivated once the factory is set.
    function addLegacyChain(uint256 chainId, address chainContract) external onlyOwner whenNotPaused {
        require(factory == address(0), FactoryAlreadySet());
        require(chainId > 0, ZeroChainId());
        require(_appchains.add(chainId), ChainAlreadyTracked(chainId));
        require(chainContract.code.length > 0, ChainNotFound(chainId));
        appchainContract[chainId] = chainContract;
        emit ChainAdded(currentEpoch, chainId, chainContract, msg.sender);
    }

    /**
     * @notice Set the maximum number of appchains that can be queried automatically
     * @dev When total appchains >= this value, off-chain aggregation is required
     * @param newMax The new maximum number of appchains for automatic aggregation
     * @custom:example If set to 10, automatic aggregation works for ≤10 appchains
     */
    function setMaxAppchainsToQuery(uint256 newMax) external onlyOwner whenNotPaused {
        maxAppchainsToQuery = newMax;
        emit UpdateMaxAppchainsToQuery(currentEpoch, maxAppchainsToQuery);
    }

    function removeAppchains(uint256[] calldata chainIds) external onlyOwner whenNotPaused {
        for (uint256 i = 0; i < chainIds.length; i++) {
            require(_appchains.remove(chainIds[i]), "appchain is not tracked");
            emit ChainRemoved(currentEpoch, chainIds[i]);
        }
    }

    /// @notice Set the fee required to add a chain to the registry
    /// @param newFee The new fee amount
    function setAddChainFee(uint256 newFee) external onlyOwner {
        uint256 oldFee = addChainFee;
        addChainFee = newFee;
        emit AddChainFeeUpdated(oldFee, newFee);
    }

    /// @notice Withdraw collected fees from the contract (admin only)
    /// @dev Allows admins to withdraw fees collected from chain additions.
    ///      Can withdraw a specific amount or the entire balance.
    /// @param to Address to send the funds to (cannot be zero address)
    /// @param amount Amount to withdraw in wei (0 to withdraw all available funds)
    function withdrawFees(address payable to, uint256 amount) external onlyOwner {
        if (to == address(0)) revert ZeroAddress();

        uint256 withdrawAmount = amount == 0 ? address(this).balance : amount;
        if (withdrawAmount > address(this).balance) {
            revert InvalidFee(withdrawAmount, address(this).balance);
        }

        (bool success,) = to.call{value: withdrawAmount}("");
        require(success, "Transfer failed");
    }

    /// @notice Set the factory contract address (admin only)
    /// @dev Updates the factory and recalculates the proxy bytecode hash for address computations.
    /// @param newFactory The address of the new factory contract
    /// @param bytecodeHash The bytecode hash of the proxy that the factory deploys
    function setFactory(address newFactory, bytes32 bytecodeHash) external onlyOwner {
        require(factory == address(0), FactoryAlreadySet());
        require(newFactory != address(0), ZeroAddress());
        require(bytecodeHash != 0, InvalidDataHash());
        factory = newFactory;
        syndicateProxyBytecodeHash = bytecodeHash;
    }

    /**
     * @notice Pause the contract
     * @dev Only callable by the contract owner. Disables the contract.
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @notice Unpause the contract
     * @dev Only callable by the contract owner. Enables the contract.
     */
    function unpause() external onlyOwner {
        // clear pending aggregation when unpausing
        currentAggregateIndex = 0;
        pendingDataHash = 0;
        _unpause();
    }

    /*//////////////////////////////////////////////////////////////
                            NAMESPACE STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice ERC-7201 storage slot for GasAggregator-specific data
    /// @dev Generated using: cast index-erc7201 syndicate.storage.GasAggregator
    ///      This ensures the storage slot doesn't conflict with inherited contracts
    bytes32 public constant GAS_AGGREGATOR_STORAGE_LOCATION =
        0xb7dfb3be9e2ba9b0349e11a21cd1baebde23ce111dd0651619b69a6e26aa0600;

    /// @notice Internal function to access the ERC-7201 namespaced storage
    /// @dev Uses inline assembly to access the specific storage slot for this contract's data
    /// @return $ Storage pointer to the GasAggregatorStorage struct
    function _getGasAggregatorStorage() private pure returns (GasAggregatorStorage storage $) {
        assembly {
            $.slot := GAS_AGGREGATOR_STORAGE_LOCATION
        }
    }

    /// @notice Get the aggregated epoch data hash
    /// @return The hash of the aggregated epoch data
    function aggregatedEpochDataHash(uint256 epoch) public view returns (bytes32) {
        GasAggregatorStorage storage $ = _getGasAggregatorStorage();
        return $.aggregatedEpochDataHash[epoch];
    }
}

library GasAggregatorUtils {
    /**
     * @notice Select the top-n elements from key, value arrays via quick select and truncate the arrays
     * @dev Average O(n) runtime complexity, can be O(n2) for pathological input
     * @param keys The array of keys
     * @param values The array of values
     * @param n The number of elements to select
     */
    function select(uint256[] memory keys, uint256[] memory values, uint256 n) internal pure {
        require(keys.length == values.length && n <= values.length);

        unchecked {
            uint256 begin = _begin(values);
            uint256 end = begin + 0x20 * values.length;
            uint256 targetEnd = begin + 0x20 * n;
            uint256 offset = _begin(keys) - begin;

            while (end > targetEnd) {
                // partition the range
                uint256 index = _partition(begin, end, offset);

                // if the partition point lies inside the range, adjust the start index
                // otherwise, adjust the end index
                if (index < targetEnd) {
                    begin = index;
                } else {
                    end = index;
                }
            }
        }

        // truncate array lengths
        assembly {
            mstore(keys, n)
            mstore(values, n)
        }
    }

    /**
     * @notice Quick sort key, value arrays in-place
     * @param keys The array of keys
     * @param values The array of values
     */
    function sort(uint256[] memory keys, uint256[] memory values) internal pure {
        require(keys.length == values.length);

        unchecked {
            uint256 begin = _begin(values);
            _quickSort(begin, begin + 0x20 * values.length, _begin(keys) - begin);
        }
    }

    /**
     * @notice Simple, unoptimized quick sort implementation
     * @dev Normally quick sort falls back to insertion sort for small lists
     * @param begin Pointer to the start value to sort
     * @param end Pointer past the end value to sort
     * @param offset Memory offset of the keys array from the values one
     */
    function _quickSort(uint256 begin, uint256 end, uint256 offset) private pure {
        unchecked {
            while (end - begin > 0x20) {
                // partition the range
                uint256 index = _partition(begin, end, offset);

                // recur
                _quickSort(begin, index, offset);

                // manually tail recur, solidity does not automatically optimize this
                begin = index;
            }
        }
    }

    /**
     * @notice Hoare partitioning algorithm
     * @param begin Pointer to the start value to partition
     * @param end Pointer past the end value to partition
     * @param offset Memory offset of the keys array from the values one
     * @return index The partition index in range [begin + 0x20, end)
     *         such that the ranges [begin, index) and [index, end) are partially sorted
     *         and each range contains at least one element.
     * @dev This function handles duplicate elements well - the pivot is selected
     *         near the middle of the range when duplicates are present.
     *         It makes a single pass through the data and does n/6 swaps on average.
     */
    function _partition(uint256 begin, uint256 end, uint256 offset) private pure returns (uint256 index) {
        unchecked {
            // the midpoint rounds up and is always greater than begin
            // this ensures an index greater than begin will be returned
            uint256 mid = (begin + end) / 64 * 32;
            uint256 pivot = _mload(mid);
            begin -= 0x20;
            while (true) {
                uint256 begin_value;
                do {
                    begin += 0x20;
                    begin_value = _mload(begin);
                } while (begin_value > pivot);

                uint256 end_value;
                do {
                    end -= 0x20;
                    end_value = _mload(end);
                } while (end_value < pivot);

                if (begin >= end) {
                    return begin;
                }

                _swapWithOffset(begin, begin_value, end, end_value, offset);
            }
        }
    }

    /**
     * @dev Swaps the elements in memory location `ptr1` and `ptr2`, and `ptr1 + offset`, `ptr2 + offset`.
     * @param val1 The value of ptr1
     * @param val2 The value of ptr2
     */
    function _swapWithOffset(uint256 ptr1, uint256 val1, uint256 ptr2, uint256 val2, uint256 offset) private pure {
        assembly {
            mstore(ptr1, val2)
            mstore(ptr2, val1)
        }
        unchecked {
            _swap(ptr1 + offset, ptr2 + offset);
        }
    }

    /**
     * @dev Swaps the elements memory location `ptr1` and `ptr2`.
     */
    function _swap(uint256 ptr1, uint256 ptr2) private pure {
        assembly {
            let value1 := mload(ptr1)
            mstore(ptr1, mload(ptr2))
            mstore(ptr2, value1)
        }
    }

    /**
     * @dev Pointer to the memory location of the first element of `array`.
     */
    function _begin(uint256[] memory array) internal pure returns (uint256 ptr) {
        assembly ("memory-safe") {
            ptr := add(array, 0x20)
        }
    }

    /**
     * @dev Load memory word (as a uint256) at location `ptr`.
     */
    function _mload(uint256 ptr) internal pure returns (uint256 value) {
        assembly {
            value := mload(ptr)
        }
    }
}
