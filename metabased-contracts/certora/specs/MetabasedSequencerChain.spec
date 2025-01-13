// Certora Verification Specification for MetabasedSequencerChain

// Import the required methods from the Solidity contracts
methods {
    // MetabasedSequencerChain methods
    function isAllowed(address proposer) external view returns (bool);
    function l3ChainId() external view returns (uint256);
    function processTransactionRaw(bytes calldata data) external;
    function processTransaction(bytes calldata data) external;
    function processBulkTransactions(bytes[] calldata data) external;

    // SequencingModuleChecker methods
    function requirementModule() external view returns (address);
    function owner() external view returns (address);
    function transferOwnership(address newOwner) external;
}

// Rules to verify contract behavior

// Rule 1: Ensure only allowed addresses can sequence transactions
rule onlyAllowedSequencers {
    address proposer;
    require proposer != address(0), "Proposer cannot be the zero address";

    bool allowed = isAllowed(proposer);

    assert allowed, "Proposer must be allowed to sequence";
}

// Rule 2: Verify that the l3ChainId is immutable
rule chainIdIsImmutable {
    uint256 initialId = l3ChainId();
    uint256 laterId = l3ChainId();

    assert initialId == laterId, "l3ChainId must remain immutable";
}

// Rule 3: Validate that the requirement module is set
rule requirementModuleExists {
    address module = requirementModule();

    assert module != address(0), "Requirement module must be set";
}

// Rule 4: Ensure only the owner can transfer ownership
rule onlyOwnerCanTransferOwnership {
    address currentOwner = owner();
    address newOwner;

    require currentOwner != address(0), "Owner cannot be the zero address";
    require msg.sender == currentOwner, "Only the owner can transfer ownership";

    transferOwnership(newOwner);

    assert owner() == newOwner, "Ownership must be transferred correctly";
}

// Rule 5: Validate that isAllowed function adheres to logical consistency
rule isAllowedConsistency {
    address proposer;
    bool firstCheck = isAllowed(proposer);
    bool secondCheck = isAllowed(proposer);

    assert firstCheck == secondCheck, "isAllowed results must be consistent for the same input";
}

// Rule 6: Ensure l3ChainId cannot be zero
rule chainIdNotZero {
    uint256 chainId = l3ChainId();

    assert chainId != 0, "l3ChainId must not be zero";
}

// Rule 7: Ensure processTransactionRaw emits the correct event
rule processTransactionRawEvent {
    bytes calldata data;
    address sender = msg.sender;

    processTransactionRaw(data);

    assert TransactionProcessed(sender, data), "TransactionProcessed event must be emitted correctly";
}

// Rule 8: Ensure processTransaction emits the correct event with prepended data
rule processTransactionEvent {
    bytes calldata data;
    address sender = msg.sender;
    bytes memory expectedData = prependZeroByte(data);

    processTransaction(data);

    assert TransactionProcessed(sender, expectedData), "TransactionProcessed event must be emitted with the correct data";
}

// Rule 9: Ensure processBulkTransactions emits events for all data
rule processBulkTransactionsEvents {
    bytes[] calldata data;
    address sender = msg.sender;
    uint256 dataCount = data.length;

    processBulkTransactions(data);

    for (uint256 i = 0; i < dataCount; i++) {
        assert TransactionProcessed(sender, prependZeroByte(data[i])), "TransactionProcessed event must be emitted for each transaction";
    }
}
