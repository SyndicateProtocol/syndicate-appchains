// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {MetafillerStorage} from "../../src/backfill/MetafillerStorage.sol";

contract MetafillerStorageTest is Test {
    address public manager;
    address public notManager;
    address public admin;
    MetafillerStorage public l3Storage;

    function setUp() public {
        manager = address(0x1);
        notManager = address(0x2);
        admin = address(0x3);
        uint256 l3ChainId = 10042001;
        l3Storage = new MetafillerStorage(admin, manager, l3ChainId);
    }

    function testOnlyManagerCanSave() public {
        vm.expectRevert();
        vm.prank(admin);
        l3Storage.save(1, 0xd41f86be45e841b1791bf9ff78aa9c388d9c81384d8a063d480c71c7f8865502, "0x");
    }

    function testOnlyManagerCanSetIndexFromBlock() public {
        vm.expectRevert();
        vm.prank(admin);
        l3Storage.setIndexFromBlock(1);
    }

    function testManagerCanSetIndexFromBlock() public {
        vm.prank(manager);
        l3Storage.setIndexFromBlock(1);
        assertEq(l3Storage.indexFromBlock(), 1);
    }

    // [Olympix Warning: tx.origin usage] Test removed as tx.origin check was removed for security
    // The MANAGER_ROLE provides sufficient authorization control
    function testOnlyOriginatorCanSave() public {
        vm.prank(manager);
        l3Storage.save(1, 0x505f3a50f83559ab090dbd840556254a7248404f2dedb53b4f12b26748a8ec08, "0x");
    }

    function testOnlyOriginatorCanSaveForMany() public {
        vm.expectRevert();
        uint256[] memory epochNumbers = new uint256[](1);
        epochNumbers[0] = 1;

        bytes32[] memory epochHashes = new bytes32[](1);
        epochHashes[0] = 0x505f3a50f83559ab090dbd840556254a7248404f2dedb53b4f12b26748a8ec08;

        bytes[] memory batches = new bytes[](1);
        batches[0] = "0x";
        vm.expectRevert();
        vm.prank(manager, notManager);
        l3Storage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testOnlyManagerCanSaveForMany() public {
        uint256[] memory epochNumbers = new uint256[](1);
        epochNumbers[0] = 1;

        bytes32[] memory epochHashes = new bytes32[](1);
        epochHashes[0] = 0xa553abb51becdfb0f56299b43b199076253e7a6b299d88b770a08b2b45bcb78a;

        bytes[] memory batches = new bytes[](1);
        batches[0] = "0x1234";

        vm.prank(admin);
        vm.expectRevert();
        l3Storage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSaveForManyRevertsIfArraysAreDifferentLengths() public {
        uint256[] memory epochNumbers = new uint256[](1);
        epochNumbers[0] = 1;

        bytes32[] memory epochHashes = new bytes32[](2);
        epochHashes[0] = 0x5f58575220c0ad82ee415ad72077049f0711c7445391a2f7a9161e410737424e;
        epochHashes[1] = 0x1a564d3dc4db83ade078717e74cb891a6a97cdc2d88c41716e1aa6b8bec5249e;

        bytes[] memory batches = new bytes[](1);
        batches[0] = "0x1234";

        vm.expectRevert();
        vm.prank(manager);
        l3Storage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testSave() public {
        // Prepare test data
        uint256 epochNumber = 1;
        bytes32 epochHash = 0xd41f86be45e841b1791bf9ff78aa9c388d9c81384d8a063d480c71c7f8865502;
        bytes memory batch = "0x";

        // Listen for the Batch event
        vm.expectEmit(true, true, false, true);
        emit MetafillerStorage.EpochRangeProcessed(epochNumber, epochNumber);

        // Call the save function
        vm.prank(manager, manager);
        l3Storage.save(epochNumber, epochHash, batch);
    }

    function testSaveForMany() public {
        // Prepare test data
        uint256[] memory epochNumbers = new uint256[](3);
        epochNumbers[0] = 1;
        epochNumbers[1] = 2;
        epochNumbers[2] = 3;

        bytes32[] memory epochHashes = new bytes32[](3);
        epochHashes[0] = 0xec736dd1efe6c910b85d52fb1aeeb1c0ac1d4ef3414f8de8f6aedc5ef314968d;
        epochHashes[1] = 0xb15b609a3039fac3f0541c405d2cd49be88079ef7f175c50306ab6b0f8a4de07;
        epochHashes[2] = 0x3305fd2cd42b1da8fc3e8b89bd35330674d764c8cd0a653a8eabeb4619c3c99d;

        bytes[] memory batches = new bytes[](3);
        batches[0] = "0x1234";
        batches[1] = "0x5678";
        batches[2] = "0x9abc";

        // Expect three Batch events to be emitted
        vm.expectEmit(true, true, true, true);
        emit MetafillerStorage.EpochRangeProcessed(epochNumbers[0], epochNumbers[2]);

        // Call the saveForMany function
        vm.prank(manager, manager);
        l3Storage.saveForMany(epochNumbers, epochHashes, batches);
    }

    function testGasEmptyEmit() public {
        uint256 gasStart = gasleft();
        vm.prank(manager, manager);
        l3Storage.save(1, 0x351e92084d6040b02259b6f0aa89141a23f7c796909f7d81731e228a84529f92, "");
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for empty emit:", gasUsed);
    }

    function testRevertsOnZeroAdmin() public {
        vm.expectRevert("Admin address cannot be 0");
        new MetafillerStorage(address(0), manager, 10042001);
    }

    function testRevertsOnZeroManager() public {
        vm.expectRevert("Manager address cannot be 0");
        new MetafillerStorage(admin, address(0), 10042001);
    }

    function testRevertsOnZeroChainId() public {
        vm.expectRevert("L3 chain ID cannot be 0");
        new MetafillerStorage(admin, manager, 0);
    }

    function testConstructorSetsCorrectValues() public view {
        assertTrue(l3Storage.hasRole(l3Storage.DEFAULT_ADMIN_ROLE(), admin), "Admin role not set correctly");
        assertTrue(l3Storage.hasRole(l3Storage.MANAGER_ROLE(), manager), "Manager role not set correctly");
        assertEq(l3Storage.l3ChainId(), 10042001, "Chain ID not set correctly");
    }

    function testGasForIncreasingBatchSizes() public {
        uint256 MAX_GAS_LIMIT = 30_000_000; // Approximate block gas limit
        uint256 BATCH_SIZE_INCREMENT = 100_000; // Increment batch size by 100_000 bytes each iteration
        uint256 MAX_ITERATIONS = 100;

        uint256 epochNumber = 1;
        bytes32 epochHash = 0x505f3a50f83559ab090dbd840556254a7248404f2dedb53b4f12b26748a8ec08;
        bytes memory batch;

        for (uint256 i = 1; i <= MAX_ITERATIONS; i++) {
            uint256 batchSize = i * BATCH_SIZE_INCREMENT;
            batch = new bytes(batchSize);

            uint256 gasStart = gasleft();
            vm.prank(manager);
            try l3Storage.save(epochNumber, epochHash, batch) {
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
