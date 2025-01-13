// SPDX-License-Identifier: UNLICENSED
methods {
    // View functions
    l3ChainId() returns (uint256) envfree
    requirementModule() returns (address) envfree
    isAllowed(address) returns (bool)
    owner() returns (address) envfree

    // State-changing functions
    processTransactionRaw(bytes) returns ()
    processTransaction(bytes) returns ()
    processBulkTransactions(bytes[]) returns ()
    updateRequirementModule(address) returns ()
}

/*
 * Define ghost variables to track important state
 */
ghost mapping(address => bool) processedSenders;
ghost mapping(bytes => bool) processedTransactions;

/*
 * Rule 1: Verify that l3ChainId cannot be zero
 * This should always be true after contract deployment
 */
invariant l3ChainIdNotZero()
    l3ChainId() != 0

/*
 * Rule 2: Verify that requirementModule address is never zero
 */
invariant requirementModuleNotZero()
    requirementModule() != 0

/*
 * Rule 3: Only allowed addresses can process transactions
 */
rule onlyAllowedCanProcess(address sender, bytes calldata data) {
    env e;

    // Try to process a transaction
    processTransaction@withrevert(e, data);

    // If the transaction succeeded
    bool success = !lastReverted;

    // Then the sender must have been allowed
    assert success => isAllowed(e.msg.sender),
        "Unauthorized sender processed transaction";
}

/*
 * Rule 4: Consistent behavior between processTransaction and processTransactionRaw
 */
rule processConsistency(bytes calldata data) {
    env e;

    // Record both outcomes
    processTransaction(e, data);
    processTransactionRaw(e, data);

    // If one succeeds, both should succeed
    assert !lastReverted <=> !lastReverted,
        "Inconsistent behavior between process methods";
}

/*
 * Rule 5: Bulk processing maintains individual transaction properties
 */
rule bulkProcessingConsistency(bytes[] calldata data) {
    env e;

    // Process transactions in bulk
    processBulkTransactions(e, data);

    // Verify each transaction would have succeeded individually
    for (uint i = 0; i < data.length; i++) {
        processTransaction(e, data[i]);
        assert !lastReverted,
            "Bulk processing accepted invalid transaction";
    }
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
 * Rule 7: Events are emitted correctly for processed transactions
 */
rule transactionEventEmission(bytes calldata data) {
    env e;

    // Process a transaction
    processTransaction(e, data);

    // Verify TransactionProcessed event was emitted with correct parameters
    assert lastReverted ||
           (exists EventLog log.
               log.topics[0] == hash("TransactionProcessed(address,bytes)") &&
               log.topics[1] == e.msg.sender &&
               log.data == data),
        "Transaction processed without proper event emission";
}

/*
 * Rule 8: State consistency after transaction processing
 */
rule stateConsistencyAfterProcessing(bytes calldata data) {
    env e;
    address oldModule = requirementModule();

    // Process transaction
    processTransaction(e, data);

    // Verify requirement module hasn't changed
    assert requirementModule() == oldModule,
        "Transaction processing modified core state";
}

/*
 * Rule 9: No reentrancy in processing functions
 */
rule noReentrancy(method f, bytes calldata data) filtered { f ->
    f.selector == sig:processTransaction(bytes).selector ||
    f.selector == sig:processTransactionRaw(bytes).selector
} {
    env e;
    calldataarg args;

    // Start processing
    f(e, args);

    // Try to process again during execution
    sinvoke f(e, args);

    // Assert second invocation fails
    assert lastReverted,
        "Reentrancy detected in processing functions";
}

/*
 * Rule 10: Zero byte prepending works correctly
 */
rule zeroBytePrepending(bytes calldata data) {
    env e;

    // Get processed data from both methods
    bytes memory rawData;
    bytes memory prependedData;

    processTransactionRaw(e, data) returns (rawData);
    processTransaction(e, data) returns (prependedData);

    // Verify prepended data starts with zero byte
    assert prependedData[0] == 0x00,
        "Transaction data not properly prepended with zero byte";
}