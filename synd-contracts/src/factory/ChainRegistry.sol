// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {SyndicateForwarder} from "./SyndicateForwarder.sol";

/// @title ChainRegistry
/// @notice Registry contract for managing chain registrations on L1
/// @dev This contract serves as the source of truth for chain registrations and forwards
///      deployment requests to the SyndicateFactory on L2 via the SyndicateForwarder
contract ChainRegistry is AccessControl, Pausable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing chain registrations and pausing
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    /// @notice Address of the SyndicateForwarder contract that handles cross-chain messages
    SyndicateForwarder public syndicateForwarder;

    /// @notice Mapping of chain IDs to their registration status
    mapping(uint256 chainId => bool isRegistered) public registeredChains;

    /// @notice Mapping of addresses to their nonces for chain ID generation
    mapping(address sender => uint256 nonce) public nonces;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when attempting to register a chain with an already used chain ID
    error ChainIdAlreadyRegistered();

    /// @notice Thrown when the forwarder is not set
    error ForwarderNotSet();

    /*//////////////////////////////////////////////////////////////
                             EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a new chain is registered
    /// @param sender The address that requested the registration
    /// @param chainId The unique identifier for the chain
    /// @param admin The admin address for the new chain
    /// @param permissionModule The permission module address
    /// @param ticketId The Arbitrum retryable ticket ID
    event ChainRegistered(
        address indexed sender,
        uint256 indexed chainId,
        address indexed admin,
        address permissionModule,
        uint256 ticketId
    );

    /// @notice Emitted when a deterministic chainID is generated for a user
    /// @param sender The address that requested the chain ID generation
    /// @param nonce The nonce used in the chain ID generation
    /// @param chainId The resulting deterministic chain ID
    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    /// @notice Emitted when the forwarder address is updated
    /// @param oldForwarder The previous forwarder address
    /// @param newForwarder The new forwarder address
    event ForwarderUpdated(address indexed oldForwarder, address indexed newForwarder);

    /*//////////////////////////////////////////////////////////////
                            CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Initializes the ChainRegistry
    /// @param _admin The admin address that will have DEFAULT_ADMIN_ROLE
    /// @param _manager The manager address that will have MANAGER_ROLE
    /// @param _syndicateForwarder The address of the SyndicateForwarder contract
    constructor(address _admin, address _manager, address _syndicateForwarder) {
        if (_admin == address(0) || _manager == address(0) || _syndicateForwarder == address(0)) {
            revert ZeroAddress();
        }

        _grantRole(DEFAULT_ADMIN_ROLE, _admin);
        _grantRole(MANAGER_ROLE, _manager);

        syndicateForwarder = SyndicateForwarder(_syndicateForwarder);
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Registers a new chain with deterministic chainID
    /// @param admin The admin address for the new chain on L2
    /// @param permissionModule The pre-deployed permission module
    /// @return chainId The chain ID that was registered
    function registerChain(address admin, IRequirementModule permissionModule)
        external
        payable
        whenNotPaused
        returns (uint256 chainId)
    {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }
        if (address(syndicateForwarder) == address(0)) revert ForwarderNotSet();

        // Get and increment nonce for this sender
        uint256 nonce = nonces[msg.sender]++;

        // Generate deterministic chainID
        chainId = generateDeterministicChainId(msg.sender, nonce);

        // Validate chain ID is not already registered
        if (registeredChains[chainId]) {
            revert ChainIdAlreadyRegistered();
        }

        // Mark chain as registered
        registeredChains[chainId] = true;

        // Emit deterministic chain ID generation event
        emit DeterministicChainIdGenerated(msg.sender, nonce, chainId);

        // Forward the deployment request to L2 via the forwarder
        uint256 ticketId =
            syndicateForwarder.forwardCreateChain{value: msg.value}(chainId, admin, address(permissionModule));

        emit ChainRegistered(msg.sender, chainId, admin, address(permissionModule), ticketId);

        return chainId;
    }

    /// @notice Registers a chain with a custom chainID (manager only)
    /// @param customChainId The custom chain ID to use
    /// @param admin The admin address for the new chain on L2
    /// @param permissionModule The pre-deployed permission module
    /// @return chainId The chain ID that was registered (same as customChainId)
    function registerChainWithCustomId(uint256 customChainId, address admin, IRequirementModule permissionModule)
        external
        payable
        onlyRole(MANAGER_ROLE)
        whenNotPaused
        returns (uint256 chainId)
    {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }
        if (customChainId == 0) {
            revert ZeroAddress(); // Reusing this error for zero chainID
        }
        if (address(syndicateForwarder) == address(0)) revert ForwarderNotSet();

        // Validate chain ID is not already registered
        if (registeredChains[customChainId]) {
            revert ChainIdAlreadyRegistered();
        }

        // Mark chain as registered
        registeredChains[customChainId] = true;

        // Forward the deployment request to L2 via the forwarder
        uint256 ticketId =
            syndicateForwarder.forwardCreateChain{value: msg.value}(customChainId, admin, address(permissionModule));

        emit ChainRegistered(msg.sender, customChainId, admin, address(permissionModule), ticketId);

        return customChainId;
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Generate deterministic chainID from sender address and nonce
    /// @param sender The sender address
    /// @param nonce The nonce for this sender
    /// @return chainId The deterministic chain ID
    function generateDeterministicChainId(address sender, uint256 nonce) public pure returns (uint256 chainId) {
        // Use keccak256 hash of sender + nonce, then take modulo to keep within reasonable range
        bytes32 hash = keccak256(abi.encodePacked(sender, nonce));
        // Use modulo to keep chainId in a reasonable range
        chainId = uint256(hash) % (10 ** 18); // Max 18 digits
        // Ensure chainID is never 0 as this is used as a null value indicator
        if (chainId == 0) {
            chainId = 1;
        }
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Update the forwarder address (admin only)
    /// @param newForwarder The new forwarder address
    function setForwarder(address newForwarder) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newForwarder == address(0)) revert ZeroAddress();

        address oldForwarder = address(syndicateForwarder);
        syndicateForwarder = SyndicateForwarder(newForwarder);

        emit ForwarderUpdated(oldForwarder, newForwarder);
    }

    /// @notice Pause the registry (manager only)
    function pause() external onlyRole(MANAGER_ROLE) {
        _pause();
    }

    /// @notice Unpause the registry (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
