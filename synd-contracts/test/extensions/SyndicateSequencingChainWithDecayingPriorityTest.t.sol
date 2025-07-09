// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateSequencingChainWithDecayingPriority} from
    "src/extensions/SyndicateSequencingChainWithDecayingPriority.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {RequireOrModule} from "src/requirement-modules/RequireOrModule.sol";
import {IPermissionModule} from "src/interfaces/IPermissionModule.sol";

// Mock contract to test IsAllowed behavior
contract MockIsAllowed is IPermissionModule {
    bool private allowed;

    constructor(bool _allowed) {
        allowed = _allowed;
    }

    function isAllowed(address, address, bytes calldata) external view override returns (bool) {
        return allowed;
    }
}

// Base test contract that sets up the common infrastructure
contract SyndicateSequencingChainWithDecayingPriorityTestSetUp is Test {
    SyndicateSequencingChainWithDecayingPriority public chain;
    RequireAndModule public requireAndModule;
    RequireOrModule public requireOrModule;
    address public admin;

    function setUp() public virtual {
        admin = address(0x1);
        uint256 appchainId = 1234;

        vm.startPrank(admin);
        requireAndModule = new RequireAndModule(admin);
        requireOrModule = new RequireOrModule(admin);
        chain = new SyndicateSequencingChainWithDecayingPriority(appchainId);
        chain.initialize(admin, address(requireAndModule));
        vm.stopPrank();
    }
}

// Main test contract inheriting from the setup
contract SyndicateSequencingChainWithDecayingPriorityTest is SyndicateSequencingChainWithDecayingPriorityTestSetUp {
    // Test data
    bytes public testData1 = abi.encode("test transaction data 1");
    bytes public testData2 = abi.encode("test transaction data 2");
    bytes[] internal testDataArray;
    uint256[] internal testPriorityArray;

    // Events for testing
    event TransactionProcessed(address indexed sender, bytes data, uint256 originalPriority, uint256 timestamp);

    function setUp() public override {
        super.setUp();

        // Set up test data arrays
        testDataArray = new bytes[](3);
        testDataArray[0] = abi.encode("bulk transaction 1");
        testDataArray[1] = abi.encode("bulk transaction 2");
        testDataArray[2] = abi.encode("bulk transaction 3");

        testPriorityArray = new uint256[](3);
        testPriorityArray[0] = 100;
        testPriorityArray[1] = 200;
        testPriorityArray[2] = 300;
    }

    // CONSTRUCTOR TESTS

    function testConstructorSetsCorrectValues() public view {
        // Verify the constructor sets the correct values
        assertEq(chain.appchainId(), 1234);
        assertEq(chain.PRIORITY_DECAY_RATE(), 10);
    }

    function testConstructorRevertsWithZeroChainId() public {
        // Verify that constructor reverts with chain ID 0
        vm.expectRevert("App chain ID cannot be 0");
        new SyndicateSequencingChainWithDecayingPriority(0);
    }

    // PROCESS TRANSACTION TESTS

    function testProcessTransaction() public {
        // Set up permissions
        vm.startPrank(admin);
        requireAndModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Current block timestamp will be used in the emitted event
        uint256 currentTimestamp = block.timestamp;

        // Expect the TransactionProcessed event with correct parameters
        vm.expectEmit(true, true, false, true);
        emit TransactionProcessed(
            address(this),
            abi.encodePacked(bytes1(0x00), testData1), // Zero byte prepended
            150, // Priority
            currentTimestamp
        );

        // Process the transaction
        chain.processTransactionUncompressed(testData1, 150);
    }

    function testProcessRawTransaction() public {
        // Set up permissions
        vm.startPrank(admin);
        requireAndModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Current block timestamp will be used in the emitted event
        uint256 currentTimestamp = block.timestamp;

        // Expect the TransactionProcessed event with correct parameters
        vm.expectEmit(true, true, false, true);
        emit TransactionProcessed(
            address(this), // sender
            testData2, // data without prepending zero byte
            200, // priority
            currentTimestamp
        );

        // Process the raw transaction
        chain.processTransaction(testData2, 200);
    }

    // BULK TRANSACTION TESTS

    function testProcessTransactionsBulk() public {
        // Set up permissions
        vm.startPrank(admin);
        requireAndModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Current timestamp for event verification
        uint256 currentTimestamp = block.timestamp;

        // Expect events for each transaction in the bulk process
        for (uint256 i = 0; i < testDataArray.length; i++) {
            vm.expectEmit(true, true, false, true);
            emit TransactionProcessed(
                address(this), abi.encodePacked(bytes1(0x00), testDataArray[i]), testPriorityArray[i], currentTimestamp
            );
        }

        // Process all transactions in bulk
        chain.processTransactionsBulk(testDataArray, testPriorityArray);
    }

    function testProcessTransactionsBulkRevertsWithMismatchedArrays() public {
        // Set up permissions
        vm.startPrank(admin);
        requireAndModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // Create priority array with incorrect length
        uint256[] memory shortPriorityArray = new uint256[](2);
        shortPriorityArray[0] = 100;
        shortPriorityArray[1] = 200;

        // Expect revert due to array length mismatch
        vm.expectRevert("Data and priority arrays must have the same length");
        chain.processTransactionsBulk(testDataArray, shortPriorityArray);
    }

    // PRIORITY DECAY CALCULATION TESTS

    function testCalculateEffectivePriorityWithNoDecay() public view {
        // Test case: No time has passed
        uint256 originalPriority = 100;
        uint256 submittedTimestamp = 1000;
        uint256 currentTimestamp = 1000;

        uint256 effectivePriority =
            chain.calculateEffectivePriority(originalPriority, submittedTimestamp, currentTimestamp);

        assertEq(effectivePriority, 100, "Priority should not decay if no time has passed");
    }

    function testCalculateEffectivePriorityWithPartialDecay() public view {
        // Test case: 5 seconds have passed with decay rate of 10 per second
        uint256 originalPriority = 100;
        uint256 submittedTimestamp = 1000;
        uint256 currentTimestamp = 1005;

        uint256 effectivePriority =
            chain.calculateEffectivePriority(originalPriority, submittedTimestamp, currentTimestamp);

        // 5 seconds * 10 decay rate = 50 decay
        assertEq(effectivePriority, 50, "Priority should decay by 50 after 5 seconds");
    }

    function testCalculateEffectivePriorityWithFullDecay() public view {
        // Test case: Enough time has passed to fully decay priority
        uint256 originalPriority = 100;
        uint256 submittedTimestamp = 1000;
        uint256 currentTimestamp = 1012; // 12 seconds with decay rate of 10 (more than needed)

        uint256 effectivePriority =
            chain.calculateEffectivePriority(originalPriority, submittedTimestamp, currentTimestamp);

        assertEq(effectivePriority, 0, "Priority should be 0 if decay exceeds original priority");
    }

    function testCalculateEffectivePriorityWithFutureDated() public view {
        // Edge case: Current timestamp is before submission timestamp
        uint256 originalPriority = 100;
        uint256 submittedTimestamp = 1000;
        uint256 currentTimestamp = 990;

        uint256 effectivePriority =
            chain.calculateEffectivePriority(originalPriority, submittedTimestamp, currentTimestamp);

        assertEq(effectivePriority, 100, "Priority should not change if current time < submitted time");
    }

    // PERMISSION TESTS

    function testProcessTransactionRequireAllFailure() public {
        // Create mock that will deny access
        address mockRequireAllAddr = address(new MockIsAllowed(false));

        vm.startPrank(admin);
        requireAndModule.addPermissionCheck(mockRequireAllAddr, false);
        vm.stopPrank();

        // Expect specific error from RequireAndModule
        vm.expectRevert(
            abi.encodeWithSelector(
                RequireAndModule.AndPermissionCheckFailed.selector, mockRequireAllAddr, address(this), testData2
            )
        );

        // This should revert with CheckFailed
        chain.processTransaction(testData2, 1);
    }

    function testProcessTransactionRequireAnyFailure() public {
        vm.startPrank(admin);
        chain.updateRequirementModule(address(requireOrModule));
        requireOrModule.addPermissionCheck(address(new MockIsAllowed(false)), false);
        vm.stopPrank();

        // Expect specific RequireAny module error
        vm.expectRevert(
            abi.encodeWithSelector(RequireOrModule.AllOrPermissionChecksFailed.selector, address(this), testData2)
        );

        // This should revert with CheckFailed
        chain.processTransaction(testData2, 1);
    }

    function testRequireOrModuleWithOnePassingCheck() public {
        vm.startPrank(admin);
        chain.updateRequirementModule(address(requireOrModule));
        requireOrModule.addPermissionCheck(address(new MockIsAllowed(false)), false);
        requireOrModule.addPermissionCheck(address(new MockIsAllowed(true)), false);
        vm.stopPrank();

        // This should work as at least one check passes
        chain.processTransaction(testData2, 1);
    }
}

// Additional test contracts for view function testing
contract SyndicateSequencingChainWithDecayingPriorityViewRequireAllTest is
    SyndicateSequencingChainWithDecayingPriorityTestSetUp
{
    MockIsAllowed mockRequireAll1;
    MockIsAllowed mockRequireAll2;

    function setUp() public override {
        super.setUp();
        mockRequireAll1 = new MockIsAllowed(true);
        mockRequireAll2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        requireAndModule.addPermissionCheck(address(mockRequireAll1), false);
        requireAndModule.addPermissionCheck(address(mockRequireAll2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAll() public view {
        address[] memory allChecks = requireAndModule.getAllPermissionChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAll1));
        assertEq(allChecks[1], address(mockRequireAll2));
    }
}

contract SyndicateSequencingChainWithDecayingPriorityViewRequireAnyTest is
    SyndicateSequencingChainWithDecayingPriorityTestSetUp
{
    MockIsAllowed mockRequireAny1;
    MockIsAllowed mockRequireAny2;

    function setUp() public override {
        super.setUp();

        mockRequireAny1 = new MockIsAllowed(false);
        mockRequireAny2 = new MockIsAllowed(true);

        vm.startPrank(admin);
        chain.updateRequirementModule(address(requireOrModule));

        requireOrModule.addPermissionCheck(address(mockRequireAny1), false);
        requireOrModule.addPermissionCheck(address(mockRequireAny2), false);
        vm.stopPrank();
    }

    function testGetAllRequirementsRequireAny() public view {
        address[] memory allChecks = requireOrModule.getAllPermissionChecks();
        assertEq(allChecks.length, 2);
        assertEq(allChecks[0], address(mockRequireAny1));
        assertEq(allChecks[1], address(mockRequireAny2));
    }
}
