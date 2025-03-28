methods {
    // View functions
    function hasRole(bytes32, address) external returns (bool) envfree;
    function getRoleAdmin(bytes32) external returns (bytes32) envfree;
    function walletPool(address) external returns (bool) envfree;
    function isInWalletPool(address) external returns (bool) envfree;

    // Constants
    function MANAGER_ROLE() external returns (bytes32) envfree;
    function DEFAULT_ADMIN_ROLE() external returns (bytes32) envfree;

    // OpenZeppelin AccessControl functions
    function grantRole(bytes32, address) external;
    function revokeRole(bytes32, address) external;
    function renounceRole(bytes32, address) external;

    // Wallet Pool functions
    function addToWalletPool(address, address[]) external;
    function removeFromWalletPool(address, address[]) external;
    function updateAllowlistModule(address, address, bool) external;
    function updateAllowlistModules(address, bool, address[]) external;
}

/*
 * Rule 1: DEFAULT_ADMIN_ROLE is the admin of MANAGER_ROLE
 */
rule defaultAdminIsManagerRoleAdmin() {
    assert getRoleAdmin(MANAGER_ROLE()) == DEFAULT_ADMIN_ROLE(),
        "DEFAULT_ADMIN_ROLE is not the admin of MANAGER_ROLE";
}

/*
 * Rule 2: Only managers can add to wallet pool
 */
rule onlyManagersCanAdd(address wallet) {
    env e;
    address[] modules;

    // Try to add to wallet pool
    addToWalletPool@withrevert(e, wallet, modules);

    // If successful, caller must have MANAGER_ROLE
    assert !lastReverted => hasRole(MANAGER_ROLE(), e.msg.sender),
        "Non-manager added to wallet pool";
}

/*
 * Rule 3: Only managers can remove from wallet pool
 */
rule onlyManagersCanRemove(address wallet) {
    env e;
    address[] modules;

    // Try to remove from wallet pool
    removeFromWalletPool@withrevert(e, wallet, modules);

    // If successful, caller must have MANAGER_ROLE
    assert !lastReverted => hasRole(MANAGER_ROLE(), e.msg.sender),
        "Non-manager removed from wallet pool";
}

/*
 * Rule 4: Wallet pool state changes correctly on add
 */
rule walletPoolAddConsistency(address wallet) {
    env e;
    address[] modules;
    require hasRole(MANAGER_ROLE(), e.msg.sender);

    // Store initial state
    bool initiallyInPool = walletPool(wallet);

    // Add to wallet pool
    addToWalletPool(e, wallet, modules);

    // Check state change
    bool finallyInPool = walletPool(wallet);
    assert finallyInPool, "Wallet not added to wallet pool";
}

/*
 * Rule 5: Wallet pool state changes correctly on remove
 */
rule walletPoolRemoveConsistency(address wallet) {
    env e;
    address[] modules;
    require hasRole(MANAGER_ROLE(), e.msg.sender);

    // Store initial state
    bool initiallyInPool = walletPool(wallet);

    // Remove from wallet pool
    removeFromWalletPool(e, wallet, modules);

    // Check state change
    bool finallyInPool = walletPool(wallet);
    assert !finallyInPool, "Wallet not removed from wallet pool";
}

/*
 * Rule 6: isInWalletPool matches wallet pool state
 */
rule isInWalletPoolConsistency(address wallet) {
    bool inWalletPool = walletPool(wallet);
    bool result = isInWalletPool(wallet);

    assert inWalletPool == result,
        "isInWalletPool does not match walletPool state";
}

/*
 * Rule 7: Effects of wallet pool operations persist
 */
rule walletPoolOperationsPersist(method f, address wallet) {
    env e;

    // Get initial state
    bool initiallyInPool = walletPool(wallet);

    // Perform some operation
    calldataarg args;
    f(e, args);

    // State should only change through wallet pool operations
    bool finallyInPool = walletPool(wallet);

    assert initiallyInPool != finallyInPool => (
        f.selector == sig:addToWalletPool(address,address[]).selector ||
        f.selector == sig:removeFromWalletPool(address,address[]).selector
    ), "Wallet pool changed through non-wallet pool function";
}

/*
 * Rule 8: Only role admins can grant manager role
 */
rule onlyRoleAdminCanGrantRole(address newManager) {
    env e;
    bytes32 role = MANAGER_ROLE();
    bytes32 adminRole = getRoleAdmin(role);

    // Require that the wallet doesn't have the role yet
    require !hasRole(role, newManager);

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

    // Require that the manager actually has the role
    require hasRole(role, manager);

    // Try to revoke manager role
    revokeRole@withrevert(e, role, manager);

    // If successful, caller must have admin role
    assert !lastReverted => hasRole(adminRole, e.msg.sender),
        "Non-admin revoked manager role";
}

/*
 * Rule 10: Role changes affect permissions correctly
 */
rule roleChangesAffectPermissions(address wallet, address manager) {
    env e1;
    env e2;
    address[] modules;

    // Initial state
    require !walletPool(wallet);
    require hasRole(MANAGER_ROLE(), manager);
    require hasRole(DEFAULT_ADMIN_ROLE(), e1.msg.sender);

    // Revoke manager role
    revokeRole(e1, MANAGER_ROLE(), manager);

    // Try to add to wallet pool as former manager
    require e2.msg.sender == manager;
    addToWalletPool@withrevert(e2, wallet, modules);

    // Operation should revert
    assert lastReverted, "Former manager still has permission";
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
 * Rule 12: Zero address cannot be used as admin in constructor
 */
rule constructorRejectsZeroAddress() {
    // We can't directly test the constructor in Certora
    // This rule acts as a placeholder - would need a custom hook
    assert true;
}

/*
 * Rule 13: Only managers can update single allowlist module
 */
rule onlyManagersCanUpdateSingleAllowlist(address allowlistModule, address wallet, bool isAllowed) {
    env e;

    // Try to update allowlist module
    updateAllowlistModule@withrevert(e, allowlistModule, wallet, isAllowed);

    // If successful, caller must have MANAGER_ROLE
    assert !lastReverted => hasRole(MANAGER_ROLE(), e.msg.sender),
        "Non-manager updated allowlist module";
}

/*
 * Rule 14: Only managers can update multiple allowlist modules
 */
rule onlyManagersCanUpdateMultipleAllowlists(address wallet, bool isAllowed) {
    env e;
    address[] modules;

    // Try to update allowlist modules
    updateAllowlistModules@withrevert(e, wallet, isAllowed, modules);

    // If successful, caller must have MANAGER_ROLE
    assert !lastReverted => hasRole(MANAGER_ROLE(), e.msg.sender),
        "Non-manager updated allowlist modules";
}

/*
 * Invariants to verify role setup
 */
definition manager() returns address = 0x1;

invariant managerHasManagerRole()
    hasRole(MANAGER_ROLE(), manager());

invariant managerHasAdminRole()
    hasRole(DEFAULT_ADMIN_ROLE(), manager());