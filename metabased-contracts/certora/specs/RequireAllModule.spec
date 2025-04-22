methods {
    function owner() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function isCalldataAllowed(bytes) external returns (bool) envfree;
    function getAllProposerChecks() external returns (address[]) envfree;
    function getAllCalldataChecks() external returns (address[]) envfree;
}

// Rules for proposer checks
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

// Rules for calldata checks
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

// Zero address checks for both proposer and calldata functions
rule zeroAddressChecks() {
    env e;

    // Check proposer functions
    addProposerCheck@withrevert(e, 0, true);
    assert lastReverted, "Zero address added to proposer checks";

    removeProposerCheck@withrevert(e, 0);
    assert lastReverted, "Zero address removed from proposer checks";

    // Check calldata functions
    addCalldataCheck@withrevert(e, 0, true);
    assert lastReverted, "Zero address added to calldata checks";

    removeCalldataCheck@withrevert(e, 0);
    assert lastReverted, "Zero address removed from calldata checks";
}

// Verification rules
rule requireAllProposerChecksPass(address proposer) {
    env e;

    isAllowed@withrevert(e, proposer);

    // If it doesn't revert, all checks must have passed
    assert !lastReverted => true,
        "Some proposer checks failed but function didn't revert";
}

rule requireAllCalldataChecksPass(bytes data) {
    env e;

    isCalldataAllowed@withrevert(e, data);

    // If it doesn't revert, all checks must have passed
    assert !lastReverted => true,
        "Some calldata checks failed but function didn't revert";
}