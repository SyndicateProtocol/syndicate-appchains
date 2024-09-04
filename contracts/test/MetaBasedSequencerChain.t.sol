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

        vm.startPrank(admin);
        chain = new MetabasedSequencerChain();
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

    function testProcessTransactionInvalidForm() public {
        bytes memory invalidTxn = new bytes(0);

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(MetabasedSequencerChain.InvalidTransactionForm.selector));
        chain.processTransaction(invalidTxn);
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

    function testProcessBulkTransactionsWithInvalidTxn() public {
        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("transaction 1");
        txns[1] = new bytes(0); // Invalid transaction
        txns[2] = abi.encode("transaction 3");

        vm.startPrank(admin);
        chain.addRequireAnyCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(MetabasedSequencerChain.InvalidTransactionForm.selector));
        chain.processBulkTransactions(txns);
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
