// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {IL1Bridge} from "./interfaces/IL1Bridge.sol";

/// @title SyndicateForwarder
/// @notice L1 contract that forwards cross-chain messages to L2's SyndicateFactory
/// @dev This contract is deployed on L1 and uses a bridge adapter (Arbitrum, Optimism/Base, etc.)
///      to send messages to L2. The bridge adapter abstracts the differences between L2 implementations.
contract SyndicateForwarder is AccessControl, Pausable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing pause functionality and forwarding messages
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice The bridge adapter for sending L1→L2 messages
    IL1Bridge public bridge;

    /// @notice The L2 SyndicateFactory address that will receive messages
    address public l2Target;

    /// @notice Default gas limit for L2 execution
    uint256 public defaultGasLimit;

    /// @notice Default max fee per gas for L2
    uint256 public defaultMaxFeePerGas;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /*//////////////////////////////////////////////////////////////
                             EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a message is forwarded to L2
    /// @param sender The address that initiated the forward
    /// @param l2Target The L2 target address
    /// @param chainId The chain ID being created
    /// @param messageId The bridge message ID (retryable ticket ID for Arbitrum)
    event MessageForwarded(
        address indexed sender, address indexed l2Target, uint256 indexed chainId, uint256 messageId
    );

    /// @notice Emitted when the L2 target address is updated
    /// @param oldTarget The previous L2 target address
    /// @param newTarget The new L2 target address
    event L2TargetUpdated(address indexed oldTarget, address indexed newTarget);

    /// @notice Emitted when the bridge address is updated
    /// @param oldBridge The previous bridge address
    /// @param newBridge The new bridge address
    event BridgeUpdated(address indexed oldBridge, address indexed newBridge);

    /// @notice Emitted when gas parameters are updated
    /// @param gasLimit The new gas limit
    /// @param maxFeePerGas The new max fee per gas
    event GasParametersUpdated(uint256 gasLimit, uint256 maxFeePerGas);

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Initializes the SyndicateForwarder
    /// @param _admin The admin address that will have DEFAULT_ADMIN_ROLE
    /// @param _manager The manager address that will have MANAGER_ROLE
    /// @param _bridge The bridge adapter address (ArbitrumL1Bridge or OptimismL1Bridge)
    /// @param _l2Target The L2 SyndicateFactory address
    /// @param _defaultGasLimit Default gas limit for L2 execution
    /// @param _defaultMaxFeePerGas Default max fee per gas for L2
    constructor(
        address _admin,
        address _manager,
        address _bridge,
        address _l2Target,
        uint256 _defaultGasLimit,
        uint256 _defaultMaxFeePerGas
    ) {
        if (_admin == address(0)) revert ZeroAddress();
        if (_manager == address(0)) revert ZeroAddress();
        if (_bridge == address(0)) revert ZeroAddress();
        if (_l2Target == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, _admin);
        _grantRole(MANAGER_ROLE, _manager);

        bridge = IL1Bridge(_bridge);
        l2Target = _l2Target;
        defaultGasLimit = _defaultGasLimit;
        defaultMaxFeePerGas = _defaultMaxFeePerGas;
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Forwards a chain creation request to L2's SyndicateFactory
    /// @dev Uses the configured bridge adapter to send cross-chain message
    ///      For Arbitrum: Uses retryable tickets, requires ETH for gas
    ///      For Optimism/Base: Uses CrossDomainMessenger, different fee model
    ///      Only manager can call this function
    /// @param chainId The chain ID to create
    /// @param admin The admin address for the new chain
    /// @param permissionModule The permission module address
    /// @return messageId The bridge message ID (implementation-specific)
    function forwardCreateChain(uint256 chainId, address admin, address permissionModule)
        external
        payable
        onlyRole(MANAGER_ROLE)
        whenNotPaused
        returns (uint256 messageId)
    {
        // Encode the call to SyndicateFactory.createFromForwarder
        bytes memory data =
            abi.encodeWithSignature("createFromForwarder(uint256,address,address)", chainId, admin, permissionModule);

        // Send message via bridge adapter
        // msg.value is forwarded to handle different fee models (Arbitrum requires ETH upfront)
        messageId = bridge.sendMessage{value: msg.value}(l2Target, data, defaultGasLimit, defaultMaxFeePerGas);

        emit MessageForwarded(msg.sender, l2Target, chainId, messageId);

        return messageId;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Update the L2 target address (admin only)
    /// @param newTarget The new L2 target address
    function setL2Target(address newTarget) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newTarget == address(0)) revert ZeroAddress();

        address oldTarget = l2Target;
        l2Target = newTarget;

        emit L2TargetUpdated(oldTarget, newTarget);
    }

    /// @notice Update the bridge adapter address (admin only)
    /// @dev This allows switching between different bridge implementations
    /// @param newBridge The new bridge adapter address
    function setBridge(address newBridge) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newBridge == address(0)) revert ZeroAddress();

        address oldBridge = address(bridge);
        bridge = IL1Bridge(newBridge);

        emit BridgeUpdated(oldBridge, newBridge);
    }

    /// @notice Update gas parameters (admin only)
    /// @param gasLimit The new gas limit
    /// @param maxFeePerGas The new max fee per gas
    function setGasParameters(uint256 gasLimit, uint256 maxFeePerGas) external onlyRole(DEFAULT_ADMIN_ROLE) {
        defaultGasLimit = gasLimit;
        defaultMaxFeePerGas = maxFeePerGas;

        emit GasParametersUpdated(gasLimit, maxFeePerGas);
    }

    /// @notice Pause the forwarder (manager only)
    function pause() external onlyRole(MANAGER_ROLE) {
        _pause();
    }

    /// @notice Unpause the forwarder (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
