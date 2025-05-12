using MetabasedSequencerChain as mchain;

methods {
    function mchain.isAllowed(address, address, bytes) external returns (bool) envfree;
    function mchain.processTransaction(bytes) external;
    function mchain.processTransactionRaw(bytes) external;
    function mchain.processBulkTransactions(bytes[]) external;
}

rule emptyArraysRevert() {
    env e;
    address[] chains;
    bytes[] transactions;
    bool[] isRaw;

    require chains.length == 0;
    require transactions.length == 0;
    require isRaw.length == 0;

    processTransactionsAtomically@withrevert(e, chains, transactions, isRaw);
    assert lastReverted, "Empty arrays should revert";
}

rule mismatchedLengthsRevert() {
    env e;
    address[] chains;
    bytes[] transactions;
    bool[] isRaw;

    require chains.length != transactions.length || chains.length != isRaw.length;
    require chains.length > 0;

    processTransactionsAtomically@withrevert(e, chains, transactions, isRaw);
    assert lastReverted, "Mismatched lengths should revert";
}

rule bulkProcessingLengthRequirements() {
    env e;
    address[] chains;
    bytes[][] transactions;

    require chains.length == 0 || chains.length != transactions.length;

    processBulkTransactionsAtomically@withrevert(e, chains, transactions);
    assert lastReverted, "Invalid array lengths should revert";
}