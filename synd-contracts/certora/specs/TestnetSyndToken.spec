methods {
    function DEFAULT_ADMIN_ROLE() external returns (bytes32) envfree;
    function MINTER_ROLE() external returns (bytes32) envfree;
    function name() external returns (string) envfree;
    function symbol() external returns (string) envfree;
    function decimals() external returns (uint8) envfree;
    function totalSupply() external returns (uint256) envfree;
    function balanceOf(address) external returns (uint256) envfree;
    function hasRole(bytes32,address) external returns (bool) envfree;

    function grantRole(bytes32,address) external;
    function revokeRole(bytes32,address) external;
}

/*
 * Rule 1: Only MINTER_ROLE can mint
 */
rule onlyMinterCanMint(address to, uint256 amount) {
    env e;

    adminMint@withrevert(e, to, amount);

    assert !lastReverted => hasRole(MINTER_ROLE(), e.msg.sender),
        "Non-minter able to mint";
}

/*
 * Rule 2: Mint updates balances correctly
 */
rule mintUpdatesBalance(address to, uint256 amount) {
    env e;
    require hasRole(MINTER_ROLE(), e.msg.sender);

    // Add constraints for valid amounts
    require amount > 0;

    uint256 balanceBefore = balanceOf(to);
    uint256 totalBefore = totalSupply();

    // Prevent overflow
    require balanceBefore + amount <= max_uint256;
    require totalBefore + amount <= max_uint256;

    adminMint(e, to, amount);

    mathint balanceAfter = balanceOf(to);
    mathint totalAfter = totalSupply();

    assert balanceAfter == balanceBefore + amount, "Balance not updated correctly";
    assert totalAfter == totalBefore + amount, "Total supply not updated correctly";
}

/*
 * Rule 3: Minting to zero address should revert
 */
rule noMintToZero(uint256 amount) {
    env e;
    require hasRole(MINTER_ROLE(), e.msg.sender);

    adminMint@withrevert(e, 0, amount);

    assert lastReverted, "Should not mint to zero address";
}

/*
 * Rule 4: Token metadata is immutable
 */
rule metadataImmutable(method f) {
    env e;
    string nameBefore = name();
    string symbolBefore = symbol();

    calldataarg args;
    f(e,args);

    assert name() == nameBefore && symbol() == symbolBefore,
        "Token metadata changed";
}

/*
 * Rule 5: Token decimals is immutable
 */
rule decimalsImmutable(method f) {
    env e;
    uint8 decimalsBefore = decimals();

    calldataarg args;
    f(e,args);

    assert decimals() == decimalsBefore, "Token decimals changed";
}

/*
 * Rule 6: Admin can grant minter role
 */
rule adminCanGrantMinterRole(address newMinter) {
    env e;

    // Setup requirements
    require e.msg.sender != 0;
    require newMinter != 0;
    require e.msg.sender != newMinter;  // Prevent self-granting
    require hasRole(DEFAULT_ADMIN_ROLE(), e.msg.sender);

    // Initial state check
    require !hasRole(MINTER_ROLE(), newMinter);

    // Grant role
    grantRole(e, MINTER_ROLE(), newMinter);

    // Verify
    assert hasRole(MINTER_ROLE(), newMinter),
        "Admin failed to grant minter role";
}

/*
 * Rule 7: Admin can revoke minter role
 */
rule adminCanRevokeMinterRole(address minter) {
   env e;
   require e.msg.sender != 0;
   require minter != 0;
   require hasRole(DEFAULT_ADMIN_ROLE(), e.msg.sender);
   require hasRole(MINTER_ROLE(), minter);

   bool initiallyHasRole = hasRole(MINTER_ROLE(), minter);
    require initiallyHasRole;


    revokeRole(e, MINTER_ROLE(), minter);


    bool finalHasRole = hasRole(MINTER_ROLE(), minter);
    assert initiallyHasRole && !finalHasRole,
        "Admin failed to revoke minter role";
}

/*
 * Rule 8: Minting zero amount should revert
 */
 rule noZeroMint(address to) {
    env e;
    require e.msg.sender != 0;
    require to != 0;
    require hasRole(MINTER_ROLE(), e.msg.sender);


    adminMint@withrevert(e, to, 0);

    assert lastReverted, "Should not mint zero amount";
 }