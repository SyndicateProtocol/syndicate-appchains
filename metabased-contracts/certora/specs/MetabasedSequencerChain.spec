using ProposerPermissionModuleBasic as permission;
using InitializableHarness as init;

methods {
    // View functions
    function l3ChainId() external returns (uint256) envfree;
    function proposerRequirementModule() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function owner() external returns (address) envfree;
    function init._getInitializedVersion() external returns (uint8) envfree;

    // Permission Module interface methods
    function permission.isAllowed(address) external returns (bool) envfree;
}

/*
 * Rule 1: Initialization rules
 */
rule initializeOnce(address admin, address module) {
    env e;
    require admin != 0;
    require module != 0;

    // First initialization
    initialize@withrevert(e, admin, module);
    bool firstInit = !lastReverted;

    // Try to initialize again
    initialize@withrevert(e, admin, module);

    assert firstInit => lastReverted,
        "Contract initialized more than once";
}

/*
 * Rule 2: Initialization sets correct values
 */
rule initializationCorrect(address admin, address module) {
    env e;
    require admin != 0;
    require module != 0;

    initialize(e, admin, module);

    assert proposerRequirementModule() == module, "Module not set correctly";
    assert owner() == admin, "Admin not set correctly";
}

/*
 * Rule 3: Verify that l3ChainId cannot be zero
 */
invariant l3ChainIdNotZero()
    l3ChainId() != 0;

/*
 * Rule 4: Verify that proposerRequirementModule address is never zero after initialization
 */
rule moduleNotZero(method f) {
    env e;
    require init._getInitializedVersion() > 0;

    // Get the module before
    address oldModule = proposerRequirementModule();

    // Function call
    calldataarg args;
    f(e, args);

    // Get the module after
    address newModule = proposerRequirementModule();

    // Assert module cannot be zero address
    assert newModule != 0;
}

/*
 * Rule 5: Only allowed addresses can process transactions
 */
rule onlyAllowedCanProcess(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;

    // Try to process a transaction
    processTransaction@withrevert(e, data);

    // If the transaction succeeded
    bool success = !lastReverted;

    // Then the sender must have been allowed
    assert success => permission.isAllowed(e.msg.sender),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 6: Consistent behavior between processTransaction and processTransactionRaw
 */
rule processConsistency(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;

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
 * Rule 7: Bulk processing maintains individual transaction properties
 */
rule bulkProcessingConsistency(bytes[] data) {
    env e;
    require init._getInitializedVersion() > 0;

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
 * Rule 8: Only owner can update requirement module
 */
rule onlyOwnerCanUpdateModule(address newModule) {
    env e;
    require init._getInitializedVersion() > 0;

    // Try to update the module
    updateRequirementModule@withrevert(e, newModule);

    // If successful, must have been owner
    assert !lastReverted => e.msg.sender == owner(),
        "Non-owner updated requirement module";
}

/*
 * Rule 9: Module update changes state correctly
 */
rule moduleUpdateChangesState(address newModule) {
    env e;
    require init._getInitializedVersion() > 0;
    require newModule != 0;

    // Store old module
    address oldModule = proposerRequirementModule();

    // Update module
    updateRequirementModule@withrevert(e, newModule);

    // If successful, module should be updated
    assert !lastReverted => proposerRequirementModule() == newModule,
        "Module not updated correctly";
}

/*
 * Rule 10: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;
    address oldModule = proposerRequirementModule();

    // Process transaction
    processTransaction@withrevert(e, data);

    // Verify requirement module hasn't changed
    assert proposerRequirementModule() == oldModule,
        "Transaction processing modified core state";
}

/*
 * Rule 11: Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data) {
    env e;

    // Setup variables for initialization
    address admin = e.msg.sender;
    address module = permission;

    // Initialize the contract first
    initialize(e, admin, module);

    // Verify initialization worked
    require init._getInitializedVersion() == 1;

    // Valid sender and msg parameters
    require e.msg.sender != 0;
    require e.msg.sender != currentContract;
    require e.msg.value == 0;

    // Valid data requirements
    require data.length > 0;
    require data.length < max_uint256;

    // Check permission
    bool initiallyAllowed = permission.isAllowed(e.msg.sender);

    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;

    // Bidirectional assertions
    assert txSucceeded => initiallyAllowed,
        "Transaction succeeded with unauthorized sender";
    assert initiallyAllowed => txSucceeded,
        "Transaction failed despite sender being authorized and valid preconditions";
}