// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {SyndicateAccumulator} from "src/SyndicateAccumulator.sol";

/// @notice Mock ArbSys contract for testing
contract MockArbSys {
    uint256 public blockNumber;
    bool public shouldRevert;

    constructor(uint256 _blockNumber) {
        blockNumber = _blockNumber;
    }

    function arbBlockNumber() external view returns (uint256) {
        if (shouldRevert) {
            revert("Mock ArbSys error");
        }
        return blockNumber;
    }

    function setBlockNumber(uint256 _blockNumber) external {
        blockNumber = _blockNumber;
    }

    function setShouldRevert(bool _shouldRevert) external {
        shouldRevert = _shouldRevert;
    }
}

/// @notice Test implementation that inherits SyndicateAccumulator for testing internal functions
contract TestSyndicateAccumulator is SyndicateAccumulator {
    constructor() SyndicateAccumulator() {}

    /// @notice Public wrapper for internal _transactionProcessed function
    function processTransactionForTesting(bytes memory data) external {
        _transactionProcessed(data);
    }
}

/// @notice Test implementation for testing Arbitrum chain behavior
contract TestSyndicateAccumulatorArb is SyndicateAccumulator {
    constructor() SyndicateAccumulator() {}

    function processTransactionForTesting(bytes memory data) external {
        _transactionProcessed(data);
    }
}

contract SyndicateAccumulatorTest is Test {
    TestSyndicateAccumulator public accumulator;
    MockArbSys public mockArbSys;

    address public user1;
    address public user2;

    event TransactionProcessed(address indexed sender, bytes data);

    function setUp() public {
        user1 = address(0x1);
        user2 = address(0x2);

        // Deploy on non-Arbitrum chain (default)
        accumulator = new TestSyndicateAccumulator();
    }

    // Constructor Tests
    function testConstructorNonArbChain() public {
        TestSyndicateAccumulator testAccumulator = new TestSyndicateAccumulator();
        assertFalse(testAccumulator.isArbChain());
        assertEq(testAccumulator.accumulator(), bytes32(0));
    }

    function testConstructorArbChainSuccess() public {
        // Deploy mock ArbSys with non-zero block number
        MockArbSys mockArb = new MockArbSys(100);

        // Use vm.etch to place the mock at the ArbSys precompile address
        vm.etch(address(0x0000000000000000000000000000000000000064), address(mockArb).code);

        // Set the storage slot for blockNumber to ensure it's not zero
        vm.store(
            address(0x0000000000000000000000000000000000000064),
            bytes32(uint256(0)), // slot 0 is blockNumber
            bytes32(uint256(100))
        );

        // Deploy accumulator - should detect Arbitrum chain
        TestSyndicateAccumulatorArb arbAccumulator = new TestSyndicateAccumulatorArb();
        assertTrue(arbAccumulator.isArbChain());
    }

    function testConstructorArbChainZeroBlockNumberReverts() public {
        // Deploy mock ArbSys with block number 0
        MockArbSys mockArb = new MockArbSys(0);

        // Use vm.etch to place the mock at the ArbSys precompile address
        vm.etch(address(0x0000000000000000000000000000000000000064), address(mockArb).code);

        // Should revert because arbBlockNumber returns 0
        vm.expectRevert("ArbSys precompile validation failed");
        new TestSyndicateAccumulatorArb();
    }

    // Accumulator Storage Tests
    function testAccumulatorStorageLocation() public view {
        // Verify the storage location constant
        assertEq(accumulator.ACCUMULATOR_STORAGE_LOCATION(), keccak256("syndicate.accumulator"));
    }

    function testInitialAccumulatorValue() public view {
        // Initial accumulator should be zero
        assertEq(accumulator.accumulator(), bytes32(0));
    }

    // Transaction Processing Tests
    function testProcessTransactionEmitsEvent() public {
        bytes memory testData = "test transaction data";

        vm.startPrank(user1);
        vm.expectEmit(true, false, false, true);
        emit TransactionProcessed(user1, testData);
        accumulator.processTransactionForTesting(testData);
        vm.stopPrank();
    }

    function testProcessTransactionUpdatesAccumulator() public {
        bytes memory testData = "test transaction data";
        bytes32 initialAccumulator = accumulator.accumulator();

        vm.startPrank(user1);
        accumulator.processTransactionForTesting(testData);
        vm.stopPrank();

        bytes32 newAccumulator = accumulator.accumulator();
        assertTrue(newAccumulator != initialAccumulator);
        assertTrue(newAccumulator != bytes32(0));
    }

    function testProcessTransactionDeterministicAccumulator() public {
        bytes memory testData = "test transaction data";

        // Process same transaction from same user at same block
        vm.startPrank(user1);
        vm.roll(100);
        vm.warp(1000);
        accumulator.processTransactionForTesting(testData);
        bytes32 firstResult = accumulator.accumulator();
        vm.stopPrank();

        // Deploy new accumulator and process same transaction with same conditions
        TestSyndicateAccumulator accumulator2 = new TestSyndicateAccumulator();
        vm.startPrank(user1);
        vm.roll(100);
        vm.warp(1000);
        accumulator2.processTransactionForTesting(testData);
        bytes32 secondResult = accumulator2.accumulator();
        vm.stopPrank();

        assertEq(firstResult, secondResult);
    }

    function testProcessTransactionDifferentSendersDifferentHashes() public {
        bytes memory testData = "test transaction data";

        // Process from user1
        vm.startPrank(user1);
        accumulator.processTransactionForTesting(testData);
        bytes32 result1 = accumulator.accumulator();
        vm.stopPrank();

        // Deploy new accumulator and process from user2
        TestSyndicateAccumulator accumulator2 = new TestSyndicateAccumulator();
        vm.startPrank(user2);
        accumulator2.processTransactionForTesting(testData);
        bytes32 result2 = accumulator2.accumulator();
        vm.stopPrank();

        assertTrue(result1 != result2);
    }

    function testProcessTransactionDifferentDataDifferentHashes() public {
        bytes memory data1 = "test data 1";
        bytes memory data2 = "test data 2";

        // Process first data
        vm.startPrank(user1);
        accumulator.processTransactionForTesting(data1);
        bytes32 result1 = accumulator.accumulator();
        vm.stopPrank();

        // Deploy new accumulator and process different data
        TestSyndicateAccumulator accumulator2 = new TestSyndicateAccumulator();
        vm.startPrank(user1);
        accumulator2.processTransactionForTesting(data2);
        bytes32 result2 = accumulator2.accumulator();
        vm.stopPrank();

        assertTrue(result1 != result2);
    }

    function testProcessTransactionDifferentBlocksDifferentHashes() public {
        bytes memory testData = "test transaction data";

        // Process at block 100
        vm.startPrank(user1);
        vm.roll(100);
        accumulator.processTransactionForTesting(testData);
        bytes32 result1 = accumulator.accumulator();
        vm.stopPrank();

        // Deploy new accumulator and process at block 200
        TestSyndicateAccumulator accumulator2 = new TestSyndicateAccumulator();
        vm.startPrank(user1);
        vm.roll(200);
        accumulator2.processTransactionForTesting(testData);
        bytes32 result2 = accumulator2.accumulator();
        vm.stopPrank();

        assertTrue(result1 != result2);
    }

    function testProcessTransactionDifferentTimestampsDifferentHashes() public {
        bytes memory testData = "test transaction data";

        // Process at timestamp 1000
        vm.startPrank(user1);
        vm.warp(1000);
        accumulator.processTransactionForTesting(testData);
        bytes32 result1 = accumulator.accumulator();
        vm.stopPrank();

        // Deploy new accumulator and process at timestamp 2000
        TestSyndicateAccumulator accumulator2 = new TestSyndicateAccumulator();
        vm.startPrank(user1);
        vm.warp(2000);
        accumulator2.processTransactionForTesting(testData);
        bytes32 result2 = accumulator2.accumulator();
        vm.stopPrank();

        assertTrue(result1 != result2);
    }

    function testProcessTransactionChaining() public {
        bytes memory data1 = "first transaction";
        bytes memory data2 = "second transaction";

        vm.startPrank(user1);

        // Process first transaction
        accumulator.processTransactionForTesting(data1);
        bytes32 firstAccumulator = accumulator.accumulator();

        // Process second transaction
        accumulator.processTransactionForTesting(data2);
        bytes32 secondAccumulator = accumulator.accumulator();

        vm.stopPrank();

        // Results should be different and second should depend on first
        assertTrue(firstAccumulator != secondAccumulator);
        assertTrue(firstAccumulator != bytes32(0));
        assertTrue(secondAccumulator != bytes32(0));
    }

    function testProcessTransactionEmptyData() public {
        bytes memory emptyData = "";

        vm.startPrank(user1);
        vm.expectEmit(true, false, false, true);
        emit TransactionProcessed(user1, emptyData);
        accumulator.processTransactionForTesting(emptyData);
        vm.stopPrank();

        // Should still update accumulator even with empty data
        assertTrue(accumulator.accumulator() != bytes32(0));
    }

    function testProcessTransactionLargeData() public {
        // Create large data (1KB)
        bytes memory largeData = new bytes(1024);
        for (uint256 i = 0; i < 1024; i++) {
            largeData[i] = bytes1(uint8(i % 256));
        }

        vm.startPrank(user1);
        vm.expectEmit(true, false, false, true);
        emit TransactionProcessed(user1, largeData);
        accumulator.processTransactionForTesting(largeData);
        vm.stopPrank();

        assertTrue(accumulator.accumulator() != bytes32(0));
    }

    // Arbitrum Chain Tests
    function testProcessTransactionOnArbChain() public {
        // Deploy mock ArbSys with specific block number
        MockArbSys mockArb = new MockArbSys(12345);
        vm.etch(address(0x0000000000000000000000000000000000000064), address(mockArb).code);

        // Set the storage slot for blockNumber
        vm.store(
            address(0x0000000000000000000000000000000000000064),
            bytes32(uint256(0)), // slot 0 is blockNumber
            bytes32(uint256(12345))
        );

        // Deploy accumulator on "Arbitrum"
        TestSyndicateAccumulatorArb arbAccumulator = new TestSyndicateAccumulatorArb();
        assertTrue(arbAccumulator.isArbChain());

        bytes memory testData = "arbitrum test data";

        vm.startPrank(user1);
        vm.expectEmit(true, false, false, true);
        emit TransactionProcessed(user1, testData);
        arbAccumulator.processTransactionForTesting(testData);
        vm.stopPrank();

        assertTrue(arbAccumulator.accumulator() != bytes32(0));
    }

    function testArbChainVsRegularChainDifferentHashes() public {
        bytes memory testData = "test data";

        // Process on regular chain
        vm.startPrank(user1);
        vm.roll(100);
        vm.warp(1000);
        accumulator.processTransactionForTesting(testData);
        bytes32 regularResult = accumulator.accumulator();
        vm.stopPrank();

        // Setup mock ArbSys with different block number
        MockArbSys mockArb = new MockArbSys(999);
        vm.etch(address(0x0000000000000000000000000000000000000064), address(mockArb).code);

        // Set the storage slot for blockNumber
        vm.store(
            address(0x0000000000000000000000000000000000000064),
            bytes32(uint256(0)), // slot 0 is blockNumber
            bytes32(uint256(999))
        );

        // Process on "Arbitrum" chain
        TestSyndicateAccumulatorArb arbAccumulator = new TestSyndicateAccumulatorArb();
        vm.startPrank(user1);
        vm.roll(100); // Same regular block number
        vm.warp(1000); // Same timestamp
        arbAccumulator.processTransactionForTesting(testData);
        bytes32 arbResult = arbAccumulator.accumulator();
        vm.stopPrank();

        // Results should be different due to different block numbers used
        assertTrue(regularResult != arbResult);
    }

    // Edge Cases and Fuzz Tests
    function testFuzzProcessTransaction(address sender, bytes calldata data, uint256 blockNum, uint256 timestamp)
        public
    {
        vm.assume(sender != address(0));
        vm.assume(blockNum > 0 && blockNum < type(uint64).max);
        vm.assume(timestamp > 0 && timestamp < type(uint64).max);

        TestSyndicateAccumulator fuzzAccumulator = new TestSyndicateAccumulator();

        vm.startPrank(sender);
        vm.roll(blockNum);
        vm.warp(timestamp);

        vm.expectEmit(true, false, false, true);
        emit TransactionProcessed(sender, data);
        fuzzAccumulator.processTransactionForTesting(data);

        vm.stopPrank();

        // Accumulator should always be updated
        assertTrue(fuzzAccumulator.accumulator() != bytes32(0));
    }

    function testProcessTransactionMaxValues() public {
        bytes memory testData = "max values test";

        vm.startPrank(user1);
        vm.roll(type(uint64).max);
        vm.warp(type(uint64).max);

        accumulator.processTransactionForTesting(testData);

        vm.stopPrank();

        assertTrue(accumulator.accumulator() != bytes32(0));
    }

    function testMultipleTransactionsFromDifferentSenders() public {
        bytes memory data1 = "data from user1";
        bytes memory data2 = "data from user2";

        // Process from user1
        vm.startPrank(user1);
        accumulator.processTransactionForTesting(data1);
        bytes32 afterFirst = accumulator.accumulator();
        vm.stopPrank();

        // Process from user2
        vm.startPrank(user2);
        accumulator.processTransactionForTesting(data2);
        bytes32 afterSecond = accumulator.accumulator();
        vm.stopPrank();

        // All states should be different
        assertTrue(afterFirst != bytes32(0));
        assertTrue(afterSecond != bytes32(0));
        assertTrue(afterFirst != afterSecond);
    }

    // Gas consumption tests
    function testProcessTransactionGasConsumption() public {
        bytes memory testData = "gas test data";

        vm.startPrank(user1);
        uint256 gasBefore = gasleft();
        accumulator.processTransactionForTesting(testData);
        uint256 gasAfter = gasleft();
        vm.stopPrank();

        uint256 gasUsed = gasBefore - gasAfter;

        // Should use reasonable amount of gas (less than 100k for simple operation)
        assertTrue(gasUsed < 100_000);
        assertTrue(gasUsed > 0);
    }
}
