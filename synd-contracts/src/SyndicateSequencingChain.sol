// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {GasMeter} from "./staking/GasMeter.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";
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
    UUPSUpgradeable
{
    uint256 public constant VERSION = 1_000_000; // 1.0.0

    /// @notice The address of the GasMeter contract
    /// @dev This is set during initialization and never changes
    address public immutable gasMeter;

    /*//////////////////////////////////////////////////////////////
                            STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice ERC-7201 storage slot for SyndicateSequencingChain-specific data
    /// @dev Generated using: cast keccak "erc7201:syndicate.storage.SyndicateSequencingChain"
    /// This ensures the storage slot doesn't conflict with inherited contracts
    /// cast index-erc7201 syndicate.storage.SyndicateSequencingChain
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

    /*//////////////////////////////////////////////////////////////
                            ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when no transaction data is provided to processing functions
    error NoTxData();

    /// @notice Thrown when the transaction or sender is not allowed by the permission module
    error TransactionOrSenderNotAllowed();

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when the caller of a function is not the GasMeter
    error NotGasMeterContract();

    /*//////////////////////////////////////////////////////////////
                            EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a new transaction is processed
    /// @param sender The address that submitted the transaction
    /// @param data The transaction data that was processed
    event TransactionProcessed(address indexed sender, bytes data);

    /*//////////////////////////////////////////////////////////////
                            FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disables initializers to prevent the implementation contract from being initialized
    constructor(address _gasMeter) {
        if (_gasMeter == address(0)) revert ZeroAddress();
        gasMeter = _gasMeter;
        _disableInitializers();
    }

    function getInitializedVersion() external view returns (uint64) {
        return _getInitializedVersion();
    }

    /// @notice Initializes the SyndicateSequencingChain contract
    /// @dev This function can only be called once during proxy deployment. It sets up all the core functionality
    ///      including ownership, permission modules, gas tracking, and appchain identification.
    /// @param admin The address to be set as the contract owner (receives DEFAULT_ADMIN_ROLE)
    /// @param _permissionRequirementModule The address of the permission requirement module or address(0) to allow all transactions
    /// @param _appchainId The unique identifier for the application chain this contract sequences for (must not be 0)
    function initialize(address admin, address _permissionRequirementModule, uint256 _appchainId) external initializer {
        if (admin == address(0)) revert ZeroAddress();
        require(_appchainId != 0, "App chain ID cannot be 0");
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
        __UUPSUpgradeable_init();

        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.appchainId = _appchainId;
    }

    modifier onlyGasMeter() {
        require(msg.sender == gasMeter, NotGasMeterContract());
        _;
    }

    /// @notice Authorizes contract upgrades. Only callable by the contract owner.
    /// @dev Required by UUPSUpgradeable to restrict upgradeability to the owner.
    /// @param _newImplementation The address of the new implementation contract.
    function _authorizeUpgrade(address _newImplementation) internal override onlyOwner {}

    /// @notice Encode transaction data with L2 message type prefix
    /// @dev Prepends the transaction data with the L2MessageType_SignedTx identifier
    ///      This encoding is used by off-chain systems to identify transaction types
    /// @param data The raw transaction data to encode
    /// @return The encoded transaction data with message type prefix
    function encodeTransaction(bytes calldata data) public pure returns (bytes memory) {
        return abi.encodePacked(L2MessageType_SignedTx, data);
    }

    /// @notice Processes a single signed transaction by forwarding it to the gas meter contract for gas accounting and validation.
    /// @dev Requires that the transaction data is not empty. Delegates the call to processTransactionImpl on the configured gas meter contract.
    /// @param data The transaction data to process (must not be empty).
    function processTransaction(bytes calldata data) external {
        GasMeter(gasMeter).meterCall(abi.encodeCall(SyndicateSequencingChain._processTransaction, (msg.sender, data)));
    }

    /// @notice Process a single signed transaction
    /// @dev Validates the transaction through the permission module and emits an event if authorized.
    ///      The tx.origin is intentionally used as part of the security model - see contract-level documentation.
    /// @param sequencer The address of original sequencer that is processing the transaction
    /// @param data The transaction data to process (must not be empty)
    //#olympix-ignore-required-tx-origin
    function _processTransaction(address sequencer, bytes calldata data) external onlyGasMeter {
        require(data.length > 0, NoTxData());

        // Encode transaction with L2 message type for off-chain processing
        bytes memory transaction = encodeTransaction(data);

        // Check authorization through permission module (considers both sequencer and tx.origin)
        require(isAllowed(sequencer, tx.origin, transaction), TransactionOrSenderNotAllowed());

        // Emit event for off-chain systems to execute on application chain
        emit TransactionProcessed(sequencer, transaction);
    }

    /// @notice Processes multiple signed transactions in a single call by forwarding them to the gas meter contract for gas accounting and validation.
    /// @dev Each transaction in the array must be non-empty. This function delegates processing to the configured gas meter contract for each transaction.
    /// @param data An array of transaction data to process (must not be empty).
    function processTransactionsBulk(bytes[] calldata data) external {
        GasMeter(gasMeter)
            .meterCall(abi.encodeCall(SyndicateSequencingChain._processTransactionsBulk, (msg.sender, data)));
    }

    /// @notice Processes multiple signed transactions in bulk for gas efficiency
    /// @dev Each transaction is individually validated through the permission module.
    ///      Only authorized transactions emit events, unauthorized ones are silently skipped.
    ///      The tx.origin is intentionally used as part of the security model.
    /// @param sequencer The address of original sequencer that is processing the transactions.
    /// @param data An array of transaction data to process (must not be empty)
    //#olympix-ignore
    function _processTransactionsBulk(address sequencer, bytes[] calldata data) external onlyGasMeter {
        uint256 dataCount = data.length;
        require(dataCount > 0, NoTxData());

        // Process all transactions individually
        for (uint256 i = 0; i < dataCount; i++) {
            require(data[i].length > 0, NoTxData());

            // Encode transaction with L2 message type
            bytes memory transaction = encodeTransaction(data[i]);

            // Check authorization (considers both sequencer and tx.origin)
            bool allowed = isAllowed(sequencer, tx.origin, transaction); //#olympix-ignore-any-tx-origin

            if (allowed) {
                // Only emit event for authorized transactions
                emit TransactionProcessed(sequencer, transaction);
            }
        }
    }
}
