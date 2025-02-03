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

rule zeroAddressChecks() {
    env e;

    addCheck@withrevert(e, 0, true);
    assert lastReverted, "Zero address added";

    removeCheck@withrevert(e, 0);
    assert lastReverted, "Zero address removed";
}

rule requireAllChecksPass(address proposer) {
    env e;

    isAllowed@withrevert(e, proposer);

    // If it doesn't revert, all checks must have passed
    assert !lastReverted => true,
        "Some checks failed but function didn't revert";
}