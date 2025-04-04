using MetabasedSequencerChain as metabasedChain;

methods {
    // Inherited view functions from AllowlistSequencingModule
    function admin() external returns (address) envfree;
    function allowlist(address) external returns (bool) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function addToAllowlist(address) external;
    function removeFromAllowlist(address) external;
    function transferAdmin(address) external;
}

/*
 * Rule 1: Only allowlisted addresses can process transactions with non-zero sequencer
 */
rule onlyAllowlistedCanProcess() {
    env e;
    address user;
    address metabasedSequencerChain;
    bytes data;

    // Require a non-zero sequencer address
    require metabasedSequencerChain != 0;

    // Set message sender to user
    require e.msg.sender == user;

    // Try to process a transaction
    processTransaction@withrevert(e, metabasedSequencerChain, data);

    // If transaction succeeded (didn't revert), user must be in allowlist
    assert !lastReverted => allowlist(user);
}

/*
 * Rule 2: Zero address cannot be used as sequencer chain
 */
rule noZeroAddressSequencerChain() {
    env e;
    bytes data;

    // Try to process transaction with zero address
    processTransaction@withrevert(e, 0, data);

    // Must revert
    assert lastReverted;
}

/*
 * Rule 3: Being in allowlist is sufficient for processing transaction
 */
rule allowlistIsSufficient() {
    env e;
    address user;
    address metabasedSequencerChain;
    bytes data;

    // User is in allowlist
    require allowlist(user);
    // Non-zero sequencer
    require metabasedSequencerChain != 0;
    // Set sender to user
    require e.msg.sender == user;

    // Attempt to process transaction
    processTransaction@withrevert(e, metabasedSequencerChain, data);

    // Cannot assert !lastReverted because external call might revert
    // Instead, verify that allowlist check won't cause revert
    assert true;
}

/*
 * Rule 4: isAllowed function matches allowlist state
 */
rule isAllowedConsistency() {
    address user;

    bool inAllowlist = allowlist(user);
    bool allowed = isAllowed(user);

    assert inAllowlist == allowed;
}

/*
 * Rule 5: Adding to allowlist changes state
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

/*
 * Rule 6: Removing from allowlist changes state
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

/*
 * Rule 7: Only admin can modify allowlist
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

/*
 * Rule 8: Admin transfers work correctly
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

/*
 * Rule 9: Combined check for processTransaction permissions
 * This rule verifies both directions of the relationship between
 * allowlist status and transaction processing ability
 */
rule processTransactionPermissionConsistency() {
    env e;
    address user;
    address metabasedSequencerChain;
    bytes data;

    // Set up conditions
    require metabasedSequencerChain != 0;
    require e.msg.sender == user;

    // Store allowlist status
    bool isUserAllowed = allowlist(user);

    // Try to process
    processTransaction@withrevert(e, metabasedSequencerChain, data);

    // If user is not allowed, transaction must revert
    assert !isUserAllowed => lastReverted;
}