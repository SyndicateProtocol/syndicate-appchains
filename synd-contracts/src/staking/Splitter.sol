// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./IPool.sol";

/**
 * @title Splitter
 * @notice Contract for splitting pool rewards between base, performance, and appchain pools
 * @dev This contract manages the distribution of rewards to different pool types based on configurable split percentages.
 *      The total split must always equal 100 ether (100%).
 */
contract Splitter {
    /// @notice Address of the contract admin who can update split percentages
    address public admin;

    /// @notice Address of the base pool contract
    address public basePool;

    /// @notice Address of the performance pool contract
    address public performancePool;

    /// @notice Address of the appchain pool contract
    address public appchainPool;

    /// @notice Split percentage for the base pool (in wei, where 100 ether = 100%)
    uint256 public basePoolSplit;

    /// @notice Split percentage for the performance pool (in wei, where 100 ether = 100%)
    uint256 public performancePoolSplit;

    /// @notice Split percentage for the appchain pool (in wei, where 100 ether = 100%)
    uint256 public appchainPoolSplit;

    /// @notice Emitted when rewards are split and deposited to the pools
    /// @param epochIndex The epoch index for the reward distribution
    /// @param basePoolAmount Amount of ETH deposited to the base pool
    /// @param performancePoolAmount Amount of ETH deposited to the performance pool
    /// @param appchainPoolAmount Amount of ETH deposited to the appchain pool
    event Split(uint256 epochIndex, uint256 basePoolAmount, uint256 performancePoolAmount, uint256 appchainPoolAmount);

    /// @notice Emitted when the split percentages are updated by the admin
    /// @param basePoolSplit New split percentage for the base pool (in wei)
    /// @param performancePoolSplit New split percentage for the performance pool (in wei)
    /// @param appchainPoolSplit New split percentage for the appchain pool (in wei)
    event SplitsUpdated(uint256 basePoolSplit, uint256 performancePoolSplit, uint256 appchainPoolSplit);

    /// @notice Emitted when the base pool address is set
    event BasePoolSet(address basePool);

    /// @notice Emitted when the performance pool address is set
    event PerformancePoolSet(address performancePool);

    /// @notice Emitted when the appchain pool address is set
    event AppchainPoolSet(address appchainPool);

    /// @notice Error thrown when a non-admin address tries to call admin-only functions
    error NotAdmin();

    /// @notice Error thrown when the sum of split percentages doesn't equal 100% or there is a non-zero split with a zero address pool
    error InvalidSplits();

    /**
     * @notice Initializes the Splitter contract with admin and base pool address
     * @param _admin Address of the contract admin
     * @param _basePool Address of the base pool contract
     * @dev Sets initial splits to 100% base pool, 0% performance pool, 0% appchain pool.
     *      Performance and appchain pools are initially set to zero address and can be set later.
     */
    constructor(address _admin, address _basePool) {
        admin = _admin;

        basePool = _basePool;

        basePoolSplit = 100 ether;
        performancePoolSplit = 0 ether;
        appchainPoolSplit = 0 ether;

        emit BasePoolSet(_basePool);
    }

    /**
     * @notice Modifier that restricts function access to admin only
     * @dev Reverts with NotAdmin error if caller is not the admin
     */
    modifier onlyAdmin() {
        if (msg.sender != admin) {
            revert NotAdmin();
        }
        _;
    }

    /**
     * @notice Splits incoming ETH rewards between the three pools based on configured percentages
     * @param epochIndex The epoch index for the reward distribution
     * @dev This function:
     *      - Calculates amounts for each pool based on their split percentages
     *      - Prevents dust by assigning remaining amount to base pool
     *      - Only deposits to pools that have non-zero addresses and amounts
     *      - Must be called with ETH value (msg.value > 0)
     * @custom:security This function is external and payable, allowing anyone to trigger splits
     */
    function split(uint256 epochIndex) external payable {
        uint256 total = msg.value;

        uint256 performancePoolAmount = (total * performancePoolSplit) / 100 ether;
        uint256 appchainPoolAmount = (total * appchainPoolSplit) / 100 ether;
        // Prevent dust
        uint256 basePoolAmount = total - performancePoolAmount - appchainPoolAmount;

        if (basePoolAmount > 0 && basePool != address(0)) {
            IPool(basePool).deposit{value: basePoolAmount}(epochIndex);
        }

        if (performancePoolAmount > 0 && performancePool != address(0)) {
            IPool(performancePool).deposit{value: performancePoolAmount}(epochIndex);
        }

        if (appchainPoolAmount > 0 && appchainPool != address(0)) {
            IPool(appchainPool).deposit{value: appchainPoolAmount}(epochIndex);
        }

        emit Split(epochIndex, basePoolAmount, performancePoolAmount, appchainPoolAmount);
    }

    /**
     * @notice Updates the split percentages for all three pools
     * @param _basePoolSplit New split percentage for base pool (in wei)
     * @param _performancePoolSplit New split percentage for performance pool (in wei)
     * @param _appchainPoolSplit New split percentage for appchain pool (in wei)
     * @dev This function:
     *      - Can only be called by the admin
     *      - Validates that the pool addresses with non-zero splits are not 0 address
     *      - Validates that the sum of all splits equals 100 ether (100%)
     *      - Reverts with InvalidSplits error if validation fails
     *      - Updates all split percentages atomically
     * @custom:security Only callable by admin address
     */
    function updateSplits(uint256 _basePoolSplit, uint256 _performancePoolSplit, uint256 _appchainPoolSplit)
        external
        onlyAdmin
    {
        if (_basePoolSplit != 0 && basePool == address(0)) {
            revert InvalidSplits();
        }

        if (_performancePoolSplit != 0 && performancePool == address(0)) {
            revert InvalidSplits();
        }

        if (_appchainPoolSplit != 0 && appchainPool == address(0)) {
            revert InvalidSplits();
        }

        if (_basePoolSplit + _performancePoolSplit + _appchainPoolSplit != 100 ether) {
            revert InvalidSplits();
        }

        basePoolSplit = _basePoolSplit;
        performancePoolSplit = _performancePoolSplit;
        appchainPoolSplit = _appchainPoolSplit;

        emit SplitsUpdated(_basePoolSplit, _performancePoolSplit, _appchainPoolSplit);
    }

    /**
     * @notice Sets the base pool address
     * @param _basePool New address for the base pool contract
     * @dev Can only be called by admin. Allows updating the base pool after deployment.
     */
    function setBasePool(address _basePool) external onlyAdmin {
        if (_basePool == address(0) && basePoolSplit != 0) {
            revert InvalidSplits();
        }

        basePool = _basePool;

        emit BasePoolSet(_basePool);
    }

    /**
     * @notice Sets the performance pool address
     * @param _performancePool New address for the performance pool contract
     * @dev Can only be called by admin. Allows setting the performance pool after deployment.
     */
    function setPerformancePool(address _performancePool) external onlyAdmin {
        if (_performancePool == address(0) && performancePoolSplit != 0) {
            revert InvalidSplits();
        }

        performancePool = _performancePool;

        emit PerformancePoolSet(_performancePool);
    }

    /**
     * @notice Sets the appchain pool address
     * @param _appchainPool New address for the appchain pool contract
     * @dev Can only be called by admin. Allows setting the appchain pool after deployment.
     */
    function setAppchainPool(address _appchainPool) external onlyAdmin {
        if (_appchainPool == address(0) && appchainPoolSplit != 0) {
            revert InvalidSplits();
        }

        appchainPool = _appchainPool;

        emit AppchainPoolSet(_appchainPool);
    }
}
