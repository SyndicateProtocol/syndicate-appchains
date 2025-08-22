// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IOPBridge {
    function depositERC20To(
        address _l1Token,
        address _l2Token,
        address _to,
        uint256 _amount,
        uint32 _minGasLimit,
        bytes calldata _extraData
    ) external;
}

interface IOPMessageRelayer {
    function sendMessage(address _target, bytes memory _message, uint32 _minGasLimit) external;
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
    /// @notice Admin address with privileged access to contract functions
    address public admin;

    /// @notice Minimum gas limit for Optimism operations (default: 200000)
    uint256 public minGasLimit;

    ////////////////////////////
    // Contracts Deployed on L1
    ////////////////////////////

    /// @notice The Optimism Bridge contract for token bridging operations
    address public immutable opBridge;

    /// @notice The Optimism Message Relayer contract for cross-chain messaging
    address public immutable opMessageRelayer;

    /// @notice The L1 token address that can be bridged to L2
    address public immutable l1Token;

    ////////////////////////////
    // Contracts Deployed on L2
    ////////////////////////////

    /// @notice The L2 token address corresponding to the L1 token
    address public immutable l2Token;

    /// @notice The L2 Relayer contract that receives bridged tokens and messages
    address public immutable l2Relayer;

    /**
     * @notice Error thrown when the contract has insufficient token balance for a relay operation
     * @param required The amount required for the operation
     * @param available The amount currently available
     */
    error InsufficientBalance();

    /**
     * @notice Error thrown when a non-admin address attempts to call admin-only functions
     * @param caller The address that attempted the call
     * @param admin The current admin address
     */
    error NotAdmin();

    /**
     * @notice Initializes the L1Relayer contract
     * @param _opBridge The address of the Optimism Bridge contract
     * @param _opMessageRelayer The address of the Optimism Message Relayer contract
     * @param _l1Token The address of the L1 ERC20 token to be relayed
     * @param _l2Token The address of the corresponding L2 token
     * @param _l2Relayer The address of the L2Relayer contract on L2
     * @dev Sets the deployer as admin and configures default gas settings
     *      Approves the bridge contract to spend L1 tokens on behalf of this contract
     */
    constructor(address _opBridge, address _opMessageRelayer, address _l1Token, address _l2Token, address _l2Relayer) {
        admin = msg.sender;
        setMinGasLimit(200000);

        opBridge = _opBridge;
        opMessageRelayer = _opMessageRelayer;
        l1Token = _l1Token;
        l2Token = _l2Token;
        l2Relayer = _l2Relayer;

        IERC20(l1Token).approve(opBridge, type(uint256).max);
    }

    /**
     * @notice Modifier that restricts function access to admin only
     * @dev Reverts with NotAdmin error if caller is not the admin
     */
    modifier onlyAdmin() {
        if (msg.sender != admin) revert NotAdmin();
        _;
    }

    /**
     * @notice Updates the admin address
     * @param _admin The new admin address
     * @dev Only callable by the current admin
     * @custom:security Critical function - ensure new admin is correct
     */
    function setAdmin(address _admin) external onlyAdmin {
        admin = _admin;
    }

    /**
     * @notice Updates the minimum gas limit for Optimism operations
     * @param _minGasLimit The new minimum gas limit for bridge and message operations
     * @dev Only callable by admin
     * @dev This setting affects the cost and reliability of cross-chain operations
     */
    function setMinGasLimit(uint256 _minGasLimit) external onlyAdmin {
        minGasLimit = _minGasLimit;
    }

    /**
     * @notice Relays tokens to L2 and sends a message to execute operations
     * @param amount The amount of tokens to relay
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the operation
     * @dev This function performs two operations:
     *      1. Deposits tokens to the Optimism bridge (L1 → L2)
     *      2. Sends a cross-chain message to trigger operations on L2
     * @dev Reverts if contract has insufficient token balance
     * @dev The L2Relayer contract must implement the relay function to handle the message
     */
    function relay(uint256 amount, address destination, uint256 epochIndex) external {
        if (IERC20(l1Token).balanceOf(address(this)) < amount) revert InsufficientBalance();

        _deposit(amount);
        _relay(amount, destination, epochIndex);
    }

    /**
     * @notice Internal function to deposit tokens to the Optimism bridge
     * @param amount The amount of tokens to deposit
     * @dev This is the first step in the relay process - bridges tokens from L1 to L2
     * @dev Tokens are sent to the L2Relayer contract on L2
     */
    function _deposit(uint256 amount) internal {
        IOPBridge(opBridge).depositERC20To(l1Token, l2Token, l2Relayer, amount, minGasLimit, bytes(""));
    }

    /**
     * @notice Internal function to relay the operation message to L2
     * @param amount The amount of tokens being relayed
     * @param destination The destination contract address on L2
     * @param epochIndex The epoch index for the operation
     * @dev Sends a cross-chain message to the L2Relayer contract
     * @dev The message contains the relay function call with parameters
     * @dev Uses the configured minimum gas limit for the message execution
     */
    function _relay(uint256 amount, address destination, uint256 epochIndex) internal {
        IOPMessageRelayer(opMessageRelayer).sendMessage(
            l2Relayer, abi.encodeWithSelector(this.relay.selector, amount, destination, epochIndex), minGasLimit
        );
    }
}
