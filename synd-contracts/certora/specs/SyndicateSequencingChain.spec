using PermissionModuleBasic as permissionModule;
using InitializableHarness as init;

methods {
    // View functions
    function appchainId() external returns (uint256) envfree;
    function permissionRequirementModule() external returns (address) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
    function owner() external returns (address) envfree;
    function init._getInitializedVersion() external returns (uint8) envfree;

    // Gas tracking functions
    function gasTrackingEnabled() external returns (bool) envfree;
    function disableGasTracking() external envfree;
    function enableGasTracking() external envfree;

    // Permission Module interface methods
    function permissionModule.isAllowed(address, address, bytes) external returns (bool) envfree;
}

/*
 * Rule 1: Initialization rules
 */
rule initializeOnce(address admin, address module, uint256 appchainId) {
    env e;
    require admin != 0;
    require module != 0;
    require appchainId != 0;

    // First initialization
    initialize@withrevert(e, admin, module, appchainId);
    bool firstInit = !lastReverted;

    // Try to initialize again
    initialize@withrevert(e, admin, module, appchainId);

    assert firstInit => lastReverted,
        "Contract initialized more than once";
}

/*
 * Rule 2: Initialization sets correct values
 */
rule initializationCorrect(address admin, address module, uint256 appchainId) {
    env e;
    require admin != 0;
    require module != 0;
    require appchainId != 0;

    initialize(e, admin, module, appchainId);

    assert permissionRequirementModule() == module, "Proposer module not set correctly";
    assert owner() == admin, "Admin not set correctly";
}

/*
 * Rule 3: Verify that appchainId is zero before initialization and non-zero after
 */
invariant appchainIdNotZero()
    init._getInitializedVersion() > 0 ? appchainId() != 0 : appchainId() == 0;

/*
 * Rule 4: Only allowed addresses can process transactions uncompressed
 */
rule onlyAllowedCanProcess(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;

    // Try to process a transaction
    processTransaction@withrevert(e, data);

    // If the transaction succeeded
    bool success = !lastReverted;

    // Then the sender must have been allowed
    assert success => permissionRequirementModule() == 0 || permissionModule.isAllowed(e.msg.sender, e.msg.sender, data),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 5: Consistent behavior between processTransaction and processTransactionsBulk
 */
rule processConsistency(bytes data) {
    env e;
    require init._getInitializedVersion() > 0;

    // Disable gas tracking for consistent verification
    require !gasTrackingEnabled();

    // Record both outcomes
    processTransaction@withrevert(e, data);
    bool txSuccess = !lastReverted;

    processTransactionsBulk@withrevert(e, [data]);
    bool bulkSuccess = !lastReverted;

    // If one succeeds, both should succeed under same conditions
    assert txSuccess == bulkSuccess,
        "Inconsistent behavior between process methods";
}

/*
 * Rule 6: Only owner can update requirement module
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
 * Rule 7: Module update changes state correctly
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
 * Rule 8: State consistency after transaction processing
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
 * Rule 9: Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data, uint256 appchainId) {
    env e;
    require appchainId != 0;

    // Setup variables for initialization
    address admin = e.msg.sender;
    address proposerModule = permissionModule;

    // Initialize the contract first
    initialize(e, admin, proposerModule, appchainId);

    // Verify initialization worked
    require init._getInitializedVersion() == 1;

    // Disable gas tracking for consistent verification
    require !gasTrackingEnabled();

    // Valid sender and msg parameters
    require e.msg.sender != 0;
    require e.msg.sender != currentContract;
    require e.msg.value == 0;

    // Valid data requirements
    require data.length > 0;
    require data.length < max_uint256;

    // Check permissions
    bool senderAllowed = permissionRequirementModule() == 0 || permissionModule.isAllowed(e.msg.sender, e.msg.sender, data);

    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;

    // Bidirectional assertions
    assert txSucceeded => senderAllowed,
        "Transaction succeeded with unauthorized sender";
    assert senderAllowed => txSucceeded,
        "Transaction failed despite permissions being valid and preconditions met";
}
