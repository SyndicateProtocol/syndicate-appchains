// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import {L3BackfillMapper} from "../../src/backfill/L3BackfillMapper.sol";

contract L3BackfillMapperTest is Test {
    address public manager;
    L3BackfillMapper public mapper;

    function setUp() public {
        manager = address(0x1);
        mapper = new L3BackfillMapper(manager, manager);
    }

    function testOnlyManagerCanIndex() public {
        vm.expectRevert();
        mapper.index(1, "0x1234");
    }

    function testOnlyManagerCanIndexForMany() public {
        uint256[] memory l1EpochBlockNumbers = new uint256[](1);
        l1EpochBlockNumbers[0] = 1;
        bytes32[] memory txHashes = new bytes32[](1);
        txHashes[0] = bytes32("0x1234");

        vm.expectRevert();
        mapper.indexForMany(l1EpochBlockNumbers, txHashes);
    }

    function testIndexForManyRevertsIfArraysAreDifferentLengths() public {
        uint256[] memory l1EpochBlockNumbers = new uint256[](1);
        l1EpochBlockNumbers[0] = 1;
        bytes32[] memory txHashes = new bytes32[](2);
        txHashes[0] = bytes32("0x1234");
        txHashes[1] = bytes32("0x5678");

        vm.expectRevert();
        mapper.indexForMany(l1EpochBlockNumbers, txHashes);
    }

    function testIndex() public {
        vm.prank(manager);
        mapper.index(1, "0x1234");

        assertEq(mapper.epochBlockNumberToBatchTxHashes(1, 0), bytes32("0x1234"));
    }

    function testIndexForSameEpochBlockNumber() public {
        vm.startPrank(manager);
        mapper.index(1, "0x1234");
        mapper.index(1, "0x5678");
        vm.stopPrank();

        assertEq(mapper.epochBlockNumberToBatchTxHashes(1, 0), bytes32("0x1234"));
        assertEq(mapper.epochBlockNumberToBatchTxHashes(1, 1), bytes32("0x5678"));
    }

    function testIndexForMany() public {
        uint256[] memory l1EpochBlockNumbers = new uint256[](2);
        l1EpochBlockNumbers[0] = 1;
        l1EpochBlockNumbers[1] = 2;
        bytes32[] memory txHashes = new bytes32[](2);
        txHashes[0] = bytes32("0x1234");
        txHashes[1] = bytes32("0x5678");

        vm.startPrank(manager);
        mapper.indexForMany(l1EpochBlockNumbers, txHashes);
        vm.stopPrank();

        assertEq(mapper.epochBlockNumberToBatchTxHashes(1, 0), bytes32("0x1234"));
        assertEq(mapper.epochBlockNumberToBatchTxHashes(2, 0), bytes32("0x5678"));
    }
}
