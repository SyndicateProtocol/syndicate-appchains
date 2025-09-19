// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {GasCounter} from "./staking/GasCounter.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

interface ISyndicateFactory {
    function isImplementationAllowed(address implementation) external view returns (bool);
    function notifyChainUpgrade(uint256 chainId, address newImplementation) external;
}

uint8 constant L2MessageType_SignedTx = 4; // a regular signed transaction

/// @custom:storage-location erc7201:syndicate.storage.SyndicateSequencingChain
struct SyndicateSequencingChainStorage {
    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    uint256 appchainId;
    /// @notice The address that receives emissions for this sequencing chain
    address emissionsReceiver;
    /// @notice The factory contract that deployed this chain
    address factory;
    /// @notice Whether to allow gas tracking ban on upgrade (defaults to true for backwards compatibility)
    bool allowGasTrackingBanOnUpgrade;
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
/// │ SyndicateSequencingChain: Routes to permission modules                 │
/// │ PermissionModule (Dev): Implements authorization logic                  │
/// │ Module Developer: MUST validate both msg.sender and tx.origin properly │
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
    error NoTxData();
    error TransactionOrSenderNotAllowed();

    /*//////////////////////////////////////////////////////////////
                            STORAGE
    //////////////////////////////////////////////////////////////*/

    // cast index-erc7201 syndicate.storage.SyndicateSequencingChain
    bytes32 public constant SYNDICATE_SEQUENCING_CHAIN_STORAGE_LOCATION =
        0xc541a3613bd22a8da1c897658e95c42e6bb9158c83d62ac963646ba27200a400;

    function _getSyndicateSequencingChainStorage() private pure returns (SyndicateSequencingChainStorage storage $) {
        assembly {
            $.slot := SYNDICATE_SEQUENCING_CHAIN_STORAGE_LOCATION
        }
    }

    function appchainId() public view returns (uint256) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.appchainId;
    }

    function emissionsReceiver() public view returns (address) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.emissionsReceiver;
    }

    function factory() public view returns (address) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.factory;
    }

    function allowGasTrackingBanOnUpgrade() public view returns (bool) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.allowGasTrackingBanOnUpgrade;
    }

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
    /// @dev This function can only be called once. It sets the admin, permission requirement module, and appchain ID.
    /// @param admin The address to be set as the contract owner
    /// @param _permissionRequirementModule The address of the permission requirement module (e.g., RequireAndModule) or 0 to allow tranasctions
    /// @param _appchainId The unique identifier for the application chain this contract sequences for (must not be 0)
    function initialize(address admin, address _permissionRequirementModule, uint256 _appchainId)
        external
        initializer
    {
        require(_appchainId != 0, "App chain ID cannot be 0");
        __SequencingModuleChecker_init(admin, _permissionRequirementModule);
        __UUPSUpgradeable_init();
        _enableGasTracking();

        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.appchainId = _appchainId;
        $.factory = msg.sender;
        $.allowGasTrackingBanOnUpgrade = false;
    }

    /// @notice Authorizes contract upgrades. Only callable by the contract owner.
    /// @dev Required by UUPSUpgradeable to restrict upgradeability to the owner.
    /// @param _newImplementation The address of the new implementation contract.
    function _authorizeUpgrade(address _newImplementation) internal override onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        bool isAllowed = ISyndicateFactory($.factory).isImplementationAllowed(_newImplementation);

        if (!isAllowed) {
            require($.allowGasTrackingBanOnUpgrade, "Upgrade would result in gas tracking ban");
        }

        // Notify factory about the upgrade
        ISyndicateFactory($.factory).notifyChainUpgrade($.appchainId, _newImplementation);
    }

    function encodeTransaction(bytes calldata data) public pure returns (bytes memory) {
        return abi.encodePacked(L2MessageType_SignedTx, data);
    }

    /// @notice Process a signed transaction.
    /// @param data Transaction data
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external trackGasUsage {
        require(data.length > 0, NoTxData());

        bytes memory transaction = encodeTransaction(data);
        require(isAllowed(msg.sender, tx.origin, transaction), TransactionOrSenderNotAllowed());
        emit TransactionProcessed(msg.sender, transaction);
    }

    /// @notice Processes multiple signed transactions in bulk.
    /// @param data An array of transaction data.
    //#olympix-ignore
    function processTransactionsBulk(bytes[] calldata data) external trackGasUsage {
        uint256 dataCount = data.length;
        require(dataCount > 0, NoTxData());

        // Process all transactions
        uint256 i;
        for (i = 0; i < dataCount; i++) {
            require(data[i].length > 0, NoTxData());
            bytes memory transaction = encodeTransaction(data[i]);
            bool isAllowed = isAllowed(msg.sender, tx.origin, transaction); //#olympix-ignore-any-tx-origin
            if (isAllowed) {
                // only emit the event if the transaction is allowed
                emit TransactionProcessed(msg.sender, transaction);
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                         EMISSIONS RECEIVER ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the emissions receiver address
    /// @dev Only callable by the contract owner
    /// @param _emissionsReceiver The address to receive emissions
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
    /// @dev Returns emissionsReceiver if set, otherwise returns the contract owner
    /// @return The address that should receive emissions
    function getEmissionsReceiver() external view returns (address) {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        return $.emissionsReceiver == address(0) ? owner() : $.emissionsReceiver;
    }

    /// @notice Override transferOwnership to emit EmissionsReceiverUpdated event when appropriate
    /// @dev When emissionsReceiver is not explicitly set (address(0)), transferring ownership
    /// effectively changes the emissions receiver, so we emit the event for transparency
    /// @param newOwner The address of the new owner
    function transferOwnership(address newOwner) public override onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        if ($.emissionsReceiver == address(0)) {
            emit EmissionsReceiverUpdated(owner(), newOwner);
        }
        super.transferOwnership(newOwner);
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
    /// @dev Only callable by the contract owner
    /// @param _allowGasTrackingBanOnUpgrade Whether to allow gas tracking ban on upgrade
    function setAllowGasTrackingBanOnUpgrade(bool _allowGasTrackingBanOnUpgrade) external onlyOwner {
        SyndicateSequencingChainStorage storage $ = _getSyndicateSequencingChainStorage();
        $.allowGasTrackingBanOnUpgrade = _allowGasTrackingBanOnUpgrade;
    }
}
