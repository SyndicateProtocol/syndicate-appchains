// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IPool} from "./interfaces/IPool.sol";

/**
 * @title Splitter
 * @notice Contract for splitting pool rewards between base, performance, and appchain pools
 * @dev This contract manages the distribution of rewards to different pools with a fixed allocation:
 *      - Base Pool: 30% (remaining after performance and appchain splits)
 *      - Performance Pool: 30% (user performance rewards)
 *      - Appchain Pool: 40% (appchain-specific rewards)
 */
contract Splitter {
    /// @notice Percentage of the reward allocated to the performance pool (30%)
    /// @dev This determines how much of incoming rewards go to user performance rewards
    uint256 public constant PERFORMANCE_POOL_SPLIT = 30; // 30%

    /// @notice Percentage of the reward allocated to the appchain pool (40%)
    /// @dev This determines how much of incoming rewards go to appchain-specific rewards
    uint256 public constant APPCHAIN_POOL_SPLIT = 40; // 40%

    /// @notice Percentage of the reward allocated to the base pool (30%)
    /// @dev This is calculated as remainder: 100% - 30% - 40% = 30%
    ///      This ensures no dust is lost and all rewards are distributed

    /// @notice Total percentage denominator for calculations (100%)
    /// @dev Used for percentage calculations to ensure precision
    uint256 public constant PERCENTAGE_DENOMINATOR = 100; // 100%

    /// @notice Address of the base pool contract
    /// @dev Receives 30% of all incoming rewards (base staking rewards)
    address public basePool;

    /// @notice Address of the performance pool contract
    /// @dev Receives 30% of all incoming rewards (user performance rewards)
    address public performancePool;

    /// @notice Address of the appchain pool contract
    /// @dev Receives 40% of all incoming rewards (appchain-specific rewards)
    address public appchainPool;

    /// @notice Emitted when rewards are split and deposited to the pools
    /// @param epochIndex The epoch index for the reward distribution
    /// @param basePoolAmount Amount of ETH deposited to the base pool
    /// @param performancePoolAmount Amount of ETH deposited to the performance pool
    /// @param appchainPoolAmount Amount of ETH deposited to the appchain pool
    event Split(uint256 epochIndex, uint256 basePoolAmount, uint256 performancePoolAmount, uint256 appchainPoolAmount);

    /// @notice Error thrown when no value is sent
    error NoValueSent();

    /// @notice Error thrown when an invalid address is provided
    error InvalidAddress();

    /**
     * @notice Initializes the Splitter contract with base, performance, and appchain pool addresses
     * @param _basePool Address of the base pool contract
     * @param _performancePool Address of the performance pool contract
     * @param _appchainPool Address of the appchain pool contract
     */
    constructor(address _basePool, address _performancePool, address _appchainPool) {
        if (_basePool == address(0)) {
            revert InvalidAddress();
        }
        if (_performancePool == address(0)) {
            revert InvalidAddress();
        }
        if (_appchainPool == address(0)) {
            revert InvalidAddress();
        }

        basePool = _basePool;
        performancePool = _performancePool;
        appchainPool = _appchainPool;
    }

    /**
     * @notice Splits incoming ETH rewards between the three pools based on configured percentages
     * @dev This function automatically distributes incoming ETH to the three pools:
     *      - Performance Pool: 30% of total
     *      - Appchain Pool: 40% of total
     *      - Base Pool: Remaining amount (30% + any dust)
     * @param epochIndex The epoch index for the reward distribution
     * @custom:example If 1000 ETH is sent, Performance gets 300, Appchain gets 400, Base gets 300
     * @custom:example If 1001 ETH is sent, Performance gets 300, Appchain gets 400, Base gets 301 (dust goes to base)
     */
    function deposit(uint256 epochIndex) external payable {
        if (msg.value == 0) {
            revert NoValueSent();
        }

        uint256 total = msg.value;

        uint256 performancePoolAmount = (total * PERFORMANCE_POOL_SPLIT) / PERCENTAGE_DENOMINATOR;
        uint256 appchainPoolAmount = (total * APPCHAIN_POOL_SPLIT) / PERCENTAGE_DENOMINATOR;
        uint256 basePoolAmount = total - performancePoolAmount - appchainPoolAmount;

        IPool(basePool).deposit{value: basePoolAmount}(epochIndex);
        IPool(performancePool).deposit{value: performancePoolAmount}(epochIndex);
        IPool(appchainPool).deposit{value: appchainPoolAmount}(epochIndex);

        emit Split(epochIndex, basePoolAmount, performancePoolAmount, appchainPoolAmount);
    }
}
