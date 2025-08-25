// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IPool} from "./IPool.sol";

interface IArbBridge {
    function depositERC20(uint256 amount) external returns (uint256);
    function sendContractTransaction(
        uint256 gasLimit,
        uint256 maxFeePerGas,
        address to,
        uint256 value,
        bytes calldata data
    ) external returns (uint256);
}

/**
 * @title L2Relayer
 * @notice Contract for relaying operations between L1 and L2 chains using Arbitrum Bridge
 * @dev This contract handles the bridging of ERC20 tokens and execution of contract calls
 *      on the destination chain. It acts as a relayer that can deposit tokens to the
 *      Arbitrum bridge and trigger contract interactions on L2.
 *
 * Key features:
 * - Admin-controlled gas settings for bridge operations
 * - ERC20 token bridging through Arbitrum
 * - Contract transaction execution on destination chain
 * - Batch operations for staking and other L2 interactions
 *
 * @custom:security This contract has admin controls and should be used with caution
 */
contract L2Relayer is AccessControl {
    /// @notice Minimum gas limit for Arbitrum Bridge operations (default: 210000)
    uint256 public gasLimit;

    /// @notice Maximum fee per gas for Arbitrum Bridge operations (default: 1 gwei)
    uint256 public maxFeePerGas;

    ////////////////////////////
    // Contracts Deployed on L2
    ////////////////////////////

    /// @notice The Arbitrum Bridge contract address for cross-chain operations
    address public immutable arbBridge;

    /// @notice The L2 token address that can be bridged and relayed
    address public immutable tokenAddress;

    /**
     * @notice Error thrown when the contract has insufficient token balance for a relay operation
     */
    error InsufficientBalance();

    /**
     * @notice Initializes the L2Relayer contract
     * @param _arbBridge The address of the Arbitrum Bridge contract
     * @param _tokenAddress The address of the ERC20 token to be relayed
     * @param _defaultAdmin The address of the default admin
     * @dev Sets the deployer as admin and configures default gas settings
     *      Approves the bridge contract to spend tokens on behalf of this contract
     */
    constructor(address _arbBridge, address _tokenAddress, address _defaultAdmin) {
        _grantRole(DEFAULT_ADMIN_ROLE, _defaultAdmin);
        gasLimit = 210000;
        maxFeePerGas = 1 gwei;

        arbBridge = _arbBridge;
        tokenAddress = _tokenAddress;

        IERC20(tokenAddress).approve(arbBridge, type(uint256).max);
    }

    /**
     * @notice Updates the gas settings for bridge operations
     * @param _gasLimit The new gas limit for bridge transactions
     * @param _maxFeePerGas The new maximum fee per gas for bridge transactions
     * @dev Only callable by admin
     * @dev These settings affect the cost and reliability of bridge operations
     */
    function setGasSettings(uint256 _gasLimit, uint256 _maxFeePerGas) external onlyRole(DEFAULT_ADMIN_ROLE) {
        gasLimit = _gasLimit;
        maxFeePerGas = _maxFeePerGas;
    }

    /**
     * @notice Relays tokens to the destination chain and executes a deposit operation
     * @param amount The amount of tokens to relay
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the deposit operation
     * @dev This function performs two operations:
     *      1. Deposits tokens to the Arbitrum bridge
     *      2. Sends a contract transaction to execute deposit on destination
     * @dev Reverts if contract has insufficient token balance
     * @dev The destination contract must implement IPool.deposit function
     */
    function relay(uint256 amount, address destination, uint256 epochIndex) external {
        if (IERC20(tokenAddress).balanceOf(address(this)) < amount) revert InsufficientBalance();

        _deposit(amount);
        _relay(amount, destination, epochIndex);
    }

    /**
     * @notice Internal function to deposit tokens to the Arbitrum bridge
     * @param amount The amount of tokens to deposit
     * @dev This is the first step in the relay process
     */
    function _deposit(uint256 amount) internal {
        IArbBridge(arbBridge).depositERC20(amount);
    }

    /**
     * @notice Internal function to relay the operation to the destination contract
     * @param amount The amount of tokens being relayed
     * @param destination The destination contract address
     * @param epochIndex The epoch index for the deposit operation
     * @dev Sends a contract transaction through the bridge to execute IPool.deposit
     * @dev Uses the configured gas settings for the bridge transaction
     */
    function _relay(uint256 amount, address destination, uint256 epochIndex) internal {
        IArbBridge(arbBridge).sendContractTransaction(
            gasLimit, maxFeePerGas, destination, amount, abi.encodeWithSelector(IPool.deposit.selector, epochIndex)
        );
    }
}
