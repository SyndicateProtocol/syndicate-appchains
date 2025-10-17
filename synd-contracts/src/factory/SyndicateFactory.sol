// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "../SyndicateSequencingChain.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {UUPSUpgradeable, Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {MinimalUUPSStub} from "./MinimalUUPSStub.sol";

/// @title SyndicateFactory
/// @notice Factory contract for creating SyndicateSequencingChain contracts
/// @dev Uses UUPS proxy pattern for upgradeability and CREATE2 pattern for deterministic deployments
contract SyndicateFactory is Initializable, AccessControlUpgradeable, PausableUpgradeable, UUPSUpgradeable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Stub implementation for consistent proxy deployment
    /// @dev This address is computed deterministically and never changes to ensure consistent CREATE2 addresses
    address public stubImplementation;

    /// @notice Current implementation address used for new deployments
    /// @dev This can be updated by admins to use newer versions of SyndicateSequencingChain
    address public syndicateChainImpl;

    /// @notice Version of the SyndicateFactory contract
    /// @dev Used to track the current version of the factory contract
    uint256 public version;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when attempting to create an appchain with an already used chain ID
    error ChainIdAlreadyExists();

    /// @notice Thrown when the proxy upgrade to the latest implementation fails
    error FailedToInitializeSyndicateSequencingChain();

    /*//////////////////////////////////////////////////////////////
                             EVENTS
    //////////////////////////////////////////////////////////////*/
    /// @notice Emitted when a new SyndicateSequencingChain is created
    /// @param appchainId The unique identifier for the appchain
    /// @param sequencingChainAddress The address of the deployed sequencing chain contract
    /// @param permissionModuleAddress The address of the permission module controlling access
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    /// @notice Emitted when a new implementation is added to the allowed list
    /// @param implementation The address of the implementation that was added
    event ImplementationAdded(address indexed implementation);

    /// @notice Emitted when a deterministic chainID is generated for a user
    /// @param sender The address that requested the chain ID generation
    /// @param nonce The nonce used in the chain ID generation
    /// @param chainId The resulting deterministic chain ID
    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER
    //////////////////////////////////////////////////////////////*/

    /// @notice Disables initializers to prevent the implementation contract from being initialized
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the upgradeable factory
    /// @dev MUST setup gasAggregator separately after initialization
    /// @dev This function can only be called once and sets up the entire factory infrastructure including:
    ///      - Role-based access control with the provided admin
    ///      - Deterministic stub implementation deployment
    ///      - Real SyndicateSequencingChain implementation deployment
    ///      - Initial version setting
    /// @param admin The admin address that will have DEFAULT_ADMIN_ROLE and full control over the factory
    function initialize(address admin) external initializer {
        if (admin == address(0)) revert ZeroAddress();

        __AccessControl_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // Set initial version
        version = 1_000_000; // 1.0.0

        // Deploy minimal stub implementation using CREATE2 for deterministic address
        bytes memory stubBytecode = abi.encodePacked(type(MinimalUUPSStub).creationCode);
        stubImplementation = Create2.deploy(0, bytes32("SYNDICATE_STUB_V1"), stubBytecode);

        // Deploy the real implementation and make it the default for new appchains
        syndicateChainImpl = address(new SyndicateSequencingChain());
        emit ImplementationAdded(syndicateChainImpl);
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Creates a new SyndicateSequencingChain contract with deterministic chainID to prevent squatting
    /// @param nonce The user-specified nonce for chainID generation
    /// @param admin The admin address for the new chain
    /// @param permissionModule The pre-deployed permission module
    /// @return sequencingChain The deployed sequencing chain address
    /// @return chainId The chain ID that was used
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencingChain(uint256 nonce, address admin, IRequirementModule permissionModule)
        external
        whenNotPaused
        returns (address sequencingChain, uint256 chainId)
    {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }

        // Generate chainID using user-provided nonce
        chainId = generateDeterministicChainId(msg.sender, nonce);

        // Validate chain ID is not already used
        if (isChainIdUsed(chainId)) {
            revert ChainIdAlreadyExists();
        }

        // Emit deterministic chainID generation event
        emit DeterministicChainIdGenerated(msg.sender, nonce, chainId);

        return (_doCreateChain(chainId, admin, permissionModule), chainId);
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes the address where a sequencing chain will be deployed
    /// @param chainId The chain ID to compute the address for
    /// @return The computed address
    function computeSequencingChainAddress(uint256 chainId) public view returns (address) {
        return Create2.computeAddress(bytes32(chainId), keccak256(getProxyBytecode()));
    }

    /// @notice Returns the consistent proxy bytecode used for all deployments
    /// @dev Always returns the same bytecode for predictable CREATE2 addresses
    /// @return The bytecode to be used for deployment
    function getProxyBytecode() public view returns (bytes memory) {
        return abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(stubImplementation, ""));
    }

    /// @notice Computes the deterministic stub implementation address
    /// @dev This allows computing the stub address before factory deployment
    /// @return The computed stub implementation address
    function computeStubImplementationAddress() public view returns (address) {
        bytes memory stubBytecode = abi.encodePacked(type(MinimalUUPSStub).creationCode);
        return Create2.computeAddress(bytes32("SYNDICATE_STUB_V1"), keccak256(stubBytecode));
    }

    /// @notice Check if a chain ID has been used
    /// @param chainId The chain ID to check
    /// @return true if used, false if available
    function isChainIdUsed(uint256 chainId) public view returns (bool) {
        return computeSequencingChainAddress(chainId).code.length > 0;
    }

    /// @notice Generate deterministic chainID from sender address and nonce
    /// @param sender The sender address
    /// @param nonce The nonce for this sender
    /// @return chainId The deterministic chain ID
    function generateDeterministicChainId(address sender, uint256 nonce) public pure returns (uint256 chainId) {
        // Use keccak256 hash of sender + nonce, then take modulo to keep within reasonable range
        // This prevents chainID squatting across different sequencing chains
        bytes32 hash = keccak256(abi.encodePacked(sender, nonce));
        // Use modulo to keep chainId in a reasonable range (avoid extremely large numbers)
        chainId = uint256(hash) % (10 ** 18); // Max 18 digits
        // Ensure chainID is never 0 as this is used as a null value indicator
        if (chainId == 0) {
            chainId = 1;
        }
    }

    /*//////////////////////////////////////////////////////////////
                         INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Internal function that handles the actual appchain deployment process
    ///      It deploys a proxy, upgrades it to the real implementation, and initializes it with the provided parameters.
    /// @param chainId The chain ID to use for the new appchain
    /// @param admin The admin address that will own the new appchain
    /// @param permissionModule The permission module to control access to the appchain
    /// @return sequencingChain The address of the deployed and initialized sequencing chain
    function _doCreateChain(uint256 chainId, address admin, IRequirementModule permissionModule)
        internal
        returns (address sequencingChain)
    {
        // Deploy the sequencing chain using consistent proxy bytecode for deterministic addresses
        bytes memory consistentBytecode = getProxyBytecode();
        sequencingChain = Create2.deploy(0, bytes32(chainId), consistentBytecode);

        // Upgrade the proxy to use the latest implementation (instead of the stub)
        bytes memory initData =
            abi.encodeWithSignature("initialize(address,address,uint256)", admin, address(permissionModule), chainId);
        (bool upgradeSuccess,) = sequencingChain.call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", syndicateChainImpl, initData)
        );
        if (!upgradeSuccess) {
            revert FailedToInitializeSyndicateSequencingChain();
        }

        emit SyndicateSequencingChainCreated(chainId, sequencingChain, address(permissionModule));

        return sequencingChain;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Creates a new SyndicateSequencingChain with a custom chainID (admin only)
    /// @param customChainId The custom chain ID to use (must not be 0 or already used)
    /// @param admin The admin address for the new chain
    /// @param permissionModule The pre-deployed permission module
    /// @return sequencingChain The deployed sequencing chain address
    /// @return actualChainId The chain ID that was used (same as customChainId)
    function createSyndicateSequencingChainWithCustomId(
        uint256 customChainId,
        address admin,
        IRequirementModule permissionModule
    ) external onlyRole(DEFAULT_ADMIN_ROLE) whenNotPaused returns (address sequencingChain, uint256 actualChainId) {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }
        if (customChainId == 0) {
            revert ZeroAddress(); // Reusing this error for zero chainID
        }

        // Validate chain ID is not already used
        if (isChainIdUsed(customChainId)) {
            revert ChainIdAlreadyExists();
        }

        return (_doCreateChain(customChainId, admin, permissionModule), customChainId);
    }

    /// @notice Authorizes upgrades to new implementations (admin only)
    /// @param newImplementation The address of the new implementation
    function _authorizeUpgrade(address newImplementation) internal override onlyRole(DEFAULT_ADMIN_ROLE) {}

    /// @notice Updates the contract version (admin only, typically called during upgrades)
    /// @param newVersion The new version number (e.g., 1)
    function updateVersion(uint256 newVersion) external onlyRole(DEFAULT_ADMIN_ROLE) {
        version = newVersion;
    }

    /// @notice Pause the factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    /// @notice Set the implementation for new sequencing contract deployments (admin only)
    /// @dev Updates the default implementation used for new appchain deployments.
    /// @param newImplementation The implementation address to use as default for new deployments
    function setSyndicateSequencingChainImplementation(address newImplementation)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        syndicateChainImpl = newImplementation;
    }
}
