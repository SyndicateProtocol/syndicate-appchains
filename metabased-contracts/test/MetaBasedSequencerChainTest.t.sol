// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {RequireAllModule} from "src/requirement-modules/RequireAllModule.sol";
import {RequireAnyModule} from "src/requirement-modules/RequireAnyModule.sol";
import {PermissionModule} from "src/interfaces/PermissionModule.sol";
import {Test} from "forge-std/Test.sol";

contract MockIsAllowed is PermissionModule {
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
    RequireAllModule public permissionModule;
    RequireAnyModule public permissionModuleAny;
    address public admin;

    function setUp() public virtual {
        admin = address(0x1);
        uint256 l3ChainId = 10042001;

        vm.startPrank(admin);
        permissionModule = new RequireAllModule(admin);
        permissionModuleAny = new RequireAnyModule(admin);
        chain = new MetabasedSequencerChain(l3ChainId, admin, address(permissionModule));
        vm.stopPrank();
    }
}

contract MetabasedSequencerChainTest is MetabasedSequencerChainTestSetUp {
    function testProcessRawTransaction() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit MetabasedSequencerChain.TransactionProcessed(address(this), validTxn);

        chain.processTransactionRaw(validTxn);
    }

    function testProcessTransactionRequireAllFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");
        address mockRequireAll = address(new MockIsAllowed(false));

        vm.startPrank(admin);
        permissionModule.addCheck(mockRequireAll, false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAllModule.CheckFailed.selector, mockRequireAll, address(this)));
        chain.processTransactionRaw(validTxn);
    }

    function testProcessTransactionRequireAnyFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));
        permissionModuleAny.addCheck(address(new MockIsAllowed(false)), false);
        vm.stopPrank();

        vm.expectRevert(abi.encodeWithSelector(RequireAnyModule.CheckFailed.selector, address(this)));
        chain.processTransactionRaw(validTxn);
    }

    function testProcessTransaction() public {
        bytes memory _data = abi.encode("raw transaction");
        bytes memory expectedTx = abi.encodePacked(bytes1(0x00), _data);

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit MetabasedSequencerChain.TransactionProcessed(address(this), expectedTx);

        chain.processTransaction(_data);
    }

    function testProcessBulkTransactions() public {
        bytes[] memory validTxns = new bytes[](3);
        validTxns[0] = abi.encode("transaction 1");
        validTxns[1] = abi.encode("transaction 2");
        validTxns[2] = abi.encode("transaction 3");

        vm.startPrank(admin);
        permissionModule.addCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        for (uint256 i = 0; i < validTxns.length; i++) {
            vm.expectEmit(true, false, false, true);

            emit MetabasedSequencerChain.TransactionProcessed(
                address(this), abi.encodePacked(bytes1(0x00), validTxns[i])
            );
        }

        chain.processBulkTransactions(validTxns);
    }
}

contract MetabasedSequencerChainViewRequireAllTest is MetabasedSequencerChainTestSetUp {
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        permissionModule.addCheck(address(mockRequireAll1), false);
        permissionModule.addCheck(address(mockRequireAll2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = permissionModule.getAllChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }
}

contract MetabasedSequencerChainViewRequireAnyTest is MetabasedSequencerChainTestSetUp {
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();

        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));

        permissionModuleAny.addCheck(address(mockRequireAny1), false);
        permissionModuleAny.addCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = permissionModuleAny.getAllChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }
}
