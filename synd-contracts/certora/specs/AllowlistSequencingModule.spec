methods {
    // View functions
    function admin() external returns (address) envfree;
    function allowlist(address) external returns (bool) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
}

/*
 * Rule 1: Admin cannot be zero address
 */
invariant adminNotZero()
    admin() != 0;

/*
 * Rule 2: Only admin can add to allowlist
 */
rule onlyAdminCanAdd(address user) {
    env e;

    // Try to add to allowlist
    addToAllowlist@withrevert(e, user);

    // If successful, must have been admin
    assert !lastReverted => e.msg.sender == admin(),
        "Non-admin added to allowlist";
}

/*
 * Rule 3: Only admin can remove from allowlist
 */
rule onlyAdminCanRemove(address user) {
    env e;

    // Try to remove from allowlist
    removeFromAllowlist@withrevert(e, user);

    // If successful, must have been admin
    assert !lastReverted => e.msg.sender == admin(),
        "Non-admin removed from allowlist";
}

/*
 * Rule 4: Admin transfer maintains non-zero invariant
 */
rule adminTransferNonZero(address newAdmin) {
    env e;

    // Try to transfer admin
    transferAdmin@withrevert(e, newAdmin);

    // If successful, new admin cannot be zero
    assert !lastReverted => newAdmin != 0,
        "Admin transferred to zero address";
}

/*
 * Rule 5: Allowlist state changes correctly on add
 */
rule allowlistAddConsistency(address user) {
    env e;
    require e.msg.sender == admin();

    // Store initial state
    bool initiallyAllowed = allowlist(user);

    // Add to allowlist
    addToAllowlist(e, user);

    // Check state change
    bool finallyAllowed = allowlist(user);
    assert finallyAllowed, "User not added to allowlist";
}

/*
 * Rule 6: Allowlist state changes correctly on remove
 */
rule allowlistRemoveConsistency(address user) {
    env e;
    require e.msg.sender == admin();

    // Store initial state
    bool initiallyAllowed = allowlist(user);

    // Remove from allowlist
    removeFromAllowlist(e, user);

    // Check state change
    bool finallyAllowed = allowlist(user);
    assert !finallyAllowed, "User not removed from allowlist";
}

/*
 * Rule 7: isAllowed matches allowlist state
 */
rule isAllowedConsistency(address user, bytes data) {
    env e;

    bool inAllowlist = allowlist(user);
    bool allowed = isAllowed(user, user, data);

    assert inAllowlist == allowed,
        "isAllowed does not match allowlist state";
}

/*
 * Rule 8: Admin transfer changes state correctly
 */
rule adminTransferConsistency(address newAdmin) {
    env e;
    require e.msg.sender == admin();
    require newAdmin != 0;

    // Store old admin
    address oldAdmin = admin();

    // Transfer admin
    transferAdmin(e, newAdmin);

    // Verify state changes
    assert admin() == newAdmin, "Admin not updated correctly";
    assert oldAdmin != newAdmin => admin() != oldAdmin, "Old admin still in place";
}

/*
 * Rule 9: Effects of allowlist operations persist
 */
rule allowlistOperationsPersist(method f, address user) {
    env e;

    // Get initial state
    bool initiallyAllowed = allowlist(user);

    // Perform some operation
    calldataarg args;
    f(e, args);

    // State should only change through allowlist operations
    bool finallyAllowed = allowlist(user);

    assert initiallyAllowed != finallyAllowed => (
        f.selector == sig:addToAllowlist(address).selector ||
        f.selector == sig:removeFromAllowlist(address).selector
    ), "Allowlist changed through non-allowlist function";
}

/*
 * Rule 10: Only admin can transfer admin rights
 */
rule onlyAdminCanTransfer(address newAdmin) {
    env e;

    // Store old admin
    address oldAdmin = admin();

    // Try to transfer admin
    transferAdmin@withrevert(e, newAdmin);

    // If successful, must have been the current admin
    assert !lastReverted => e.msg.sender == oldAdmin,
        "Non-admin transferred admin rights";
}