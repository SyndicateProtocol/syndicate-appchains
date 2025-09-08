// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./interfaces/IPool.sol";

/**
 * @title Splitter
 * @notice Contract for splitting pool rewards between base, performance, and appchain pools
 * @dev This contract manages the distribution of rewards to different pools: base 30%, performance 30%, and appchain 40%.
 */
contract Splitter {
    /// @notice Percentage of the reward for the performance pool
    uint256 public constant PERFORMANCE_POOL_SPLIT = 30; // 30%

    /// @notice Percentage of the reward for the appchain pool
    uint256 public constant APPCHAIN_POOL_SPLIT = 40; // 40%

    // Remaining percentage of the reward goes to the base pool: 30%

    /// @notice Total percentage of the reward
    uint256 public constant PERCENTAGE_DENOMINATOR = 100; // 100%

    /// @notice Address of the base pool contract
    address public basePool;

    /// @notice Address of the performance pool contract
    address public performancePool;

    /// @notice Address of the appchain pool contract
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
     * @param epochIndex The epoch index for the reward distribution
     * @dev This function:
     *      - Calculates amounts for each pool based on their split percentages
     *      - Prevents dust by assigning remaining amount to base pool
     *      - Must be called with ETH value (msg.value > 0)
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
