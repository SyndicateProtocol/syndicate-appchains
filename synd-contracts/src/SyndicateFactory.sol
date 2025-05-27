// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "./SyndicateSequencingChain.sol";
import {IRequirementModule} from "./interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/// @title SyndicateFactory
/// @notice Factory contract for creating SyndicateSequencingChain contracts
/// @dev Simplified version - users deploy permission modules separately
contract SyndicateFactory is AccessControl {
    /// @notice Emitted when a new SyndicateSequencingChain is created
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    /// @notice Emitted when namespace configuration is updated
    event NamespaceConfigUpdated(
        uint256 oldNamespacePrefix,
        uint256 oldNamespaceMultiplier,
        uint256 newNamespacePrefix,
        uint256 newNamespaceMultiplier
    );

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    error ZeroAddress();
    error ZeroValue();
    error ReservedNamespace();
    error ChainIdAlreadyExists();

    // Namespace configuration
    uint256 private _namespacePrefix;
    uint256 private _namespaceMultiplier;
    uint256 private _nextAutoChainId;
    mapping(uint256 => bool) private _usedChainIds;

    constructor(address admin) {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);

        _namespacePrefix = 510;
        _namespaceMultiplier = 1000;
        _nextAutoChainId = 1;
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
    ) external returns (address sequencingChain, uint256 actualChainId) {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }

        // Determine actual chain ID
        actualChainId = appchainId == 0 ? _getNextChainId() : appchainId;

        // Validate chain ID
        if (appchainId != 0 && actualChainId / _namespaceMultiplier == _namespacePrefix) {
            revert ReservedNamespace();
        }
        if (_usedChainIds[actualChainId]) {
            revert ChainIdAlreadyExists();
        }

        // Reserve the chain ID
        _usedChainIds[actualChainId] = true;
        if (appchainId == 0) {
            _nextAutoChainId++;
        }

        // Deploy the sequencing chain
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
    /// @return The next available chain ID in the namespace
    function getNextAutoChainId() external view returns (uint256) {
        return _getNextChainId();
    }

    /// @notice Check if a chain ID has been used
    /// @param chainId The chain ID to check
    /// @return 1 if used, 0 if available
    function isChainIdUsed(uint256 chainId) external view returns (uint256) {
        return _usedChainIds[chainId] ? 1 : 0;
    }

    /// @notice Get the namespace prefix
    function namespacePrefix() external view returns (uint256) {
        return _namespacePrefix;
    }

    /// @notice Get the namespace multiplier
    function namespaceMultiplier() external view returns (uint256) {
        return _namespaceMultiplier;
    }

    /// @notice Update namespace configuration (manager only)
    /// @param newPrefix The new namespace prefix
    /// @param newMultiplier The new namespace multiplier
    function updateNamespaceConfig(uint256 newPrefix, uint256 newMultiplier) external onlyRole(MANAGER_ROLE) {
        uint256 oldPrefix = _namespacePrefix;
        uint256 oldMultiplier = _namespaceMultiplier;

        _namespacePrefix = newPrefix;
        _namespaceMultiplier = newMultiplier;

        emit NamespaceConfigUpdated(oldPrefix, oldMultiplier, newPrefix, newMultiplier);
    }

    /// @notice Internal function to get the next chain ID in namespace
    function _getNextChainId() internal view returns (uint256) {
        return _namespacePrefix * _namespaceMultiplier + _nextAutoChainId;
    }
}
