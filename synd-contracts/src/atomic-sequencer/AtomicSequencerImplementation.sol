// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";

/// @title AtomicSequencerImplementation
/// @notice Implementation contract for atomic cross-chain transaction coordination
///
/// @dev SECURITY MODEL - NO ACCESS CONTROL BY DESIGN:
/// This contract intentionally lacks access control modifiers (onlyAuthorized, onlyOwner, etc.)
/// because it implements the Syndicate protocol's "secure by module design" architecture:
///
/// DESIGN PRINCIPLES:
/// • SEPARATION OF CONCERNS: AtomicSequencer coordinates, SyndicateSequencingChains authorize
/// • MODULAR SECURITY: Permission logic is delegated to customizable modules per chain
/// • MIDDLEWARE SUPPORT: Enables trusted routing contracts without breaking authorization
/// • DEVELOPER CONTROL: Module authors define who can use their chains and how
///
/// SECURITY RESPONSIBILITY DISTRIBUTION:
/// ┌─────────────────┬─────────────────────────────────────────────────────────┐
/// │ Component       │ Responsibility                                          │
/// ├─────────────────┼─────────────────────────────────────────────────────────┤
/// │ AtomicSequencer │ • Transaction routing and coordination                  │
/// │                 │ • Input validation (array lengths, size limits)        │
/// │                 │ • DoS attack prevention (MAX_ATOMIC_* constants)       │
/// ├─────────────────┼─────────────────────────────────────────────────────────┤
/// │ SequencingChain │ • Delegate to permission modules                       │
/// │                 │ • Emit transaction events                              │
/// ├─────────────────┼─────────────────────────────────────────────────────────┤
/// │ PermissionModule│ • Authorization logic (who can transact)               │
/// │ (Developer)     │ • Middleware validation (which routers to trust)       │
/// │                 │ • tx.origin + msg.sender evaluation                    │
/// └─────────────────┴─────────────────────────────────────────────────────────┘
///
/// EXAMPLE MODULE IMPLEMENTATION FOR ATOMIC SEQUENCER SUPPORT:
/// ```solidity
/// contract AtomicSequencerAwareModule is IPermissionModule {
///     mapping(address => bool) public trustedMiddleware;
///     mapping(address => bool) public authorizedUsers;
///
///     function isAllowed(address msgSender, address txOrigin, bytes calldata data)
///         external view returns (bool) {
///         // Direct calls: msg.sender == tx.origin
///         if (msgSender == txOrigin) {
///             return authorizedUsers[msgSender];
///         }
///         // Middleware calls: validate both router and user
///         return trustedMiddleware[msgSender] && authorizedUsers[txOrigin];
///     }
/// }
/// ```
///
/// @dev DoS Protection: MAX_ATOMIC_* constants prevent resource exhaustion attacks
contract AtomicSequencerImplementation {
    // Maximum number of chains that can be processed atomically to prevent DoS attacks
    uint256 public constant MAX_ATOMIC_CHAINS = 20;
    // Maximum number of transactions per chain in bulk atomic operations to prevent DoS attacks
    uint256 public constant MAX_ATOMIC_BULK_TRANSACTIONS = 50;

    /// @dev Thrown when input array lengths don't match or are invalid
    error InputLengthMismatchError();
    /// @dev Thrown when too many chains are provided for atomic processing
    error TooManyAtomicChains();
    /// @dev Thrown when transaction array exceeds maximum size for atomic bulk processing
    error TransactionArrayExceedsMaximum();

    /// @notice Processes transactions on multiple Syndicate chains atomically
    ///
    /// @dev SECURITY: This function has NO access control by design. Security is enforced by:
    /// • Each SyndicateSequencingChain's permission module evaluating both msg.sender and tx.origin
    /// • Module developers implementing logic to trust/reject middleware contracts
    /// • Chain-level authorization prevents unauthorized transaction processing
    ///
    /// @param chains Array of SyndicateSequencingChain contracts to process transactions on
    /// @param transactions Array of transaction data corresponding to each chain
    /// @param isRaw Array indicating whether each transaction uses compressed (true) or uncompressed (false) processing
    function processTransactionsAtomically(
        SyndicateSequencingChain[] calldata chains,
        bytes[] calldata transactions,
        bool[] calldata isRaw
    ) external {
        // Check array lengths match
        if (chains.length == 0 || chains.length != transactions.length || chains.length != isRaw.length) {
            revert InputLengthMismatchError();
        }
        if (chains.length > MAX_ATOMIC_CHAINS) revert TooManyAtomicChains();

        for (uint256 i = 0; i < chains.length; i++) {
            if (isRaw[i]) {
                chains[i].processTransaction(transactions[i]);
            } else {
                chains[i].processTransactionUncompressed(transactions[i]);
            }
        }
    }

    /// @notice Processes bulk transactions on multiple Syndicate chains atomically
    ///
    /// @dev SECURITY: This function has NO access control by design. Security is enforced by:
    /// • Each SyndicateSequencingChain's permission module evaluating both msg.sender and tx.origin
    /// • Module developers implementing logic to trust/reject middleware contracts
    /// • Chain-level authorization prevents unauthorized bulk transaction processing
    ///
    /// @param chains Array of SyndicateSequencingChain contracts to process bulk transactions on
    /// @param transactions Array of transaction arrays corresponding to each chain (compressed format only)
    function processTransactionsBulkAtomically(
        SyndicateSequencingChain[] calldata chains,
        bytes[][] calldata transactions
    ) external {
        if (chains.length == 0 || chains.length != transactions.length) {
            revert InputLengthMismatchError();
        }
        if (chains.length > MAX_ATOMIC_CHAINS) revert TooManyAtomicChains();

        // Check individual transaction arrays don't exceed bulk limits
        for (uint256 i = 0; i < transactions.length; i++) {
            if (transactions[i].length > MAX_ATOMIC_BULK_TRANSACTIONS) {
                revert TransactionArrayExceedsMaximum();
            }
        }

        for (uint256 i = 0; i < chains.length; i++) {
            chains[i].processTransactionsBulk(transactions[i]);
        }
    }
}
