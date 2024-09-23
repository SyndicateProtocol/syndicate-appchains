// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test} from "forge-std/Test.sol";
import {L3BackfillData} from "src/L3BackfillData.sol";

import {IAccessControl} from "openzeppelin-contracts/contracts/access/IAccessControl.sol";

contract L3BackfillDataTest is Test {
    L3BackfillData public backfill;
    address admin = address(0x1);
    address manager = address(0x2);
    address nonManager = address(0x3);

    function setUp() public {
        vm.startPrank(admin);
        backfill = new L3BackfillData(admin, manager);
        vm.stopPrank();
    }

    function testSubmitBlock() public {
        bytes32[8] memory bloom;
        for (uint256 i = 0; i < 8; i++) {
            bloom[i] = bytes32(uint256(i + 1));
        }

        L3BackfillData.BlockHeader memory blockHeader = L3BackfillData.BlockHeader({
            parentHash: bytes32(uint256(1)),
            uncleHash: bytes32(uint256(2)),
            coinbase: address(3),
            root: bytes32(uint256(4)),
            txHash: bytes32(uint256(5)),
            receiptHash: bytes32(uint256(6)),
            bloom: bloom,
            difficulty: 8,
            number: 9,
            gasLimit: 10,
            gasUsed: 11,
            time: 12,
            extra: bytes32(uint256(13)),
            mixDigest: bytes32(uint256(14)),
            nonce: 15,
            baseFee: 16,
            withdrawalsHash: bytes32(uint256(17)),
            blobGasUsed: 18,
            excessBlobGas: 19,
            parentBeaconRoot: bytes32(uint256(20))
        });
        bytes32 blockHash = keccak256(abi.encode(blockHeader));

        vm.expectEmit(true, true, true, true);
        emit L3BackfillData.Block(blockHeader.number, blockHeader, blockHash);

        vm.startPrank(manager);
        backfill.submitBlock(blockHeader, blockHash);
        vm.stopPrank();
    }

    function testSubmitTransactions() public {
        uint256 blockNumber = 1;
        bytes memory txData = abi.encode("compressed transaction data");
        uint256 segmentIndex = 0;
        uint256 segmentTotal = 1;

        vm.expectEmit(true, true, true, true);
        emit L3BackfillData.Transactions(blockNumber, txData, segmentIndex, segmentTotal);

        vm.startPrank(manager);
        backfill.submitTransactions(blockNumber, txData, segmentIndex, segmentTotal);
        vm.stopPrank();
    }

    function testSubmitMultipleTransactionSegments() public {
        uint256 blockNumber = 1;
        bytes memory txData1 = abi.encode("compressed transaction data part 1");
        bytes memory txData2 = abi.encode("compressed transaction data part 2");
        uint256 segmentTotal = 2;

        vm.startPrank(manager);

        vm.expectEmit(true, true, true, true);
        emit L3BackfillData.Transactions(blockNumber, txData1, 0, segmentTotal);
        backfill.submitTransactions(blockNumber, txData1, 0, segmentTotal);

        vm.expectEmit(true, true, true, true);
        emit L3BackfillData.Transactions(blockNumber, txData2, 1, segmentTotal);
        backfill.submitTransactions(blockNumber, txData2, 1, segmentTotal);

        vm.stopPrank();
    }

    function testOnlyManagerCanSubmitBlock() public {
        L3BackfillData.BlockHeader memory blockHeader;
        bytes32 blockHash;

        bytes memory errorMessage = abi.encodeWithSelector(
            IAccessControl.AccessControlUnauthorizedAccount.selector, nonManager, backfill.MANAGER_ROLE()
        );

        vm.expectRevert(errorMessage);
        vm.startPrank(nonManager);
        backfill.submitBlock(blockHeader, blockHash);
        vm.stopPrank();
    }

    function testOnlyManagerCanSubmitTransactions() public {
        bytes memory errorMessage = abi.encodeWithSelector(
            IAccessControl.AccessControlUnauthorizedAccount.selector, nonManager, backfill.MANAGER_ROLE()
        );

        vm.expectRevert(errorMessage);
        vm.startPrank(nonManager);
        backfill.submitTransactions(1, "", 0, 1);
        vm.stopPrank();
    }

    function testAdminCanGrantManagerRole() public {
        address newManager = address(4);

        vm.startPrank(admin);
        backfill.grantRole(backfill.MANAGER_ROLE(), newManager);
        vm.stopPrank();

        assertTrue(backfill.hasRole(backfill.MANAGER_ROLE(), newManager));
    }

    function testManagerCannotGrantManagerRole() public {
        address newManager = address(4);

        bytes memory errorMessage = abi.encodeWithSelector(
            IAccessControl.AccessControlUnauthorizedAccount.selector, manager, backfill.DEFAULT_ADMIN_ROLE()
        );

        bytes32 managerRole = backfill.MANAGER_ROLE();

        vm.expectRevert(errorMessage);
        vm.startPrank(manager);
        backfill.grantRole(managerRole, newManager);
        vm.stopPrank();
    }
}
