// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IOPBridge {
    function depositERC20To(
        IERC20 _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _minGasLimit,
        bytes calldata _extraData
    ) external;
    function messenger() external view returns (IOPMessageRelayer);
}

interface IOPMessageRelayer {
    function sendMessage(address _target, bytes memory _message, uint32 _minGasLimit) external;
}

interface IL2Relayer {
    function relay(address destination, uint256 epochIndex) external;
}

/**
 * @title L1Relayer
 * @notice Contract for relaying operations from L1 to L2 chains using Optimism Bridge
 * @dev This contract handles the bridging of ERC20 tokens and message relaying from L1 to L2.
 *      It acts as a relayer that can deposit tokens to the Optimism bridge and send
 *      cross-chain messages to trigger operations on L2.
 *
 * Key features:
 * - Admin-controlled gas settings for bridge operations
 * - ERC20 token bridging through Optimism
 * - Cross-chain message relaying to L2
 * - Integration with L2Relayer for complete cross-chain operations
 */
contract L1Relayer {
    /// @notice Minimum gas limit for Optimism operations
    uint32 public immutable minGasLimit;

    ////////////////////////////
    // Contracts Deployed on L1
    ////////////////////////////

    /// @notice The Optimism Bridge contract for token bridging operations
    IOPBridge public immutable opBridge;

    /// @notice The Optimism Message Relayer contract for cross-chain messaging
    IOPMessageRelayer public immutable opMessageRelayer;

    /// @notice The L1 token address that can be bridged to L2
    IERC20 public immutable l1Token;

    ////////////////////////////
    // Contracts Deployed on L2
    ////////////////////////////

    /// @notice The L2 token address corresponding to the L1 token
    address public immutable l2Token;

    /// @notice The L2 Relayer contract that receives bridged tokens and messages
    address public immutable l2Relayer;

    /**
     * @notice Error thrown when the contract has insufficient token balance for a relay operation
     */
    error InsufficientBalance();

    /**
     * @notice Initializes the L1Relayer contract
     * @param _opBridge The address of the Optimism Standard Bridge contract
     * @param _l2Token The address of the L2 token
     * @param _l2Relayer The address of the L2Relayer contract on L2
     * @param _minGasLimit The gas limit for relayed transactions
     * @dev Sets the deployer as admin and configures default gas settings
     *      Approves the bridge contract to spend L1 tokens on behalf of this contract
     */
    constructor(address _opBridge, address _l1Token, address _l2Token, address _l2Relayer, uint32 _minGasLimit) {
        opBridge = IOPBridge(_opBridge);
        opMessageRelayer = opBridge.messenger();
        l1Token = IERC20(_l1Token);
        l2Token = _l2Token;
        l2Relayer = _l2Relayer;
        minGasLimit = _minGasLimit;
        if (minGasLimit == 0) {
            minGasLimit = 200_000;
        }

        l1Token.approve(_opBridge, type(uint256).max);
    }

    /**
     * @notice Relays tokens to L2 and sends a message to execute operations
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the operation
     * @dev This function performs two operations:
     *      1. Deposits tokens to the Optimism bridge (L1 → L2)
     *      2. Sends a cross-chain message to trigger operations on L2
     * @dev Reverts if contract has insufficient token balance
     * @dev The L2Relayer contract must implement the relay function to handle the message
     */
    function relay(address destination, uint256 epochIndex) external {
        uint256 amount = l1Token.balanceOf(address(this));
        if (amount == 0) revert InsufficientBalance();

        _deposit(amount);
        _relay(destination, epochIndex);
    }

    /**
     * @notice Internal function to deposit tokens to the Optimism bridge
     * @param amount The amount of tokens to deposit
     * @dev This is the first step in the relay process - bridges tokens from L1 to L2
     * @dev Tokens are sent to the L2Relayer contract on L2
     */
    function _deposit(uint256 amount) internal {
        opBridge.depositERC20To(l1Token, l2Token, l2Relayer, amount, minGasLimit, bytes(""));
    }

    /**
     * @notice Internal function to relay the operation message to L2
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the operation
     * @dev Sends a cross-chain message to the L2Relayer contract
     * @dev The message contains the relay function call with parameters
     * @dev Uses the configured minimum gas limit for the message execution
     */
    function _relay(address destination, uint256 epochIndex) internal {
        opMessageRelayer.sendMessage(
            l2Relayer, abi.encodeCall(IL2Relayer.relay, (destination, epochIndex)), minGasLimit
        );
    }
}
