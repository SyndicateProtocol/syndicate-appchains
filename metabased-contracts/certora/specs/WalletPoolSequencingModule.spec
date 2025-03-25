methods {
    // View functions
    function hasRole(bytes32, address) external returns (bool) envfree;
    function getRoleAdmin(bytes32) external returns (bytes32) envfree;
    function walletPool(address) external returns (bool) envfree;
    function isAllowed(address) external returns (bool) envfree;

    // Constants
    function MANAGER_ROLE() external returns (bytes32) envfree;
    function DEFAULT_ADMIN_ROLE() external returns (bytes32) envfree;
}

/*
 * Rule 1: DEFAULT_ADMIN_ROLE is the admin of MANAGER_ROLE
 */
invariant managerRoleAdminIsDefaultAdmin()
    getRoleAdmin(MANAGER_ROLE()) == DEFAULT_ADMIN_ROLE();

/*
 * Rule 2: Only managers can add to wallet pool
 */
rule onlyManagersCanAdd(address user) {
    env e;

    // Try to add to wallet pool
    addToWalletPool@withrevert(e, user);

    // If successful, must have been a manager
    assert !lastReverted => hasRole(MANAGER_ROLE(), e.msg.sender),
        "Non-manager added to wallet pool";
}

/*
 * Rule 3: Only managers can remove from wallet pool
 */
rule onlyManagersCanRemove(address user) {
    env e;

    // Try to remove from wallet pool
    removeFromWalletPool@withrevert(e, user);

    // If successful, must have been a manager
    assert !lastReverted => hasRole(MANAGER_ROLE(), e.msg.sender),
        "Non-manager removed from wallet pool";
}

/*
 * Rule 4: Wallet pool state changes correctly on add
 */
rule walletPoolAddConsistency(address user) {
    env e;
    require hasRole(MANAGER_ROLE(), e.msg.sender);

    // Store initial state
    bool initiallyInPool = walletPool(user);

    // Add to wallet pool
    addToWalletPool(e, user);

    // Check state change
    bool finallyInPool = walletPool(user);
    assert finallyInPool, "User not added to wallet pool";
}

/*
 * Rule 5: Wallet pool state changes correctly on remove
 */
rule walletPoolRemoveConsistency(address user) {
    env e;
    require hasRole(MANAGER_ROLE(), e.msg.sender);

    // Store initial state
    bool initiallyInPool = walletPool(user);

    // Remove from wallet pool
    removeFromWalletPool(e, user);

    // Check state change
    bool finallyInPool = walletPool(user);
    assert !finallyInPool, "User not removed from wallet pool";
}

/*
 * Rule 6: isAllowed matches wallet pool state
 */
rule isAllowedConsistency(address user) {
    env e;

    bool inWalletPool = walletPool(user);
    bool allowed = isAllowed(e, user);

    assert inWalletPool == allowed,
        "isAllowed does not match wallet pool state";
}

/*
 * Rule 7: Effects of wallet pool operations persist
 */
rule walletPoolOperationsPersist(method f, address user) {
    env e;

    // Get initial state
    bool initiallyInPool = walletPool(user);

    // Perform some operation
    calldataarg args;
    f(e, args);

    // State should only change through wallet pool operations
    bool finallyInPool = walletPool(user);

    assert initiallyInPool != finallyInPool => (
        f.selector == sig:addToWalletPool(address).selector ||
        f.selector == sig:removeFromWalletPool(address).selector
    ), "Wallet pool changed through non-wallet pool function";
}

/*
 * Rule 8: Only role admins can grant manager role
 */
rule onlyRoleAdminCanGrantRole(address newManager) {
    env e;
    bytes32 role = MANAGER_ROLE();
    bytes32 adminRole = getRoleAdmin(role);

    // Try to grant manager role
    grantRole@withrevert(e, role, newManager);

    // If successful, caller must have admin role
    assert !lastReverted => hasRole(adminRole, e.msg.sender),
        "Non-admin granted manager role";
}

/*
 * Rule 9: Only role admins can revoke manager role
 */
rule onlyRoleAdminCanRevokeRole(address manager) {
    env e;
    bytes32 role = MANAGER_ROLE();
    bytes32 adminRole = getRoleAdmin(role);

    // Try to revoke manager role
    revokeRole@withrevert(e, role, manager);

    // If successful, caller must have admin role
    assert !lastReverted => hasRole(adminRole, e.msg.sender),
        "Non-admin revoked manager role";
}

/*
 * Rule 10: Role changes affect permissions correctly
 */
rule roleChangesAffectPermissions(address user, address manager) {
    env e1;
    env e2;

    // Initial state
    require !walletPool(user);
    require hasRole(MANAGER_ROLE(), manager);

    // Revoke manager role
    require hasRole(DEFAULT_ADMIN_ROLE(), e1.msg.sender);
    revokeRole(e1, MANAGER_ROLE(), manager);

    // Try to add to wallet pool
    addToWalletPool@withrevert(e2, user);

    // If caller is the former manager, operation should revert
    assert e2.msg.sender == manager => lastReverted,
        "Former manager still has permission";
}

/*
 * Rule 11: Self-renunciation works correctly
 */
rule selfRenunciationConsistency(address manager) {
    env e;
    require e.msg.sender == manager;
    require hasRole(MANAGER_ROLE(), manager);

    // Renounce role
    renounceRole(e, MANAGER_ROLE(), manager);

    // Should no longer have role
    assert !hasRole(MANAGER_ROLE(), manager),
        "Manager still has role after renunciation";
}

/*
 * Additional invariants to verify role setup
 */
definition manager() returns address = 0x1; // Representing the manager address

invariant managerHasManagerRole()
    hasRole(MANAGER_ROLE(), manager());

invariant managerHasAdminRole()
    hasRole(DEFAULT_ADMIN_ROLE(), manager());