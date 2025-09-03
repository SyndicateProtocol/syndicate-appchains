using SyndicateSequencingChain as syndicateChain;

methods {
    function syndicateChain.isAllowed(address, address, bytes) external returns (bool) envfree;
}

rule emptyArraysRevert() {
    env e;
    address[] chains;
    bytes[] transactions;

    require chains.length == 0;
    require transactions.length == 0;

    processTransactionsAtomically@withrevert(e, chains, transactions);
    assert lastReverted, "Empty arrays should revert";
}

rule mismatchedLengthsRevert() {
    env e;
    address[] chains;
    bytes[] transactions;

    require chains.length != transactions.length;
    require chains.length > 0;

    processTransactionsAtomically@withrevert(e, chains, transactions);
    assert lastReverted, "Mismatched lengths should revert";
}

rule bulkProcessingLengthRequirements() {
    env e;
    address[] chains;
    bytes[][] transactions;

    require chains.length == 0 || chains.length != transactions.length;

    processTransactionsBulkAtomically@withrevert(e, chains, transactions);
    assert lastReverted, "Invalid array lengths should revert";
}
