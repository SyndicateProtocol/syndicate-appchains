// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./IPool.sol";

/**
 * @title Splitter
 * @notice Contract for splitting pool rewards between base, performance, and appchain pools
 * @dev This contract manages the distribution of rewards to different pools: base 30%, performance 30%, and appchain 40%.
 */
contract Splitter {
    // uint256 public constant BASE_POOL_SPLIT = 30 ether;
    uint256 public constant PERFORMANCE_POOL_SPLIT = 30 ether;
    uint256 public constant APPCHAIN_POOL_SPLIT = 40 ether;

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

    /**
     * @notice Initializes the Splitter contract with base, performance, and appchain pool addresses
     * @param _basePool Address of the base pool contract
     * @param _performancePool Address of the performance pool contract
     * @param _appchainPool Address of the appchain pool contract
     */
    constructor(address _basePool, address _performancePool, address _appchainPool) {
        require(_basePool != address(0), "Base pool cannot be 0 address");
        require(_performancePool != address(0), "Performance pool cannot be 0 address");
        require(_appchainPool != address(0), "Appchain pool cannot be 0 address");

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
     * @custom:security This function is external and payable, allowing anyone to trigger splits
     */
    function split(uint256 epochIndex) external payable {
        require(msg.value > 0, "No value sent");
        uint256 total = msg.value;

        uint256 performancePoolAmount = (total * PERFORMANCE_POOL_SPLIT) / 100 ether;
        uint256 appchainPoolAmount = (total * APPCHAIN_POOL_SPLIT) / 100 ether;
        uint256 basePoolAmount = total - performancePoolAmount - appchainPoolAmount;
        // Prevent dust
        if (basePoolAmount > 0) {
            basePoolAmount = total - performancePoolAmount - appchainPoolAmount;
        }

        IPool(basePool).deposit{value: basePoolAmount}(epochIndex);
        IPool(performancePool).deposit{value: performancePoolAmount}(epochIndex);
        IPool(appchainPool).deposit{value: appchainPoolAmount}(epochIndex);

        emit Split(epochIndex, basePoolAmount, performancePoolAmount, appchainPoolAmount);
    }
}
