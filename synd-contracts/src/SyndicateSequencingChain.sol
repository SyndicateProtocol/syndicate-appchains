// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {GasCounter} from "./staking/GasCounter.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

interface ISyndicateFactory {
    function isImplementationAllowed(address implementation) external view returns (bool);
    function notifyChainUpgrade(uint256 chainId, address newImplementation) external;
}

enum TransactionType {
    Unsigned, // an unsigned tx
    Contract, // a contract tx (unsigned, nonceless)
    _Reserved,
    Compressed, // compressed batch of transactions (signed)
    Signed // a regular signed tx

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
/// 1. Transaction is submitted via processTransaction, processTransactionsCompressed, or processTransactionsBulk
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
                STATE VARIABLES - DO NOT REORDER
    //////////////////////////////////////////////////////////////*/

    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    /// Storage slot 0
    uint256 public appchainId;

    /// @notice The address that receives emissions for this sequencing chain
    /// Storage slot 1
    address public emissionsReceiver;

    /// @notice The factory contract that deployed this chain
    /// Storage slot 2
    address public factory;

    // We use per-address contract nonces instead of a global one to increase the predictability of the request id
    // and store per-address contract tx counts for debugging purposes.
    // Note that gaps are allowed in the request id, unlike a regular nonce.
    /// Storage slot 3
    mapping(address => uint256) public contractNonce;

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
        appchainId = _appchainId;
        factory = msg.sender;
    }

    /// @notice Check if an upgrade would result in gas tracking ban without executing it
    /// @param _newImplementation The address of the new implementation contract
    /// @return wouldBeBanned True if the upgrade would result in gas tracking ban
    function checkUpgradeImpact(address _newImplementation) external view returns (bool wouldBeBanned) {
        if (factory == address(0)) return false;
        return !ISyndicateFactory(factory).isImplementationAllowed(_newImplementation);
    }

    /// @notice Authorizes contract upgrades with optional gas tracking ban protection
    /// @param _newImplementation The address of the new implementation contract
    /// @param allowGasTrackingBan If false, reverts when factory would ban this chain from gas tracking
    function authorizeUpgrade(address _newImplementation, bool allowGasTrackingBan) external onlyOwner {
        _authorizeUpgradeInternal(_newImplementation, allowGasTrackingBan);
    }

    /// @notice Authorizes contract upgrades. Only callable by the contract owner.
    /// @dev Required by UUPSUpgradeable to restrict upgradeability to the owner.
    /// @param _newImplementation The address of the new implementation contract.
    function _authorizeUpgrade(address _newImplementation) internal override onlyOwner {
        _authorizeUpgradeInternal(_newImplementation, true);
    }

    /// @notice Internal upgrade authorization logic
    /// @param _newImplementation The address of the new implementation contract
    /// @param allowGasTrackingBan Whether to allow upgrades that result in gas tracking ban
    function _authorizeUpgradeInternal(address _newImplementation, bool allowGasTrackingBan) internal {
        bool isAllowed = ISyndicateFactory(factory).isImplementationAllowed(_newImplementation);

        if (!isAllowed) {
            require(allowGasTrackingBan, "Upgrade would result in gas tracking ban");
        }

        // Notify factory about the upgrade
        ISyndicateFactory(factory).notifyChainUpgrade(appchainId, _newImplementation);
    }

    /// this is intentionally different from the standard offset used by rollups to prevent collisions
    uint160 public constant OFFSET = uint160(0x1000000000000000000000000000000000000001);

    /// @notice Utility function that converts the address in the sequencing chain
    /// that submitted a tx to the inbox to the msg.sender viewed in the appchain.
    /// @param seqAddress the address in the sequencing chain that triggered the tx to appchain
    /// @return appAddress appchain address as viewed in msg.sender
    function applyAlias(address seqAddress) public pure returns (address appAddress) {
        unchecked {
            appAddress = address(uint160(seqAddress) + OFFSET);
        }
    }

    /// @notice Utility function that converts the msg.sender viewed in the appchain
    /// to the address in the sequencing chain that submitted a tx to the inbox.
    /// @param appAddress appchain address as viewed in msg.sender
    /// @return seqAddress the address in the sequencing chain that triggered the tx to appchain
    function undoAlias(address appAddress) public pure returns (address seqAddress) {
        unchecked {
            seqAddress = address(uint160(appAddress) - OFFSET);
        }
    }

    /// @notice Send a contract transaction to the appchain using applyAlias to alias msg.sender.
    /// @param gasLimit appchain gas limit
    /// @param maxFeePerGas appchain max gas price
    /// @param to appchain destination address or zero to deploy a contract
    /// @param value appchain tx value
    /// @param data appchain tx calldata
    /// @return requestId the request id used to determine the appchain tx hash
    /// Note that unlike the inbox function, no max data size is enforced.
    function sendContractTransaction(
        uint64 gasLimit,
        uint256 maxFeePerGas,
        address to,
        uint256 value,
        bytes calldata data
    ) external trackGasUsage returns (uint256) {
        uint256 requestId = contractNonce[msg.sender]++;
        bytes memory transaction = abi.encodePacked(
            TransactionType.Contract,
            applyAlias(msg.sender),
            requestId,
            uint256(gasLimit),
            maxFeePerGas,
            uint256(uint160(to)),
            value,
            data
        );
        require(isAllowed(msg.sender, tx.origin, transaction), TransactionOrSenderNotAllowed());
        emit TransactionProcessed(msg.sender, transaction);
        return requestId;
    }

    /// @notice Send an unsigned transaction to the appchain using applyAlias to alias msg.sender.
    /// @param gasLimit appchain gas limit
    /// @param maxFeePerGas appchain max gas price
    /// @param to appchain destination address or zero to deploy a contract
    /// @param value appchain tx value
    /// @param data appchain tx calldata
    /// Note that unlike the inbox function, no max data size is enforced.
    function sendUnsignedTransaction(
        uint64 gasLimit,
        uint256 maxFeePerGas,
        uint256 nonce,
        address to,
        uint256 value,
        bytes calldata data
    ) external trackGasUsage {
        bytes memory transaction = abi.encodePacked(
            TransactionType.Unsigned,
            applyAlias(msg.sender),
            uint256(gasLimit),
            maxFeePerGas,
            nonce,
            uint256(uint160(to)),
            value,
            data
        );
        require(isAllowed(msg.sender, tx.origin, transaction), TransactionOrSenderNotAllowed());
        emit TransactionProcessed(msg.sender, transaction);
    }

    /// @notice Processes a compressed batch of signed transactions.
    /// @param data The compressed transaction data.
    //#olympix-ignore-required-tx-origin
    function processTransactionsCompressed(bytes calldata data) external trackGasUsage {
        require(data.length > 0, NoTxData());

        // ignore tx data as the tx is compressed
        require(
            isAllowed(msg.sender, tx.origin, abi.encodePacked(TransactionType.Compressed)),
            TransactionOrSenderNotAllowed()
        );
        emit TransactionProcessed(msg.sender, abi.encodePacked(TransactionType.Compressed, data));
    }

    /// @notice Process a signed transaction.
    /// @param data Transaction data
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external trackGasUsage {
        require(data.length > 0, NoTxData());

        bytes memory transaction = abi.encodePacked(TransactionType.Signed, data);
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
            bytes memory transaction = abi.encodePacked(TransactionType.Signed, data[i]);
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
        address oldReceiver = emissionsReceiver;
        emissionsReceiver = _emissionsReceiver;
        if (emissionsReceiver != address(0)) {
            emit EmissionsReceiverUpdated(oldReceiver, _emissionsReceiver);
        } else {
            emit EmissionsReceiverUpdated(oldReceiver, owner());
        }
    }

    /// @notice Get the effective emissions receiver address
    /// @dev Returns emissionsReceiver if set, otherwise returns the contract owner
    /// @return The address that should receive emissions
    function getEmissionsReceiver() external view returns (address) {
        return emissionsReceiver == address(0) ? owner() : emissionsReceiver;
    }

    /// @notice Override transferOwnership to emit EmissionsReceiverUpdated event when appropriate
    /// @dev When emissionsReceiver is not explicitly set (address(0)), transferring ownership
    /// effectively changes the emissions receiver, so we emit the event for transparency
    /// @param newOwner The address of the new owner
    function transferOwnership(address newOwner) public override onlyOwner {
        if (emissionsReceiver == address(0)) {
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
}
