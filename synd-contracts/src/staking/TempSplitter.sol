// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {IPool} from "./interfaces/IPool.sol";

/**
 * @title TempSplitter
 * @notice Temporary contract for splitting pool rewards between base and appchain pools only
 * @dev This contract manages the distribution of rewards to different pools with a fixed allocation:
 *      - Base Pool: 60% (base 30% + performance 30% combined)
 *      - Appchain Pool: 40% (appchain-specific rewards)
 *      Performance pool rewards are redirected to base pool.
 */
contract TempSplitter {
    /// @notice Percentage of the reward allocated to the appchain pool (40%)
    /// @dev This determines how much of incoming rewards go to appchain-specific rewards
    uint256 public constant APPCHAIN_POOL_SPLIT = 40; // 40%

    /// @notice Total percentage denominator for calculations (100%)
    /// @dev Used for percentage calculations to ensure precision
    uint256 public constant PERCENTAGE_DENOMINATOR = 100; // 100%

    /// @notice Address of the base pool contract
    /// @dev Receives 60% of all incoming rewards (base + performance combined)
    address public basePool;

    /// @notice Address of the appchain pool contract
    /// @dev Receives 40% of all incoming rewards (appchain-specific rewards)
    address public appchainPool;

    /// @notice Emitted when rewards are split and deposited to the pools
    /// @param epochIndex The epoch index for the reward distribution
    /// @param basePoolAmount Amount of SYND deposited to the base pool
    /// @param appchainPoolAmount Amount of SYND deposited to the appchain pool
    event Split(uint256 epochIndex, uint256 basePoolAmount, uint256 appchainPoolAmount);

    /// @notice Error thrown when no value is sent
    error NoValueSent();

    /// @notice Error thrown when an invalid address is provided
    error InvalidAddress();

    /**
     * @notice Initializes the TempSplitter contract with base and appchain pool addresses
     * @param _basePool Address of the base pool contract
     * @param _appchainPool Address of the appchain pool contract
     */
    constructor(address _basePool, address _appchainPool) {
        if (_basePool == address(0)) {
            revert InvalidAddress();
        }
        if (_appchainPool == address(0)) {
            revert InvalidAddress();
        }

        basePool = _basePool;
        appchainPool = _appchainPool;
    }

    /**
     * @notice Splits incoming SYND rewards between the two pools based on configured percentages
     * @dev This function automatically distributes incoming SYND to the two pools:
     *      - Appchain Pool: 40% of total
     *      - Base Pool: Remaining amount (60% + any dust)
     * @param epochIndex The epoch index for the reward distribution
     * @custom:example If 1000 SYND is sent, Appchain gets 400, Base gets 600
     * @custom:example If 1001 SYND is sent, Appchain gets 400, Base gets 601 (dust goes to base)
     */
    function deposit(uint256 epochIndex) external payable {
        if (msg.value == 0) {
            revert NoValueSent();
        }

        uint256 total = msg.value;

        uint256 appchainPoolAmount = (total * APPCHAIN_POOL_SPLIT) / PERCENTAGE_DENOMINATOR;
        uint256 basePoolAmount = total - appchainPoolAmount;

        IPool(basePool).deposit{value: basePoolAmount}(epochIndex);
        IPool(appchainPool).deposit{value: appchainPoolAmount}(epochIndex);

        emit Split(epochIndex, basePoolAmount, appchainPoolAmount);
    }
}
