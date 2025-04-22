methods {
    // Core methods
    function owner() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function isCalldataAllowed(bytes) external returns (bool) envfree;

    // Proposer check methods
    function getAllProposerChecks() external returns (address[]) envfree;

    // Calldata check methods
    function getAllCalldataChecks() external returns (address[]) envfree;
}

// --- Rules for proposer checks ---

rule onlyOwnerCanAddProposerCheck(address check, bool addToHead) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    addProposerCheck@withrevert(e, check, addToHead);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner added proposer check";
}

rule onlyOwnerCanRemoveProposerCheck(address check) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    removeProposerCheck@withrevert(e, check);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner removed proposer check";
}

rule requireAnyProposerCheckPass(address proposer) {
    env e;
    require proposer != 0;

    // If isAllowed doesn't revert, at least one check passed
    isAllowed@withrevert(e, proposer);

    assert !lastReverted => true;
}

// --- Rules for calldata checks ---

rule onlyOwnerCanAddCalldataCheck(address check, bool addToHead) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    addCalldataCheck@withrevert(e, check, addToHead);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner added calldata check";
}

rule onlyOwnerCanRemoveCalldataCheck(address check) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    removeCalldataCheck@withrevert(e, check);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner removed calldata check";
}

rule requireAnyCalldataCheckPass() {
    env e;
    bytes data;

    // If isCalldataAllowed doesn't revert, at least one check passed
    isCalldataAllowed@withrevert(e, data);

    assert !lastReverted => true;
}

// --- Additional invariants and rules ---

rule addingProposerCheckAddsExactlyOne(address check, bool addToHead) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    address[] beforeChecks = getAllProposerChecks();
    addProposerCheck(e, check, addToHead);
    address[] afterChecks = getAllProposerChecks();

    assert afterChecks.length == beforeChecks.length + 1,
        "Adding a proposer check should increase the count by exactly 1";
}

rule addingCalldataCheckAddsExactlyOne(address check, bool addToHead) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    address[] beforeChecks = getAllCalldataChecks();
    addCalldataCheck(e, check, addToHead);
    address[] afterChecks = getAllCalldataChecks();

    assert afterChecks.length == beforeChecks.length + 1,
        "Adding a calldata check should increase the count by exactly 1";
}

rule removingProposerCheckRemovesExactlyOne(address check) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    address[] beforeChecks = getAllProposerChecks();

    // Only continue if the check exists
    require exists uint i. 0 <= i && i < beforeChecks.length && beforeChecks[i] == check;

    removeProposerCheck(e, check);
    address[] afterChecks = getAllProposerChecks();

    assert afterChecks.length == beforeChecks.length - 1,
        "Removing a proposer check should decrease the count by exactly 1";
}

rule removingCalldataCheckRemovesExactlyOne(address check) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    address[] beforeChecks = getAllCalldataChecks();

    // Only continue if the check exists
    require exists uint i. 0 <= i && i < beforeChecks.length && beforeChecks[i] == check;

    removeCalldataCheck(e, check);
    address[] afterChecks = getAllCalldataChecks();

    assert afterChecks.length == beforeChecks.length - 1,
        "Removing a calldata check should decrease the count by exactly 1";
}

// Ensure duplicate checks can't be added
rule cantAddDuplicateProposerCheck(address check, bool addToHead) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    // Add the check first
    addProposerCheck(e, check, addToHead);

    // Try to add it again
    addProposerCheck@withrevert(e, check, addToHead);

    assert lastReverted,
        "Should revert when adding a duplicate proposer check";
}

rule cantAddDuplicateCalldataCheck(address check, bool addToHead) {
    env e;
    require e.msg.sender == owner();
    require check != 0;

    // Add the check first
    addCalldataCheck(e, check, addToHead);

    // Try to add it again
    addCalldataCheck@withrevert(e, check, addToHead);

    assert lastReverted,
        "Should revert when adding a duplicate calldata check";
}

rule noCrossContaminationBetweenChecklists() {
    env e;
    require e.msg.sender == owner();

    address check = e.msg.sender; // use sender as a valid non-zero address

    // Initial state
    address[] initialProposerChecks = getAllProposerChecks();
    address[] initialCalldataChecks = getAllCalldataChecks();

    // Add to proposer checks
    addProposerCheck(e, check, true);

    // Verify calldata checks unchanged
    address[] afterProposerAddCalldataChecks = getAllCalldataChecks();
    assert afterProposerAddCalldataChecks.length == initialCalldataChecks.length,
        "Adding to proposer checks should not affect calldata checks";

    // Now add to calldata checks
    addCalldataCheck(e, check, true);

    // Verify both are updated correctly
    address[] finalProposerChecks = getAllProposerChecks();
    address[] finalCalldataChecks = getAllCalldataChecks();

    assert finalProposerChecks.length == initialProposerChecks.length + 1,
        "Proposer checks count should be increased by 1";
    assert finalCalldataChecks.length == initialCalldataChecks.length + 1,
        "Calldata checks count should be increased by 1";
}