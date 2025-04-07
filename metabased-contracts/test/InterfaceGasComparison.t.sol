// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import {Test, console} from "forge-std/Test.sol";

// Option 1: Consolidated Interfaces
import {OptionOneMetabasedSequencerChain} from "src/OptionOneMetabasedSequencerChain.sol";
import {ConsolidatedAllowlistModule} from "src/sequencing-modules/ConsolidatedAllowlistModule.sol";

// Option 2: Split Interfaces
import {MetabasedSequencerChain} from "src/MetabasedSequencerChain.sol";
import {AllowlistSequencingModuleExtended} from "src/sequencing-modules/AllowlistSequencingModuleExtended.sol";

// Gas Test Contract
contract InterfaceGasComparison is Test {
    // Test chains
    OptionOneMetabasedSequencerChain public option1Chain;
    MetabasedSequencerChain public option2Chain;
    MetabasedSequencerChain public zeroAddressChain; // Pre-deploy for zero address test

    // Modules
    ConsolidatedAllowlistModule public consolidatedModule;
    AllowlistSequencingModuleExtended public moduleExtended;
    AllowlistSequencingModuleExtended public proposerOnlyModule;

    // Test data
    bytes smallData; // 32 bytes
    bytes mediumData; // 512 bytes
    bytes largeData; // 4 KB

    // Test addresses
    address admin = address(1);
    address sequencer = address(2);

    // Events for logging results
    event GasUsageResult(string testName, string dataSize, uint256 option1Gas, uint256 option2Gas, int256 difference);
    event ZeroAddressResult(string dataSize, uint256 normalGas, uint256 zeroAddressGas, uint256 savings);
    event RawFunctionCallResult(string callType, uint256 gasUsed);

    function setUp() public {
        // Initialize test data
        smallData = abi.encodePacked(bytes32(uint256(1))); // 32 bytes

        // Create medium data (512 bytes)
        mediumData = "";
        for (uint256 i = 0; i < 16; i++) {
            mediumData = bytes.concat(mediumData, bytes32(uint256(i + 1)));
        }

        // Create large data (4 KB)
        largeData = "";
        for (uint256 i = 0; i < 128; i++) {
            largeData = bytes.concat(largeData, bytes32(uint256(i + 1)));
        }

        // Log data sizes via events
        emit GasUsageResult("Data Sizes", "Small", smallData.length, 0, 0);
        emit GasUsageResult("Data Sizes", "Medium", mediumData.length, 0, 0);
        emit GasUsageResult("Data Sizes", "Large", largeData.length, 0, 0);

        // Deploy modules
        vm.startPrank(admin);

        // Deploy Option 1 (Consolidated) modules and chain
        consolidatedModule = new ConsolidatedAllowlistModule(admin);
        consolidatedModule.addToAllowlist(sequencer);

        option1Chain = new OptionOneMetabasedSequencerChain(1);
        option1Chain.initialize(admin, address(consolidatedModule));

        // Deploy Option 2 (Split) modules and chain
        moduleExtended = new AllowlistSequencingModuleExtended(admin);
        moduleExtended.addToAllowlist(sequencer);

        // Create module just for proposer checks
        proposerOnlyModule = new AllowlistSequencingModuleExtended(admin);
        proposerOnlyModule.addToAllowlist(sequencer);

        option2Chain = new MetabasedSequencerChain(1);
        option2Chain.initialize(admin, address(moduleExtended));
        option2Chain.updateRequirementModule(address(moduleExtended));

        // Create a separate chain with just proposer checks (no calldata module)
        // This simulates the zero address optimization by not setting calldata module at all
        zeroAddressChain = new MetabasedSequencerChain(1);
        zeroAddressChain.initialize(admin, address(proposerOnlyModule));
        // We intentionally don't set the calldata module for the zero address chain

        vm.stopPrank();
    }

    function testGasComparisonSmallData() public {
        // Option 1: Consolidated Interface
        vm.startPrank(sequencer);
        uint256 gasStart1 = gasleft();
        option1Chain.processTransaction(smallData);
        uint256 option1Gas = gasStart1 - gasleft();
        console.log("Option 1 Gas Used:", option1Gas);

        vm.stopPrank();

        // Option 2: Split Interface
        vm.startPrank(sequencer);
        uint256 gasStart2 = gasleft();
        option2Chain.processTransaction(smallData);
        uint256 option2Gas = gasStart2 - gasleft();
        console.log("Option 2 Gas Used:", option2Gas);
        console.log("Gas Difference:", int256(option2Gas) - int256(option1Gas));
        vm.stopPrank();

        // Log results via event
        emit GasUsageResult(
            "Basic Comparison", "Small", option1Gas, option2Gas, int256(option2Gas) - int256(option1Gas)
        );
    }

    function testGasComparisonMediumData() public {
        // Option 1: Consolidated Interface
        vm.startPrank(sequencer);
        uint256 gasStart1 = gasleft();
        option1Chain.processTransaction(mediumData);
        uint256 option1Gas = gasStart1 - gasleft();
        console.log("Option 1 Gas Used:", option1Gas);
        vm.stopPrank();

        // Option 2: Split Interface
        vm.startPrank(sequencer);
        uint256 gasStart2 = gasleft();
        option2Chain.processTransaction(mediumData);
        uint256 option2Gas = gasStart2 - gasleft();
        console.log("Option 2 Gas Used:", option2Gas);
        console.log("Gas Difference:", int256(option2Gas) - int256(option1Gas));

        vm.stopPrank();

        // Log results via event
        emit GasUsageResult(
            "Basic Comparison", "Medium", option1Gas, option2Gas, int256(option2Gas) - int256(option1Gas)
        );
    }

    function testGasComparisonLargeData() public {
        // Option 1: Consolidated Interface
        vm.startPrank(sequencer);
        uint256 gasStart1 = gasleft();
        option1Chain.processTransaction(largeData);
        uint256 option1Gas = gasStart1 - gasleft();
        console.log("Option 1 Gas Used:", option1Gas);

        vm.stopPrank();

        // Option 2: Split Interface
        vm.startPrank(sequencer);
        uint256 gasStart2 = gasleft();
        option2Chain.processTransaction(largeData);
        uint256 option2Gas = gasStart2 - gasleft();
        console.log("Option 2 Gas Used:", option2Gas);
        console.log("Gas Difference:", int256(option2Gas) - int256(option1Gas));
        vm.stopPrank();

        // Log results via event
        emit GasUsageResult(
            "Basic Comparison", "Large", option1Gas, option2Gas, int256(option2Gas) - int256(option1Gas)
        );
    }

    function testComprehensiveGasReport() public {
        // Test data sizes
        bytes[3] memory testData = [smallData, mediumData, largeData];
        string[3] memory sizeNames = ["Small", "Medium", "Large"];

        for (uint256 i = 0; i < 3; i++) {
            // Option 1: Consolidated Interface
            vm.startPrank(sequencer);
            uint256 gasStart1 = gasleft();
            option1Chain.processTransaction(testData[i]);
            uint256 option1Gas = gasStart1 - gasleft();
            console.log("Option 1 Gas Used:", option1Gas);
            vm.stopPrank();

            // Option 2: Split Interface
            vm.startPrank(sequencer);
            uint256 gasStart2 = gasleft();
            option2Chain.processTransaction(testData[i]);
            uint256 option2Gas = gasStart2 - gasleft();
            console.log("Option 2 Gas Used:", option2Gas);
            console.log("Gas Difference:", int256(option2Gas) - int256(option1Gas));
            vm.stopPrank();

            // Calculate difference
            int256 gasDiff = int256(option2Gas) - int256(option1Gas);
            console.log("********************");
            console.log("          ");
            // Log results via event
            emit GasUsageResult("Comprehensive", sizeNames[i], option1Gas, option2Gas, gasDiff);
        }
    }

    // Test Option 2 with zero address optimization (calldata module disabled)
    function testOption2WithCallDataDisabled() public {
        // Test data sizes
        bytes[3] memory testData = [smallData, mediumData, largeData];
        string[3] memory sizeNames = ["Small", "Medium", "Large"];

        for (uint256 i = 0; i < 3; i++) {
            // Option 2 (Normal with both checks)
            vm.startPrank(sequencer);
            uint256 gasStart1 = gasleft();
            option2Chain.processTransaction(testData[i]);
            uint256 normalGas = gasStart1 - gasleft();
            vm.stopPrank();

            // Option 2 (Zero Address - only proposer check)
            vm.startPrank(sequencer);
            uint256 gasStart2 = gasleft();
            zeroAddressChain.processTransaction(testData[i]);
            uint256 zeroAddressGas = gasStart2 - gasleft();
            vm.stopPrank();

            // Calculate savings
            uint256 gasSavings = normalGas > zeroAddressGas ? normalGas - zeroAddressGas : 0;
            console.log("Normal Gas Used:", normalGas);
            console.log("Zero Address Gas Used:", zeroAddressGas);
            console.log("Gas Savings:", gasSavings);
            console.log("********************");
            console.log("          ");
            // Log results via event
            emit ZeroAddressResult(sizeNames[i], normalGas, zeroAddressGas, gasSavings);
        }
    }

    // Test raw function call gas costs
    function testRawFunctionCallGas() public {
        uint256 gasStart;
        uint256 gasUsed;

        // Option 1: Consolidated interface call
        gasStart = gasleft();
        consolidatedModule.isAllowed(sequencer, smallData);
        gasUsed = gasStart - gasleft();
        console.log("Consolidated call gas used:", gasUsed);

        emit RawFunctionCallResult("Consolidated call", gasUsed);

        // Option 2: Split interface calls - proposer only
        gasStart = gasleft();
        moduleExtended.isAllowed(sequencer);
        gasUsed = gasStart - gasleft();

        console.log("Split call (proposer only) gas used:", gasUsed);

        emit RawFunctionCallResult("Split call (proposer only)", gasUsed);

        // Option 2: Split interface calls - data only
        gasStart = gasleft();
        moduleExtended.isCalldataAllowed(smallData);
        gasUsed = gasStart - gasleft();

        console.log("Split call (data only) gas used:", gasUsed);

        emit RawFunctionCallResult("Split call (data only)", gasUsed);

        // Calculate overhead of two calls
        gasStart = gasleft();
        moduleExtended.isAllowed(sequencer);
        moduleExtended.isCalldataAllowed(smallData);
        gasUsed = gasStart - gasleft();

        console.log("Two separate calls total gas used:", gasUsed);

        // Log results via event
        emit RawFunctionCallResult("Two separate calls total", gasUsed);
    }
}
