// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {IsAllowed} from "src/interfaces/IsAllowed.sol";

contract MockIsAllowed is IsAllowed {
    function isAllowed(address) external pure override returns (bool) {
        return true;
    }
}

contract MetabaseSequencerChainLimitTest is Test {
    MetabasedSequencerChain public chain;
    address public admin;

    function setUp() public {
        admin = address(0x1);
        vm.startPrank(admin);
        uint256 l3ChainId = 10042001;
        chain = new MetabasedSequencerChain(l3ChainId, admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed()), false);
        vm.stopPrank();
    }

    function testLargeTransaction() public {
        uint256 size = 128 * 1024; // 128KB
        bytes memory largeTxn = new bytes(size);

        for (uint256 i = 0; i < size; i++) {
            largeTxn[i] = bytes1(uint8(i % 256));
        }

        uint256 gasStart = gasleft();
        bool txIsCompressed = true;
        chain.processTransaction(largeTxn, txIsCompressed);
        uint256 gasUsed = gasStart - gasleft();

        console.log("Gas used for 128KB transaction:", gasUsed);
        console.log("Transaction size:", size, "bytes");
    }

    function testBulkTransactionsApproachingLimit() public {
        uint256 individualSize = 32 * 1024; // 32KB
        uint256 count = 4; // 4 * 32KB = 128KB total
        bytes[] memory txns = new bytes[](count);
        bool[] memory isCompressed = new bool[](count);

        for (uint256 i = 0; i < count; i++) {
            bytes memory txn = new bytes(individualSize);
            for (uint256 j = 0; j < individualSize; j++) {
                txn[j] = bytes1(uint8((i * individualSize + j) % 256));
            }
            txns[i] = txn;
            isCompressed[i] = true;
        }

        uint256 gasStart = gasleft();
        chain.processBulkTransactions(txns, isCompressed);
        uint256 gasUsed = gasStart - gasleft();

        console.log("Gas used for bulk transactions (4 * 32KB):", gasUsed);
        console.log("Total size:", individualSize * count, "bytes");
        console.log("Number of transactions:", count);
    }

    function testMaximumBulkTransactions() public {
        uint256 maxGas = 30_000_000; // Approximate max gas limit on Ethereum mainnet
        uint256 individualSize = 1024; // 1KB
        uint256 count = 0;
        bytes[] memory txns = new bytes[](128); // Start with 128 transactions
        bool[] memory isCompressed = new bool[](128);

        uint256 gasStart = gasleft();
        uint256 gasUsed = 0;

        while (gasUsed < maxGas && count < 128) {
            bytes memory txn = new bytes(individualSize);
            for (uint256 j = 0; j < individualSize; j++) {
                txn[j] = bytes1(uint8((count * individualSize + j) % 256));
            }
            txns[count] = txn;
            isCompressed[count] = true;
            count++;

            try chain.processBulkTransactions(txns, isCompressed) {
                gasUsed = gasStart - gasleft();
            } catch {
                count--;
                break;
            }
        }

        console.log("Maximum number of 1KB transactions processed:", count);
        console.log("Total size processed:", individualSize * count, "bytes");
        console.log("Gas used:", gasUsed);
    }
}
