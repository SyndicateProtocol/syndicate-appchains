// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain, SequencingModuleChecker} from "src/SyndicateSequencingChain.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {
    SyndicateSequencingChain, L2MessageType_SignedTx, SequencingModuleChecker
} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";
import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

contract MockIsAllowed is IPermissionModule {
    bool allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address, address, bytes calldata) external view override returns (bool) {
        return allowed;
    }
}

contract MockIsAllowedWithInvalidData is IPermissionModule {
    function isAllowed(address, address, bytes calldata data) external pure override returns (bool) {
        return keccak256(data) != keccak256(abi.encode("invalid"));
    }
}

contract DirectMockModule is IPermissionModule {
    mapping(bytes => bool) public allowedData;

    function setAllowed(bytes memory data, bool allowed) external {
        allowedData[data] = allowed;
    }

    function isAllowed(address, address, bytes calldata data) external view override returns (bool) {
        return allowedData[data];
    }
}

contract SyndicateSequencingChainTestSetUp is Test {
    SyndicateSequencingChain public chain;
    SyndicateFactory public factory;
    RequireAndModule public permissionModule;
    RequireOrModule public permissionModuleAny;
    address public admin;

    function deployFromFactory(RequireAndModule _permissionModule) public returns (SyndicateSequencingChain) {
        uint256 appchainId = 10042001;
        vm.startPrank(admin);
        factory = new SyndicateFactory(admin);
        (address chainAddress,) = factory.createSyndicateSequencingChain(
            appchainId, admin, _permissionModule, keccak256(abi.encodePacked("test-salt"))
        );
        vm.stopPrank();
        return SyndicateSequencingChain(chainAddress);
    }

    function setUp() public virtual {
        // Warp to START_TIMESTAMP to avoid underflow in epoch calculations
        vm.warp(1754089200); // START_TIMESTAMP from EpochTracker.sol

        admin = address(0x1);
        permissionModule = new RequireAndModule(admin);
        permissionModuleAny = new RequireOrModule(admin);
        chain = deployFromFactory(permissionModule);
    }
}

contract SyndicateSequencingChainTest is SyndicateSequencingChainTestSetUp {
    function testProcessRawTransaction() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit SyndicateSequencingChain.TransactionProcessed(
            address(this), abi.encodePacked(L2MessageType_SignedTx, validTxn)
        );

        chain.processTransaction(validTxn);
    }

    function testProcessTransactionRequireAllFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");
        address mockRequireAll = address(new MockIsAllowed(false));

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(mockRequireAll, false);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(
                RequireAndModule.AndPermissionCheckFailed.selector, mockRequireAll, address(this), validTxn
            )
        );
        chain.processTransaction(validTxn);
    }

    function testProcessTransactionRequireAnyFailure() public {
        bytes memory validTxn = abi.encode("valid transaction");

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));
        permissionModuleAny.addPermissionCheck(address(new MockIsAllowed(false)), false);
        vm.stopPrank();

        vm.expectRevert(
            abi.encodeWithSelector(RequireOrModule.AllOrPermissionChecksFailed.selector, address(this), validTxn)
        );
        chain.processTransaction(validTxn);
    }

    function testProcessTransaction() public {
        bytes memory data = abi.encode("raw transaction");

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit SyndicateSequencingChain.TransactionProcessed(
            address(this), abi.encodePacked(L2MessageType_SignedTx, data)
        );

        chain.processTransaction(data);
    }

    function testProcessTransactionsBulk() public {
        bytes[] memory validTxns = new bytes[](3);
        validTxns[0] = abi.encode("transaction 1");
        validTxns[1] = abi.encode("transaction 2");
        validTxns[2] = abi.encode("transaction 3");

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        for (uint256 i = 0; i < validTxns.length; i++) {
            vm.expectEmit(true, false, false, true);

            emit SyndicateSequencingChain.TransactionProcessed(
                address(this), abi.encodePacked(L2MessageType_SignedTx, validTxns[i])
            );
        }

        chain.processTransactionsBulk(validTxns);
    }

    function testConstructorWithZeroAppChainId() public {
        vm.expectRevert("App chain ID cannot be 0");
        new SyndicateSequencingChain(0, false);
    }

    function testProcessTransactionsBulkAllAllowed() public {
        // Deploy a module we can directly control
        DirectMockModule directMock = new DirectMockModule();

        // Set up the chain with our custom module
        vm.startPrank(admin);
        chain.updateRequirementModule(address(directMock));
        vm.stopPrank();

        // Prepare test data
        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("transaction 1");
        txns[1] = abi.encode("transaction 2");
        txns[2] = abi.encode("transaction 3");

        // Configure mock to allow all transactions
        directMock.setAllowed(txns[0], true);
        directMock.setAllowed(txns[1], true);
        directMock.setAllowed(txns[2], true);

        // Expect events for all transactions
        for (uint256 i = 0; i < txns.length; i++) {
            vm.expectEmit(true, false, false, true);
            emit SyndicateSequencingChain.TransactionProcessed(
                address(this), abi.encodePacked(L2MessageType_SignedTx, txns[i])
            );
        }

        // Process all transactions
        chain.processTransactionsBulk(txns);
    }

    function testProcessTransactionsBulkBranchCoverage() public {
        // Deploy a module we can directly control
        DirectMockModule directMock = new DirectMockModule();

        // Set up the chain with our custom module
        vm.startPrank(admin);
        chain.updateRequirementModule(address(directMock));
        vm.stopPrank();

        // Part 1: Test the failure branch
        bytes[] memory failingTxns = new bytes[](2);
        failingTxns[0] = abi.encode("allowed tx");
        failingTxns[1] = abi.encode("disallowed tx");

        directMock.setAllowed(failingTxns[0], true);
        directMock.setAllowed(failingTxns[1], false);

        chain.processTransactionsBulk(failingTxns);

        // Part 2: Test the success branch
        bytes[] memory successTxns = new bytes[](2);
        successTxns[0] = abi.encode("allowed tx 1");
        successTxns[1] = abi.encode("allowed tx 2");

        directMock.setAllowed(successTxns[0], true);
        directMock.setAllowed(successTxns[1], true);

        // Expect events for successful transactions
        for (uint256 i = 0; i < successTxns.length; i++) {
            vm.expectEmit(true, false, false, true);
            emit SyndicateSequencingChain.TransactionProcessed(
                address(this), abi.encodePacked(L2MessageType_SignedTx, successTxns[i])
            );
        }

        chain.processTransactionsBulk(successTxns);
    }

    function testProcessTransactionsBulkOnlyEmitsValidTransactionsAsEvents() public {
        chain = deployFromFactory(RequireAndModule(address(new MockIsAllowedWithInvalidData())));

        bytes[] memory txns = new bytes[](3);
        txns[0] = abi.encode("valid");
        txns[1] = abi.encode("invalid");
        txns[2] = abi.encode("valid");

        vm.recordLogs();
        chain.processTransactionsBulk(txns);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 expectedSig = keccak256("TransactionProcessed(address,bytes)");

        uint256 validEventCount = 0;
        uint256 expectedValidEventCount = 2;

        for (uint256 i = 0; i < logs.length; i++) {
            Vm.Log memory log = logs[i];

            if (log.topics.length > 0 && log.topics[0] == expectedSig) {
                // Decode data to make sure it's not for the "invalid" txn
                (, bytes memory emittedData) = abi.decode(log.data, (address, bytes));

                if (keccak256(emittedData) == keccak256(abi.encodePacked(bytes1(0x00), abi.encode("invalid")))) {
                    fail();
                }

                validEventCount++;
            }
        }

        assertEq(validEventCount, expectedValidEventCount, "Wrong amount of valid transaction events emitted");
    }

    function testOnlyWhenAllowedModifierBranches() public {
        // Deploy a module we can directly control
        DirectMockModule directMock = new DirectMockModule();

        // Set up the chain with our custom module
        vm.startPrank(admin);
        chain.updateRequirementModule(address(directMock));
        vm.stopPrank();

        bytes memory allowedData = abi.encode("allowed data");
        bytes memory disallowedData = abi.encode("disallowed data");

        // Configure permissions
        directMock.setAllowed(allowedData, true);
        directMock.setAllowed(disallowedData, false);

        // Test 1: Failure path of onlyWhenAllowed (processTransaction)
        vm.expectRevert(SequencingModuleChecker.TransactionOrSenderNotAllowed.selector);
        chain.processTransaction(disallowedData);

        // Test 2: Success path of onlyWhenAllowed (processTransaction)
        vm.expectEmit(true, false, false, true);
        emit SyndicateSequencingChain.TransactionProcessed(
            address(this), abi.encodePacked(L2MessageType_SignedTx, allowedData)
        );
        chain.processTransaction(allowedData);
    }

    function testProcessTransactionsBulkWithEmptyArray() public {
        bytes[] memory emptyArray = new bytes[](0);

        // This should execute without errors or events
        chain.processTransactionsBulk(emptyArray);
    }
}

contract SyndicateSequencingChainViewRequireAllTest is SyndicateSequencingChainTestSetUp {
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        permissionModule.addPermissionCheck(address(mockRequireAll1), false);
        permissionModule.addPermissionCheck(address(mockRequireAll2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = permissionModule.getAllPermissionChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }
}

contract SyndicateSequencingChainViewRequireAnyTest is SyndicateSequencingChainTestSetUp {
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();

        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.updateRequirementModule(address(permissionModuleAny));

        permissionModuleAny.addPermissionCheck(address(mockRequireAny1), false);
        permissionModuleAny.addPermissionCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = permissionModuleAny.getAllPermissionChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }
}
