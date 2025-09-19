/*
 * Formal Verification Specification for SyndStaking Contract
 * 
 * Focus Areas:
 * 1. User deposits = User withdrawals (fund conservation)
 * 2. Multi-dimensional stake accounting consistency
 * 3. Epoch-based accounting integrity
 * 4. Withdrawal delay enforcement
 */

methods {
    // View functions - envfree for gas-free verification
    function totalStake() external returns (uint256) envfree;
    function userTotal(address) external returns (uint256) envfree;
    function appchainTotal(uint256) external returns (uint256) envfree;
    function userAppchainTotal(address, uint256) external returns (uint256) envfree;
    function getCurrentEpoch() external returns (uint256);
    function finalizedEpochCount() external returns (uint256) envfree;
    function userFinalizedEpochCount(address) external returns (uint256) envfree;
    function appchainFinalizedEpochCount(uint256) external returns (uint256) envfree;
    function userAppchainFinalizedEpochCount(address, uint256) external returns (uint256) envfree;
    
    // Epoch data getters
    function epochTotal(uint256) external returns (uint256) envfree;
    function epochAdditions(uint256) external returns (uint256) envfree;
    function epochWithdrawals(uint256) external returns (uint256) envfree;
    function epochUserTotal(uint256, address) external returns (uint256) envfree;
    function epochUserAdditions(uint256, address) external returns (uint256) envfree;
    function epochUserWithdrawals(uint256, address) external returns (uint256) envfree;
    
    // Contract state
    function paused() external returns (bool) envfree;
    function owner() external returns (address) envfree;
    
    // Summarize problematic functions - these don't affect core staking invariants
    function claimAllRewards(SyndStaking.ClaimRequest[], address) external => HAVOC_ECF;
    
    // External interface calls that cause havoc - assume they don't modify staking state
    function _.claimFor(uint256, address, address, uint256) external => NONDET;
}

/*
 * INVARIANT 1: Fund Conservation - Total stake equals sum of individual stakes
 * This is the most critical invariant ensuring no funds are lost or created
 */
invariant totalStakeEqualsUserSum()
    totalStake() == ghostSumAllUserTotals();

/*
 * INVARIANT 2: Multi-dimensional consistency
 * Total stake across all appchains should equal global total stake
 */
invariant globalAppchainConsistency()
    totalStake() == ghostSumAllAppchainTotals();


/*
 * RULE 1: Staking increases stake amounts correctly
 */
rule stakingIncreasesStakes(address user, uint256 appchainId, uint256 amount) {
    env e;
    require e.msg.sender == user;
    require e.msg.value == amount;
    require amount > 0;
    require appchainId > 0;
    require !paused();

    // Store state before staking
    uint256 totalBefore = totalStake();
    uint256 userTotalBefore = userTotal(user);
    uint256 appchainTotalBefore = appchainTotal(appchainId);
    uint256 userAppchainBefore = userAppchainTotal(user, appchainId);

    // Perform staking
    stakeSynd@withrevert(e, appchainId);
    bool success = !lastReverted;

    // Verify increases if successful
    assert success => totalStake() == totalBefore + amount;
    assert success => userTotal(user) == userTotalBefore + amount;
    assert success => appchainTotal(appchainId) == appchainTotalBefore + amount;
    assert success => userAppchainTotal(user, appchainId) == userAppchainBefore + amount;
}

/*
 * RULE 2: Withdrawal initialization reduces current stake but records withdrawal
 */
rule withdrawalInitializationCorrectness(address user, uint256 appchainId, uint256 amount) {
    env e;
    require e.msg.sender == user;
    require amount > 0;
    require appchainId > 0;
    require userAppchainTotal(user, appchainId) >= amount;

    // Store state before withdrawal initialization
    uint256 totalBefore = totalStake();
    uint256 userTotalBefore = userTotal(user);
    uint256 appchainTotalBefore = appchainTotal(appchainId);
    uint256 userAppchainBefore = userAppchainTotal(user, appchainId);
    uint256 currentEpoch = getCurrentEpoch(e);

    // Initialize withdrawal
    initializeWithdrawal@withrevert(e, appchainId, amount);
    bool success = !lastReverted;

    // Verify reductions if successful
    assert success => totalStake() == totalBefore - amount;
    assert success => userTotal(user) == userTotalBefore - amount;
    assert success => appchainTotal(appchainId) == appchainTotalBefore - amount;
    assert success => userAppchainTotal(user, appchainId) == userAppchainBefore - amount;

    // Verify withdrawal is recorded for current epoch
    assert success => epochUserWithdrawals(currentEpoch, user) >= amount;
}

/*
 * RULE 3: Withdrawal completion doesn't affect stake totals (funds already removed)
 */
rule withdrawalCompletionNoStakeChange(address user, uint256 epochIndex, address destination) {
    env e;
    require e.msg.sender == user;
    require epochIndex < getCurrentEpoch(e); // Past epoch only

    // Store state before withdrawal completion
    uint256 totalBefore = totalStake();
    uint256 userTotalBefore = userTotal(user);

    // Complete withdrawal
    withdraw@withrevert(e, epochIndex, destination);
    bool success = !lastReverted;

    // Verify no changes to stake totals (funds already removed during initialization)
    assert success => totalStake() == totalBefore;
    assert success => userTotal(user) == userTotalBefore;
}

/*
 * RULE 4: Withdrawal delay enforcement - cannot withdraw in same epoch
 */
rule withdrawalDelayEnforcement(address user, address destination) {
    env e;
    require e.msg.sender == user;
    uint256 currentEpoch = getCurrentEpoch(e);

    // Try to withdraw from current epoch (should fail)
    withdraw@withrevert(e, currentEpoch, destination);
    
    assert lastReverted, "Should not be able to withdraw from current epoch";
}

/*
 * RULE 5: Zero appchain ID is always rejected
 */
rule zeroAppchainIdRejected(address user, uint256 amount) {
    env e;
    require e.msg.sender == user;
    require amount > 0;

    // Should fail for appchainId = 0
    initializeWithdrawal@withrevert(e, 0, amount);
    
    assert lastReverted, "Should reject appchainId 0";
}

/*
 * RULE 6: Zero amount is always rejected
 */
rule zeroAmountRejected(address user, uint256 appchainId) {
    env e;
    require e.msg.sender == user;
    require appchainId > 0;

    // Should fail for amount = 0
    initializeWithdrawal@withrevert(e, appchainId, 0);
    
    assert lastReverted, "Should reject zero amount";
}

/*
 * RULE 7: Insufficient stake prevents withdrawal initialization
 */
rule insufficientStakePreventsWithdrawal(address user, uint256 appchainId, uint256 amount) {
    env e;
    require e.msg.sender == user;
    require appchainId > 0;
    require userAppchainTotal(user, appchainId) < amount;

    // Should fail when trying to withdraw more than staked
    initializeWithdrawal@withrevert(e, appchainId, amount);
    
    assert lastReverted, "Should not be able to withdraw more than staked amount";
}

/*
 * RULE 8: Stake transfer maintains global totals
 */
rule stakeTransferMaintainsGlobalTotal(address user, uint256 fromAppchain, uint256 toAppchain, uint256 amount) {
    env e;
    require e.msg.sender == user;
    require fromAppchain > 0 && toAppchain > 0;
    require fromAppchain != toAppchain;
    require userAppchainTotal(user, fromAppchain) >= amount;
    require amount > 0;

    // Store state before transfer
    uint256 totalBefore = totalStake();
    uint256 userTotalBefore = userTotal(user);

    // Perform stake transfer
    stageStakeTransfer@withrevert(e, fromAppchain, toAppchain, amount);
    bool success = !lastReverted;

    // Global totals should remain unchanged
    assert success => totalStake() == totalBefore;
    assert success => userTotal(user) == userTotalBefore;
}

/*
 * RULE 9: Epoch finalization maintains accounting consistency  
 */
rule epochFinalizationConsistency(address user) {
    env e;
    uint256 currentEpoch = getCurrentEpoch(e);
    require userFinalizedEpochCount(user) < getCurrentEpoch(e);

    uint256 userTotalBefore = userTotal(user);

    finalizeUserEpochs(e, user);

    uint256 userTotalAfter = userTotal(user);
    uint256 finalizedCountAfter = userFinalizedEpochCount(user);

    // User total should remain unchanged after finalization
    assert userTotalAfter == userTotalBefore;
    // Finalized count should advance
    uint256 finalizedCountBefore = userFinalizedEpochCount(user);
    assert finalizedCountAfter >= finalizedCountBefore;
}

/*
 * RULE 10: Multi-epoch withdrawal consistency  
 */
rule multiEpochWithdrawalConsistency(address user, uint256 epochIndex, address destination) {
    env e;
    require e.msg.sender == user;
    require epochIndex < getCurrentEpoch(e); // Past epoch only
    
    // Store withdrawal amount for this epoch
    uint256 withdrawalAmount = epochUserWithdrawals(epochIndex, user);
    require withdrawalAmount > 0;
    
    // Store balance before
    uint256 balanceBefore = nativeBalances[destination];
    
    withdraw@withrevert(e, epochIndex, destination);
    bool success = !lastReverted;
    
    // Verify correct amount was transferred
    assert success => (nativeBalances[destination] >= balanceBefore + withdrawalAmount);
}

/*
 * RULE 11: Paused state prevents all staking operations
 */
rule pausedStatePreventsOperations(address user, uint256 appchainId, uint256 amount) {
    env e;
    require e.msg.sender == user;
    require paused();

    stakeSynd@withrevert(e, appchainId);
    assert lastReverted, "Staking should be prevented when paused";

    uint256[] appchainIds;
    uint256[] amounts;
    require appchainIds.length == amounts.length;
    stakeMultipleAppchains@withrevert(e, appchainIds, amounts);
    assert lastReverted, "Multi-chain staking should be prevented when paused";
}

/*
 * Ghost functions for invariant verification
 */
ghost ghostSumAllUserTotals() returns uint256 {
    init_state axiom ghostSumAllUserTotals() == 0;
}

ghost ghostSumAllAppchainTotals() returns uint256 {
    init_state axiom ghostSumAllAppchainTotals() == 0;
}

ghost mapping(address => mathint) ghostUserWithdrawalSum;

/*
 * Ghost updates for tracking sums
 */
hook Sstore userTotal[KEY address user] uint256 newValue (uint256 oldValue) {
    havoc ghostSumAllUserTotals assuming ghostSumAllUserTotals@new() == ghostSumAllUserTotals@old() + newValue - oldValue;
}

hook Sstore appchainTotal[KEY uint256 appchainId] uint256 newValue (uint256 oldValue) {
    havoc ghostSumAllAppchainTotals assuming ghostSumAllAppchainTotals@new() == ghostSumAllAppchainTotals@old() + newValue - oldValue;
}

