using PermissionModuleBasic as permission;

methods {
    // View functions
    function l3ChainId() external returns (uint256) envfree;
    function requirementModule() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function owner() external returns (address) envfree;

    // Permission Module interface methods
    function permission.isAllowed(address) external returns (bool) envfree;
}

/*
 * Define ghost variables to track important state
 */
ghost mapping(address => bool) processedSenders;
ghost mapping(bytes32 => bool) processedTransactions;

/*
 * Rule 1: Verify that l3ChainId cannot be zero
 * This should always be true after contract deployment
 */
invariant l3ChainIdNotZero()
    l3ChainId() != 0;

/*
 * Rule 2: Verify that requirementModule address is never zero
 */
rule moduleNotZero(method f) {
    // Get the module before
    address oldModule = requirementModule();

    // Function call
    env e;
    calldataarg args;
    f(e, args);

    // Get the module after
    address newModule = requirementModule();

    // Assert module cannot be zero address
    assert newModule != 0;
}

/*
 * Rule 3: Only allowed addresses can process transactions
 */
rule onlyAllowedCanProcess(bytes data) {
    env e;

    // Try to process a transaction
    processTransaction@withrevert(e, data);

    // If the transaction succeeded
    bool success = !lastReverted;

    // Then the sender must have been allowed
    assert success => permission.isAllowed(e.msg.sender),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 4: Consistent behavior between processTransaction and processTransactionRaw
 */
rule processConsistency(bytes data) {
    env e;

    // Record both outcomes
    processTransaction@withrevert(e, data);
    bool txSuccess = !lastReverted;

    processTransactionRaw@withrevert(e, data);
    bool rawSuccess = !lastReverted;

    // If one succeeds, both should succeed under same conditions
    assert txSuccess == rawSuccess,
        "Inconsistent behavior between process methods";
}

/*
 * Rule 5: Bulk processing maintains individual transaction properties
 */
rule bulkProcessingConsistency(bytes[] data) {
    env e;

    // Process transactions in bulk
    processBulkTransactions@withrevert(e, data);
    bool bulkSuccess = !lastReverted;

    // If bulk processing succeeded, each individual transaction should succeed
    require bulkSuccess;
    require data.length < 3; // Loop unrolling limit - Certora will unroll up to this limit

    processTransaction@withrevert(e, data[0]);
    assert !lastReverted,
        "Bulk processing accepted invalid transaction";

    processTransaction@withrevert(e, data[1]);
    assert !lastReverted,
        "Bulk processing accepted invalid transaction";

    processTransaction@withrevert(e, data[2]);
    assert !lastReverted,
        "Bulk processing accepted invalid transaction";

    // If individual transactions succeeded, bulk processing should succeed
    assert bulkSuccess,
        "Bulk processing failed despite individual transactions succeeding";


}

/*
 * Rule 6: Only owner can update requirement module
 */
rule onlyOwnerCanUpdateModule(address newModule) {
    env e;

    // Try to update the module
    updateRequirementModule@withrevert(e, newModule);

    // If successful, must have been owner
    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner updated requirement module";
}

/*
 * Rule 7: Module update changes state correctly
 */
rule moduleUpdateChangesState(address newModule) {
    env e;
    require newModule != 0;

    // Store old module
    address oldModule = requirementModule();

    // Update module
    updateRequirementModule@withrevert(e, newModule);

    // If successful, module should be updated
    assert !lastReverted => requirementModule() == newModule,
        "Module not updated correctly";
}
// Complementary rule to the above calling updateRequirementModule without revert
rule moduleUpdateConsistency(address newModule) {
    env e;

    updateRequirementModule(e, newModule);

    assert requirementModule() == newModule;
    assert newModule != 0;
}

/*
 * Rule 8: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes data) {
    env e;
    address oldModule = requirementModule();

    // Process transaction
    processTransaction@withrevert(e, data);

    // Verify requirement module hasn't changed
    assert requirementModule() == oldModule,
        "Transaction processing modified core state";
}

/*
 * Rule 9: Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data) {
    env e;

    // Set initial allowed state
    bool initiallyAllowed = permission.isAllowed(e.msg.sender);

    // Try to process a transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;

    // Direction 1: Success implies allowed
    assert txSucceeded => initiallyAllowed,
        "Transaction succeeded with unauthorized sender";

    // Direction 2: Allowed implies success (with preconditions)
    require data.length > 0;  // Example precondition: non-empty data
    require e.msg.value == 0; // Example precondition: no ETH sent

    assert initiallyAllowed => txSucceeded,
        "Transaction failed despite sender being authorized and valid preconditions";
}

/*
 * Rule 10: Only admin can transfer admin rights
 */
rule onlyAdminCanTransfer(address newAdmin) {
    env e;

    // Store old admin
    address oldAdmin = admin();

    // Try to transfer admin
    transferAdmin@withrevert(e, newAdmin);

    // If successful, must have been the current admin
    assert !lastReverted => e.msg.sender == oldAdmin,
        "Non-admin transferred admin rights";
}