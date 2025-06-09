// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "../SyndicateSequencingChain.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/// @title SyndicateFactory
/// @notice Factory contract for creating SyndicateSequencingChain contracts
/// @dev Uses CREATE2 pattern for deterministic deployments - users deploy permission modules separately
contract SyndicateFactory is AccessControl, Pausable {
    /// @notice Emitted when a new SyndicateSequencingChain is created
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    /// @notice Emitted when namespace configuration is updated
    event NamespaceConfigUpdated(uint256 oldNamespacePrefix, uint256 newNamespacePrefix);

    /// @notice Emitted when a chain ID is manually marked as used
    event ChainIdManuallyMarked(uint256 indexed chainId);

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    error ZeroAddress();
    error ZeroValue();
    error ChainIdAlreadyExists();

    // Namespace configuration - made public for frontend access
    uint256 public namespacePrefix;
    uint256 public nextAutoChainId;
    mapping(uint256 => bool) public usedChainIds;

    constructor(address admin) {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);

        namespacePrefix = 510;
        nextAutoChainId = 1;
    }

    /// @notice Creates a new SyndicateSequencingChain contract
    /// @param appchainId The app chain ID (0 for auto-increment)
    /// @param admin The admin address for the new chain
    /// @param permissionModule The pre-deployed permission module
    /// @param salt The salt for CREATE2 deployment
    /// @return sequencingChain The deployed sequencing chain address
    /// @return actualChainId The chain ID that was used
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencingChain(
        uint256 appchainId,
        address admin,
        IRequirementModule permissionModule,
        bytes32 salt
    ) external whenNotPaused returns (address sequencingChain, uint256 actualChainId) {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }

        // Determine actual chain ID
        actualChainId = appchainId == 0 ? getNextChainId() : appchainId;

        // Validate chain ID is not already used
        if (usedChainIds[actualChainId]) {
            revert ChainIdAlreadyExists();
        }

        // Reserve the chain ID
        usedChainIds[actualChainId] = true;
        if (appchainId == 0) {
            nextAutoChainId++;
        }

        // Deploy the sequencing chain using CREATE2
        bytes memory bytecode = getBytecode(actualChainId);
        sequencingChain = Create2.deploy(0, salt, bytecode);

        // Initialize the contract
        SyndicateSequencingChain(sequencingChain).initialize(admin, address(permissionModule));

        emit SyndicateSequencingChainCreated(actualChainId, sequencingChain, address(permissionModule));

        return (sequencingChain, actualChainId);
    }

    /// @notice Computes the address where a sequencing chain will be deployed
    /// @param salt The salt for CREATE2 deployment
    /// @param chainId The chain ID
    /// @return The computed address
    function computeSequencingChainAddress(bytes32 salt, uint256 chainId) external view returns (address) {
        return Create2.computeAddress(salt, keccak256(getBytecode(chainId)));
    }

    /// @notice Returns the bytecode for deploying a SyndicateSequencingChain
    /// @param chainId The chain ID
    /// @return The bytecode with constructor parameters
    function getBytecode(uint256 chainId) public pure returns (bytes memory) {
        return abi.encodePacked(type(SyndicateSequencingChain).creationCode, abi.encode(chainId));
    }

    /// @notice Get the next auto-generated chain ID
    /// @dev Chain ID calculation: concatenates namespacePrefix with nextAutoChainId
    /// @dev Example: with namespacePrefix=510 and nextAutoChainId=1, result is 5101
    /// @dev Example: with namespacePrefix=510 and nextAutoChainId=1000, result is 5101000
    /// @return The next available chain ID in the namespace
    function getNextChainId() public view returns (uint256) {
        // Concatenate namespace prefix with auto chain ID
        // This ensures we always stay within the 510 namespace
        string memory prefixStr = Strings.toString(namespacePrefix);
        string memory chainIdStr = Strings.toString(nextAutoChainId);
        //#olympix-ignore-abi-encode-packed-dynamic-types
        string memory combined = string(abi.encodePacked(prefixStr, chainIdStr));
        return Strings.parseUint(combined);
    }

    /// @notice Check if a chain ID has been used
    /// @param chainId The chain ID to check
    /// @return 1 if used, 0 if available
    function isChainIdUsed(uint256 chainId) external view returns (uint256) {
        return usedChainIds[chainId] ? 1 : 0;
    }

    /// @notice Manually mark a chain ID as used to reserve it
    /// @param chainId The chain ID to mark as used
    /// @dev Useful for reserving specific chain IDs or marking externally deployed chains
    function markChainIdAsUsed(uint256 chainId) external onlyRole(MANAGER_ROLE) {
        if (usedChainIds[chainId]) {
            revert ChainIdAlreadyExists();
        }
        usedChainIds[chainId] = true;
        emit ChainIdManuallyMarked(chainId);
    }

    /// @notice Update namespace configuration (manager only)
    /// @param newPrefix The new namespace prefix
    function updateNamespaceConfig(uint256 newPrefix) external onlyRole(MANAGER_ROLE) {
        uint256 oldPrefix = namespacePrefix;
        namespacePrefix = newPrefix;
        emit NamespaceConfigUpdated(oldPrefix, newPrefix);
    }

    /// @notice Pause the factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
