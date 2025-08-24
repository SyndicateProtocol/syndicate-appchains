// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";
import {ISyndicateSequencingChain} from "./interfaces/ISyndicateSequencingChain.sol";

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
contract SyndicateSequencingChain is SequencingModuleChecker, ISyndicateSequencingChain {
    /// @notice The ID of the App chain that this contract is sequencing transactions for.
    uint256 public immutable appchainId;

    /// @notice Emitted when a new transaction is processed
    /// @param sender The address that submitted the transaction
    /// @param data The transaction data that was processed
    event TransactionProcessed(address indexed sender, bytes data);

    /// @notice Constructs the SyndicateSequencingChain contract.
    /// @param _appchainId The ID of the App chain that this contract is sequencing transactions for.
    //#olympix-ignore-missing-revert-reason-tests
    constructor(uint256 _appchainId) SequencingModuleChecker() {
        // chain id zero has no replay protection: https://eips.ethereum.org/EIPS/eip-3788
        require(_appchainId != 0, "App chain ID cannot be 0");
        appchainId = _appchainId;
    }

    // We use per-address contract nonces instead of a global one to increase the predictability of the request id
    // and store per-address contract tx counts for debugging purposes.
    // Note that gaps are allowed in the request id, unlike a regular nonce.
    mapping(address => uint256) public contractNonce;

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
    ) external onlyWhenAllowedUnsigned(msg.sender, tx.origin) returns (uint256) {
        uint256 requestId = contractNonce[msg.sender]++;
        emit TransactionProcessed(
            msg.sender,
            abi.encodePacked(
                TransactionType.Contract,
                applyAlias(msg.sender),
                requestId,
                uint256(gasLimit),
                maxFeePerGas,
                uint256(uint160(to)),
                value,
                data
            )
        );
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
    ) external onlyWhenAllowedUnsigned(msg.sender, tx.origin) {
        emit TransactionProcessed(
            msg.sender,
            abi.encodePacked(
                TransactionType.Unsigned,
                applyAlias(msg.sender),
                uint256(gasLimit),
                maxFeePerGas,
                nonce,
                uint256(uint160(to)),
                value,
                data
            )
        );
    }

    /// @notice Processes a compressed batch of signed transactions.
    /// @param data The compressed transaction data.
    //#olympix-ignore-required-tx-origin
    function processTransactionsCompressed(bytes calldata data)
        external
        onlyWhenAllowedCompressed(msg.sender, tx.origin)
    {
        require(data.length > 0, NoTxData());
        emit TransactionProcessed(msg.sender, abi.encodePacked(TransactionType.Compressed, data));
    }

    /// @notice Process a signed transaction.
    /// @param data Transaction data
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data) {
        require(data.length > 0, NoTxData());
        emit TransactionProcessed(msg.sender, abi.encodePacked(TransactionType.Signed, data));
    }

    /// @notice Processes multiple signed transactions in bulk.
    /// @param data An array of transaction data.
    //#olympix-ignore
    function processTransactionsBulk(bytes[] calldata data) external {
        uint256 dataCount = data.length;

        // Process all transactions
        uint256 i;
        for (i = 0; i < dataCount; i++) {
            bool isAllowed = data.length > 0 && isAllowed(msg.sender, tx.origin, data[i]); //#olympix-ignore-any-tx-origin
            if (isAllowed) {
                // only emit the event if the transaction is allowed
                emit TransactionProcessed(msg.sender, abi.encodePacked(TransactionType.Signed, data[i]));
            }
        }
    }
}
