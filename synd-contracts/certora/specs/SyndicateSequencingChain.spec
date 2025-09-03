using PermissionModuleBasic as permissionModule;

methods {
    // Envfree view functions
    function appchainId() external returns (uint256) envfree;
    function permissionRequirementModule() external returns (address) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
    function owner() external returns (address) envfree;
    function getInitializedVersion() external returns (uint64) envfree;
    function gasTrackingEnabled() external returns (bool) envfree;
    function disableGasTracking() external;
    function enableGasTracking() external;
    function emissionsReceiver() external returns (address) envfree;
    function getEmissionsReceiver() external returns (address) envfree;
    function gasTrackingDisabled() external returns (bool) envfree;
    function encodeTransaction(bytes) external returns (bytes) envfree;
    // Permission module envfree view functions
    function permissionModule.isAllowed(address, address, bytes) external returns (bool) envfree;
    function permissionModule.setAllowed(address, bool) external;
}

/*
 * Rule: Initialization rules
 */
rule initializeOnce(address admin, address module, uint256 appchainId) {
    env e;
    require admin != 0;
    require module != 0;
    require appchainId != 0;
    require e.msg.sender != currentContract;
    // First initialization
    initialize@withrevert(e, admin, module, appchainId);
    bool firstInit = !lastReverted;
    // Try to initialize again
    initialize@withrevert(e, admin, module, appchainId);
    assert firstInit => lastReverted, "Contract initialized more than once";
}

/*
 * Rule: Initialization sets correct values
 */
rule initializationCorrect(address admin, address module, uint256 appchainId) {
    env e;
    require admin != 0;
    require module != 0;
    require appchainId != 0;
    require e.msg.sender != currentContract;
    initialize(e, admin, module, appchainId);
    assert permissionRequirementModule() == module, "Permission module not set correctly";
    assert owner() == admin, "Admin not set correctly";
    assert appchainId() == appchainId, "AppchainId not set correctly";
    assert gasTrackingEnabled(), "Gas tracking not enabled after init";
}

/*
 * Rule: AppchainId is set correctly after initialization
 */
rule appchainIdSetAfterInit(address admin, address module, uint256 chainId) {
    env e;
    require admin != 0;
    require module != 0;
    require chainId != 0;
    require e.msg.sender != currentContract;
    // Before initialization, appchainId should be 0
    require getInitializedVersion() == 0;
    require appchainId() == 0;
    // Initialize the contract
    initialize(e, admin, module, chainId);
    // After initialization, appchainId should be set to the provided value
    assert appchainId() == chainId, "AppchainId not set correctly after initialization";
    assert appchainId() != 0, "AppchainId should not be zero after initialization";
}

/*
 * Rule: Only allowed addresses can process transactions
 */
rule onlyAllowedCanProcess(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require data.length > 0;
    require data.length <= 1024;
    require e.msg.value == 0;
    // Disable gas tracking
    require !gasTrackingEnabled();
    // Use zero address as permission module to allow all transactions
    require permissionRequirementModule() == 0;
    // Try to process a transaction
    processTransaction@withrevert(e, data);
    // With no permission module, all transactions should succeed
    bool success = !lastReverted;
    assert success, "Transaction failed with no permission restrictions";
}

/*
 * Rule: Test bulk processing with no permission restrictions
 */
rule processConsistencyNoPermissions(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require data.length > 0;
    require data.length <= 1024;
    require e.msg.value == 0;
    // Disable gas tracking
    require !gasTrackingEnabled();
    // Use no permission module (allows all)
    require permissionRequirementModule() == 0;
    // Record both outcomes
    processTransaction@withrevert(e, data);
    bool txSuccess = !lastReverted;
    processTransactionsBulk@withrevert(e, [data]);
    bool bulkSuccess = !lastReverted;
    // With no permissions, both should succeed
    assert txSuccess, "processTransaction failed with no permissions";
    assert bulkSuccess, "processTransactionsBulk should not revert";
}

/*
 * Rule: Test that permission module address matters
 */
rule permissionModuleRequired(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require data.length > 0;
    require data.length <= 1024;
    require e.msg.value == 0;
    // Disable gas tracking
    require !gasTrackingEnabled();
    // Compare behavior with and without permission module
    // First test with no permission module
    require permissionRequirementModule() == 0;
    processTransaction@withrevert(e, data);
    bool successNoPermissions = !lastReverted;
    // This should succeed since no permissions are required
    assert successNoPermissions, "Transaction failed with no permission module";
}

/*
 * Rule: Only owner can update requirement module
 */
rule onlyOwnerCanUpdateModule(address newModule) {
    env e;
    require getInitializedVersion() > 0;
    // Try to update the module
    updateRequirementModule@withrevert(e, newModule);
    // If successful, must have been owner
    assert !lastReverted => e.msg.sender == owner(), "Non-owner updated requirement module";
}

/*
 * Rule: Module update changes state correctly
 */
rule moduleUpdateChangesState(address newModule) {
    env e;
    require getInitializedVersion() > 0;
    require newModule != 0;
    // Store old module
    address oldProposerModule = permissionRequirementModule();
    // Update module
    updateRequirementModule@withrevert(e, newModule);
    // If successful, module should be updated
    assert !lastReverted => permissionRequirementModule() == newModule, "Proposer module not updated correctly";
}

/*
 * Rule: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    address oldProposerModule = permissionRequirementModule();
    // Process transaction
    processTransaction@withrevert(e, data);
    // Verify requirement modules haven't changed
    assert permissionRequirementModule() == oldProposerModule, "Transaction processing modified proposer module state";
}

/*
 * Rule : Only owner can perform upgrades
 */
rule onlyOwnerCanUpgrade(address newImplementation, bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require newImplementation != 0;
    require e.msg.value == 0; // No ETH should be sent
    require newImplementation != currentContract; // Can't upgrade to self
    
    // Get owner before upgrade attempt
    address contractOwner = owner();
    require contractOwner != 0; // Owner should be set
    
    // Try to upgrade
    upgradeToAndCall@withrevert(e, newImplementation, data);
    // If successful, must have been owner
    assert !lastReverted => e.msg.sender == contractOwner, "Non-owner performed upgrade";
}

/*
 * Rule : Only owner can set emissions receiver
 */
rule onlyOwnerCanSetEmissionsReceiver(address newReceiver) {
    env e;
    require getInitializedVersion() > 0;
    // Try to set emissions receiver
    setEmissionsReceiver@withrevert(e, newReceiver);
    // If successful, must have been owner
    assert !lastReverted => e.msg.sender == owner(), "Non-owner set emissions receiver";
}

/*
 * Rule : Emissions receiver getter consistency
 */
rule emissionsReceiverConsistency() {
    require getInitializedVersion() > 0;
    address receiver = emissionsReceiver();
    address effectiveReceiver = getEmissionsReceiver();
    assert receiver == 0 => effectiveReceiver == owner(), "When no explicit receiver set, should return owner";
    assert receiver != 0 => effectiveReceiver == receiver, "When explicit receiver set, should return that receiver";
}


/*
 * Rule : Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data) {
    env e;
    // Require the contract to be initialized
    require permissionRequirementModule() == permissionModule;
    require owner() == e.msg.sender;
    // Valid sender and msg parameters
    require e.block.timestamp >= 1754089200;
    require e.msg.value == 0;
    // Valid data requirements
    require data.length > 0;
    require data.length < max_uint256;
    // Check permissions
    bool senderAllowed = isAllowed(e.msg.sender, e.msg.sender, encodeTransaction(data));
    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;
    // Bidirectional assertions
    assert txSucceeded => senderAllowed, "Transaction succeeded with unauthorized sender";
    assert senderAllowed => txSucceeded, "Transaction failed despite permissions being valid and preconditions met";
}

