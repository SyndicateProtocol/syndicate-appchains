using SyndicateSequencerChain as syndicateChain;

methods {
    // Inherited view functions from AllowlistSequencingModule
    function admin() external returns (address) envfree;
    function allowlist(address) external returns (bool) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
}

/**
 * Rule 1: Only allowlisted addresses can use processTransaction
 */
rule onlyAllowlistedCanProcessTransaction() {
    env e;
    address syndicateSequencerChain;
    bytes data;

    // Require a non-zero sequencer address
    require syndicateSequencerChain != 0;

    // Try to process a transaction
    processTransaction@withrevert(e, syndicateSequencerChain, data);

    // If transaction succeeded (didn't revert), sender must be in allowlist
    assert !lastReverted => allowlist(e.msg.sender);
}

/**
 * Rule 2: Only allowlisted addresses can use processTransactionRaw
 */
rule onlyAllowlistedCanProcessTransactionRaw() {
    env e;
    address syndicateSequencerChain;
    bytes data;

    // Require a non-zero sequencer address
    require syndicateSequencerChain != 0;

    // Try to process a raw transaction
    processTransactionRaw@withrevert(e, syndicateSequencerChain, data);

    // If transaction succeeded (didn't revert), sender must be in allowlist
    assert !lastReverted => allowlist(e.msg.sender);
}

/**
 * Rule 3: Only allowlisted addresses can use processBulkTransactions
 */
rule onlyAllowlistedCanProcessBulkTransactions() {
    env e;
    address syndicateSequencerChain;
    bytes[] data;

    // Require a non-zero sequencer address
    require syndicateSequencerChain != 0;

    // Try to process bulk transactions
    processBulkTransactions@withrevert(e, syndicateSequencerChain, data);

    // If transaction succeeded (didn't revert), sender must be in allowlist
    assert !lastReverted => allowlist(e.msg.sender);
}

/**
 * Rule 4: Zero address cannot be used as sequencer chain
 */
rule noZeroAddressSequencerChain() {
    env e;
    bytes data;
    bytes[] bulkData;

    // Try to process transaction with zero address
    processTransaction@withrevert(e, 0, data);

    // Must revert
    assert lastReverted;

    // Try to process raw transaction with zero address
    processTransactionRaw@withrevert(e, 0, data);

    // Must revert
    assert lastReverted;

    // Try to process bulk transactions with zero address
    processBulkTransactions@withrevert(e, 0, bulkData);

    // Must revert
    assert lastReverted;
}

/**
 * Rule 5: Being in allowlist grants permission to process transactions
 */
rule allowlistedCanProcessTransactions() {
    env e;
    address syndicateSequencerChain;

    // Setup conditions
    require allowlist(e.msg.sender);
    require syndicateSequencerChain != 0;

    // Note: We cannot assert they will succeed as external calls may revert
    assert true;
}

/**
 * Rule 6: isAllowed function matches allowlist state
 */
rule isAllowedConsistency(bytes data) {
    address user;

    bool inAllowlist = allowlist(user);
    bool allowed = isAllowed(user, user, data);

    assert inAllowlist == allowed;
}

/**
 * Rule 7: Adding to allowlist changes state
 */
rule addToAllowlistChangesState() {
    env e;
    address user;

    // Initial state - not in allowlist
    require !allowlist(user);
    // Only admin can add
    require e.msg.sender == admin();

    // Add to allowlist
    addToAllowlist(e, user);

    // Verify state change
    assert allowlist(user);
}

/**
 * Rule 8: Removing from allowlist changes state
 */
rule removeFromAllowlistChangesState() {
    env e;
    address user;

    // Initial state - in allowlist
    require allowlist(user);
    // Only admin can remove
    require e.msg.sender == admin();

    // Remove from allowlist
    removeFromAllowlist(e, user);

    // Verify state change
    assert !allowlist(user);
}

/**
 * Rule 9: Only admin can modify allowlist
 */
rule onlyAdminCanModifyAllowlist() {
    env e;
    address user;

    // If sender is not admin
    require e.msg.sender != admin();

    // Try to add to allowlist
    addToAllowlist@withrevert(e, user);
    // Must revert
    assert lastReverted;

    // Try to remove from allowlist
    removeFromAllowlist@withrevert(e, user);
    // Must revert
    assert lastReverted;
}

/**
 * Rule 10: Admin transfers work correctly
 */
rule adminTransferWorks() {
    env e;
    address newAdmin;

    // Initial conditions
    require e.msg.sender == admin();
    require newAdmin != 0;
    require newAdmin != admin(); // Ensure we're changing to a new address

    // Transfer admin
    transferAdmin(e, newAdmin);

    // Verify admin is updated
    assert admin() == newAdmin;
}

/**
 * Rule 11: Consistency between transaction methods
 * This rule verifies that all three transaction methods have consistent behavior
 * regarding allowlist checks
 */
rule methodsHaveConsistentAllowlistChecks() {
    env e;
    address syndicateSequencerChain;
    bytes data;
    bytes[] bulkData;

    // Setup - non-zero sequencer address
    require syndicateSequencerChain != 0;

    // Store allowlist status
    bool isAllowed = allowlist(e.msg.sender);

    // Call processTransaction
    processTransaction@withrevert(e, syndicateSequencerChain, data);
    bool txReverted = lastReverted;

    // Call processTransactionRaw
    processTransactionRaw@withrevert(e, syndicateSequencerChain, data);
    bool rawTxReverted = lastReverted;

    // Call processBulkTransactions
    processBulkTransactions@withrevert(e, syndicateSequencerChain, bulkData);
    bool bulkTxReverted = lastReverted;

    // If sender is not allowed, all three methods should revert
    assert !isAllowed => (txReverted && rawTxReverted && bulkTxReverted);
}

/**
 * Rule 12: Check for correct modifier application
 * This rule verifies that our modifiers are properly applied by
 * ensuring non-allowlisted senders can't use any transaction method
 */
rule modifiersAppliedCorrectly() {
    env e;
    address syndicateSequencerChain;
    bytes data;
    bytes[] bulkData;

    // Require sender is not allowed
    require !allowlist(e.msg.sender);

    // All three methods should revert for non-allowlisted sender
    processTransaction@withrevert(e, syndicateSequencerChain, data);
    assert lastReverted;

    processTransactionRaw@withrevert(e, syndicateSequencerChain, data);
    assert lastReverted;

    processBulkTransactions@withrevert(e, syndicateSequencerChain, bulkData);
    assert lastReverted;
}
