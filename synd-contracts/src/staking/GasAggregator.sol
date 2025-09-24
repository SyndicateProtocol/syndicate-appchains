// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";

interface GasCounter {
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
    function getEmissionsReceiver() external view returns (address);
}

interface AppchainFactory {
    function getTotalAppchains() external view returns (uint256);
    function getContractsForAppchains(uint256[] memory chainIDs) external view returns (address[] memory);
    function getAppchainsAndContracts() external view returns (uint256[] memory chainIDs, address[] memory contracts);
}

/**
 * @title GasAggregator
 * @notice Aggregates gas usage data from appchains
 * @dev This contract manages the collection and aggregation of gas usage data from multiple appchains.
 *      It supports both automatic aggregation (for small numbers of appchains) and off-chain aggregation
 *      (for larger numbers of appchains) with a challenge mechanism for data integrity.
 * @dev Inherits from EpochTracker for epoch management and AccessControlUpgradeable for admin functions
 */
contract GasAggregator is Initializable, EpochTracker, AccessControlUpgradeable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Factory contract for managing appchain contracts
    /// @dev Used to get appchain addresses and total count
    AppchainFactory public factory;

    /// @notice Maximum number of appchains that can be queried automatically
    /// @dev When total appchains >= this value, off-chain aggregation is required
    uint256 public maxAppchainsToQuery;

    /// @notice Challenge window duration for off-chain aggregation submissions
    /// @dev Time period after first submission during which new submissions can be made
    ///      After this window, the data must be sealed for the next epoch aggregation to start
    uint256 public challengeWindow;

    /// @notice Current epoch being processed for aggregation
    /// @dev Tracks which epoch is pending aggregation
    uint256 public pendingEpoch;

    /// @notice Timestamp of the first submission for the current pending epoch
    /// @dev Used to calculate challenge window expiration
    uint256 public pendingEpochFirstSubmissionTime;

    /// @notice Hash of the pending data for the current epoch
    /// @dev Stores the hash of (appchainIDs, tokens, emissionsReceivers) for verification
    bytes32 public pendingDataHash;

    /// @notice Total tokens used in the pending epoch
    /// @dev Used to ensure new submissions have higher total than previous ones
    uint256 public pendingTotalTokensUsed;

    /// @notice Mapping from epoch to aggregated data hash
    /// @dev Stores the final hash for each completed epoch (can be used for re-submissions)
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Error thrown when automatic aggregation is attempted but off-chain aggregation is required
    /// @dev Triggered when total appchains >= maxAppchainsToQuery
    error MustUseOffchainAggregation();

    /// @notice Error thrown when off-chain aggregation is attempted but automatic aggregation should be used
    /// @dev Triggered when total appchains < maxAppchainsToQuery
    error MustUseAutomaticAggregation();

    /// @notice Error thrown when new submission total is not higher than pending total
    /// @dev Ensures submissions improve upon previous ones
    error NotHigherThanPendingTotal(uint256 submitted, uint256 pending);

    /// @notice Error thrown when attempting to aggregate an epoch that hasn't ended
    /// @dev Prevents aggregation of current or future epochs
    error EpochNotOver(uint256 epoch, uint256 currentEpoch);

    /// @notice Error thrown when attempting to seal before challenge window expires
    /// @dev Ensures challenge window has elapsed before sealing
    error WindowNotOver(uint256 currentEpoch, uint256 challengeWindow);

    /// @notice Error thrown when attempting to submit after challenge window has expired
    /// @dev Prevents submissions after the challenge period
    error WindowOver(uint256 currentEpoch, uint256 challengeWindow);

    /// @notice Error thrown when appchain IDs are not submitted in ascending order
    /// @dev Ensures consistent ordering for data integrity
    error ChainIDsMustBeInAscendingOrder();

    /// @notice Error thrown when challenge window is set to zero
    /// @dev Prevents invalid configuration
    error ZeroChallengeWindow();

    /// @notice Error thrown when a zero address is provided
    /// @dev Prevents invalid contract addresses
    error ZeroAddress();

    /// @notice Error thrown when data hash is invalid
    /// @dev Ensures data integrity
    error InvalidDataHash();

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER 
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Constructor that disables initializers
     * @dev Prevents direct initialization of implementation contract
     */
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initialize the GasAggregator contract
     * @dev Sets up the contract with factory, admin, and challenge window configuration
     * @param _factory The appchain factory contract address
     * @param admin The address to be granted admin privileges
     * @param _challengeWindow The challenge window duration for off-chain aggregation
     */
    function initialize(AppchainFactory _factory, address admin, uint256 _challengeWindow) external initializer {
        if (address(_factory) == address(0)) revert ZeroAddress();
        if (admin == address(0)) revert ZeroAddress();
        if (_challengeWindow == 0) revert ZeroChallengeWindow();

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // consider all past epochs ignored
        pendingEpoch = getCurrentEpoch();
        factory = _factory;
        challengeWindow = _challengeWindow;
    }

    /*//////////////////////////////////////////////////////////////
                            MODIFIERS 
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Modifier that ensures the epoch has completed before allowing aggregation
     * @dev Prevents aggregation of current or future epochs
     * @param epoch The epoch index to check
     */
    modifier onlyCompletedEpoch(uint256 epoch) {
        uint256 currentEpoch = getCurrentEpoch();
        if (currentEpoch <= epoch) {
            revert EpochNotOver(epoch, currentEpoch);
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Triggers automatic aggregation of appchain gas usage data
     * @dev Only usable when total appchains < maxAppchainsToQuery.
     *      Queries all appchains directly and aggregates their gas usage data.
     *      After aggregation, moves to the next epoch.
     * @custom:example If 5 appchains exist and maxAppchainsToQuery is 10, this function will work
     * @custom:example If 15 appchains exist and maxAppchainsToQuery is 10, this function will revert
     */
    function aggregateTokensUsed() external onlyCompletedEpoch(pendingEpoch) {
        if (fallbackToOffchainAggregation()) {
            revert MustUseOffchainAggregation();
        }
        (uint256[] memory appchains, address[] memory contracts) = factory.getAppchainsAndContracts();
        uint256[] memory tokens = new uint256[](appchains.length);
        address[] memory emissionsReceivers = new address[](appchains.length);
        for (uint256 i = 0; i < appchains.length; i++) {
            tokens[i] = GasCounter(contracts[i]).getTokensForEpoch(pendingEpoch);
            emissionsReceivers[i] = GasCounter(contracts[i]).getEmissionsReceiver();
        }
        aggregatedEpochDataHash[pendingEpoch] = keccak256(abi.encode(appchains, tokens, emissionsReceivers));
        pendingEpoch++;
    }

    /**
     * @notice Submit top appchains aggregated off-chain
     * @dev Used when total appchains >= maxAppchainsToQuery.
     *      Allows submission of top-performing appchains with challenge mechanism.
     *      AppchainIDs must be submitted in ascending order for data integrity.
     * @param appchainIDs The chainIDs of the top appchains for the current epoch
     * @custom:example Submit [1, 5, 10] for top 3 appchains (must be in ascending order)
     * @custom:example If challenge window is 1 hour, submissions are only allowed for 1 hour after first submission
     */
    function submitOffchainTopChains(uint256[] calldata appchainIDs) external onlyCompletedEpoch(pendingEpoch) {
        if (!fallbackToOffchainAggregation()) {
            revert MustUseAutomaticAggregation();
        }
        if (pendingEpochFirstSubmissionTime != 0 && block.timestamp > pendingEpochFirstSubmissionTime + challengeWindow)
        {
            revert WindowOver(pendingEpoch, challengeWindow);
        }
        uint256 total = 0;
        address[] memory contracts = factory.getContractsForAppchains(appchainIDs);
        uint256[] memory tokens = new uint256[](appchainIDs.length);
        address[] memory emissionsReceivers = new address[](appchainIDs.length);
        for (uint256 i = 0; i < appchainIDs.length; i++) {
            if (i > 0 && appchainIDs[i] <= appchainIDs[i - 1]) {
                revert ChainIDsMustBeInAscendingOrder();
            }
            tokens[i] = GasCounter(contracts[i]).getTokensForEpoch(pendingEpoch);
            emissionsReceivers[i] = GasCounter(contracts[i]).getEmissionsReceiver();
            total += tokens[i];
        }
        if (total <= pendingTotalTokensUsed) {
            revert NotHigherThanPendingTotal(total, pendingTotalTokensUsed);
        }
        if (pendingEpochFirstSubmissionTime == 0) {
            pendingEpochFirstSubmissionTime = block.timestamp;
        }
        pendingDataHash = keccak256(abi.encode(appchainIDs, tokens, emissionsReceivers));
        pendingTotalTokensUsed = total;
    }

    /**
     * @notice Seal the pending epoch after challenge window expires
     * @dev Finalizes the off-chain aggregation by sealing the pending data.
     *      Can only be called after the challenge window has expired.
     *      Resets all pending state variables for the next epoch.
     * @custom:example If challenge window is 1 hour, can only seal after 1 hour from first submission
     */
    function sealPendingEpoch() external onlyCompletedEpoch(pendingEpoch) {
        if (
            pendingEpochFirstSubmissionTime == 0 || block.timestamp <= pendingEpochFirstSubmissionTime + challengeWindow
        ) {
            revert WindowNotOver(pendingEpoch, challengeWindow);
        }
        aggregatedEpochDataHash[pendingEpoch] = pendingDataHash;
        pendingEpoch++;
        pendingEpochFirstSubmissionTime = 0;
        pendingDataHash = bytes32(0);
        pendingTotalTokensUsed = 0;
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Check if off-chain aggregation is required
     * @dev Returns true when total appchains >= maxAppchainsToQuery
     * @return True if off-chain aggregation is required, false for automatic aggregation
     * @custom:example If 15 appchains exist and maxAppchainsToQuery is 10, returns true
     * @custom:example If 5 appchains exist and maxAppchainsToQuery is 10, returns false
     */
    function fallbackToOffchainAggregation() public view returns (bool) {
        uint256 totalAppchains = factory.getTotalAppchains();
        return totalAppchains >= maxAppchainsToQuery;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set the maximum number of appchains that can be queried automatically
     * @dev When total appchains >= this value, off-chain aggregation is required
     * @param newMax The new maximum number of appchains for automatic aggregation
     * @custom:example If set to 10, automatic aggregation works for ≤10 appchains
     */
    function setMaxAppchainsToQuery(uint256 newMax) external onlyRole(DEFAULT_ADMIN_ROLE) {
        maxAppchainsToQuery = newMax;
    }

    /**
     * @notice Set the challenge window duration for off-chain aggregation
     * @dev Time period after first submission during which new submissions can be made
     * @param newChallengeWindow The new challenge window duration in seconds
     * @custom:example If set to 3600, challenge window is 1 hour
     */
    function setChallengeWindow(uint256 newChallengeWindow) external onlyRole(DEFAULT_ADMIN_ROLE) {
        challengeWindow = newChallengeWindow;
    }

    /**
     * @notice Set the appchain factory contract
     * @dev Updates the factory contract used to get appchain addresses and count
     * @param newFactory The new factory contract address
     */
    function setFactory(AppchainFactory newFactory) external onlyRole(DEFAULT_ADMIN_ROLE) {
        factory = newFactory;
    }
}
