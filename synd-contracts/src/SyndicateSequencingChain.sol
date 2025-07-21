// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SequencingModuleChecker} from "./SequencingModuleChecker.sol";

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
/// 1. Transaction is submitted via processTransaction, processTransactionUncompressed, or processTransactionsBulk
/// 2. onlyWhenAllowed modifier passes both msg.sender AND tx.origin to SequencingModuleChecker
/// 3. SequencingModuleChecker delegates to the configured permissionRequirementModule
/// 4. Permission module evaluates BOTH addresses using custom logic (developer responsibility)
/// 5. If allowed, TransactionProcessed event is emitted for off-chain processing
/// 6. External systems observe events to execute transactions on the application chain
///
/// This event-based design provides scalability and gas efficiency while maintaining security
/// through modular, developer-controlled permission systems.
contract SyndicateSequencingChain is SequencingModuleChecker {
    // Maximum number of transactions that can be processed in a single bulk call to prevent DoS attacks
    uint256 public constant MAX_BULK_TRANSACTIONS = 100;

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

    /// @notice Processes a compressed transaction.
    /// @param data The compressed transaction data.
    //#olympix-ignore-required-tx-origin
    function processTransaction(bytes calldata data) external onlyWhenAllowed(msg.sender, tx.origin, data) {
        emit TransactionProcessed(msg.sender, data);
    }

    /// @notice Processes multiple uncompressed transactions in bulk.
    /// @param data An array of transaction data without prepended zero bytes.
    //#olympix-ignore-required-tx-origin
    function processTransactionUncompressed(bytes calldata data)
        external
        onlyWhenAllowed(msg.sender, tx.origin, data)
    {
        emit TransactionProcessed(msg.sender, prependZeroByte(data));
    }

    /// @notice Processes multiple transactions in bulk.
    /// @dev It prepends a zero byte to the transaction data to signal uncompressed data
    /// @param data An array of transaction data.
    //#olympix-ignore
    function processTransactionsBulk(bytes[] calldata data) external {
        if (data.length > MAX_BULK_TRANSACTIONS) revert BatchSizeExceedsMaximum();
        uint256 dataCount = data.length;

        // Process all transactions
        uint256 i;
        for (i = 0; i < dataCount; i++) {
            bool isAllowed = isAllowed(msg.sender, tx.origin, data[i]); //#olympix-ignore-any-tx-origin
            if (isAllowed) {
                // only emit the event if the transaction is allowed
                emit TransactionProcessed(msg.sender, prependZeroByte(data[i]));
            }
        }
    }

    /// @notice Prepends a zero byte to the transaction data
    /// @dev This helps op-translator identify uncompressed data
    /// @param _data The original transaction data
    /// @return bytes The transaction data with a zero byte prepended
    function prependZeroByte(bytes calldata _data) public pure returns (bytes memory) {
        return abi.encodePacked(bytes1(0x00), _data);
    }
}
