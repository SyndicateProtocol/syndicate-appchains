// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Script} from "forge-std/Script.sol";

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";

contract AddBatchTransactionsToSyndicateSequencingChainContract is Script {
    SyndicateSequencingChain public SyndicateSequencingChainContract;

    function run() public {
        vm.startBroadcast();
        // SyndicateSequencingChain address on Base Sepolia
        SyndicateSequencingChainContract = SyndicateSequencingChain(0xb1567B5DFa038e4F279d3b585D4D45b8bDD2263D);

        // Example of adding multiple transactions in bulk.

        // create transaction list
        bytes[] memory transactionList = new bytes[](2);
        // send 1 wei to the SyndicateSequencingChain contract
        address payable recipient = payable(address(SyndicateSequencingChainContract));
        transactionList[0] = abi.encode(recipient, uint256(1));
        // send 2 wei to the SyndicateSequencingChain contract
        transactionList[1] = abi.encode(recipient, uint256(2));

        SyndicateSequencingChainContract.processTransactionsBulk(transactionList);

        vm.stopBroadcast();
    }
}
