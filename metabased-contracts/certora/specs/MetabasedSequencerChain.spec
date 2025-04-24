using PermissionModuleBasic as permissionModule;
using InitializableHarness as init;

methods {
    // View functions
    function l3ChainId() external returns (uint256) envfree;
    function permissionRequirementModule() external returns (address) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
    function owner() external returns (address) envfree;
    function init._getInitializedVersion() external returns (uint8) envfree;

    // Permission Module interface methods
    function permissionModule.isAllowed(address, address, bytes) external returns (bool) envfree;
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

    assert permissionRequirementModule() == module, "Proposer module not set correctly";
    assert owner() == admin, "Admin not set correctly";
}

/*
 * Rule 3: Verify that l3ChainId cannot be zero
 */
invariant l3ChainIdNotZero()
    l3ChainId() != 0;

/*
 * Rule 4: Verify that permissionRequirementModule address is never zero after initialization
 */
rule moduleNotZero(method f) {
    env e;

    // Get modules before
    address oldProposerModule = permissionRequirementModule();

    // Function call
    calldataarg args;
    f(e, args);

    // Get modules after
    address newProposerModule = permissionRequirementModule();

    // Assert modules cannot be zero address after initialization
    require init._getInitializedVersion() > 0 => oldProposerModule != 0;
    assert init._getInitializedVersion() > 0 => newProposerModule != 0, "Proposer module changed to zero";
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
    assert success => permissionModule.isAllowed(e.msg.sender, e.msg.sender, data),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 6: Consistent behavior between processTransaction and processTransactionRaw
 */
rule processConsistency(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;
    require permissionModule.isAllowed(e.msg.sender, e.msg.sender, data);

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
 * Rule 8: Bulk processing maintains individual transaction properties
 */
rule bulkProcessingConsistency(bytes[] data) {
    env e;
    require init._getInitializedVersion() > 0;
    require permissionModule.isAllowed(e.msg.sender, e.msg.sender, data[0]);
    require data.length > 0;
    require data.length < 3; // Loop unrolling limit - Certora will unroll up to this limit


    // Process transactions in bulk
    processBulkTransactions@withrevert(e, data);
    bool bulkSuccess = !lastReverted;

    // If bulk processing succeeded, each individual transaction should succeed
    require bulkSuccess;

    // Check individual transactions would succeed
    processTransaction@withrevert(e, data[0]);
    assert !lastReverted, "Bulk processing accepted invalid transaction";


    processTransaction@withrevert(e, data[1]);
    assert !lastReverted, "Bulk processing accepted invalid transaction";


    processTransaction@withrevert(e, data[2]);
    assert !lastReverted, "Bulk processing accepted invalid transaction";
}

/*
 * Rule 9: Only owner can update requirement module
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
 * Rule 10: Module update changes state correctly
 */
rule moduleUpdateChangesState(address newModule) {
    env e;
    require init._getInitializedVersion() > 0;
    require newModule != 0;

    // Store old module
    address oldProposerModule = permissionRequirementModule();

    // Update module
    updateRequirementModule@withrevert(e, newModule);

    // If successful, module should be updated
    assert !lastReverted => permissionRequirementModule() == newModule,
        "Proposer module not updated correctly";
}

/*
 * Rule 11: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;
    address oldProposerModule = permissionRequirementModule();

    // Process transaction
    processTransaction@withrevert(e, data);

    // Verify requirement modules haven't changed
    assert permissionRequirementModule() == oldProposerModule,
        "Transaction processing modified proposer module state";
}

/*
 * Rule 12: Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data) {
    env e;

    // Setup variables for initialization
    address admin = e.msg.sender;
    address proposerModule = permissionModule;

    // Initialize the contract first
    initialize(e, admin, proposerModule);

    // Verify initialization worked
    require init._getInitializedVersion() == 1;

    // Valid sender and msg parameters
    require e.msg.sender != 0;
    require e.msg.sender != currentContract;
    require e.msg.value == 0;

    // Valid data requirements
    require data.length > 0;
    require data.length < max_uint256;

    // Check permissions
    bool senderAllowed = permissionModule.isAllowed(e.msg.sender, e.msg.sender, data);

    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;

    // Bidirectional assertions
    assert txSucceeded => senderAllowed,
        "Transaction succeeded with unauthorized sender";
    assert senderAllowed => txSucceeded,
        "Transaction failed despite permissions being valid and preconditions met";
}