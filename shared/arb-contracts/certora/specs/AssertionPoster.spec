methods {
    // Public functions
    function owner() external returns (address) envfree;

    // Optional accessors for private state
    function rollup() external returns (address) optional;
    function executor() external returns (address) optional;
    function legacy() external returns (bool) optional;
    function nodeNum() external returns (uint64) optional;
    function currentInboxSize() external returns (uint64) optional;
    function initialize() external optional;
    function postAssertion(bytes32, bytes32) external optional;
}

// Access control rules
rule onlyOwnerCanPostAssertion(bytes32 blockHash, bytes32 sendRoot) {
    env e;
    require e.msg.sender != 0;

    postAssertion@withrevert(e, blockHash, sendRoot);

    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner successfully posted assertion";
}

// Initialize must revert when called directly
rule initializeRevertsDirectly() {
    env e;
    // Use a sender that isn't the executor to guarantee the direct call will revert
    require e.msg.sender != executor();
    initialize@withrevert(e);
    assert lastReverted,
        "Initialize didn't revert when called directly";
}
// Only owner should be able to call postAssertion without reverting (in ideal conditions)
rule onlyOwnerCanCallPostAssertion(bytes32 blockHash, bytes32 sendRoot) {
    env e;
    // If a call succeeds, sender must be owner
    postAssertion@withrevert(e, blockHash, sendRoot);
    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner successfully called postAssertion";
}
// Non-owners cannot post assertions
rule nonOwnersCannotPostAssertions(bytes32 blockHash, bytes32 sendRoot) {
    env e;
    require e.msg.sender != owner();
    require e.msg.sender != 0;
    postAssertion@withrevert(e, blockHash, sendRoot);
    assert lastReverted,
        "Non-owner was able to post assertion";
}
// Initialize cannot be called multiple times
rule initializeCanBeCalledOnlyOnce() {
    env e1;
    env e2;
    // First call (will revert when called directly)
    initialize@withrevert(e1);
    bool firstReverted = lastReverted;
    // Second call (should always revert)
    initialize@withrevert(e2);
    bool secondReverted = lastReverted;
    // Either both revert or the second one always reverts
    assert firstReverted || secondReverted,
        "Initialize was called successfully twice";
}