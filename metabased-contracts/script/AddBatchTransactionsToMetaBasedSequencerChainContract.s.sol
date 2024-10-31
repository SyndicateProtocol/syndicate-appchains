// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";

contract AddBatchTransactionsToMetabasedSequencerChainContract is Script {
    MetabasedSequencerChain public metabasedSequencerChainContract;

    function run() public {
        vm.startBroadcast();
        // MetabasedSequencerChain address on Base Sepolia
        metabasedSequencerChainContract = MetabasedSequencerChain(0x73Ba7D784d13Ec0070a4Ea6F49Ff57dc007Bb48d);

        // Example of adding multiple transactions in bulk.

        // create transaction list
        bytes[] memory transactionList = new bytes[](2);
        // send 1 wei to the MetabasedSequencerChain contract
        address payable recipient = payable(address(metabasedSequencerChainContract));
        transactionList[0] = abi.encode(recipient, uint256(1));
        // send 2 wei to the MetabasedSequencerChain contract
        transactionList[1] = abi.encode(recipient, uint256(2));

        metabasedSequencerChainContract.processBulkTransactions(transactionList);

        vm.stopBroadcast();
    }
}
