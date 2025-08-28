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
    // Contract nonce function
    function contractNonce(address) external returns (uint256) envfree;
    // Permission module envfree view functions
    function permissionModule.isAllowed(address, address, bytes) external returns (bool) envfree;
    function permissionModule.setAllowed(address, bool) external;
}

/*
 * Rule 1: Initialization rules
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
 * Rule 2: Initialization sets correct values
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
 * Rule 3: AppchainId is set correctly after initialization
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
 * Rule 4: Only allowed addresses can process transactions
 */
rule onlyAllowedCanProcess(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require data.length > 0;
    require data.length <= 1024;
    require e.msg.value == 0;
    // CRITICAL: Disable gas tracking
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
 * Rule 5: Test bulk processing with no permission restrictions
 */
rule processConsistencyNoPermissions(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require data.length > 0;
    require data.length <= 1024;
    require e.msg.value == 0;
    // CRITICAL: Disable gas tracking
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
 * Rule 6: Test that permission module address matters
 */
rule permissionModuleRequired(bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require data.length > 0;
    require data.length <= 1024;
    require e.msg.value == 0;
    // CRITICAL: Disable gas tracking
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
 * Rule 6: Only owner can update requirement module
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
 * Rule 7: Module update changes state correctly
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
 * Rule 8: State consistency after transaction processing
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
    require getInitializedVersion() == 1;
    // Disable gas tracking for consistent verification
    require !gasTrackingEnabled();
    // Valid sender and msg parameters
    require e.msg.sender != 0;
    require e.msg.sender != currentContract;
    require e.msg.value == 0;
    // Valid data requirements
    require data.length > 0;
    require data.length < max_uint256;
    // Check permissions using bound permission module directly
    bool senderAllowed = permissionModule.isAllowed(e.msg.sender, e.tx.origin, data);
    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;
    // Bidirectional assertions
    assert txSucceeded => senderAllowed, "Transaction succeeded with unauthorized sender";
    assert senderAllowed => txSucceeded, "Transaction failed despite permissions being valid and preconditions met";
}

/*
 * Rule 10: Only owner can perform upgrades
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
 * Rule 11: Only owner can set emissions receiver
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
 * Rule 12: Emissions receiver getter consistency
 */
rule emissionsReceiverConsistency() {
    require getInitializedVersion() > 0;
    address receiver = emissionsReceiver();
    address effectiveReceiver = getEmissionsReceiver();
    assert receiver == 0 => effectiveReceiver == owner(), "When no explicit receiver set, should return owner";
    assert receiver != 0 => effectiveReceiver == receiver, "When explicit receiver set, should return that receiver";
}

/*
 * Rule 13: Contract nonce only increases for sendContractTransaction
 */
rule contractNonceIncreasesOnContractTx(address user, uint64 gasLimit, uint256 maxFeePerGas, address to, uint256 value, bytes data) {
    env e;
    require getInitializedVersion() > 0;
    require e.msg.sender == user;
    uint256 nonceBefore = contractNonce(user);
    sendContractTransaction(e, gasLimit, maxFeePerGas, to, value, data);
    uint256 nonceAfter = contractNonce(user);
    assert nonceAfter == nonceBefore + 1, "Contract nonce should increment by 1 for sendContractTransaction";
}

/*
 * Rule 14: Contract nonce unchanged by other operations
 */
rule contractNonceUnchangedByOtherOps(address user, bytes txData) {
    env e;
    require getInitializedVersion() > 0;
    require e.msg.sender != user; // Different sender
    uint256 nonceBefore = contractNonce(user);
    // Test with processTransaction (should not affect nonces)
    processTransaction@withrevert(e, txData);
    uint256 nonceAfter = contractNonce(user);
    assert nonceAfter == nonceBefore, "Contract nonce should not change for non-contract transactions";
}

