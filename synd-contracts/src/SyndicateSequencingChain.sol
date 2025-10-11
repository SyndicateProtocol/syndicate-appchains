// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

uint8 constant L2MessageType_SignedTx = 4; // a regular signed transaction

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
    UUPSUpgradeable
{
    uint256 public constant VERSION = 1_000_000; // 1.0.0 (major * 1_000_000 + minor * 1_000 + patch)

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
    /// @param _permissionRequirementModule The address of the permission requirement module or address(1) to allow all transactions
    function initialize(address admin, address _permissionRequirementModule) external initializer {
        if (admin == address(0)) revert ZeroAddress();
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
        __UUPSUpgradeable_init();
    }

    /// @notice Authorizes contract upgrades. Only callable by the contract owner.
    /// @dev Required by UUPSUpgradeable to restrict upgradeability to the owner.
    function _authorizeUpgrade(address) internal override onlyOwner {}

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
    function processTransaction(bytes calldata data) external {
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
    function processTransactionsBulk(bytes[] calldata data) external {
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
}
