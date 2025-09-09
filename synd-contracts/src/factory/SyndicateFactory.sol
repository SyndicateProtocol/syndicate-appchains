// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "../SyndicateSequencingChain.sol";
import {IRequirementModule} from "../interfaces/IRequirementModule.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

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
/// @notice Factory contract for creating SyndicateSequencingChain contracts with centralized gas tracking
/// @dev Uses UUPS proxy pattern for upgradeability and CREATE2 pattern for deterministic deployments
contract SyndicateFactory is Initializable, AccessControlUpgradeable, PausableUpgradeable, UUPSUpgradeable {
    /// @notice Emitted when a new SyndicateSequencingChain is created
    event SyndicateSequencingChainCreated(
        uint256 indexed appchainId, address indexed sequencingChainAddress, address indexed permissionModuleAddress
    );

    /// @notice Emitted when a chain ID is manually marked as used
    event ChainIdManuallyMarked(uint256 indexed chainId);

    /// @notice Emitted when a new implementation is added to allowed list
    event ImplementationAdded(address indexed implementation);

    /// @notice Emitted when a chain is banned from gas tracking
    event ChainBannedFromGasTracking(uint256 indexed chainId, address indexed notAllowedImplementation);

    /// @notice Emitted when a deterministic chainID is generated
    event DeterministicChainIdGenerated(address indexed sender, uint256 indexed nonce, uint256 indexed chainId);

    /// @notice Emitted when the creation fee is updated
    event CreationFeeUpdated(uint256 oldFee, uint256 newFee);

    error ZeroAddress();
    error ChainIdAlreadyExists();
    error ImplementationNotAllowed();
    error ImplementationAlreadyAllowed();
    error OnlyChainCanNotifyUpgrade();
    error CannotRemoveDefaultImplementation();
    error FailedToUpgradeToLatestImplementation();
    error InsufficientFee();

    /// @notice Mapping from appchain ID to the sequencing contract address
    mapping(uint256 => address) public appchainContracts;
    uint256[] public chainIDs;

    /// @notice Stub implementation for consistent proxy deployment
    address public stubImplementation;

    /// @notice Current implementation address used for new deployments
    address public syndicateChainImpl;

    /// @notice List of allowed implementation addresses for sequencing chains
    address[] public allowedImplementations;
    mapping(address => bool) public isImplementationAllowed;

    /// @notice Chains banned from gas tracking due to not allowed implementation
    mapping(uint256 => bool) public gasTrackingBanlist;
    uint256 public numberOfChainsBannedFromGasTracking;

    /// @notice Per-sender nonce tracking for deterministic chainID generation
    mapping(address sender => uint256 nonce) public senderNonces;

    /// @notice Fee required to create a sequencing chain (in native token)
    uint256 public creationFee;

    /// @notice Disables initializers to prevent the implementation contract from being initialized
    constructor() {
        _disableInitializers();
    }

    /// @notice Modifier to check that sufficient fee is paid
    modifier paysCreationFee() {
        if (msg.value < creationFee) {
            revert InsufficientFee();
        }
        _;
    }

    /// @notice Initializes the upgradeable factory
    /// @param admin The admin address that will have DEFAULT_ADMIN_ROLE
    function initialize(address admin) external initializer {
        if (admin == address(0)) revert ZeroAddress();

        __AccessControl_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);

        // Deploy minimal stub implementation using CREATE2 for deterministic address
        bytes memory stubBytecode = abi.encodePacked(type(MinimalUUPSStub).creationCode);
        stubImplementation = Create2.deploy(0, bytes32("SYNDICATE_STUB_V1"), stubBytecode);

        // Deploy the real implementation and make it the default
        syndicateChainImpl = address(new SyndicateSequencingChain());

        // Add real implementation to allowed list
        allowedImplementations.push(syndicateChainImpl);
        isImplementationAllowed[syndicateChainImpl] = true;
        emit ImplementationAdded(syndicateChainImpl);
    }

    /// @notice Authorizes upgrades to new implementations (admin only)
    /// @param newImplementation The address of the new implementation
    function _authorizeUpgrade(address newImplementation) internal override onlyRole(DEFAULT_ADMIN_ROLE) {}

    /// @notice Creates a new SyndicateSequencingChain contract with deterministic chainID to prevent squatting
    /// @param admin The admin address for the new chain
    /// @param permissionModule The pre-deployed permission module
    /// @return sequencingChain The deployed sequencing chain address
    /// @return actualChainId The chain ID that was used
    //#olympix-ignore-reentrancy-events
    function createSyndicateSequencingChain(address admin, IRequirementModule permissionModule)
        external
        payable
        whenNotPaused
        paysCreationFee
        returns (address sequencingChain, uint256 actualChainId)
    {
        if (admin == address(0) || address(permissionModule) == address(0)) {
            revert ZeroAddress();
        }

        // Use current nonce for this sender and increment it
        uint256 actualNonce = senderNonces[msg.sender];
        actualChainId = generateDeterministicChainId(msg.sender, actualNonce);

        // Validate chain ID is not already used
        if (appchainContracts[actualChainId] != address(0)) {
            revert ChainIdAlreadyExists();
        }

        // Increment the sender's nonce for next use
        senderNonces[msg.sender]++;

        // Emit deterministic chainID generation event
        emit DeterministicChainIdGenerated(msg.sender, actualNonce, actualChainId);

        // Deploy the sequencing chain using consistent proxy bytecode
        bytes memory consistentBytecode = getProxyBytecode();
        sequencingChain = Create2.deploy(0, bytes32(actualChainId), consistentBytecode);

        // Store the mapping of appchain ID to contract address
        appchainContracts[actualChainId] = sequencingChain;
        chainIDs.push(actualChainId);

        // Upgrade the proxy to use the latest implementation (instead of the stub)
        bytes memory initData = abi.encodeWithSignature(
            "initialize(address,address,uint256)", address(this), address(permissionModule), actualChainId
        );
        (bool upgradeSuccess,) = sequencingChain.call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", syndicateChainImpl, initData)
        );
        if (!upgradeSuccess) {
            revert FailedToUpgradeToLatestImplementation();
        }

        // Transfer ownership to the intended admin
        SyndicateSequencingChain(sequencingChain).transferOwnership(admin);

        emit SyndicateSequencingChainCreated(actualChainId, sequencingChain, address(permissionModule));

        return (sequencingChain, actualChainId);
    }

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
    )
        external
        payable
        onlyRole(DEFAULT_ADMIN_ROLE)
        whenNotPaused
        paysCreationFee
        returns (address sequencingChain, uint256 actualChainId)
    {
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
            "initialize(address,address,uint256)", address(this), address(permissionModule), actualChainId
        );
        (bool upgradeSuccess,) = sequencingChain.call(
            abi.encodeWithSignature("upgradeToAndCall(address,bytes)", syndicateChainImpl, initData)
        );
        if (!upgradeSuccess) {
            revert FailedToUpgradeToLatestImplementation();
        }

        // Transfer ownership to the intended admin
        SyndicateSequencingChain(sequencingChain).transferOwnership(admin);

        emit SyndicateSequencingChainCreated(actualChainId, sequencingChain, address(permissionModule));

        return (sequencingChain, actualChainId);
    }

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

    /// @notice Returns the creation bytecode for an ERC1967Proxy with the given implementation address.
    /// @dev Used for deterministic deployment of proxy contracts via CREATE2.
    /// @param impl The address of the implementation contract.
    /// @return The bytecode to be used for deployment.
    function getImplBytecode(address impl) public pure returns (bytes memory) {
        return abi.encodePacked(type(ERC1967Proxy).creationCode, abi.encode(impl, ""));
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

    /// @notice Get the next nonce for a sender
    /// @param sender The sender address
    /// @return The next nonce for this sender
    function getNextNonceForSender(address sender) external view returns (uint256) {
        return senderNonces[sender];
    }

    /// @notice returns the number of appchains not banned from gas tracking
    function getTotalAppchainsForGasTracking() external view returns (uint256) {
        return chainIDs.length - numberOfChainsBannedFromGasTracking;
    }

    /// @notice returns the contracts for a given list of appchain chainIDs that are not banned from gas tracking
    /// @param _chainIDs the list of chain IDs
    /// @return _contracts contracts for the given chain IDs (zero address if banned)
    function getContractsForGasTracking(uint256[] memory _chainIDs)
        external
        view
        returns (address[] memory _contracts)
    {
        address[] memory contracts = new address[](_chainIDs.length);
        for (uint256 i = 0; i < _chainIDs.length; i++) {
            if (!gasTrackingBanlist[_chainIDs[i]]) {
                contracts[i] = appchainContracts[_chainIDs[i]];
            }
        }
        return contracts;
    }

    /// @notice returns all appchains chainIDs and associated contracts that are not banned from gas tracking
    /// @return _chainIDs chain IDs not banned from gas tracking
    /// @return _contracts contracts for non-banned chains
    function getAppchainsAndContractsForGasTracking()
        external
        view
        returns (uint256[] memory _chainIDs, address[] memory _contracts)
    {
        uint256 validCount = chainIDs.length - numberOfChainsBannedFromGasTracking;
        uint256[] memory validChainIDs = new uint256[](validCount);
        address[] memory validContracts = new address[](validCount);
        uint256 index = 0;

        for (uint256 i = 0; i < chainIDs.length; i++) {
            if (!gasTrackingBanlist[chainIDs[i]]) {
                validChainIDs[index] = chainIDs[i];
                validContracts[index] = appchainContracts[chainIDs[i]];
                index++;
            }
        }

        return (validChainIDs, validContracts);
    }

    /// @notice Set the creation fee (admin only)
    /// @param newFee The new fee amount in native token
    function setCreationFee(uint256 newFee) external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 oldFee = creationFee;
        creationFee = newFee;
        emit CreationFeeUpdated(oldFee, newFee);
    }

    /// @notice Withdraw collected fees (admin only)
    /// @param to The address to send the fees to
    function withdrawFees(address payable to) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        uint256 balance = address(this).balance;
        (bool success,) = to.call{value: balance}("");
        require(success, "Fee withdrawal failed");
    }

    /// @notice Pause the factory (admin only)
    function pause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _pause();
    }

    /// @notice Unpause the factory (admin only)
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    /// @notice Add a new allowed implementation (admin only)
    /// @param implementation The implementation address to allow
    /// @param makeDefault Whether to make this the default for new deployments
    function addAllowedImplementation(address implementation, bool makeDefault) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (implementation == address(0)) revert ZeroAddress();
        if (isImplementationAllowed[implementation]) revert ImplementationAlreadyAllowed();

        allowedImplementations.push(implementation);
        isImplementationAllowed[implementation] = true;

        if (makeDefault) {
            syndicateChainImpl = implementation;
        }

        emit ImplementationAdded(implementation);
    }

    /// @notice Remove an allowed implementation (admin only)
    /// @param implementation The implementation address to remove
    function removeAllowedImplementation(address implementation) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (!isImplementationAllowed[implementation]) revert ImplementationNotAllowed();
        if (implementation == syndicateChainImpl) revert CannotRemoveDefaultImplementation();
        for (uint256 i = 0; i < allowedImplementations.length; i++) {
            if (allowedImplementations[i] == implementation) {
                allowedImplementations[i] = allowedImplementations[allowedImplementations.length - 1];
                allowedImplementations.pop();
                break;
            }
        }
        isImplementationAllowed[implementation] = false;
    }

    /// @notice Set the default implementation for new deployments (admin only)
    /// @param implementation The implementation address to use as default
    function setDefaultImplementation(address implementation) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (!isImplementationAllowed[implementation]) revert ImplementationNotAllowed();
        syndicateChainImpl = implementation;
    }

    /// @notice Ban a chain from gas tracking due to not allowed implementation (admin only)
    /// @param chainId The chain ID to ban
    /// @param notAllowedImplementation The address of the not allowed implementation
    function banChainFromGasTracking(uint256 chainId, address notAllowedImplementation)
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        _banChainFromGasTracking(chainId);
        emit ChainBannedFromGasTracking(chainId, notAllowedImplementation);
    }

    /// @notice Check if a chain is banned from gas tracking
    /// @param chainId The chain ID to check
    /// @return True if the chain is banned from gas tracking
    function isChainBannedFromGasTracking(uint256 chainId) external view returns (bool) {
        return gasTrackingBanlist[chainId];
    }

    /// @notice Get all allowed implementation addresses
    /// @return Array of allowed implementation addresses
    function getAllowedImplementations() external view returns (address[] memory) {
        return allowedImplementations;
    }

    /// @notice Internal function to ban a chain from gas tracking
    /// @param chainId The chain ID to ban
    function _banChainFromGasTracking(uint256 chainId) internal {
        if (!gasTrackingBanlist[chainId]) {
            gasTrackingBanlist[chainId] = true;
            numberOfChainsBannedFromGasTracking++;
        }
    }

    /// @notice Called by sequencing chains to notify about upgrades
    /// @dev Automatically bans chain from gas tracking if implementation is not allowed
    /// @param chainId The chain ID that is upgrading
    /// @param newImplementation The address of the new implementation
    function notifyChainUpgrade(uint256 chainId, address newImplementation) external {
        if (appchainContracts[chainId] != msg.sender) revert OnlyChainCanNotifyUpgrade();

        if (!isImplementationAllowed[newImplementation]) {
            _banChainFromGasTracking(chainId);
            emit ChainBannedFromGasTracking(chainId, newImplementation);
        }
    }
}
