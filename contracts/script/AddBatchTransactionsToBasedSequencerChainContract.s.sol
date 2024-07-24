// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";

import {BasedSequencerChain} from "src/BasedSequencerChain.sol";

contract AddBatchTransactionsToBasedSequencerChainContract is Script {
    BasedSequencerChain public basedSequencerChain;

    function run() public {
        vm.startBroadcast();
        // BaseSequencerChain address on Base Sepolia
        basedSequencerChain = BasedSequencerChain(0xdf4F1Be1fc0222Bb45c3a6B85F977AC89f49a9C5);

        // Example of adding batch transactions to the BasedSequencerChain
        // get last non-empty epoch number
        uint256 lastNonEmptyEpochNumber = basedSequencerChain.lastNonEmptyEpochNumber();
        if (lastNonEmptyEpochNumber == 0) {
            lastNonEmptyEpochNumber = basedSequencerChain.INITIAL_EPOCH_NUMBER();
        }

        // get Batch struct from last epoch number
        (,,, bytes32 previousBatchEpochHash) = basedSequencerChain.batches(lastNonEmptyEpochNumber);

        // create transaction list
        bytes[] memory transactionList = new bytes[](1);
        // send 1 wei to the BasedSequencerChain contract
        address payable recipient = payable(address(basedSequencerChain));
        transactionList[0] = abi.encode(recipient, uint256(1));

        BasedSequencerChain.UserProvidedBatch memory userProvidedBatch = BasedSequencerChain.UserProvidedBatch({
            non_empty_parent_hash: previousBatchEpochHash,
            transaction_list: transactionList
        });

        // Assumes that AlwaysAllowedModule is added to the addRequireAllCheck() of BasedSequencerChain
        basedSequencerChain.sequenceNextBatch(userProvidedBatch);

        vm.stopBroadcast();
    }
}
