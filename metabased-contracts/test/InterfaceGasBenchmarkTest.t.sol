// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test, console2} from "forge-std/Test.sol";

// Option 1: Consolidated Interfaces
import {OptionOneMetabasedSequencerChain} from "src/OptionOneMetabasedSequencerChain.sol";
import {ConsolidatedAllowlistModule} from "src/sequencing-modules/ConsolidatedAllowlistModule.sol";

// Option 2: Split Interfaces
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {AllowlistSequencingModuleExtended} from "src/sequencing-modules/AllowlistSequencingModuleExtended.sol";

/**
 * @title InterfaceGasBenchmarkTest
 * @dev Comprehensive benchmark comparing gas efficiency between different interface architectures
 */
contract InterfaceGasBenchmarkTest is Test {
    // Test chains
    OptionOneMetabasedSequencerChain public option1Chain;
    MetabasedSequencerChain public option2Chain;

    // Modules
    ConsolidatedAllowlistModule public consolidatedModule;
    AllowlistSequencingModuleExtended public extendedModule;

    // Test addresses
    address public admin = address(0x1);
    address public sequencer = address(0x2);
    address public nonSequencer = address(0x3);
    uint256 public chainId = 1234;

    // Test data of different sizes
    bytes public tinyData = hex"1234"; // 2 bytes
    bytes public smallData = abi.encodePacked(bytes32(uint256(1))); // 32 bytes
    bytes public mediumData; // 512 bytes
    bytes public largeData; // 4 KB

    // Arrays for bulk testing
    bytes[] public tinyDataArray;
    bytes[] public smallDataArray;
    bytes[] public mediumDataArray;
    bytes[] public largeDataArray;

    // For storing and logging benchmark results
    struct BenchmarkResult {
        string operation;
        string dataSize;
        uint256 option1Gas;
        uint256 option2Gas;
        int256 difference;
    }

    // Event for logging results
    event GasBenchmarkResult(
        string operation, string dataSize, uint256 option1Gas, uint256 option2Gas, int256 difference, string winner
    );

    function setUp() public {
        // Initialize test data
        // Medium data (512 bytes)
        for (uint256 i = 0; i < 16; i++) {
            mediumData = bytes.concat(mediumData, bytes32(uint256(i + 1)));
        }

        // Large data (4 KB)
        for (uint256 i = 0; i < 128; i++) {
            largeData = bytes.concat(largeData, bytes32(uint256(i + 1)));
        }

        // Prepare arrays for bulk testing (3 items each)
        // Tiny
        tinyDataArray.push(hex"1234");
        tinyDataArray.push(hex"5678");
        tinyDataArray.push(hex"9abc");

        // Small
        smallDataArray.push(abi.encodePacked(bytes32(uint256(1))));
        smallDataArray.push(abi.encodePacked(bytes32(uint256(2))));
        smallDataArray.push(abi.encodePacked(bytes32(uint256(3))));

        // Medium
        bytes memory medData1 = "";
        bytes memory medData2 = "";
        bytes memory medData3 = "";
        for (uint256 i = 0; i < 16; i++) {
            medData1 = bytes.concat(medData1, bytes32(uint256(i + 1)));
            medData2 = bytes.concat(medData2, bytes32(uint256(i + 100)));
            medData3 = bytes.concat(medData3, bytes32(uint256(i + 200)));
        }
        mediumDataArray.push(medData1);
        mediumDataArray.push(medData2);
        mediumDataArray.push(medData3);

        // Large (just 2 items to avoid exceeding gas limits)
        bytes memory lgData1 = "";
        bytes memory lgData2 = "";
        for (uint256 i = 0; i < 64; i++) {
            lgData1 = bytes.concat(lgData1, bytes32(uint256(i + 1)));
            lgData2 = bytes.concat(lgData2, bytes32(uint256(i + 100)));
        }
        largeDataArray.push(lgData1);
        largeDataArray.push(lgData2);

        // Deploy contracts
        vm.startPrank(admin);

        // Set up Option 1 (Consolidated approach)
        option1Chain = new OptionOneMetabasedSequencerChain(chainId);
        consolidatedModule = new ConsolidatedAllowlistModule(admin);
        option1Chain.initialize(admin, address(consolidatedModule));
        consolidatedModule.addToAllowlist(sequencer);

        // Set up Option 2 (Separate modules approach)
        option2Chain = new MetabasedSequencerChain(chainId);
        extendedModule = new AllowlistSequencingModuleExtended(admin);
        option2Chain.initialize(admin, address(extendedModule));
        extendedModule.addToAllowlist(sequencer);

        vm.stopPrank();

        // Log data sizes
        console2.log("=== TEST DATA SIZES ===");
        console2.log("Tiny data size: %d bytes", tinyData.length);
        console2.log("Small data size: %d bytes", smallData.length);
        console2.log("Medium data size: %d bytes", mediumData.length);
        console2.log("Large data size: %d bytes", largeData.length);
        console2.log("");
    }

    // Helper to log benchmark results
    function logResult(string memory operation, string memory dataSize, uint256 option1Gas, uint256 option2Gas)
        internal
    {
        int256 difference = int256(option2Gas) - int256(option1Gas);
        string memory winner = difference > 0 ? "Option 1" : difference < 0 ? "Option 2" : "Tie";

        console2.log("--- %s (%s) ---", operation, dataSize);
        console2.log("Option 1 gas: %d", option1Gas);
        console2.log("Option 2 gas: %d", option2Gas);
        console2.log("Difference: %d", difference);
        console2.log("Winner: %s", winner);
        console2.log("");

        emit GasBenchmarkResult(operation, dataSize, option1Gas, option2Gas, difference, winner);
    }

    //////////////////////////////////////////////////////////////
    // INDIVIDUAL BENCHMARKS FOR EACH OPERATION AND DATA SIZE
    //////////////////////////////////////////////////////////////

    function testBenchmarkIndividualTiny() public {
        // Option 1: Process transaction - Tiny data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransaction(tinyData);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process transaction - Tiny data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransaction(tinyData);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processTransaction", "Tiny", option1Gas, option2Gas);
    }

    function testBenchmarkIndividualSmall() public {
        // Option 1: Process transaction - Small data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransaction(smallData);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process transaction - Small data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransaction(smallData);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processTransaction", "Small", option1Gas, option2Gas);
    }

    function testBenchmarkIndividualMedium() public {
        // Option 1: Process transaction - Medium data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransaction(mediumData);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process transaction - Medium data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransaction(mediumData);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processTransaction", "Medium", option1Gas, option2Gas);
    }

    function testBenchmarkIndividualLarge() public {
        // Option 1: Process transaction - Large data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransaction(largeData);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process transaction - Large data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransaction(largeData);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processTransaction", "Large", option1Gas, option2Gas);
    }

    function testBenchmarkRawTiny() public {
        // Option 1: Process raw transaction - Tiny data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransactionRaw(tinyData);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process raw transaction - Tiny data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransactionRaw(tinyData);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processTransactionRaw", "Tiny", option1Gas, option2Gas);
    }

    function testBenchmarkRawSmall() public {
        // Option 1: Process raw transaction - Small data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransactionRaw(smallData);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process raw transaction - Small data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransactionRaw(smallData);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processTransactionRaw", "Small", option1Gas, option2Gas);
    }

    function testBenchmarkBulkTiny() public {
        // Option 1: Process bulk transactions - Tiny data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processBulkTransactions(tinyDataArray);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process bulk transactions - Tiny data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processBulkTransactions(tinyDataArray);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processBulkTransactions", "Tiny", option1Gas, option2Gas);
    }

    function testBenchmarkBulkSmall() public {
        // Option 1: Process bulk transactions - Small data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processBulkTransactions(smallDataArray);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process bulk transactions - Small data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processBulkTransactions(smallDataArray);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processBulkTransactions", "Small", option1Gas, option2Gas);
    }

    function testBenchmarkBulkMedium() public {
        // Option 1: Process bulk transactions - Medium data
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processBulkTransactions(mediumDataArray);
        uint256 option1Gas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Process bulk transactions - Medium data
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processBulkTransactions(mediumDataArray);
        uint256 option2Gas = gasStart - gasleft();
        vm.stopPrank();

        logResult("processBulkTransactions", "Medium", option1Gas, option2Gas);
    }

    //////////////////////////////////////////////////////////////
    // SEQUENTIAL MIXED OPERATIONS (COMPREHENSIVE TESTS)
    //////////////////////////////////////////////////////////////

    // This test reflects real-world usage where a contract might perform
    // a mix of different operations in sequence
    function testComprehensiveMixedOperations() public {
        console2.log("=== COMPREHENSIVE MIXED OPERATIONS BENCHMARK ===");

        uint256 option1TotalGas = 0;
        uint256 option2TotalGas = 0;
        uint256 gasStart;

        // Option 1: Sequence of mixed operations
        vm.startPrank(sequencer);

        // Process a single transaction
        gasStart = gasleft();
        option1Chain.processTransaction(smallData);
        option1TotalGas += gasStart - gasleft();

        // Process a raw transaction
        gasStart = gasleft();
        option1Chain.processTransactionRaw(mediumData);
        option1TotalGas += gasStart - gasleft();

        // Process bulk transactions
        gasStart = gasleft();
        option1Chain.processBulkTransactions(tinyDataArray);
        option1TotalGas += gasStart - gasleft();

        vm.stopPrank();

        // Option 2: Same sequence of mixed operations
        vm.startPrank(sequencer);

        // Process a single transaction
        gasStart = gasleft();
        option2Chain.processTransaction(smallData);
        option2TotalGas += gasStart - gasleft();

        // Process a raw transaction
        gasStart = gasleft();
        option2Chain.processTransactionRaw(mediumData);
        option2TotalGas += gasStart - gasleft();

        // Process bulk transactions
        gasStart = gasleft();
        option2Chain.processBulkTransactions(tinyDataArray);
        option2TotalGas += gasStart - gasleft();

        vm.stopPrank();

        logResult("Mixed Operations", "Comprehensive", option1TotalGas, option2TotalGas);
    }

    // This test helps understand how gas costs change when sequential similar operations
    // are performed (might show different pattern than mixed operations)
    function testComprehensiveSequentialOperations() public {
        console2.log("=== COMPREHENSIVE SEQUENTIAL OPERATIONS BENCHMARK ===");

        uint256 option1TotalGas = 0;
        uint256 option2TotalGas = 0;
        uint256 gasStart;

        // Option 1: Sequential identical operations
        vm.startPrank(sequencer);

        // Sequential processing of transactions with different data sizes
        gasStart = gasleft();
        option1Chain.processTransaction(tinyData);
        option1TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option1Chain.processTransaction(smallData);
        option1TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option1Chain.processTransaction(mediumData);
        option1TotalGas += gasStart - gasleft();

        vm.stopPrank();

        // Option 2: Same sequential operations
        vm.startPrank(sequencer);

        gasStart = gasleft();
        option2Chain.processTransaction(tinyData);
        option2TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option2Chain.processTransaction(smallData);
        option2TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option2Chain.processTransaction(mediumData);
        option2TotalGas += gasStart - gasleft();

        vm.stopPrank();

        logResult("Sequential Identical Operations", "Comprehensive", option1TotalGas, option2TotalGas);
    }

    //////////////////////////////////////////////////////////////
    // RAW INTERFACE CALL BENCHMARKS
    //////////////////////////////////////////////////////////////

    // This test directly compares the gas cost of the interface calls themselves
    function testRawInterfaceCalls() public {
        console2.log("=== RAW INTERFACE CALLS BENCHMARK ===");

        // Option 1: Consolidated interface call
        uint256 gasStart = gasleft();
        consolidatedModule.isAllowed(sequencer, smallData);
        uint256 option1Gas = gasStart - gasleft();

        // Option 2: Split interface calls
        gasStart = gasleft();
        extendedModule.isAllowed(sequencer);
        extendedModule.isCalldataAllowed(smallData);
        uint256 option2Gas = gasStart - gasleft();

        logResult("Raw Interface Calls", "Direct", option1Gas, option2Gas);

        // Also measure individual calls for Option 2
        gasStart = gasleft();
        extendedModule.isAllowed(sequencer);
        uint256 proposerCheckGas = gasStart - gasleft();

        gasStart = gasleft();
        extendedModule.isCalldataAllowed(smallData);
        uint256 calldataCheckGas = gasStart - gasleft();

        console2.log("Option 2 breakdown:");
        console2.log("  Proposer check gas: %d", proposerCheckGas);
        console2.log("  Calldata check gas: %d", calldataCheckGas);
        console2.log("  Sum of individual checks: %d", proposerCheckGas + calldataCheckGas);
        console2.log("");
    }

    //////////////////////////////////////////////////////////////
    // HOT/COLD STORAGE ACCESS PATTERN TESTS
    //////////////////////////////////////////////////////////////

    // This test demonstrates how storage access patterns affect gas costs
    function testHotColdStoragePatterns() public {
        console2.log("=== HOT/COLD STORAGE ACCESS PATTERN BENCHMARK ===");

        uint256 option1FirstCallGas;
        uint256 option1SecondCallGas;
        uint256 option2FirstCallGas;
        uint256 option2SecondCallGas;

        // Option 1: First call (cold storage)
        vm.startPrank(sequencer);
        uint256 gasStart = gasleft();
        option1Chain.processTransaction(smallData);
        option1FirstCallGas = gasStart - gasleft();
        vm.stopPrank();

        // Option 1: Second call (warm storage)
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option1Chain.processTransaction(smallData);
        option1SecondCallGas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: First call (cold storage)
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransaction(smallData);
        option2FirstCallGas = gasStart - gasleft();
        vm.stopPrank();

        // Option 2: Second call (warm storage)
        vm.startPrank(sequencer);
        gasStart = gasleft();
        option2Chain.processTransaction(smallData);
        option2SecondCallGas = gasStart - gasleft();
        vm.stopPrank();

        console2.log("Option 1 (Consolidated):");
        console2.log("  First call (cold): %d", option1FirstCallGas);
        console2.log("  Second call (warm): %d", option1SecondCallGas);
        console2.log("  Cold-Warm difference: %d", option1FirstCallGas - option1SecondCallGas);
        console2.log("");

        console2.log("Option 2 (Split):");
        console2.log("  First call (cold): %d", option2FirstCallGas);
        console2.log("  Second call (warm): %d", option2SecondCallGas);
        console2.log("  Cold-Warm difference: %d", option2FirstCallGas - option2SecondCallGas);
        console2.log("");

        // Compare cold and warm states between architectures
        logResult("Cold Storage Access", "First Call", option1FirstCallGas, option2FirstCallGas);
        logResult("Warm Storage Access", "Second Call", option1SecondCallGas, option2SecondCallGas);
    }

    //////////////////////////////////////////////////////////////
    // FINAL COMPREHENSIVE BENCHMARK SUMMARY
    //////////////////////////////////////////////////////////////

    // This function runs all key benchmarks and aggregates results
    function testFinalBenchmarkSummary() public {
        console2.log("======== FINAL BENCHMARK SUMMARY ========");

        // Individual operations with different data sizes
        _runAndLogSingleOpBenchmark("processTransaction", tinyData);
        _runAndLogSingleOpBenchmark("processTransaction", smallData);
        _runAndLogSingleOpBenchmark("processTransaction", mediumData);

        _runAndLogSingleOpBenchmark("processTransactionRaw", tinyData);
        _runAndLogSingleOpBenchmark("processTransactionRaw", smallData);

        _runAndLogBulkOpBenchmark("processBulkTransactions", tinyDataArray);
        _runAndLogBulkOpBenchmark("processBulkTransactions", smallDataArray);

        // Comprehensive tests
        _runComprehensiveBenchmarks();

        console2.log("======== END OF BENCHMARK SUMMARY ========");
    }

    // Helper functions for the final benchmark
    function _runAndLogSingleOpBenchmark(string memory operation, bytes memory data) internal {
        uint256 option1Gas;
        uint256 option2Gas;
        string memory dataSize = _getDataSizeLabel(data.length);

        vm.startPrank(sequencer);

        if (keccak256(bytes(operation)) == keccak256(bytes("processTransaction"))) {
            uint256 gasStart = gasleft();
            option1Chain.processTransaction(data);
            option1Gas = gasStart - gasleft();

            gasStart = gasleft();
            option2Chain.processTransaction(data);
            option2Gas = gasStart - gasleft();
        } else if (keccak256(bytes(operation)) == keccak256(bytes("processTransactionRaw"))) {
            uint256 gasStart = gasleft();
            option1Chain.processTransactionRaw(data);
            option1Gas = gasStart - gasleft();

            gasStart = gasleft();
            option2Chain.processTransactionRaw(data);
            option2Gas = gasStart - gasleft();
        }

        vm.stopPrank();

        logResult(operation, dataSize, option1Gas, option2Gas);
    }

    function _runAndLogBulkOpBenchmark(string memory operation, bytes[] memory dataArray) internal {
        uint256 option1Gas;
        uint256 option2Gas;
        string memory dataSize = _getDataSizeLabel(dataArray[0].length);

        vm.startPrank(sequencer);

        if (keccak256(bytes(operation)) == keccak256(bytes("processBulkTransactions"))) {
            uint256 gasStart = gasleft();
            option1Chain.processBulkTransactions(dataArray);
            option1Gas = gasStart - gasleft();

            gasStart = gasleft();
            option2Chain.processBulkTransactions(dataArray);
            option2Gas = gasStart - gasleft();
        }

        vm.stopPrank();

        console2.log("Bulk operation: %s", operation);
        logResult(operation, dataSize, option1Gas, option2Gas);
    }

    function _runComprehensiveBenchmarks() internal {
        uint256 option1TotalGas = 0;
        uint256 option2TotalGas = 0;
        uint256 gasStart;

        vm.startPrank(sequencer);

        // Mixed operations for Option 1
        gasStart = gasleft();
        option1Chain.processTransaction(smallData);
        option1TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option1Chain.processTransactionRaw(tinyData);
        option1TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option1Chain.processBulkTransactions(tinyDataArray);
        option1TotalGas += gasStart - gasleft();

        // Same mixed operations for Option 2
        gasStart = gasleft();
        option2Chain.processTransaction(smallData);
        option2TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option2Chain.processTransactionRaw(tinyData);
        option2TotalGas += gasStart - gasleft();

        gasStart = gasleft();
        option2Chain.processBulkTransactions(tinyDataArray);
        option2TotalGas += gasStart - gasleft();

        vm.stopPrank();

        logResult("Comprehensive", "Mixed Operations", option1TotalGas, option2TotalGas);
    }

    function _getDataSizeLabel(uint256 length) internal pure returns (string memory) {
        if (length <= 4) return "Tiny";
        if (length <= 32) return "Small";
        if (length <= 512) return "Medium";
        return "Large";
    }
}
