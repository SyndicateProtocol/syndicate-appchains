// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {ISyndicateSequencingChain} from "../interfaces/ISyndicateSequencingChain.sol";
import {GasCounter} from "./GasCounter.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

interface ISyndicateProxy {
    // This function exists in both the new proxy and legacy implementation contract
    function tokensUsedPerEpoch(uint256 epoch) external view returns (uint256);
}

/**
 * @title GasAggregator
 * @notice Aggregates gas usage data from appchains
 * @dev This contract manages the collection and aggregation of gas usage data from multiple appchains.
 *      It supports both automatic aggregation (for small numbers of appchains) and off-chain aggregation
 *      (for larger numbers of appchains) with a challenge mechanism for data integrity.
 * @dev Inherits from Ownable for admin functions
 */
contract GasAggregator is Ownable(msg.sender), Pausable {
    using EnumerableSet for EnumerableSet.UintSet;

    /*//////////////////////////////////////////////////////////////
                            CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Version of the GasAggregator contract (updatable during upgrades)
    /// @dev Semantic version string to track contract upgrades and compatibility
    string public constant VERSION = "1.0.0";

    /*//////////////////////////////////////////////////////////////
                            FIXED STORAGE SLOTS
    //////////////////////////////////////////////////////////////*/

    /// SLOT 0: aggregatedEpochDataHash
    /// @notice Storage slot is 0 for aggregatedEpochDataHash in GasAggregator (see `forge inspect GasAggregator storageLayout`)
    /// @dev Stores the final hash for each completed epoch.
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

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

        aggregatedEpochDataHash[currentEpoch] = pendingDataHash;
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
        uint256 count = _appchains.length() - aggregateIndex;
        require(count > 0, NoChainsAdded());
        if (maxAppchainsToQuery < count) {
            count = maxAppchainsToQuery;
            nextAggregateIndex = aggregateIndex + count;
        }
        chainIds = new uint256[](count);
        tokens = new uint256[](count);
        for (uint256 i = 0; i < count; i++) {
            chainIds[i] = _appchains.at(aggregateIndex + i);
            tokens[i] = ISyndicateProxy(appchainContract[chainIds[i]]).tokensUsedPerEpoch(currentEpoch);
        }

        // If we are aggregating the first chunk, we can skip the merge sort
        if (aggregateIndex == 0) {
            require(prevChainIds.length == 0, InvalidDataHash());
            require(prevTokens.length == 0, InvalidDataHash());
            // don't bother sorting if there is only one chunk
            if (nextAggregateIndex > 0) {
                _quickSort(chainIds, tokens);
            }
            // the first chunk does not need to be merged
            return (nextAggregateIndex, chainIds, tokens);
        }

        _quickSort(chainIds, tokens);
        uint256[] memory newChainIds = new uint256[](maxAppchainsToQuery);
        uint256[] memory newTokens = new uint256[](maxAppchainsToQuery);

        // merge sort the sorted data
        uint256 prevIndex = 0;
        uint256 index = 0;
        for (uint256 i = 0; i < maxAppchainsToQuery; i++) {
            if (index == count || prevTokens[prevIndex] >= tokens[index]) {
                newTokens[i] = prevTokens[prevIndex];
                newChainIds[i] = prevChainIds[prevIndex];
                prevIndex++;
            } else {
                newTokens[i] = tokens[index];
                newChainIds[i] = chainIds[index];
                index++;
            }
        }

        return (nextAggregateIndex, newChainIds, newTokens);
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
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    // sort key, value arrays by values from high to low
    function _quickSort(uint256[] memory keys, uint256[] memory values) internal pure {
        unchecked {
            _quickSort(_begin(values), _end(values), _begin(keys) - _begin(values));
        }
    }

    /**
     * Fork of the openzeppelin array _quickSort function that also swaps an offset array
     * @dev Performs a quick sort of a segment of memory. The segment sorted starts at `begin` (inclusive), and stops
     * at end (exclusive). Sorting follows the `comp` comparator.
     *
     * Invariant: `begin <= end`. This is the case when initially called by {sort} and is preserved in subcalls.
     *
     * IMPORTANT: Memory locations between `begin` and `end` are not validated/zeroed. This function should
     * be used only if the limits are within a memory array.
     */
    function _quickSort(uint256 begin, uint256 end, uint256 offset) internal pure {
        unchecked {
            if (end - begin < 0x40) return;

            // Use first element as pivot
            uint256 pivot = _mload(begin);
            // Position where the pivot should be at the end of the loop
            uint256 pos = begin;

            for (uint256 it = begin + 0x20; it < end; it += 0x20) {
                if (_mload(it) > pivot) {
                    // If the value stored at the iterator's position comes before the pivot, we increment the
                    // position of the pivot and move the value there.
                    pos += 0x20;
                    _swap(pos, it);
                    _swap(pos + offset, it + offset);
                }
            }

            _swap(begin, pos); // Swap pivot into place
            _swap(begin + offset, pos + offset);
            _quickSort(begin, pos, offset); // Sort the left side of the pivot
            _quickSort(pos + 0x20, end, offset); // Sort the right side of the pivot
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
     * @dev Pointer to the memory location of the first memory word (32bytes) after `array`. This is the memory word
     * that comes just after the last element of the array.
     */
    function _end(uint256[] memory array) internal pure returns (uint256 ptr) {
        unchecked {
            return _begin(array) + array.length * 0x20;
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

    /**
     * @dev Swaps the elements memory location `ptr1` and `ptr2`.
     */
    function _swap(uint256 ptr1, uint256 ptr2) internal pure {
        assembly {
            let value1 := mload(ptr1)
            let value2 := mload(ptr2)
            mstore(ptr1, value2)
            mstore(ptr2, value1)
        }
    }
}
