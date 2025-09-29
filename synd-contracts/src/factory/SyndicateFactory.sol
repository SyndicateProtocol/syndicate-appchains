// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "../SyndicateSequencingChain.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Proxy} from "@openzeppelin/contracts/proxy/Proxy.sol";
import {GasAggregator} from "../staking/GasAggregator.sol";
import {IGasAggregator} from "../interfaces/IGasAggregator.sol";

/// @title MinimalUUPSStub
/// @notice Minimal UUPS implementation stub for deterministic proxy deployments
/// @dev This contract will NEVER change to ensure deterministic CREATE2 addresses across all deployments
contract MinimalUUPSStub is UUPSUpgradeable {
    /// @notice this is only used to get a reliably deterministic address, the proxy will immediately be upgraded
    function _authorizeUpgrade(address) internal view override {}

    /// @notice Receive function that reverts - this stub should not receive ETH
    receive() external payable {
        revert("Stub: ETH not accepted");
    }

    /// @notice Fallback that reverts - this stub has no logic
    fallback() external payable {
        revert("Stub: no logic implemented");
    }
}

enum NamespaceState {
    Available,
    Used,
    Reserved
}

/// @title SyndicateFactory
/// @notice Factory contract for creating SyndicateSequencingChain contracts
/// @dev Uses UUPS proxy pattern for upgradeability and CREATE2 pattern for deterministic deployments
contract SyndicateFactory is Initializable, AccessControlUpgradeable, PausableUpgradeable, UUPSUpgradeable {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Mapping from appchain ID to the sequencing contract address
    mapping(uint256 => address) public appchainContracts;
    uint256[] public chainIDs;

    /// @notice Stub implementation for consistent proxy deployment
    address public stubImplementation;

    /// @notice Current implementation address used for new deployments
    address public syndicateChainImpl;

    /// @notice Version of the SyndicateFactory contract (updatable during upgrades)
    string public version;

    IGasAggregator public gasAggregator;

    /*//////////////////////////////////////////////////////////////
                              ERRORS
    //////////////////////////////////////////////////////////////*/

    error ZeroAddress();
    error ChainIdAlreadyExists();
    error FailedToUpgradeToLatestImplementation();
    error FailedToUpgradeGasAggregator();

    /*//////////////////////////////////////////////////////////////
                             EVENTS 
    //////////////////////////////////////////////////////////////*/
    /// @notice Emitted when a new SyndicateSequencingChain is created
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    /// @notice Emitted when a chain ID is manually marked as used
    event ChainIdManuallyMarked(uint256 indexed chainId);

    /// @notice Emitted when a new implementation is added to allowed list
    event ImplementationAdded(address indexed implementation);

    /// @notice Emitted when a deterministic chainID is generated
    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    /// @notice Emitted when a new implementation is added to allowed list
    event gasAggregatorNotificationFailed();

    /*//////////////////////////////////////////////////////////////
                            INITIALIZER
    //////////////////////////////////////////////////////////////*/

    /// @notice Disables initializers to prevent the implementation contract from being initialized
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the upgradeable factory
    /// @param admin The admin address that will have DEFAULT_ADMIN_ROLE
    function initialize(address admin) external initializer {
        if (admin == address(0)) revert ZeroAddress();

        __AccessControl_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // Set initial version
        version = "1.0.0";

        // Deploy minimal stub implementation using CREATE2 for deterministic address
        bytes memory stubBytecode = abi.encodePacked(type(MinimalUUPSStub).creationCode);
        stubImplementation = Create2.deploy(0, bytes32("SYNDICATE_STUB_V1"), stubBytecode);

        // Deploy the real implementation and make it the default
        syndicateChainImpl = address(new SyndicateSequencingChain());
        emit ImplementationAdded(syndicateChainImpl);

        // deploy a new gas aggregator with a deterministic address
        address gasAggregatorProxy = Create2.deploy(0, bytes32("SYNDICATE_GAS_AGGREGATOR"), getProxyBytecode());
        bytes memory initData =
            abi.encodeWithSignature("initialize(address,address,address)", admin, address(this), syndicateChainImpl);
        (bool upgradeSuccess,) = gasAggregatorProxy.call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", new GasAggregator(), initData)
        );
        if (!upgradeSuccess) {
            revert FailedToUpgradeGasAggregator();
        }
        gasAggregator = IGasAggregator(gasAggregatorProxy);
    }

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Creates a new SyndicateSequencingChain contract with deterministic chainID to prevent squatting
    /// @param nonce The user-specified nonce for chainID generation
    /// @param admin The admin address for the new chain
    /// @param permissionModule The pre-deployed permission module
    /// @return sequencingChain The deployed sequencing chain address
    /// @return actualChainId The chain ID that was used
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencingChain(uint256 nonce, address admin, IRequirementModule permissionModule)
        external
        whenNotPaused
        returns (address sequencingChain, uint256 actualChainId)
    {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }

        // Generate chainID using user-provided nonce
        actualChainId = generateDeterministicChainId(msg.sender, nonce);

        // Validate chain ID is not already used
        if (appchainContracts[actualChainId] != address(0)) {
            revert ChainIdAlreadyExists();
        }

        // Emit deterministic chainID generation event
        emit DeterministicChainIdGenerated(msg.sender, nonce, actualChainId);

        // Deploy the sequencing chain using consistent proxy bytecode
        bytes memory consistentBytecode = getProxyBytecode();
        sequencingChain = Create2.deploy(0, bytes32(actualChainId), consistentBytecode);

        // Store the mapping of appchain ID to contract address
        appchainContracts[actualChainId] = sequencingChain;
        chainIDs.push(actualChainId);

        // Upgrade the proxy to use the latest implementation (instead of the stub)
        bytes memory initData = abi.encodeWithSignature(
            "initialize(address,address,address,uint256)",
            admin,
            address(gasAggregator),
            address(permissionModule),
            actualChainId
        );
        (bool upgradeSuccess,) = sequencingChain.call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", syndicateChainImpl, initData)
        );
        if (!upgradeSuccess) {
            revert FailedToUpgradeToLatestImplementation();
        }

        emit SyndicateSequencingChainCreated(actualChainId, sequencingChain, address(permissionModule));

        return (sequencingChain, actualChainId);
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes the address where a sequencing chain will be deployed
    /// @param chainId The chain ID to compute the address for
    /// @return The computed address
    function computeSequencingChainAddress(uint256 chainId) external view returns (address) {
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
        // Use modulo to keep chainIDs in a reasonable range (avoid extremely large numbers)
        chainId = uint256(hash) % (10 ** 18); // Max 18 digits
        // Ensure chainID is never 0
        if (chainId == 0) {
            chainId = 1;
        }
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Creates a new SyndicateSequencingChain with a custom chainID (admin only)
    /// @param customChainId The custom chain ID to use
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
        if (appchainContracts[customChainId] != address(0)) {
            revert ChainIdAlreadyExists();
        }

        actualChainId = customChainId;

        // Deploy the sequencing chain using consistent proxy bytecode
        bytes memory consistentBytecode = getProxyBytecode();
        sequencingChain = Create2.deploy(0, bytes32(actualChainId), consistentBytecode);

        // Store the mapping of appchain ID to contract address
        appchainContracts[actualChainId] = sequencingChain;
        chainIDs.push(actualChainId);

        // Upgrade the proxy to use the latest implementation (instead of the stub)
        bytes memory initData = abi.encodeWithSignature(
            "initialize(address,address,address,uint256)",
            admin,
            address(gasAggregator),
            address(permissionModule),
            actualChainId
        );
        (bool upgradeSuccess,) = sequencingChain.call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", syndicateChainImpl, initData)
        );
        if (!upgradeSuccess) {
            revert FailedToUpgradeToLatestImplementation();
        }

        emit SyndicateSequencingChainCreated(actualChainId, sequencingChain, address(permissionModule));

        return (sequencingChain, actualChainId);
    }

    /// @notice Authorizes upgrades to new implementations (admin only)
    /// @param newImplementation The address of the new implementation
    function _authorizeUpgrade(address newImplementation) internal override onlyRole(DEFAULT_ADMIN_ROLE) {}

    /// @notice Updates the contract version (admin only, typically called during upgrades)
    /// @param newVersion The new version string (e.g., "1.1.0")
    function updateVersion(string calldata newVersion) external onlyRole(DEFAULT_ADMIN_ROLE) {
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
    /// @param newImplementation The implementation address to use as default
    function setSyndicateSequencingChainImplementation(address newImplementation)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        syndicateChainImpl = newImplementation;
        try gasAggregator.notifyNewImplementation(newImplementation) {}
        catch {
            emit gasAggregatorNotificationFailed();
        }
    }

    function setGasAggregator(IGasAggregator newGasAggregator) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (address(newGasAggregator) == address(0)) revert ZeroAddress();
        gasAggregator = newGasAggregator;
    }

    function migrateLegacyAppchain() external onlyRole(DEFAULT_ADMIN_ROLE) {
        // TODO copy the current epoch gas data from the origin contract
    }
}
