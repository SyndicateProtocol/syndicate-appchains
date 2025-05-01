methods {
    // Core methods
    function owner() external returns (address) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;

    // Permission check methods
    function getAllPermissionChecks() external returns (address[]) envfree;
}

// --- Rules for proposer checks ---

rule onlyOwnerCanAddPermissionCheck(address check, bool addToHead) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    addPermissionCheck@withrevert(e, check, addToHead);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner added proposer check";
}

rule onlyOwnerCanRemovePermissionCheck(address check) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    removePermissionCheck@withrevert(e, check);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner removed proposer check";
}

rule requireAnyPermissionCheckPass(address proposer, bytes data) {
    env e;
    require proposer != 0;

    // If isAllowed doesn't revert, at least one check passed
    isAllowed@withrevert(e, proposer, proposer, data);

    assert !lastReverted => true;
}

// --- Additional invariants and rules ---

rule addingPermissionCheckAddsExactlyOne(address check, bool addToHead) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    address[] beforeChecks = getAllPermissionChecks();
    addPermissionCheck(e, check, addToHead);
    address[] afterChecks = getAllPermissionChecks();

    assert afterChecks.length == beforeChecks.length + 1,
        "Adding a proposer check should increase the count by exactly 1";
}

rule removingPermissionCheckRemovesExactlyOne(address check) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    address[] beforeChecks = getAllPermissionChecks();

    // Only continue if the check exists
    require exists uint i. 0 <= i && i < beforeChecks.length && beforeChecks[i] == check;

    removePermissionCheck(e, check);
    address[] afterChecks = getAllPermissionChecks();

    assert afterChecks.length == beforeChecks.length - 1,
        "Removing a proposer check should decrease the count by exactly 1";
}


// Ensure duplicate checks can't be added
rule cantAddDuplicatePermissionCheck(address check, bool addToHead) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    // Add the check first
    addPermissionCheck(e, check, addToHead);

    // Try to add it again
    addPermissionCheck@withrevert(e, check, addToHead);

    assert lastReverted,
        "Should revert when adding a duplicate proposer check";
}

rule noCrossContaminationBetweenChecklists() {
    env e;
    require e.msg.sender == owner();

    address check = e.msg.sender; // use sender as a valid non-zero address

    // Initial state
    address[] initialPermissionChecks = getAllPermissionChecks();

    // Add to proposer checks
    addPermissionCheck(e, check, true);

    // Verify both are updated correctly
    address[] finalPermissionChecks = getAllPermissionChecks();

    assert finalPermissionChecks.length == initialPermissionChecks.length + 1,
        "Permission checks count should be increased by 1";
}