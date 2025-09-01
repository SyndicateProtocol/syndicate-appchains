using PermissionModuleBasic as permissionModule;

methods {
    // Envfree view functions
    function appchainId() external returns (uint256) envfree;
    function permissionRequirementModule() external returns (address) envfree;
    function isAllowed(address, address, bytes) external returns (bool) envfree;
    function owner() external returns (address) envfree;
    function gasTrackingDisabled() external returns (bool) envfree;

    // Permission module envfree view functions
    function permissionModule.isAllowed(address, address, bytes) external returns (bool) envfree;
}

/*
 * Rule 1: Verify that appchainId cannot be zero
 */
invariant appchainIdNotZero()
    appchainId() != 0;

/*
 * Rule 2: Only allowed addresses can process transactions
 */
rule onlyAllowedCanProcess(bytes data) {
    env e;

    // Try to process a transaction
    processTransaction@withrevert(e, data);

    // If the transaction succeeded
    bool success = !lastReverted;

    // Then the sender must have been allowed
    assert success => isAllowed(e.msg.sender, e.msg.sender, data),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 3: Consistent behavior between processTransaction and processTransactionsBulk
 */
rule processConsistency(bytes data) {
    env e;

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
 * Rule 4: Only owner can update requirement module
 */
rule onlyOwnerCanUpdateModule(address newModule) {
    env e;
    require e.msg.value == 0;

    // Try to update the module
    updateRequirementModule@withrevert(e, newModule);
    bool txSucceeded = !lastReverted;

    // Bidirectional assertions
    assert txSucceeded => e.msg.sender == owner(),
        "Non-owner updated requirement module";
    assert !txSucceeded => e.msg.sender != owner(),
        "Owner failed to update requirement module";
}

/*
 * Rule 5: Module update changes state correctly
 */
rule moduleUpdateChangesState(address newModule) {
    env e;
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
 * Rule 6: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes data) {
    env e;
    address oldProposerModule = permissionRequirementModule();

    // Process transaction
    processTransaction@withrevert(e, data);

    // Verify requirement modules haven't changed
    assert permissionRequirementModule() == oldProposerModule,
        "Transaction processing modified proposer module state";
}

/*
 * Rule 7: Verify permissions are correctly enforced
 */
rule permissionsCorrectlyEnforced(bytes data) {
    env e;

    // Require the contract to be initialized
    require permissionRequirementModule() == permissionModule;
    require owner() == e.msg.sender;

    // Valid sender and msg parameters
    require e.msg.sender != 0;
    require e.msg.sender != currentContract;
    require e.msg.value == 0;

    // Valid data requirements
    require data.length > 0;
    require data.length < max_uint256;

    // Check permissions
    bool senderAllowed = isAllowed(e.msg.sender, e.msg.sender, abi.encodePacked(uint8(4), data));

    // Process transaction
    processTransaction@withrevert(e, data);
    bool txSucceeded = !lastReverted;

    // Bidirectional assertions
    assert txSucceeded => senderAllowed,
        "Transaction succeeded with unauthorized sender";
    assert senderAllowed => txSucceeded,
        "Transaction failed despite permissions being valid and preconditions met";
}
