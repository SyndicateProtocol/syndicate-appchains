// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain, RequireListManager} from "src/MetabasedSequencerChain.sol";
import {IsAllowed} from "src/interfaces/IsAllowed.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowed is IsAllowed {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address) external view override returns (bool) {
        return allowed;
    }
}

contract MetabasedSequencerChainTestSetUp is Test {
    MetabasedSequencerChain public chain;
    address public admin;

    function setUp() public virtual {
        admin = address(0x1);
        uint256 l3ChainId = 10042001;

        vm.startPrank(admin);
        chain = new MetabasedSequencerChain(l3ChainId, admin);

        assertEq(chain.l3ChainId(), l3ChainId, "L3 chain ID should be set correctly");
        vm.stopPrank();
    }
}

contract MetabasedSequencerChainTest is MetabasedSequencerChainTestSetUp {
    function testProcessTransaction() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit MetabasedSequencerChain.TransactionProcessed(address(this), validTxn);

        chain.processTransaction(validTxn);
    }

    function testProcessRawTransaction() public {
        bytes memory rawTx = abi.encode("raw transaction");
        bytes memory expectedTx = abi.encodePacked(bytes1(0x00), rawTx);

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit MetabasedSequencerChain.TransactionProcessed(address(this), expectedTx);

        chain.processRawTransaction(rawTx);
    }

    function testProcessTransactionRequireAllFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");
        address mockRequireAll = address(new MockIsAllowed(false));

        vm.startPrank(admin);
        chain.addRequireAllCheck(mockRequireAll, false);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                RequireListManager.RequireAllCheckFailed.selector, address(mockRequireAll), address(this)
            )
        );
        chain.processTransaction(validTxn);
    }

    function testProcessTransactionRequireAnyFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(false)), false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireListManager.RequireAnyCheckFailed.selector, address(this)));
        chain.processTransaction(validTxn);
    }

    function testProcessBulkRawTransactions() public {
        bytes[] memory validTxns = new bytes[](3);
        validTxns[0] = abi.encode("transaction 1");
        validTxns[1] = abi.encode("transaction 2");
        validTxns[2] = abi.encode("transaction 3");

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        for (uint256 i = 0; i < validTxns.length; i++) {
            vm.expectEmit(true, false, false, true);
            emit MetabasedSequencerChain.TransactionProcessed(
                address(this), abi.encodePacked(bytes1(0x00), validTxns[i])
            );
        }

        chain.processBulkRawTransactions(validTxns);
    }

    function testProcessBulkTransactions() public {
        bytes[] memory validTxns = new bytes[](3);
        validTxns[0] = abi.encode("transaction 1");
        validTxns[1] = abi.encode("transaction 2");
        validTxns[2] = abi.encode("transaction 3");

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        for (uint256 i = 0; i < validTxns.length; i++) {
            vm.expectEmit(true, false, false, true);
            emit MetabasedSequencerChain.TransactionProcessed(address(this), validTxns[i]);
        }

        chain.processBulkTransactions(validTxns);
    }

    function testProcessChunk() public {
        bytes[] memory validTxns = new bytes[](5);
        for (uint256 i = 0; i < 5; i++) {
            validTxns[i] = abi.encode(string(abi.encodePacked("transaction ", vm.toString(i + 1))));
        }

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        for (uint256 i = 0; i < 3; i++) {
            vm.expectEmit(true, false, false, true);
            emit MetabasedSequencerChain.TransactionProcessed(address(this), validTxns[i]);
        }

        vm.expectEmit(true, true, false, true);
        emit MetabasedSequencerChain.ChunkProcessed(1, 3);

        chain.processChunk(validTxns, 0, 3, 1);
    }

    function testProcessChunkInvalidSize() public {
        bytes[] memory validTxns = new bytes[](5);
        for (uint256 i = 0; i < 5; i++) {
            validTxns[i] = abi.encode(string(abi.encodePacked("transaction ", vm.toString(i + 1))));
        }

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(MetabasedSequencerChain.InvalidChunkSize.selector));
        chain.processChunk(validTxns, 0, 0, 1);
    }

    function testProcessChunkExceedsBatchSize() public {
        bytes[] memory validTxns = new bytes[](5);
        for (uint256 i = 0; i < 5; i++) {
            validTxns[i] = abi.encode(string(abi.encodePacked("transaction ", vm.toString(i + 1))));
        }

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectRevert("Chunk exceeds batch size");
        chain.processChunk(validTxns, 3, 3, 1);
    }
}

contract MetabasedSequencerChainViewTest is MetabasedSequencerChainTestSetUp {
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);
        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.addRequireAllCheck(address(mockRequireAll1), false);
        chain.addRequireAllCheck(address(mockRequireAll2), false);
        chain.addRequireAnyCheck(address(mockRequireAny1), false);
        chain.addRequireAnyCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = chain.getAllRequirements(true);
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = chain.getAllRequirements(false);
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }
}
