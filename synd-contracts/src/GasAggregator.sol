// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {GasEpoch} from "./GasEpoch.sol";

// TODO docs
abstract contract GasAggregator is GasEpoch {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/
    uint8 public maxAppchainsToQuery;

    uint256 public challengeWindow = 24 hours;

    uint256 public currentEpochToAggregate;

    uint256 public curentEpochKnownMaxGas;

    /// @notice used for the offchain aggregation mechanism
    uint256[] public pendingChainIDs;
    uint256[] public pendingTokensUsed;
    uint256 public pendingTotalTokensUsed;

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR 
    //////////////////////////////////////////////////////////////*/
    constructor() {
        // consider all past epochs ignoed
        currentEpochToAggregate = getCurrentEpoch();
    }

    /*//////////////////////////////////////////////////////////////
                            MODIFIERS 
    //////////////////////////////////////////////////////////////*/

    modifier onlyCompletedPeriod() {
        uint256 currentEpoch = getCurrentEpoch();
        if (currentEpoch <= currentEpochToAggregate) {
            revert CurrentEpochToAggregateNotOver(currentEpochToAggregate, currentEpoch);
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                            ERRORS
    //////////////////////////////////////////////////////////////*/
    error MustUseOffchainAggregation();
    error MustUseAutomaticAggregation();
    error NotHigherThanPendingTotal(uint256 submitted, uint256 pending);
    error CurrentEpochToAggregateNotOver(uint256 currentEpochToAggregate, uint256 currentEpoch);
    error WindowNotOver(uint256 currentEpoch, uint256 challengeWindow);

    /*//////////////////////////////////////////////////////////////
                            INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function getTokensUsed(uint256 chainID) internal view returns (uint256) {
        address appContract = getAppchainContract(chainID);
        return GasCounter(appContract).getTokensForEpoch(currentEpochToAggregate);
    }

    function pushToStakingAppchain(uint256[] memory appchainIDs, uint256[] memory tokens) internal {
        // TODO implement me
    }

    /*//////////////////////////////////////////////////////////////
                            PUBLIC FUNCTIONS
    //////////////////////////////////////////////////////////////*/
    function aggregateTokensUsed() public onlyCompletedPeriod {
        if (fallbackToOffchainAggregation()) {
            revert MustUseOffchainAggregation();
        }
        uint256[] memory appchains = getAppchainChainIDs();
        uint256[] memory tokens = new uint256[](appchains.length);
        for (uint256 i = 0; i < appchains.length; i++) {
            tokens[i] = getTokensUsed(appchains[i]);
        }
        pushToStakingAppchain(appchains, tokens);
        currentEpochToAggregate++;
    }

    function sumbitOffchainTopChains(uint256[] memory appchainIDs) public onlyCompletedPeriod {
        uint256 total = 0;
        uint256[] memory tokens = new uint256[](appchainIDs.length);
        for (uint256 i = 0; i < appchainIDs.length; i++) {
            tokens[i] = getTokensUsed(appchainIDs[i]);
            total += tokens[i];
        }
        if (total <= pendingTotalTokensUsed) {
            revert NotHigherThanPendingTotal(total, pendingTotalTokensUsed);
        }
        pendingChainIDs = appchainIDs;
        pendingTokensUsed = tokens;
        pendingTotalTokensUsed = total;
    }

    function pushTopChainsDataToStakingAppchain() public onlyCompletedPeriod {
        if (!fallbackToOffchainAggregation()) {
            revert MustUseAutomaticAggregation();
        }
        if (block.timestamp < getEpochEndTime(currentEpochToAggregate) + challengeWindow) {
            revert WindowNotOver(currentEpochToAggregate, challengeWindow);
        }
        pushToStakingAppchain(pendingChainIDs, pendingTokensUsed);
        // cleanup pending data
        currentEpochToAggregate++;
        delete pendingChainIDs;
        delete pendingTokensUsed;
        pendingTotalTokensUsed = 0;
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function fallbackToOffchainAggregation() public view returns (bool) {
        uint256 totalAppchains = getTotalAppchains();
        return totalAppchains >= maxAppchainsToQuery;
    }

    function getTotalAppchains() public view virtual returns (uint256);
    function getAppchainChainIDs() public view virtual returns (uint256[] memory);
    function getAppchainContract(uint256 chainId) public view virtual returns (address);

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice set the max number of appchains to query
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _setMaxAppchainsToQuery(uint8 n) internal {
        maxAppchainsToQuery = n;
    }

    /// @notice set the challenge window
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _setChallengeWindow(uint256 n) internal {
        challengeWindow = n;
    }
}

interface GasCounter {
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
}
