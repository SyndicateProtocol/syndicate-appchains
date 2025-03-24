using ProposerPermissionModuleBasic as proposerPermission;
using CalldataPermissionModuleBasic as dataPermission;
using InitializableHarness as init;

methods {
    // View functions
    function l3ChainId() external returns (uint256) envfree;
    function proposerRequirementModule() external returns (address) envfree;
    function calldataRequirementModule() external returns (address) envfree;
    function isAllowed(address) external returns (bool) envfree;
    function isCalldataAllowed(bytes) external returns (bool) envfree;
    function owner() external returns (address) envfree;
    function init._getInitializedVersion() external returns (uint8) envfree;

    // Permission Module interface methods
    function proposerPermission.isAllowed(address) external returns (bool) envfree;
    function dataPermission.isCalldataAllowed(bytes) external returns (bool) envfree;
    function dataPermission.allCalldataAllowed() external returns (bool) envfree;
    function dataPermission.setAllowedAll(bool) external;
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

    assert proposerRequirementModule() == module, "Proposer module not set correctly";
    assert calldataRequirementModule() == module, "Calldata module not set correctly";
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

    // Get modules before
    address oldProposerModule = proposerRequirementModule();
    address oldCalldataModule = calldataRequirementModule();

    // Function call
    calldataarg args;
    f(e, args);

    // Get modules after
    address newProposerModule = proposerRequirementModule();
    address newCalldataModule = calldataRequirementModule();

    // Assert modules cannot be zero address after initialization
    require init._getInitializedVersion() > 0 => oldProposerModule != 0;
    assert init._getInitializedVersion() > 0 => newProposerModule != 0, "Proposer module changed to zero";

    require init._getInitializedVersion() > 0 => oldCalldataModule != 0;
    assert init._getInitializedVersion() > 0 => newCalldataModule != 0, "Calldata module changed to zero";
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
    assert success => proposerPermission.isAllowed(e.msg.sender),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 6: Only allowed calldata can be processed
 */
rule onlyAllowedCalldata(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;
    require proposerPermission.isAllowed(e.msg.sender);

    // Set calldata permission to false
    env e_perm;
    dataPermission.setAllowedAll(false);
    require !dataPermission.allCalldataAllowed();

    // Try to process a transaction with disallowed calldata
    processTransaction@withrevert(e, data);
    bool disallowedSuccess = !lastReverted;

    // Verify transaction is rejected
    assert !disallowedSuccess, "Transaction succeeded with disallowed calldata";

    // Now set calldata permission to true
    dataPermission.setAllowedAll(true);
    require dataPermission.allCalldataAllowed();

    // Try to process again
    processTransaction@withrevert(e, data);
    bool allowedSuccess = !lastReverted;

    // Verify transaction succeeds
    assert allowedSuccess, "Transaction failed with allowed calldata";
}

/*
 * Rule 7: Consistent behavior between processTransaction and processTransactionRaw
 */
rule processConsistency(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;
    require proposerPermission.isAllowed(e.msg.sender);
    require dataPermission.allCalldataAllowed();

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
    require proposerPermission.isAllowed(e.msg.sender);
    require dataPermission.allCalldataAllowed();
    require data.length < 3; // Loop unrolling limit - Certora will unroll up to this limit

    // Process transactions in bulk
    processBulkTransactions@withrevert(e, data);
    bool bulkSuccess = !lastReverted;

    // If bulk processing succeeded, each individual transaction should succeed
    require bulkSuccess;

    // Check individual transactions would succeed
    if (data.length > 0) {
        processTransaction@withrevert(e, data[0]);
        assert !lastReverted, "Bulk processing accepted invalid transaction";
    }

    if (data.length > 1) {
        processTransaction@withrevert(e, data[1]);
        assert !lastReverted, "Bulk processing accepted invalid transaction";
    }

    if (data.length > 2) {
        processTransaction@withrevert(e, data[2]);
        assert !lastReverted, "Bulk processing accepted invalid transaction";
    }
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
    address oldProposerModule = proposerRequirementModule();

    // Update module
    updateRequirementModule@withrevert(e, newModule);

    // If successful, module should be updated
    assert !lastReverted => proposerRequirementModule() == newModule,
        "Proposer module not updated correctly";
}

/*
 * Rule 11: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;
    address oldProposerModule = proposerRequirementModule();
    address oldCalldataModule = calldataRequirementModule();

    // Process transaction
    processTransaction@withrevert(e, data);

    // Verify requirement modules haven't changed
    assert proposerRequirementModule() == oldProposerModule,
        "Transaction processing modified proposer module state";
    assert calldataRequirementModule() == oldCalldataModule,
        "Transaction processing modified calldata module state";
}

/*
 * Rule 12: Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data) {
    env e;

    // Setup variables for initialization
    address admin = e.msg.sender;
    address proposerModule = proposerPermission;
    address dataModule = dataPermission;

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
    bool senderAllowed = proposerPermission.isAllowed(e.msg.sender);
    bool dataAllowed = dataPermission.allCalldataAllowed();

    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;

    // Bidirectional assertions
    assert txSucceeded => (senderAllowed && dataAllowed),
        "Transaction succeeded with unauthorized sender or unallowed calldata";
    assert (senderAllowed && dataAllowed) => txSucceeded,
        "Transaction failed despite permissions being valid and preconditions met";
}