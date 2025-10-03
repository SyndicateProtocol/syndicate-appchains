// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IPool} from "./interfaces/IPool.sol";

interface IArbBridge {
    function unsafeCreateRetryableTicket(
        address to,
        uint256 l2CallValue,
        uint256 maxSubmissionCost,
        address excessFeeRefundAddress,
        address callValueRefundAddress,
        uint256 gasLimit,
        uint256 maxFeePerGas,
        uint256 tokenTotalFeeAmount,
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
 * - ERC20 token bridging and contract callthrough Arbitrum with retryable ticket
 *
 * @custom:security This contract has admin controls and should be used with caution
 */
contract L2Relayer is Ownable(msg.sender) {
    /// @notice Minimum gas limit for Arbitrum Bridge operations
    uint256 public gasLimit = 200000;

    /// @notice Maximum gas price for Arbitrum Bridge operations
    uint256 public maxFeePerGas = 1 gwei;

    ////////////////////////////
    // Contracts Deployed on L2
    ////////////////////////////

    /// @notice The Arbitrum Bridge contract address for cross-chain operations
    IArbBridge public immutable arbBridge;

    /// @notice The L2 token address that can be bridged and relayed
    IERC20 public immutable tokenAddress;

    ////////////////////////////
    // Contracts Deployed on L3
    ////////////////////////////

    /// @notice The refunder contract address on L3 that handles excess funds
    address public immutable refunder;

    /**
     * @notice Error thrown when the contract has insufficient token balance for a relay operation
     */
    error InsufficientBalance();

    /**
     * @notice Initializes the L2Relayer contract
     * @param _arbBridge The address of the Arbitrum Bridge contract
     * @param _tokenAddress The address of the ERC20 token to be relayed
     * @param _refunder The address of the refunder contract on L3
     * @dev Sets the deployer as admin and configures default gas settings
     *      Approves the bridge contract to spend tokens on behalf of this contract
     */
    constructor(address _arbBridge, address _tokenAddress, address _refunder) {
        arbBridge = IArbBridge(_arbBridge);
        tokenAddress = IERC20(_tokenAddress);
        refunder = _refunder;

        tokenAddress.approve(_arbBridge, type(uint256).max);
    }

    /**
     * @notice Updates the gas settings for bridge operations
     * @param _gasLimit The new gas limit for bridge transactions
     * @param _maxFeePerGas The new maximum fee per gas for bridge transactions
     * @dev Only callable by admin
     * @dev These settings affect the cost and reliability of bridge operations
     */
    function setGasSettings(uint256 _gasLimit, uint256 _maxFeePerGas) external onlyOwner {
        gasLimit = _gasLimit;
        maxFeePerGas = _maxFeePerGas;
    }

    /**
     * @notice Relays tokens to the destination chain and executes a deposit operation
     * @param destination The destination contract address on L3
     * @param epochIndex The epoch index for the deposit operation
     * @dev This function gets the current balance of the token and creates a retryable ticket to deposit and send to a pool contract on the L3
     * @dev The relay will revert if the gas cost is greater than the amount being relayed
     * @dev The retryable ticket is created with the refunder address as the call value refund address
     * @dev Sends a contract transaction through the bridge to execute IPool.deposit
     * @dev Uses the configured gas settings for the bridge transaction
     */
    function relay(address destination, uint256 epochIndex) external {
        uint256 amount = tokenAddress.balanceOf(address(this));
        uint256 gasCost = gasLimit * maxFeePerGas;

        // Validate that the gas cost is less than the amount being relayed
        require(gasCost < amount, InsufficientBalance());

        // We use unsafe so the refunder address doesn't get aliased
        arbBridge.unsafeCreateRetryableTicket(
            destination,
            amount - gasCost,
            0, // Always 0 for custom gas token chains
            refunder,
            refunder,
            gasLimit,
            maxFeePerGas,
            amount,
            abi.encodeCall(IPool.deposit, (epochIndex))
        );
    }
}
