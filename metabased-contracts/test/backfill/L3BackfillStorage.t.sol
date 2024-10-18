// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {L3BackfillStorage} from "../../src/backfill/L3BackfillStorage.sol";

contract L3BackfillStorageTest is Test {
    address public manager;
    address public nonManager;
    L3BackfillStorage public l3Storage;

    function setUp() public {
        manager = address(0x1);
        nonManager = address(0x2);
        uint256 l3ChainId = 10042001;
        l3Storage = new L3BackfillStorage(manager, manager, l3ChainId);
    }

    function testOnlyManagerCanSave() public {
        vm.expectRevert();
        vm.prank(nonManager);
        l3Storage.save(1, 1, "0x");
    }

    function testOnlyManagerCanSaveForMany() public {
        uint256[] memory l1EpochBlockNumbers = new uint256[](1);
        l1EpochBlockNumbers[0] = 1;

        uint256[] memory l3BlockNumbers = new uint256[](1);
        l3BlockNumbers[0] = 100;

        bytes[] memory batches = new bytes[](1);
        batches[0] = "0x1234";

        vm.prank(nonManager);
        vm.expectRevert();
        l3Storage.saveForMany(l1EpochBlockNumbers, l3BlockNumbers, batches);
    }

    function testSaveForManyRevertsIfArraysAreDifferentLengths() public {
        uint256[] memory l1EpochBlockNumbers = new uint256[](1);
        l1EpochBlockNumbers[0] = 1;

        uint256[] memory l3BlockNumbers = new uint256[](2);
        l3BlockNumbers[0] = 100;
        l3BlockNumbers[1] = 101;

        bytes[] memory batches = new bytes[](1);
        batches[0] = "0x1234";

        vm.expectRevert();
        vm.prank(manager);
        l3Storage.saveForMany(l1EpochBlockNumbers, l3BlockNumbers, batches);
    }

    function testSave() public {
        // Prepare test data
        uint256 expectedL1EpochBlockNumber = 1;
        uint256 expectedL3BlockNumber = 1;
        bytes memory expectedBatch = "0x";

        // Listen for the Batch event
        vm.expectEmit(true, true, false, true);
        emit L3BackfillStorage.Batch(expectedL1EpochBlockNumber, expectedL3BlockNumber, expectedBatch);

        // Call the save function
        vm.prank(manager);
        uint256 gasStart = gasleft();
        l3Storage.save(expectedL1EpochBlockNumber, expectedL3BlockNumber, expectedBatch);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for addInterval:", gasUsed);
    }

    function testSaveForMany() public {
        // Prepare test data
        uint256[] memory l1EpochBlockNumbers = new uint256[](3);
        l1EpochBlockNumbers[0] = 1;
        l1EpochBlockNumbers[1] = 2;
        l1EpochBlockNumbers[2] = 3;

        uint256[] memory l3BlockNumbers = new uint256[](3);
        l3BlockNumbers[0] = 100;
        l3BlockNumbers[1] = 101;
        l3BlockNumbers[2] = 102;

        bytes[] memory batches = new bytes[](3);
        batches[0] = "0x1234";
        batches[1] = "0x5678";
        batches[2] = "0x9abc";

        // Expect three Batch events to be emitted
        for (uint256 i = 0; i < 3; i++) {
            vm.expectEmit(true, true, false, true);
            emit L3BackfillStorage.Batch(l1EpochBlockNumbers[i], l3BlockNumbers[i], batches[i]);
        }

        // Call the saveForMany function
        vm.prank(manager);
        l3Storage.saveForMany(l1EpochBlockNumbers, l3BlockNumbers, batches);
    }

    function testGasEmptyEmit() public {
        uint256 gasStart = gasleft();
        vm.prank(manager);
        l3Storage.save(1, 1, "");
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for empty emit:", gasUsed);
    }

    function testGasForIncreasingBatchSizes() public {
        uint256 MAX_GAS_LIMIT = 30_000_000; // Approximate block gas limit
        uint256 BATCH_SIZE_INCREMENT = 100_000; // Increment batch size by 100_000 bytes each iteration
        uint256 MAX_ITERATIONS = 100;

        uint256 l1EpochBlockNumber = 1;
        uint256 l3BlockNumber = 1;
        bytes memory batch;

        for (uint256 i = 1; i <= MAX_ITERATIONS; i++) {
            uint256 batchSize = i * BATCH_SIZE_INCREMENT;
            batch = new bytes(batchSize);

            uint256 gasStart = gasleft();
            vm.prank(manager);
            try l3Storage.save(l1EpochBlockNumber, l3BlockNumber, batch) {
                uint256 gasUsed = gasStart - gasleft();
                console.log("Batch size:", batchSize, "Gas used:", gasUsed);

                if (gasUsed > MAX_GAS_LIMIT) {
                    console.log("Gas limit reached. Max batch size:", (i - 1) * BATCH_SIZE_INCREMENT);
                    break;
                }
            } catch {
                console.log("Transaction reverted. Max batch size:", (i - 1) * BATCH_SIZE_INCREMENT);
                break;
            }
        }
    }
}
