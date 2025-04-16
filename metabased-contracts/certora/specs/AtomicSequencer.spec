methods {
    // View functions
    function implementation() external returns (address) envfree;
}

/*
 * Rule 1: Implementation address never changes
 */
invariant implementationAddressImmutable()
    implementation() != 0;

/*
 * Rule 2: Implementation address persists across operations
 */
rule implementationPersistence(method f) {
    env e;
    address beforeImpl = implementation();

    calldataarg args;
    f(e, args);

    address afterImpl = implementation();
    assert beforeImpl == afterImpl,
        "Implementation address changed";
}