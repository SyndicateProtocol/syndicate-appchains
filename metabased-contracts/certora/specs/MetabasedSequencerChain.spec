using PermissionModuleBasic as permission;

methods {
    // View functions
    function l3ChainId() external returns (uint256) envfree;
    function requirementModule() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function owner() external returns (address) envfree;

    // State-changing functions
    function processTransactionRaw(bytes) external;
    function processTransaction(bytes) external;
    function processBulkTransactions(bytes[]) external;
    function updateRequirementModule(address) external;

    // Permission Module interface methods
    function permission.isAllowed(address) external returns (bool) envfree;
    function permission.setAllowed(address, bool) external;
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
invariant requirementModuleNotZero()
    requirementModule() != 0;

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

    // Check first transaction as representative (full loop not supported in CVL)
    require data.length > 0;
    processTransaction@withrevert(e, data[0]);
    assert !lastReverted,
        "Bulk processing accepted invalid transaction";
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

    // Transaction should only succeed if sender was allowed
    assert !lastReverted => initiallyAllowed,
        "Transaction succeeded with unauthorized sender";
}