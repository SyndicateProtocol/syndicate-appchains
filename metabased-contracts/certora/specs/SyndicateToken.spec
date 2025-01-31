methods {
    function MINTER_ROLE() external returns (bytes32) envfree;
    function name() external returns (string) envfree;
    function symbol() external returns (string) envfree;
    function decimals() external returns (uint8) envfree;
    function totalSupply() external returns (uint256) envfree;
    function balanceOf(address) external returns (uint256) envfree;
    function hasRole(bytes32,address) external returns (bool) envfree;
    function getRoleAdmin(bytes32) external returns (bytes32) envfree;
}

/*
 * Rule 1: Only MINTER_ROLE can mint
 */
rule onlyMinterCanMint(address to, uint256 amount) {
    env e;

    mint@withrevert(e, to, amount);

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

    mint(e, to, amount);

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

    mint@withrevert(e, 0, amount);

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
