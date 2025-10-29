// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

/// @title SyndicateForwarder
/// @notice Forwarder contract that handles cross-chain message forwarding from L1 to L2
/// @dev This contract receives calls from ChainRegistry on L1 and forwards them to SyndicateFactory on L2
///      It handles address aliasing for cross-chain communication
contract SyndicateForwarder is AccessControl, Pausable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing pause functionality
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice The L2 destination address (SyndicateFactory on L2)
    address public destination;

    /// @notice The expected sender address (aliased ChainRegistry from L1)
    address public expectedSender;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when the caller is not the expected sender
    error UnauthorizedSender();

    /// @notice Thrown when a forwarded call fails
    error ForwardCallFailed();

    /// @notice Thrown when the destination is not set
    error DestinationNotSet();

    /*//////////////////////////////////////////////////////////////
                             EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a call is forwarded to the destination
    /// @param sender The address that initiated the forward
    /// @param dest The destination address
    /// @param data The calldata that was forwarded
    /// @param value The ETH value sent with the call
    event CallForwarded(address indexed sender, address indexed dest, bytes data, uint256 value);

    /// @notice Emitted when the destination address is updated
    /// @param oldDestination The previous destination address
    /// @param newDestination The new destination address
    event DestinationUpdated(address indexed oldDestination, address indexed newDestination);

    /// @notice Emitted when the expected sender is updated
    /// @param oldSender The previous expected sender
    /// @param newSender The new expected sender
    event ExpectedSenderUpdated(address indexed oldSender, address indexed newSender);

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Initializes the SyndicateForwarder
    /// @param _admin The admin address that will have DEFAULT_ADMIN_ROLE
    /// @param _manager The manager address that will have MANAGER_ROLE
    /// @param _destination The address of the SyndicateFactory on L2
    /// @param _expectedSender The expected sender address (aliased ChainRegistry from L1)
    constructor(address _admin, address _manager, address _destination, address _expectedSender) {
        if (_admin == address(0)) revert ZeroAddress();
        if (_manager == address(0)) revert ZeroAddress();
        if (_destination == address(0)) revert ZeroAddress();
        if (_expectedSender == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, _admin);
        _grantRole(MANAGER_ROLE, _manager);

        destination = _destination;
        expectedSender = _expectedSender;
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Forwards a call to the destination address
    /// @dev Only callable by the expected sender (aliased ChainRegistry address)
    /// @param data The calldata to forward
    function forward(bytes calldata data) external payable whenNotPaused {
        // Verify sender is authorized
        if (msg.sender != expectedSender) {
            revert UnauthorizedSender();
        }

        if (destination == address(0)) {
            revert DestinationNotSet();
        }

        // Forward the call to the destination
        (bool success,) = payable(destination).call{value: msg.value}(data);
        if (!success) {
            revert ForwardCallFailed();
        }

        emit CallForwarded(msg.sender, destination, data, msg.value);
    }

    /// @notice Generic call function for forwarding arbitrary calls
    /// @dev Only callable by the expected sender
    /// @param dest The destination address for the call
    /// @param data The calldata to send
    function call(address dest, bytes calldata data) external payable whenNotPaused {
        // Verify sender is authorized
        if (msg.sender != expectedSender) {
            revert UnauthorizedSender();
        }

        if (dest == address(0)) {
            revert ZeroAddress();
        }

        // Execute the call
        (bool success,) = payable(dest).call{value: msg.value}(data);
        if (!success) {
            revert ForwardCallFailed();
        }

        emit CallForwarded(msg.sender, dest, data, msg.value);
    }

    /// @notice Convenience function to forward chain creation with custom ID
    /// @dev This wraps the SyndicateFactory.createFromForwarder call
    /// @param chainId The chain ID to create
    /// @param admin The admin address for the new chain
    /// @param permissionModule The permission module address
    function forwardCreateChain(uint256 chainId, address admin, address permissionModule)
        external
        whenNotPaused
        returns (address sequencingChain)
    {
        // Verify sender is authorized
        if (msg.sender != expectedSender) {
            revert UnauthorizedSender();
        }

        if (destination == address(0)) {
            revert DestinationNotSet();
        }

        // Encode the call to SyndicateFactory.createFromForwarder
        bytes memory data =
            abi.encodeWithSignature("createFromForwarder(uint256,address,address)", chainId, admin, permissionModule);

        // Forward the call
        (bool success, bytes memory returnData) = destination.call(data);
        if (!success) {
            revert ForwardCallFailed();
        }

        // Decode the return value
        sequencingChain = abi.decode(returnData, (address));

        emit CallForwarded(msg.sender, destination, data, 0);

        return sequencingChain;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Update the destination address (admin only)
    /// @param newDestination The new destination address
    function setDestination(address newDestination) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newDestination == address(0)) revert ZeroAddress();

        address oldDestination = destination;
        destination = newDestination;

        emit DestinationUpdated(oldDestination, newDestination);
    }

    /// @notice Update the expected sender address (admin only)
    /// @param newSender The new expected sender address
    function setExpectedSender(address newSender) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newSender == address(0)) revert ZeroAddress();

        address oldSender = expectedSender;
        expectedSender = newSender;

        emit ExpectedSenderUpdated(oldSender, newSender);
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
