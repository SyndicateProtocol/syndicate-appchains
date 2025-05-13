// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Script} from "forge-std/Script.sol";

import {SyndicateSequencerChain} from "src/SyndicateSequencerChain.sol";

contract AddBatchTransactionsToSyndicateSequencerChainContract is Script {
    SyndicateSequencerChain public syndicateSequencerChainContract;

    function run() public {
        vm.startBroadcast();
        // SyndicateSequencerChain address on Base Sepolia
        syndicateSequencerChainContract = SyndicateSequencerChain(0xb1567B5DFa038e4F279d3b585D4D45b8bDD2263D);

        // Example of adding multiple transactions in bulk.

        // create transaction list
        bytes[] memory transactionList = new bytes[](2);
        // send 1 wei to the SyndicateSequencerChain contract
        address payable recipient = payable(address(syndicateSequencerChainContract));
        transactionList[0] = abi.encode(recipient, uint256(1));
        // send 2 wei to the SyndicateSequencerChain contract
        transactionList[1] = abi.encode(recipient, uint256(2));

        syndicateSequencerChainContract.processBulkTransactions(transactionList);

        vm.stopBroadcast();
    }
}
