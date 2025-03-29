// AssertionPoster.spec

methods {
    // Public functions
    function owner() external returns (address) envfree;

    // Optional accessors for private state
    function rollup() external returns (address) optional;
    function executor() external returns (address) optional;
    function legacy() external returns (bool) optional;
    function nodeNum() external returns (uint64) optional;
    function currentInboxSize() external returns (uint64) optional;
    function configure() external optional;
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

// Configure must revert when called directly
rule configureRevertsDirectly() {
    env e;

    // Use a sender that isn't the executor to guarantee the direct call will revert
    require e.msg.sender != executor();

    configure@withrevert(e);

    assert lastReverted,
        "Configure didn't revert when called directly";
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
