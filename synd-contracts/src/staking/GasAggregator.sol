// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";

interface GasCounter {
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
}

interface AppchainFactory {
    function getTotalAppchainsForGasTracking() external view returns (uint256);
    function getContractsForGasTracking(uint256[] memory chainIDs) external view returns (address[] memory);
    function getAppchainsAndContractsForGasTracking()
        external
        view
        returns (uint256[] memory chainIDs, address[] memory contracts);
}

/// @title GasAggregator
/// @notice Aggregates gas usage data from appchains and pushes it to the staking appchain
contract GasAggregator is Initializable, EpochTracker, AccessControlUpgradeable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    AppchainFactory public factory;

    uint256 public maxAppchainsToQuery;

    /// @notice used for the offchain aggregation mechanism.
    /// The challenge window is the time period after the first submission during which new submission can be made
    /// After the challenge window has elapsed, the data must be pushed to the staking appchain for the next epoch aggregation to start
    uint256 public challengeWindow;
    uint256 public pendingEpoch;
    uint256 public pendingEpochFirstSubmissionTime;
    bytes32 public pendingDataHash;
    uint256 public pendingTotalTokensUsed;

    /// @notice last epoch that was aggregated using the offchain mechanism (this data can be used for re-submissions)
    mapping(uint256 => bytes32) public aggregatedEpochDataHash;

    /*//////////////////////////////////////////////////////////////
      ERRORS
    //////////////////////////////////////////////////////////////*/
    error MustUseOffchainAggregation();
    error MustUseAutomaticAggregation();
    error NotHigherThanPendingTotal(uint256 submitted, uint256 pending);
    error EpochNotOver(uint256 epoch, uint256 currentEpoch);
    error WindowNotOver(uint256 currentEpoch, uint256 challengeWindow);
    error WindowOver(uint256 currentEpoch, uint256 challengeWindow);
    error ChainIDsMustBeInAscendingOrder();
    error ZeroChallengeWindow();
    error ZeroAddress();
    error InvalidDataHash();

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER 
    //////////////////////////////////////////////////////////////*/

    constructor() {
        _disableInitializers();
    }

    function initialize(AppchainFactory _factory, address admin, uint256 _challengeWindow) external initializer {
        if (address(_factory) == address(0)) revert ZeroAddress();
        if (admin == address(0)) revert ZeroAddress();
        if (_challengeWindow == 0) revert ZeroChallengeWindow();

        __AccessControl_init();
        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // consider all past epochs ignoed
        pendingEpoch = getCurrentEpoch();
        factory = _factory;
        challengeWindow = _challengeWindow;
    }

    /*//////////////////////////////////////////////////////////////
                            MODIFIERS 
    //////////////////////////////////////////////////////////////*/

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

    /// @notice triggers automatic aggregation of the appchain gas usage data and pushes it to the staking appchain
    /// @dev only usable until there are up to `maxAppchainsToQuery` appchains, after that point the offchain aggregation mechanism must be used
    function aggregateTokensUsed() external onlyCompletedEpoch(pendingEpoch) {
        if (fallbackToOffchainAggregation()) {
            revert MustUseOffchainAggregation();
        }
        (uint256[] memory appchains, address[] memory contracts) = factory.getAppchainsAndContractsForGasTracking();
        uint256[] memory tokens = new uint256[](appchains.length);
        for (uint256 i = 0; i < appchains.length; i++) {
            tokens[i] = GasCounter(contracts[i]).getTokensForEpoch(pendingEpoch);
        }
        aggregatedEpochDataHash[pendingEpoch] = keccak256(abi.encode(appchains, tokens));
        pendingEpoch++;
    }

    /// @notice Submission of top appchains aggregated off-chain
    /// @dev appchainIDs must be submitted in ascending order
    /// @param appchainIDs the chainIDs of the top appchains for the current epoch
    function submitOffchainTopChains(uint256[] calldata appchainIDs) external onlyCompletedEpoch(pendingEpoch) {
        if (!fallbackToOffchainAggregation()) {
            revert MustUseAutomaticAggregation();
        }
        if (pendingEpochFirstSubmissionTime != 0 && block.timestamp > pendingEpochFirstSubmissionTime + challengeWindow)
        {
            revert WindowOver(pendingEpoch, challengeWindow);
        }
        uint256 total = 0;
        address[] memory contracts = factory.getContractsForGasTracking(appchainIDs);
        uint256[] memory tokens = new uint256[](appchainIDs.length);
        for (uint256 i = 0; i < appchainIDs.length; i++) {
            if (i > 0 && appchainIDs[i] <= appchainIDs[i - 1]) {
                revert ChainIDsMustBeInAscendingOrder();
            }
            tokens[i] = GasCounter(contracts[i]).getTokensForEpoch(pendingEpoch);
            total += tokens[i];
        }
        if (total <= pendingTotalTokensUsed) {
            revert NotHigherThanPendingTotal(total, pendingTotalTokensUsed);
        }
        if (pendingEpochFirstSubmissionTime == 0) {
            pendingEpochFirstSubmissionTime = block.timestamp;
        }
        pendingDataHash = keccak256(abi.encode(appchainIDs, tokens));
        pendingTotalTokensUsed = total;
    }

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

    function fallbackToOffchainAggregation() public view returns (bool) {
        uint256 totalAppchains = factory.getTotalAppchainsForGasTracking();
        return totalAppchains >= maxAppchainsToQuery;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice set the max number of appchains to query
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function setMaxAppchainsToQuery(uint256 newMax) external onlyRole(DEFAULT_ADMIN_ROLE) {
        maxAppchainsToQuery = newMax;
    }

    /// @notice set the challenge window
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function setChallengeWindow(uint256 newChallengeWindow) external onlyRole(DEFAULT_ADMIN_ROLE) {
        challengeWindow = newChallengeWindow;
    }

    /// @notice Sets the appchain factory contract
    /// @param newFactory The new factory contract address
    function setFactory(AppchainFactory newFactory) external onlyRole(DEFAULT_ADMIN_ROLE) {
        factory = newFactory;
    }
}
