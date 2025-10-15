// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "../SyndicateSequencingChain.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControlEnumerableUpgradeable} from
    "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlEnumerableUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {UUPSUpgradeable, Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IGasAggregator} from "../interfaces/IGasAggregator.sol";
import {EpochTracker} from "../staking/EpochTracker.sol";
import {SyndicateProxy} from "../SyndicateProxy.sol";

/// @title SyndicateFactory
/// @notice Factory contract for creating SyndicateSequencingChain contracts
/// @dev Uses UUPS proxy pattern for upgradeability and CREATE2 pattern for deterministic deployments
contract SyndicateFactory is
    Initializable,
    AccessControlEnumerableUpgradeable,
    PausableUpgradeable,
    UUPSUpgradeable,
    EpochTracker
{
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Mapping from appchain ID to the sequencing contract address
    /// @dev Used to track all deployed appchains and prevent duplicates
    mapping(uint256 appchainId => address sequencingContractAddress) public appchainContracts;

    /// @notice Current implementation address used for new deployments
    /// @dev This can be updated by admins to use newer versions of SyndicateSequencingChain
    address public syndicateChainImpl;

    /// @notice Version of the SyndicateFactory contract (updatable during upgrades)
    /// @dev Semantic version string to track factory upgrades
    uint256 public constant VERSION = 1_000_000; // 1.0.0 (major * 1_000_000 + minor * 1_000 + patch)

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when attempting to create an appchain with an already used chain ID
    error ChainIdAlreadyExists();

    /// @notice Thrown when the provided legacy appchain address is invalid (no code deployed)
    error InvalidAppchainAddress();

    /// @notice Thrown when the provided chain ID is not found in the appchains mapping
    error ChainIdNotFound();

    /// @notice Thrown when the gas aggregator is not set but required for certain operations
    error GasAggregatorNotSet();

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

    /// @notice Emitted when a chain ID is manually marked as used (currently unused)
    /// @param chainId The chain ID that was marked as used
    event ChainIdManuallyMarked(uint256 indexed chainId);

    /// @notice Emitted when a deterministic chainID is generated for a user
    /// @param sender The address that requested the chain ID generation
    /// @param nonce The nonce used in the chain ID generation
    /// @param chainId The resulting deterministic chain ID
    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    /// @notice Emitted when an appchain is migrated from a legacy implementation
    /// @param oldAppchainContract The address of the legacy appchain being migrated
    /// @param newAppchainContract The address of the newly deployed appchain
    /// @param appchainId The chain ID assigned to the new appchain
    /// @param epoch The epoch during which the migration occurred
    /// @param migratedGasTokensUsedForCurrentEpoch Gas tokens used data migrated from the legacy chain
    event AppchainMigrated(
        address indexed oldAppchainContract,
        address indexed newAppchainContract,
        uint256 indexed appchainId,
        uint256 epoch,
        uint256 migratedGasTokensUsedForCurrentEpoch
    );

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

        // Deploy the real implementation and make it the default for new appchains
        syndicateChainImpl = address(new SyndicateSequencingChain(address(0)));
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
        if (admin == address(0)) {
            revert ZeroAddress();
        }

        // Generate chainID using user-provided nonce
        chainId = generateDeterministicChainId(msg.sender, nonce);

        // Validate chain ID is not already used
        if (appchainContracts[chainId] != address(0)) {
            revert ChainIdAlreadyExists();
        }

        // Emit deterministic chainID generation event
        emit DeterministicChainIdGenerated(msg.sender, nonce, chainId);

        return (_doCreateChain(chainId, admin, permissionModule, 0), chainId);
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes the address where a sequencing chain will be deployed
    /// @param chainId The chain ID to compute the address for
    /// @return The computed address
    function computeSequencingChainAddress(uint256 chainId) external view returns (address) {
        return Create2.computeAddress(bytes32(chainId), keccak256(type(SyndicateProxy).creationCode));
    }

    /// @notice Check if a chain ID has been used
    /// @param chainId The chain ID to check
    /// @return 1 if used, 0 if available
    function isChainIdUsed(uint256 chainId) public view returns (bool) {
        return appchainContracts[chainId] != address(0);
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
    /// @param gasTokensUsedForCurrentEpoch Gas tokens to initialize with (used for legacy migrations)
    /// @return sequencingChain The address of the deployed and initialized sequencing chain
    function _doCreateChain(
        uint256 chainId,
        address admin,
        IRequirementModule permissionModule,
        uint256 gasTokensUsedForCurrentEpoch
    ) internal returns (address sequencingChain) {
        // Deploy the sequencing chain using the syndicate proxy
        sequencingChain = Create2.deploy(0, bytes32(chainId), type(SyndicateProxy).creationCode);

        // Initialize the proxy
        SyndicateProxy(payable(sequencingChain)).initializeProxy(
            syndicateChainImpl, 0, uint128(gasTokensUsedForCurrentEpoch)
        );

        // Store the mapping of appchain ID to contract address
        appchainContracts[chainId] = sequencingChain;

        // Initialize the implementation
        SyndicateSequencingChain(sequencingChain).initialize(admin, address(permissionModule));

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
        require(admin != address(0) && address(permissionModule) != address(0) && customChainId != 0, ZeroAddress());

        // Validate chain ID is not already used
        if (appchainContracts[customChainId] != address(0)) {
            revert ChainIdAlreadyExists();
        }

        return (_doCreateChain(customChainId, admin, permissionModule, 0), customChainId);
    }

    /// @notice Authorizes upgrades to new implementations (admin only)
    /// @param newImplementation The address of the new implementation
    function _authorizeUpgrade(address newImplementation) internal override onlyRole(DEFAULT_ADMIN_ROLE) {}

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

    /// @notice Migrates a legacy appchain to a new deployment while preserving gas counter data
    /// @param legacyAppchain The address of the existing appchain contract to migrate from
    /// @param appchainId The chain ID for the new appchain deployment
    /// @param admin The admin address for the new appchain
    /// @param permissionModule The permission module for the new appchain
    /// @return newSyndicateChain The address of the newly deployed appchain
    function migrateLegacyAppchain(
        address legacyAppchain,
        uint256 appchainId,
        address admin,
        IRequirementModule permissionModule
    ) external onlyRole(DEFAULT_ADMIN_ROLE) whenNotPaused returns (address newSyndicateChain) {
        require(
            legacyAppchain != address(0) && admin != address(0) && address(permissionModule) != address(0)
                && appchainId != 0,
            ZeroAddress()
        );
        require(appchainContracts[appchainId] == address(0), ChainIdAlreadyExists());

        // Verify the legacy appchain exists by checking if there's actually code at the address
        require(address(legacyAppchain).code.length > 0, InvalidAppchainAddress());

        // Extract tokens used for gas from the legacy appchain for the current epoch
        uint256 epoch = getCurrentEpoch();
        uint256 gasTokensUsedForCurrentEpoch = SyndicateProxy(payable(legacyAppchain)).tokensUsedPerEpoch(epoch);

        // Create the new appchain with migrated gas data
        newSyndicateChain = _doCreateChain(appchainId, admin, permissionModule, gasTokensUsedForCurrentEpoch);
        emit AppchainMigrated(legacyAppchain, newSyndicateChain, appchainId, epoch, gasTokensUsedForCurrentEpoch);

        return newSyndicateChain;
    }
}
