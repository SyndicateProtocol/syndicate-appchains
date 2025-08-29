// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {GasCounter} from "./staking/GasCounter.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";
import {
    L2MessageType_unsignedEOATx,
    L2MessageType_unsignedContractTx
} from "@arbitrum/nitro-contracts/src/libraries/MessageTypes.sol";

uint8 constant L2MessageType_Batch = 3; // compressed batch of transactions (signed)
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
contract SyndicateSequencingChain is SequencingModuleChecker, ISyndicateSequencingChain, GasCounter {
    /// this is intentionally different from the standard offset used by rollups to prevent collisions
    uint160 public constant OFFSET = uint160(0x1000000000000000000000000000000000000001);

    error NoTxData();
    error TransactionOrSenderNotAllowed();
    error SyndicateForkDisabled();
    error GasTrackingAlreadyEnabled();
    error GasTrackingAlreadyDisabled();

    /// @notice Emitted when a new transaction is processed
    /// @param sender The address that submitted the transaction
    /// @param data The transaction data that was processed
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Emitted when the emissions receiver is updated
    /// @param oldReceiver The previous emissions receiver address
    /// @param newReceiver The new emissions receiver address
    event EmissionsReceiverUpdated(address indexed oldReceiver, address indexed newReceiver);

    uint256 public immutable appchainId;

    /// @notice The address that receives emissions for this sequencing chain
    address public emissionsReceiver;

    bool public syndicateForkEnabled;

    // We use per-address contract nonces instead of a global one to increase the predictability of the request id
    // and store per-address contract tx counts for debugging purposes.
    // Note that gaps are allowed in the request id, unlike a regular nonce.
    mapping(address => uint256) public contractNonce;

    /// @notice Constructs the SyndicateSequencingChain contract.
    /// @param _appchainId The ID of the App chain that this contract is sequencing transactions for.
    //#olympix-ignore-missing-revert-reason-tests
    constructor(uint256 _appchainId, bool _syndicateForkEnabled) SequencingModuleChecker() {
        // chain id zero has no replay protection: https://eips.ethereum.org/EIPS/eip-3788
        require(_appchainId != 0, "App chain ID cannot be 0");
        appchainId = _appchainId;
        syndicateForkEnabled = _syndicateForkEnabled;
    }

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
        require(syndicateForkEnabled, SyndicateForkDisabled());
        uint256 requestId = contractNonce[msg.sender]++;
        bytes memory transaction = abi.encodePacked(
            L2MessageType_unsignedContractTx,
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
        require(syndicateForkEnabled, SyndicateForkDisabled());
        bytes memory transaction = abi.encodePacked(
            L2MessageType_unsignedEOATx,
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
            isAllowed(msg.sender, tx.origin, abi.encodePacked(L2MessageType_Batch)), TransactionOrSenderNotAllowed()
        );
        emit TransactionProcessed(msg.sender, abi.encodePacked(L2MessageType_Batch, data));
    }

    /// @notice Process a signed transaction.
    /// @param data Transaction data
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external trackGasUsage {
        require(data.length > 0, NoTxData());

        bytes memory transaction = abi.encodePacked(L2MessageType_SignedTx, data);
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
            bytes memory transaction = abi.encodePacked(L2MessageType_SignedTx, data[i]);
            bool isAllowed = isAllowed(msg.sender, tx.origin, transaction); //#olympix-ignore-any-tx-origin
            if (isAllowed) {
                // only emit the event if the transaction is allowed
                emit TransactionProcessed(msg.sender, transaction);
            }
        }
    }

    function enableSyndicateFork() external onlyOwner {
        syndicateForkEnabled = true;
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
        require(!gasTrackingDisabled, GasTrackingAlreadyDisabled());
        gasTrackingDisabled = true;
    }

    /// @notice Enable gas tracking
    /// @dev Only callable by the contract owner
    function enableGasTracking() external onlyOwner {
        require(gasTrackingDisabled, GasTrackingAlreadyEnabled());
        gasTrackingDisabled = false;
    }
}
