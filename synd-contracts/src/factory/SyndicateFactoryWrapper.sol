// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateFactory} from "./SyndicateFactory.sol";
import {RequireAndModuleFactory, RequireOrModuleFactory} from "./PermissionModuleFactories.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

/// @title SyndicateFactoryWrapper
/// @notice Wrapper factory that deploys both permission modules and sequencing chains together
/// @dev Combines functionality from individual factories for a complete deployment experience
contract SyndicateFactoryWrapper is AccessControl, Pausable {
    /// @notice Emitted when a complete syndicate deployment is created
    event CompleteSyndicateDeployed(
        uint256 indexed chainId,
        address indexed sequencingChain,
        address indexed permissionModule,
        ModuleType moduleType,
        address admin
    );

    /// @notice Types of permission modules supported
    enum ModuleType {
        RequireAnd,
        RequireOr
    }

    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    error ZeroAddress();
    error InvalidModuleType();

    // Factory contract references
    SyndicateFactory public immutable syndicateFactory;
    RequireAndModuleFactory public immutable requireAndFactory;
    RequireOrModuleFactory public immutable requireOrFactory;

    constructor(address admin, address _syndicateFactory, address _requireAndFactory, address _requireOrFactory) {
        if (
            admin == address(0) || _syndicateFactory == address(0) || _requireAndFactory == address(0)
                || _requireOrFactory == address(0)
        ) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);

        syndicateFactory = SyndicateFactory(_syndicateFactory);
        requireAndFactory = RequireAndModuleFactory(_requireAndFactory);
        requireOrFactory = RequireOrModuleFactory(_requireOrFactory);
    }

    /// @notice Deploy a complete syndicate with permission module and sequencing chain
    /// @param appchainId The app chain ID (0 for auto-increment)
    /// @param admin The admin address for both contracts
    /// @param moduleType The type of permission module to deploy (And or Or)
    /// @param moduleSalt The salt for the permission module deployment
    /// @param chainSalt The salt for the sequencing chain deployment
    /// @return sequencingChain The deployed sequencing chain address
    /// @return permissionModule The deployed permission module address
    /// @return actualChainId The chain ID that was used
    //#olympix-ignore-reentrancy-events
    function deployCompleteSyndicate(
        uint256 appchainId,
        address admin,
        ModuleType moduleType,
        bytes32 moduleSalt,
        bytes32 chainSalt
    ) public whenNotPaused returns (address sequencingChain, address permissionModule, uint256 actualChainId) {
        if (admin == address(0)) revert ZeroAddress();

        // Deploy the appropriate permission module
        if (moduleType == ModuleType.RequireAnd) {
            permissionModule = requireAndFactory.createRequireAndModule(admin, moduleSalt);
        } else if (moduleType == ModuleType.RequireOr) {
            permissionModule = requireOrFactory.createRequireOrModule(admin, moduleSalt);
        } else {
            revert InvalidModuleType();
        }

        // Deploy the sequencing chain with the permission module
        (sequencingChain, actualChainId) = syndicateFactory.createSyndicateSequencingChain(
            appchainId, admin, IRequirementModule(permissionModule), chainSalt
        );

        emit CompleteSyndicateDeployed(actualChainId, sequencingChain, permissionModule, moduleType, admin);

        return (sequencingChain, permissionModule, actualChainId);
    }

    /// @notice Compute addresses for a complete syndicate deployment
    /// @param admin The admin address for the module
    /// @param moduleType The type of permission module
    /// @param moduleSalt The salt for the permission module
    /// @param chainSalt The salt for the sequencing chain
    /// @param chainId The chain ID for the sequencing chain
    /// @return permissionModuleAddress The computed permission module address
    /// @return sequencingChainAddress The computed sequencing chain address
    function computeCompleteSyndicateAddresses(
        address admin,
        ModuleType moduleType,
        bytes32 moduleSalt,
        bytes32 chainSalt,
        uint256 chainId
    ) external view returns (address permissionModuleAddress, address sequencingChainAddress) {
        // Compute permission module address
        if (moduleType == ModuleType.RequireAnd) {
            permissionModuleAddress = requireAndFactory.computeModuleAddress(admin, moduleSalt);
        } else if (moduleType == ModuleType.RequireOr) {
            permissionModuleAddress = requireOrFactory.computeModuleAddress(admin, moduleSalt);
        } else {
            revert InvalidModuleType();
        }

        // Compute sequencing chain address
        sequencingChainAddress = syndicateFactory.computeSequencingChainAddress(chainSalt, chainId);

        return (permissionModuleAddress, sequencingChainAddress);
    }

    /// @notice Get the next auto-generated chain ID from the syndicate factory
    /// @return The next available chain ID
    function getNextAutoChainId() external view returns (uint256) {
        return syndicateFactory.getNextChainId();
    }

    /// @notice Check if a chain ID has been used in the syndicate factory
    /// @param chainId The chain ID to check
    /// @return 1 if used, 0 if available
    function isChainIdUsed(uint256 chainId) external view returns (uint256) {
        return syndicateFactory.isChainIdUsed(chainId);
    }

    /// @notice Get the namespace prefix from the syndicate factory
    /// @return The current namespace prefix
    function getNamespacePrefix() external view returns (uint256) {
        return syndicateFactory.namespacePrefix();
    }

    /// @notice Deploy a syndicate with RequireAndModule
    /// @param appchainId The app chain ID (0 for auto-increment)
    /// @param admin The admin address for both contracts
    /// @param moduleSalt The salt for the permission module deployment
    /// @param chainSalt The salt for the sequencing chain deployment
    /// @return sequencingChain The deployed sequencing chain address
    /// @return permissionModule The deployed permission module address
    /// @return actualChainId The chain ID that was used
    function deployWithRequireAndModule(uint256 appchainId, address admin, bytes32 moduleSalt, bytes32 chainSalt)
        external
        whenNotPaused
        returns (address sequencingChain, address permissionModule, uint256 actualChainId)
    {
        return deployCompleteSyndicate(appchainId, admin, ModuleType.RequireAnd, moduleSalt, chainSalt);
    }

    /// @notice Deploy a syndicate with RequireOrModule
    /// @param appchainId The app chain ID (0 for auto-increment)
    /// @param admin The admin address for both contracts
    /// @param moduleSalt The salt for the permission module deployment
    /// @param chainSalt The salt for the sequencing chain deployment
    /// @return sequencingChain The deployed sequencing chain address
    /// @return permissionModule The deployed permission module address
    /// @return actualChainId The chain ID that was used
    function deployWithRequireOrModule(uint256 appchainId, address admin, bytes32 moduleSalt, bytes32 chainSalt)
        external
        whenNotPaused
        returns (address sequencingChain, address permissionModule, uint256 actualChainId)
    {
        return deployCompleteSyndicate(appchainId, admin, ModuleType.RequireOr, moduleSalt, chainSalt);
    }

    /// @notice Pause the wrapper factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the wrapper factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
