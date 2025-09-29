// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {GasCounter} from "./staking/GasCounter.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";
import {IGasAggregator} from "./interfaces/IGasAggregator.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

uint8 constant L2MessageType_SignedTx = 4; // a regular signed transaction

/// @notice Storage struct for SyndicateSequencingChain using ERC-7201 namespaced storage pattern
/// @dev This struct contains all the state variables specific to the sequencing chain functionality.
///      Using ERC-7201 ensures storage slots don't conflict during upgrades.
/// @custom:storage-location erc7201:syndicate.storage.SyndicateSequencingChain
struct SyndicateSequencingChainStorage {
    /// @notice The ID of the App chain that this contract is sequencing transactions for
    /// @dev This is set during initialization and never changes
    uint256 appchainId;
    /// @notice The address that receives emissions for this sequencing chain
    /// @dev If set to address(0), emissions go to the contract owner
    address emissionsReceiver;
    /// @notice Whether to allow gas tracking ban on upgrade (defaults to false for security)
    /// @dev When false, prevents upgrades to implementations not allowed by the gas aggregator
    bool allowGasTrackingBanOnUpgrade;
    /// @notice Gas aggregator contract for tracking gas usage across epochs
    /// @dev Used to report gas usage and receive notifications about implementation changes
    IGasAggregator gasAggregator;
    /// @notice Address of the factory that created this sequencing chain
    address factory;
    /// @notice Version of the SyndicateSequencingChain contract (updatable during upgrades)
    /// @dev Semantic version string to track implementation upgrades
    string version;
}

/// @title SyndicateSequencingChain
/// @notice Core contract for transaction sequencing using Syndicate's "secure by module design" architecture
///
/// @dev ARCHITECTURAL DESIGN - tx.origin USAGE BY DESIGN:
/// This contract intentionally uses tx.origin alongside msg.sender to enable sophisticated middleware patterns:
///
/// USE CASES ENABLED:
/// • ATOMIC CROSS-CHAIN SEQUENCING: AtomicSequencer coordinating multiple chains
/// • TRUSTED MIDDLEWARE: Third-party contracts adding logic layers
/// • BATCH PROCESSING: Routing contracts that aggregate transactions
/// • COMPLEX AUTHORIZATION: Modules that consider both caller and originator
///
/// SECURITY MODEL - "SECURE BY MODULE DESIGN":
/// Security is NOT enforced by this contract, but by developer-implemented permission modules:
///
/// ┌─────────────────────────────────────────────────────────────────────────┐
/// │ RESPONSIBILITY DISTRIBUTION:                                            │
/// ├─────────────────────────────────────────────────────────────────────────┤
/// │ SyndicateSequencingChain: Routes to permission modules                  │
/// │ PermissionModule (Dev): Implements authorization logic                  │
/// │ Module Developer: MUST validate both msg.sender and tx.origin properly  │
/// └─────────────────────────────────────────────────────────────────────────┘
///
/// @dev Transaction Lifecycle:
/// 1. Transaction is submitted via processTransaction or processTransactionsBulk
/// 2. onlyWhenAllowed modifier passes both msg.sender AND tx.origin to SequencingModuleChecker
/// 3. SequencingModuleChecker delegates to the configured permissionRequirementModule
/// 4. Permission module evaluates BOTH addresses using custom logic (developer responsibility)
/// 5. If allowed, TransactionProcessed event is emitted for off-chain processing
/// 6. External systems observe events to execute transactions on the application chain
///
/// This event-based design provides scalability and gas efficiency while maintaining security
/// through modular, developer-controlled permission systems.
///
/// To view the storage layout, run "forge inspect SyndicateSequencingChain storageLayout"
contract SyndicateSequencingChain is
    Initializable,
    SequencingModuleChecker,
    ISyndicateSequencingChain,
    GasCounter,
    UUPSUpgradeable
{
    /*//////////////////////////////////////////////////////////////
                            STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice ERC-7201 storage slot for SyndicateSequencingChain-specific data
    /// @dev Generated using: cast keccak "erc7201:syndicate.storage.SyndicateSequencingChain"
    ///      This ensures the storage slot doesn't conflict with inherited contracts
    // cast index-erc7201 syndicate.storage.SyndicateSequencingChain
    bytes32 public constant SYNDICATE_SEQUENCING_CHAIN_STORAGE_LOCATION =
        0xc541a3613bd22a8da1c897658e95c42e6bb9158c83d62ac963646ba27200a400;

    /// @notice Internal function to access the ERC-7201 namespaced storage
    /// @dev Uses inline assembly to access the specific storage slot for this contract's data
    /// @return $ Storage pointer to the SyndicateSequencingChainStorage struct
    function _getSyndicateSequencingChainStorage() private pure returns (SyndicateSequencingChainStorage storage $) {
        assembly {
            $.slot := SYNDICATE_SEQUENCING_CHAIN_STORAGE_LOCATION
        }
    }

    /// @notice Get the appchain ID that this contract sequences for
    /// @return The unique identifier of the application chain
    function appchainId() public view returns (uint256) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.appchainId;
    }

    /// @notice Get the configured emissions receiver address
    /// @dev Returns the specific receiver if set, or address(0) if using default (owner)
    /// @return The address configured to receive emissions, or address(0) for default behavior
    function emissionsReceiver() public view returns (address) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.emissionsReceiver;
    }

    /// @notice Check if gas tracking ban on upgrade is allowed
    /// @dev When false, prevents upgrades to implementations not approved by the gas aggregator
    /// @return True if upgrades that would cause gas tracking bans are allowed
    function allowGasTrackingBanOnUpgrade() public view returns (bool) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.allowGasTrackingBanOnUpgrade;
    }

    /// @notice Get the gas aggregator contract address
    /// @return The gas aggregator contract interface
    function gasAggregator() public view returns (IGasAggregator) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.gasAggregator;
    }

    /// @notice Get the factory address that created this sequencing chain
    /// @return The factory address
    function factory() public view returns (address) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.factory;
    }

    /// @notice Get the current version of this contract implementation
    /// @return The semantic version string of this contract
    function version() public view returns (string memory) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.version;
    }

    /*//////////////////////////////////////////////////////////////
                            ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when no transaction data is provided to processing functions
    error NoTxData();

    /// @notice Thrown when the transaction or sender is not allowed by the permission module
    error TransactionOrSenderNotAllowed();

    /// @notice Thrown when an upgrade would result in gas tracking being banned
    /// @dev This protects against upgrades to non-approved implementations when protection is enabled
    error UpgradeWouldResultInGasTrackingBan();

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when a non-factory address tries to call a factory-only function
    error OnlyFactory();

    /*//////////////////////////////////////////////////////////////
                            EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a new transaction is processed
    /// @param sender The address that submitted the transaction
    /// @param data The transaction data that was processed
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Emitted when the emissions receiver is updated
    /// @param oldReceiver The previous emissions receiver address
    /// @param newReceiver The new emissions receiver address
    event EmissionsReceiverUpdated(address indexed oldReceiver, address indexed newReceiver);

    /// @notice Emitted when the gas aggregator notification failed
    /// @param gasAggregator The address of the gas aggregator
    event gasAggregatorNotificationFailed(address indexed gasAggregator);

    /*//////////////////////////////////////////////////////////////
                            FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disables initializers to prevent the implementation contract from being initialized
    constructor() {
        _disableInitializers();
    }

    function getInitializedVersion() external view returns (uint64) {
        return _getInitializedVersion();
    }

    /// @notice Initializes the SyndicateSequencingChain contract
    /// @dev This function can only be called once during proxy deployment. It sets up all the core functionality
    ///      including ownership, permission modules, gas tracking, and appchain identification.
    /// @param admin The address to be set as the contract owner (receives DEFAULT_ADMIN_ROLE)
    /// @param _gasAggregator The gas aggregator contract for tracking gas usage across epochs
    /// @param _permissionRequirementModule The address of the permission requirement module or address(0) to allow all transactions
    /// @param _appchainId The unique identifier for the application chain this contract sequences for (must not be 0)
    /// @param _gasTokensUsedForCurrentEpoch The amount of gas tokens already used for the current epoch (used for legacy migrations, 0 for new chains)
    function initialize(
        address admin,
        address _factory,
        address _gasAggregator,
        address _permissionRequirementModule,
        uint256 _appchainId,
        uint256 _gasTokensUsedForCurrentEpoch
    ) external initializer {
        if (admin == address(0)) revert ZeroAddress();
        if (_factory == address(0)) revert ZeroAddress();
        if (_gasAggregator == address(0)) revert ZeroAddress();
        require(_appchainId != 0, "App chain ID cannot be 0");
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
        __UUPSUpgradeable_init();
        _enableGasTracking();

        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.appchainId = _appchainId;
        $.allowGasTrackingBanOnUpgrade = false;
        $.version = "1.0.0";
        $.gasAggregator = IGasAggregator(_gasAggregator);
        $.factory = _factory;

        // Set initial gas usage for migrations (0 for new chains)
        if (_gasTokensUsedForCurrentEpoch > 0) {
            _getGasCounterStorage().tokensUsedPerEpoch[getCurrentEpoch()] = _gasTokensUsedForCurrentEpoch;
        }
    }

    /// @notice Authorizes contract upgrades. Only callable by the contract owner.
    /// @dev Required by UUPSUpgradeable to restrict upgradeability to the owner.
    /// @param _newImplementation The address of the new implementation contract.
    function _authorizeUpgrade(address _newImplementation) internal override onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        IGasAggregator gasAggr = $.gasAggregator;

        // Check if upgrade protection is enabled and implementation is allowed
        if (!$.allowGasTrackingBanOnUpgrade) {
            bool isAllowed = gasAggr.allowedImplementations(_newImplementation);
            if (!isAllowed) {
                revert UpgradeWouldResultInGasTrackingBan();
            }
        }

        // Notify gas aggregator about the upgrade
        try gasAggr.notifyChainUpgrade(appchainId(), _newImplementation) {}
        catch {
            emit gasAggregatorNotificationFailed(address(gasAggr));
        }
    }

    /// @notice Encode transaction data with L2 message type prefix
    /// @dev Prepends the transaction data with the L2MessageType_SignedTx identifier
    ///      This encoding is used by off-chain systems to identify transaction types
    /// @param data The raw transaction data to encode
    /// @return The encoded transaction data with message type prefix
    function encodeTransaction(bytes calldata data) public pure returns (bytes memory) {
        return abi.encodePacked(L2MessageType_SignedTx, data);
    }

    /// @notice Process a single signed transaction
    /// @dev Validates the transaction through the permission module and emits an event if authorized.
    ///      The tx.origin is intentionally used as part of the security model - see contract-level documentation.
    /// @param data The transaction data to process (must not be empty)
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external trackGasUsage {
        require(data.length > 0, NoTxData());

        // Encode transaction with L2 message type for off-chain processing
        bytes memory transaction = encodeTransaction(data);

        // Check authorization through permission module (considers both msg.sender and tx.origin)
        require(isAllowed(msg.sender, tx.origin, transaction), TransactionOrSenderNotAllowed());

        // Emit event for off-chain systems to execute on application chain
        emit TransactionProcessed(msg.sender, transaction);
    }

    /// @notice Processes multiple signed transactions in bulk for gas efficiency
    /// @dev Each transaction is individually validated through the permission module.
    ///      Only authorized transactions emit events, unauthorized ones are silently skipped.
    ///      The tx.origin is intentionally used as part of the security model.
    /// @param data An array of transaction data to process (must not be empty)
    //#olympix-ignore
    function processTransactionsBulk(bytes[] calldata data) external trackGasUsage {
        uint256 dataCount = data.length;
        require(dataCount > 0, NoTxData());

        // Process all transactions individually
        uint256 i;
        for (i = 0; i < dataCount; i++) {
            require(data[i].length > 0, NoTxData());

            // Encode transaction with L2 message type
            bytes memory transaction = encodeTransaction(data[i]);

            // Check authorization (considers both msg.sender and tx.origin)
            bool allowed = isAllowed(msg.sender, tx.origin, transaction); //#olympix-ignore-any-tx-origin

            if (allowed) {
                // Only emit event for authorized transactions
                emit TransactionProcessed(msg.sender, transaction);
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                         EMISSIONS RECEIVER ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the emissions receiver address
    /// @dev Only callable by the contract owner. Setting to address(0) reverts to using the owner as receiver.
    /// @param _emissionsReceiver The address to receive emissions, or address(0) to use owner
    function setEmissionsReceiver(address _emissionsReceiver) external onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        address oldReceiver = $.emissionsReceiver;
        $.emissionsReceiver = _emissionsReceiver;

        if ($.emissionsReceiver != address(0)) {
            emit EmissionsReceiverUpdated(oldReceiver, _emissionsReceiver);
        } else {
            emit EmissionsReceiverUpdated(oldReceiver, owner());
        }
    }

    /// @notice Get the effective emissions receiver address
    /// @dev Returns the configured emissionsReceiver if set, otherwise returns the contract owner.
    ///      This is the address that should actually receive emissions from the system.
    /// @return The address that should receive emissions for this sequencing chain
    function getEmissionsReceiver() external view returns (address) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.emissionsReceiver == address(0) ? owner() : $.emissionsReceiver;
    }

    /// @notice Override transferOwnership to emit EmissionsReceiverUpdated event when appropriate
    /// @dev When emissionsReceiver is not explicitly set (address(0)), transferring ownership
    ///      effectively changes the emissions receiver, so we emit the event for transparency.
    ///      This ensures emissions tracking remains accurate across ownership changes.
    /// @param newOwner The address of the new owner
    function transferOwnership(address newOwner) public override onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();

        // If using default emissions receiver (owner), emit event about the change
        if ($.emissionsReceiver == address(0)) {
            emit EmissionsReceiverUpdated(owner(), newOwner);
        }
        super.transferOwnership(newOwner);
    }

    /// @notice Updates the contract version string (owner only, typically called during upgrades)
    /// @dev This is for tracking and debugging purposes, allowing operators to identify which version is running.
    /// @param newVersion The new semantic version string (e.g., "1.1.0")
    function updateVersion(string calldata newVersion) external onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.version = newVersion;
    }

    /*//////////////////////////////////////////////////////////////
                         GAS TRACKING ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disable gas tracking if needed
    /// @dev Only callable by the contract owner
    function disableGasTracking() external onlyOwner {
        _disableGasTracking();
    }

    /// @notice Enable gas tracking
    /// @dev Only callable by the contract owner
    function enableGasTracking() external onlyOwner {
        _enableGasTracking();
    }

    /// @notice Set whether to allow gas tracking ban on upgrade
    /// @dev Only callable by the contract owner. When set to false (default), upgrades to non-approved
    ///      implementations will be blocked to maintain gas tracking eligibility. When true, allows
    ///      upgrades that might result in gas tracking being banned.
    /// @param _allowGasTrackingBanOnUpgrade Whether to allow upgrades that would result in gas tracking bans
    function setAllowGasTrackingBanOnUpgrade(bool _allowGasTrackingBanOnUpgrade) external onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.allowGasTrackingBanOnUpgrade = _allowGasTrackingBanOnUpgrade;
    }

    /// @notice Set the gas aggregator address
    /// @dev Only callable by the factory contract
    /// @param newGasAggregator The address of the new gas aggregator
    function setGasAggregator(IGasAggregator newGasAggregator) external {
        if (msg.sender != _getSyndicateSequencingChainStorage().factory) revert OnlyFactory();
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.gasAggregator = newGasAggregator;
    }
}
