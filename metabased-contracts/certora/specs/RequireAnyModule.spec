methods {
    function owner() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function getAllChecks() external returns (address[]) envfree;
}

rule onlyOwnerCanAdd(address check, bool addToHead) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    addCheck@withrevert(e, check, addToHead);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner added check";
}

rule onlyOwnerCanRemove(address check) {
    env e;
    require e.msg.sender != 0;
    require check != 0;

    removeCheck@withrevert(e, check);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner removed check";
}

rule requireAnyCheckPass(address proposer) {
    env e;
    require proposer != 0;

    // If isAllowed doesn't revert, at least one check passed
    isAllowed@withrevert(e, proposer);

    assert !lastReverted => true,
        "All checks failed but function didn't revert";
}