// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

interface GasCounter {
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
}

interface AppchainFactory {
    function getTotalAppchains() external view returns (uint256);
    function getContractsForAppchains(uint256[] memory chainIDs) external view returns (address[] memory);
    function getAppchainsAndContracts() external view returns (uint256[] memory chainIDs, address[] memory contracts);
}

// TODO SEQ-1283: this is just a placeholder until we have the actual usage designed
interface StakingAppchain {
    function pushData(uint256[] memory chainIDs, uint256[] memory tokens, uint256 epoch) external;
}

/// @title GasAggregator
/// @notice Aggregates gas usage data from appchains and pushes it to the staking appchain
contract GasAggregator is EpochTracker, AccessControl {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    AppchainFactory public factory;
    StakingAppchain public stakingAppchain;

    uint8 public maxAppchainsToQuery;

    uint256 public challengeWindow = 24 hours;

    /// @notice used for the offchain aggregation mechanism
    uint256 public currentEpochToAggregate;
    uint256 public pendingEpoch;
    uint256[] public pendingChainIDs;
    uint256[] public pendingTokensUsed;
    uint256 public pendingTotalTokensUsed;

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
    error NoPendingDataToPush();
    error ZeroAddress();

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR 
    //////////////////////////////////////////////////////////////*/
    constructor(AppchainFactory _factory, StakingAppchain _stakingAppchain, address admin, uint256 _epochStartTimestamp)
        EpochTracker(_epochStartTimestamp)
    {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // consider all past epochs ignoed
        currentEpochToAggregate = getCurrentEpoch();
        factory = _factory;
        stakingAppchain = _stakingAppchain;
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
                            INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function _pushToStakingAppchain(uint256[] memory appchainIDs, uint256[] memory tokens, uint256 epoch) internal {
        stakingAppchain.pushData(appchainIDs, tokens, epoch);
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice triggers automatic aggregation of the appchain gas usage data and pushes it to the staking appchain
    /// @dev only usable until there are up to `maxAppchainsToQuery` appchains, after that point the offchain aggregation mechanism must be used
    function aggregateTokensUsed(uint256 epoch) external onlyCompletedEpoch(epoch) {
        if (fallbackToOffchainAggregation()) {
            revert MustUseOffchainAggregation();
        }
        (uint256[] memory appchains, address[] memory contracts) = factory.getAppchainsAndContracts();
        uint256[] memory tokens = new uint256[](appchains.length);
        for (uint256 i = 0; i < appchains.length; i++) {
            tokens[i] = GasCounter(contracts[i]).getTokensForEpoch(epoch);
        }
        if (currentEpochToAggregate <= epoch) {
            currentEpochToAggregate = epoch + 1;
        }
        _pushToStakingAppchain(appchains, tokens, epoch);
    }

    /// @notice Submission of top appchains aggregated off-chain
    /// @dev appchainIDs must be submitted in ascending order
    /// @param appchainIDs the chainIDs of the top appchains for the current epoch
    function submitOffchainTopChains(uint256[] calldata appchainIDs)
        external
        onlyCompletedEpoch(currentEpochToAggregate)
    {
        if (block.timestamp > getEpochEnd(currentEpochToAggregate) + challengeWindow) {
            revert WindowOver(currentEpochToAggregate, challengeWindow);
        }
        uint256 total = 0;
        address[] memory contracts = factory.getContractsForAppchains(appchainIDs);
        uint256[] memory tokens = new uint256[](appchainIDs.length);
        for (uint256 i = 0; i < appchainIDs.length; i++) {
            if (i > 0 && appchainIDs[i] <= appchainIDs[i - 1]) {
                revert ChainIDsMustBeInAscendingOrder();
            }
            tokens[i] = GasCounter(contracts[i]).getTokensForEpoch(currentEpochToAggregate);
            total += tokens[i];
        }
        if (total <= pendingTotalTokensUsed) {
            revert NotHigherThanPendingTotal(total, pendingTotalTokensUsed);
        }
        pendingEpoch = currentEpochToAggregate;
        pendingChainIDs = appchainIDs;
        pendingTokensUsed = tokens;
        pendingTotalTokensUsed = total;
    }

    /// @notice Pushes the pending data of the top appchains to the stakin appchain
    /// @dev only callable after the current epoch has ended and the challengeWindow period has elapsed
    function pushTopChainsDataToStakingAppchain() external {
        if (!fallbackToOffchainAggregation()) {
            revert MustUseAutomaticAggregation();
        }
        if (block.timestamp <= getEpochEnd(currentEpochToAggregate) + challengeWindow) {
            revert WindowNotOver(currentEpochToAggregate, challengeWindow);
        }
        if (pendingChainIDs.length == 0) {
            revert NoPendingDataToPush();
        }
        if (pendingEpoch == currentEpochToAggregate) {
            currentEpochToAggregate++;
        }
        _pushToStakingAppchain(pendingChainIDs, pendingTokensUsed, pendingEpoch);
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function fallbackToOffchainAggregation() public view returns (bool) {
        uint256 totalAppchains = factory.getTotalAppchains();
        return totalAppchains >= maxAppchainsToQuery;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice set the max number of appchains to query
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function setMaxAppchainsToQuery(uint8 newMax) external onlyRole(DEFAULT_ADMIN_ROLE) {
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
