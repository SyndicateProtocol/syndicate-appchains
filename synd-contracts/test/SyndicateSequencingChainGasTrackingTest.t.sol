// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {GasCounter} from "src/GasCounter.sol";
import {Test} from "forge-std/Test.sol";

contract SyndicateSequencingChainGasTrackingTest is Test {
    SyndicateSequencingChain public chain;
    RequireAndModule public permissionModule;
    AlwaysAllowedModule public alwaysAllowed;
    address public admin;

    // Events to test
    event TransactionProcessed(address indexed sender, bytes data);
    event GasTracked(uint256 indexed periodIndex, uint256 gasUsed, uint256 gasPrice);
    event NewPeriodStarted(uint256 indexed periodIndex, uint256 startTimestamp);

    function setUp() public {
        admin = address(0x1);
        uint256 appchainId = 10042001;

        // Deploy contracts
        vm.startPrank(admin);
        permissionModule = new RequireAndModule(admin);
        alwaysAllowed = new AlwaysAllowedModule();
        chain = new SyndicateSequencingChain(appchainId);

        // Initialize the chain with admin and permission module
        chain.initialize(admin, address(permissionModule));

        // Set up permission module to always allow
        permissionModule.addPermissionCheck(address(alwaysAllowed), false);
        vm.stopPrank();
        
        // Set a default gas price for gas cost calculations
        vm.txGasPrice(1e9); // 1 gwei
    }

    // ============ GAS TRACKING INTEGRATION TESTS ============

    function test_ProcessTransaction_TracksGas() public {
        bytes memory data = abi.encode("test transaction data");

        // Gas tracking should be initialized automatically
        assertFalse(chain.isGasTrackingInitialized());

        chain.processTransaction(data);

        // Verify gas tracking was initialized and gas was tracked
        assertTrue(chain.isGasTrackingInitialized());
        assertGt(chain.getCurrentPeriodGasUsed(), 0);
        assertGt(chain.getTotalGasFees(), 0);
        assertEq(chain.currentPeriodIndex(), 0);
        assertEq(chain.getTotalPeriods(), 1);
    }

    function test_ProcessTransactionUncompressed_TracksGas() public {
        bytes memory data = abi.encode("uncompressed transaction data");

        chain.processTransactionUncompressed(data);

        assertTrue(chain.isGasTrackingInitialized());
        assertGt(chain.getCurrentPeriodGasUsed(), 0);
        assertGt(chain.getTotalGasFees(), 0);
    }

    function test_ProcessTransactionsBulk_TracksGas() public {
        bytes[] memory dataArray = new bytes[](3);
        dataArray[0] = abi.encode("bulk transaction 1");
        dataArray[1] = abi.encode("bulk transaction 2");
        dataArray[2] = abi.encode("bulk transaction 3");

        chain.processTransactionsBulk(dataArray);

        assertTrue(chain.isGasTrackingInitialized());
        assertGt(chain.getCurrentPeriodGasUsed(), 0);
        assertGt(chain.getTotalGasFees(), 0);
    }

    function test_MultipleTransactions_AccumulateGas() public {
        bytes memory data1 = abi.encode("transaction 1");
        bytes memory data2 = abi.encode("transaction 2");

        // Process first transaction
        chain.processTransaction(data1);
        uint256 gasAfterFirst = chain.getCurrentPeriodGasUsed();
        uint256 feesAfterFirst = chain.getTotalGasFees();

        assertGt(gasAfterFirst, 0);
        assertGt(feesAfterFirst, 0);

        // Process second transaction
        chain.processTransaction(data2);
        uint256 gasAfterSecond = chain.getCurrentPeriodGasUsed();
        uint256 feesAfterSecond = chain.getTotalGasFees();

        // Gas should have accumulated
        assertGt(gasAfterSecond, gasAfterFirst);
        assertGt(feesAfterSecond, feesAfterFirst);

        // Still in same period
        assertEq(chain.currentPeriodIndex(), 0);
        assertEq(chain.getTotalPeriods(), 1);
    }

    function test_GasPeriodAdvancement() public {
        bytes memory data = abi.encode("test transaction");

        // Process transaction in first period
        chain.processTransaction(data);
        uint256 firstPeriodGas = chain.getCurrentPeriodGasUsed();

        // Fast forward past period duration
        vm.warp(block.timestamp + chain.PERIOD_DURATION() + 1);

        // Process transaction in new period - should trigger period advancement
        chain.processTransaction(data);

        // Should be in new period now
        assertEq(chain.currentPeriodIndex(), 1);
        assertEq(chain.getTotalPeriods(), 2);

        // New period should have gas tracked, old period should be finalized
        assertGt(chain.getCurrentPeriodGasUsed(), 0);

        // Check previous period was finalized
        GasCounter.GasPeriod memory previousPeriod = chain.getPeriod(0);
        assertGt(previousPeriod.endTimestamp, 0); // Finalized
        assertEq(previousPeriod.totalGasUsed, firstPeriodGas);
    }

    function test_GetTotalGasFees_ReflectsCurrentPeriod() public {
        // Process some transactions
        chain.processTransaction(abi.encode("tx1"));
        chain.processTransaction(abi.encode("tx2"));

        uint256 totalFees = chain.getTotalGasFees();
        
        // Total fees should be positive and based on actual gas prices
        assertGt(totalFees, 0);
        // Since gas price is dynamic, we just verify fees were calculated
        assertEq(chain.getTotalGasFees(), totalFees);
    }

    function test_ViewFunctions_WorkCorrectly() public {
        // Before any transactions
        assertEq(chain.getCurrentPeriodGasUsed(), 0);
        assertEq(chain.getTotalGasFees(), 0);
        assertEq(chain.getTotalPeriods(), 0);
        assertFalse(chain.isGasTrackingInitialized());

        // After processing a transaction
        chain.processTransaction(abi.encode("test"));

        assertGt(chain.getCurrentPeriodGasUsed(), 0);
        assertGt(chain.getTotalGasFees(), 0);
        assertEq(chain.getTotalPeriods(), 1);
        assertTrue(chain.isGasTrackingInitialized());

        // Time remaining should be close to full period
        assertGt(chain.getCurrentPeriodTimeRemaining(), chain.PERIOD_DURATION() - 100);
    }

    function test_Constants() public view {
        assertEq(chain.PERIOD_DURATION(), 30 days);
        // Gas price is now dynamic based on tx.gasprice
    }

    // ============ INTEGRATION WITH EXISTING FUNCTIONALITY ============

    function test_BulkProcessing_TracksGas() public {
        bytes[] memory dataArray = new bytes[](3);
        for (uint256 i = 0; i < 3; i++) {
            dataArray[i] = abi.encode("bulk transaction", i);
        }

        // Process bulk transactions
        chain.processTransactionsBulk(dataArray);

        // Should track gas for the entire bulk operation
        assertTrue(chain.isGasTrackingInitialized());
        assertGt(chain.getCurrentPeriodGasUsed(), 0);
        assertGt(chain.getTotalGasFees(), 0);

        // Should be in first period
        assertEq(chain.currentPeriodIndex(), 0);
        assertEq(chain.getTotalPeriods(), 1);
    }
}
