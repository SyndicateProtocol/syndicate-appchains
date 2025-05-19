// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {SyndicateSequencerChain} from "./SyndicateSequencerChain.sol";
import {RequireAllModule} from "./requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "./requirement-modules/RequireAnyModule.sol";
import {IRequirementModule} from "./interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/// @title SyndicateFactory
/// @notice Factory contract for creating SyndicateSequencerChain and related contracts
/// with namespace management and auto-incrementing chain IDs
contract SyndicateFactory is AccessControl {
    /// @notice Emitted when a new SyndicateSequencerChain is created
    event SyndicateSequencerChainCreated(
        uint256 indexed appChainId,
        address indexed syndicateSequencerChainAddress,
        address indexed permissionModuleAddress
    );

    /// @notice Emitted when namespace configuration is updated
    event NamespaceConfigUpdated(
        uint256 oldNamespacePrefix,
        uint256 oldNamespaceMultiplier,
        uint256 newNamespacePrefix,
        uint256 newNamespaceMultiplier
    );

    // Roles
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    // Errors
    error ZeroAddress();
    error ZeroValue();
    error ReservedNamespace();
    error ChainIdAlreadyExists();

    // Namespace configuration - now mutable
    uint256 private _namespacePrefix;
    uint256 private _namespaceMultiplier;

    // State variables
    uint256 private _nextAutoChainId;
    mapping(uint256 => bool) private _usedChainIds;

    constructor(address admin) {
        if (admin == address(0)) {
            revert ZeroAddress();
        }

        // Set up initial roles
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MANAGER_ROLE, admin);

        // Set initial namespace configuration
        _namespacePrefix = 510;
        _namespaceMultiplier = 1000;
        _nextAutoChainId = 1; // Start auto-incrementing from 1
    }

    modifier zeroValuesChainAndTwoAddressesNotAllowed(
        uint256 appChainId,
        address firstAddrCheck,
        address secondAddrCheck
    ) {
        if (appChainId == 0) {
            revert ZeroValue();
        }
        if (firstAddrCheck == address(0) || secondAddrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    modifier zeroValuesChainAndAddressNotAllowed(uint256 appChainId, address addrCheck) {
        if (appChainId == 0) {
            revert ZeroValue();
        }
        if (addrCheck == address(0)) {
            revert ZeroAddress();
        }
        _;
    }

    modifier validateChainId(uint256 appChainId, bool isManuallySpecified) {
        // If manually specified, ensure it's not in our reserved namespace
        if (isManuallySpecified) {
            // Check if the chainId is in the reserved namespace
            if (appChainId / namespaceMultiplier() == namespacePrefix()) {
                revert ReservedNamespace();
            }
        }

        // Check if chain ID already exists
        if (_usedChainIds[appChainId]) {
            revert ChainIdAlreadyExists();
        }

        _;
    }

    /// @notice Creates a new SyndicateSequencerChain contract with a permission module
    /// @param appChainId The app chain the contract refers to (0 for auto-increment)
    /// @param admin The address that will be the admin
    /// @param permissionModule The address of the permission module
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created SyndicateSequencerChain
    /// @return actualChainId The chain ID that was used (auto-generated or specified)
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencerChain(
        uint256 appChainId,
        address admin,
        IRequirementModule permissionModule,
        bytes32 salt
    )
        public
        zeroValuesChainAndTwoAddressesNotAllowed(
            appChainId == 0 ? _getNextChainId() : appChainId,
            admin,
            address(permissionModule)
        )
        validateChainId(appChainId == 0 ? _getNextChainId() : appChainId, appChainId != 0)
        returns (address sequencerChain, uint256 actualChainId)
    {
        // Determine the actual chain ID to use
        actualChainId = appChainId == 0 ? _getNextChainId() : appChainId;

        // Mark this chain ID as used
        _usedChainIds[actualChainId] = true;

        // Increment the auto-chain ID counter if we used an auto-generated ID
        if (appChainId == 0) {
            _nextAutoChainId++;
        }

        bytes memory bytecode = getBytecode(actualChainId);
        address deployedAddress = Create2.deploy(0, salt, bytecode);

        SyndicateSequencerChain newSequencerChain = SyndicateSequencerChain(deployedAddress);
        newSequencerChain.initialize(admin, address(permissionModule));

        emit SyndicateSequencerChainCreated(actualChainId, deployedAddress, address(permissionModule));

        return (deployedAddress, actualChainId);
    }

    /// @notice Creates a SyndicateSequencerChain with RequireAllModule
    /// @param admin The address that will be the default admin role
    /// @param appChainId The app chain ID (0 for auto-increment)
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created SyndicateSequencerChain
    /// @return permissionModule The address of the newly created RequireAllModule
    /// @return actualChainId The chain ID that was used (auto-generated or specified)
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencerChainWithRequireAllModule(address admin, uint256 appChainId, bytes32 salt)
        public
        zeroValuesChainAndAddressNotAllowed(appChainId == 0 ? _getNextChainId() : appChainId, admin)
        returns (address sequencerChain, IRequirementModule permissionModule, uint256 actualChainId)
    {
        permissionModule = IRequirementModule(new RequireAllModule(admin));
        (sequencerChain, actualChainId) = createSyndicateSequencerChain(appChainId, admin, permissionModule, salt);

        emit SyndicateSequencerChainCreated(actualChainId, sequencerChain, address(permissionModule));

        return (sequencerChain, permissionModule, actualChainId);
    }

    /// @notice Creates a SyndicateSequencerChain with RequireAnyModule
    /// @param admin The address that will be the default admin role
    /// @param appChainId The app chain ID (0 for auto-increment)
    /// @param salt The salt to use for the deployment
    /// @return sequencerChain The address of the newly created SyndicateSequencerChain
    /// @return permissionModule The address of the newly created RequireAnyModule
    /// @return actualChainId The chain ID that was used (auto-generated or specified)
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencerChainWithRequireAnyModule(address admin, uint256 appChainId, bytes32 salt)
        public
        zeroValuesChainAndAddressNotAllowed(appChainId == 0 ? _getNextChainId() : appChainId, admin)
        returns (address sequencerChain, IRequirementModule permissionModule, uint256 actualChainId)
    {
        permissionModule = IRequirementModule(new RequireAnyModule(admin));
        (sequencerChain, actualChainId) = createSyndicateSequencerChain(appChainId, admin, permissionModule, salt);

        emit SyndicateSequencerChainCreated(actualChainId, sequencerChain, address(permissionModule));

        return (sequencerChain, permissionModule, actualChainId);
    }

    /// @notice Computes the address of the SyndicateSequencerChain contract
    /// @param salt The salt to use for the deployment
    /// @param chainId The ID of the app chain
    /// @return The address of the SyndicateSequencerChain contract
    function computeSequencerChainAddress(bytes32 salt, uint256 chainId) public view returns (address) {
        return Create2.computeAddress(salt, keccak256(getBytecode(chainId)));
    }

    /// @notice Returns the bytecode of the SyndicateSequencerChain contract
    /// @param chainId The ID of the app chain
    /// @return The bytecode of the SyndicateSequencerChain contract
    function getBytecode(uint256 chainId) public pure returns (bytes memory) {
        return abi.encodePacked(type(SyndicateSequencerChain).creationCode, abi.encode(chainId));
    }

    /// @notice Get the next available auto-generated chain ID in the namespace
    /// @return The next available chain ID
    function _getNextChainId() internal view returns (uint256) {
        return _namespacePrefix * _namespaceMultiplier + _nextAutoChainId;
    }

    /// @notice Get the current next auto-incremented chain ID
    /// @return The current value of the auto-incrementing chain ID counter
    function getNextAutoChainId() external view returns (uint256) {
        return _getNextChainId();
    }

    /// @notice Check if a chain ID has already been used
    /// @param chainId The chain ID to check
    /// @return 1 if the chain ID has been used, 0 otherwise
    function isChainIdUsed(uint256 chainId) external view returns (uint256) {
        return _usedChainIds[chainId] ? 1 : 0;
    }

    // Namespace configuration getters

    /// @notice Get the current namespace prefix
    /// @return The current namespace prefix
    function namespacePrefix() public view returns (uint256) {
        return _namespacePrefix;
    }

    /// @notice Get the current namespace multiplier
    /// @return The current namespace multiplier
    function namespaceMultiplier() public view returns (uint256) {
        return _namespaceMultiplier;
    }

    /// @notice Update the namespace configuration (manager only)
    /// @param newPrefix The new namespace prefix
    /// @param newMultiplier The new namespace multiplier
    function updateNamespaceConfig(uint256 newPrefix, uint256 newMultiplier) external onlyRole(MANAGER_ROLE) {
        // Store old values for the event
        uint256 oldPrefix = _namespacePrefix;
        uint256 oldMultiplier = _namespaceMultiplier;

        // Update values
        _namespacePrefix = newPrefix;
        _namespaceMultiplier = newMultiplier;

        // Emit event
        emit NamespaceConfigUpdated(oldPrefix, oldMultiplier, newPrefix, newMultiplier);
    }
}
