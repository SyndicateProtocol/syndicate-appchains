methods {
    function owner() external returns (address) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
    function getAllPermissionChecks() external returns (address[]) envfree;
}

// Rules for proposer checks
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

// Zero address checks for both proposer and calldata functions
rule zeroAddressChecks() {
    env e;

    // Check proposer functions
    addPermissionCheck@withrevert(e, 0, true);
    assert lastReverted, "Zero address added to proposer checks";

    removePermissionCheck@withrevert(e, 0);
    assert lastReverted, "Zero address removed from proposer checks";
}

// Verification rules
rule requireAllPermissionChecksPass(address proposer, bytes data) {
    env e;

    isAllowed@withrevert(e, proposer, proposer, data);

    // If it doesn't revert, all checks must have passed
    assert !lastReverted => true,
        "Some proposer checks failed but function didn't revert";
}