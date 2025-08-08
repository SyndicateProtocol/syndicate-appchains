// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateSequencingChain} from "src/SyndicateSequencingChain.sol";
import {SyndicateFactory} from "src/factory/SyndicateFactory.sol";
import {RequireAndModule} from "src/requirement-modules/RequireAndModule.sol";
import {AlwaysAllowedModule} from "src/sequencing-modules/AlwaysAllowedModule.sol";
import {Test} from "forge-std/Test.sol";

contract SyndicateSequencingChainGasTrackingTest is Test {
    SyndicateSequencingChain public chain;
    SyndicateFactory public factory;
    RequireAndModule public permissionModule;
    AlwaysAllowedModule public alwaysAllowed;
    address public admin;
    uint256 public appchainId;

    // Events to test
    event TransactionProcessed(address indexed sender, bytes data);
    event GasTracked(uint256 indexed periodIndex, uint256 gasUsed, uint256 gasPrice);
    event NewPeriodStarted(uint256 indexed periodIndex, uint256 startTimestamp);

    function setUp() public {
        admin = address(0x1);
        appchainId = 10042001;

        // Deploy contracts
        vm.startPrank(admin);

        // Deploy factory first
        factory = new SyndicateFactory(admin);

        // Deploy permission modules
        permissionModule = new RequireAndModule(admin);
        alwaysAllowed = new AlwaysAllowedModule();

        // Deploy chain through factory
        (address chainAddress,) = factory.createSyndicateSequencingChain(
            appchainId, admin, permissionModule, keccak256(abi.encodePacked("test-salt"))
        );
        chain = SyndicateSequencingChain(chainAddress);

        // Set up permission module to always allow
        permissionModule.addPermissionCheck(address(alwaysAllowed), false);
        vm.stopPrank();

        // Set a default gas price for gas cost calculations
        vm.txGasPrice(1e9); // 1 gwei
    }

    // ============ GAS TRACKING INTEGRATION TESTS ============

    function test_ProcessTransaction_TracksGas() public {
        bytes memory data = abi.encode("test transaction data");

        // Gas tracking should not be initialized before first transaction
        assertFalse(factory.gasTrackingInitialized());

        chain.processTransaction(data);

        // Verify gas tracking was initialized and gas was tracked
        assertTrue(factory.gasTrackingInitialized());

        SyndicateFactory.AppchainGasPeriod memory period = factory.getCurrentAppchainData(appchainId);
        assertGt(period.totalGasUsed, 0);
        assertGt(period.totalGasCost, 0);
        assertEq(factory.currentEpoch(), 0);
    }

    function test_ProcessTransactionUncompressed_TracksGas() public {
        bytes memory data = abi.encode("uncompressed transaction data");

        chain.processTransactionUncompressed(data);

        assertTrue(factory.gasTrackingInitialized());
        SyndicateFactory.AppchainGasPeriod memory period = factory.getCurrentAppchainData(appchainId);
        assertGt(period.totalGasUsed, 0);
        assertGt(period.totalGasCost, 0);
    }

    function test_ProcessTransactionsBulk_TracksGas() public {
        bytes[] memory dataArray = new bytes[](3);
        dataArray[0] = abi.encode("bulk transaction 1");
        dataArray[1] = abi.encode("bulk transaction 2");
        dataArray[2] = abi.encode("bulk transaction 3");

        chain.processTransactionsBulk(dataArray);

        assertTrue(factory.gasTrackingInitialized());
        SyndicateFactory.AppchainGasPeriod memory period = factory.getCurrentAppchainData(appchainId);
        assertGt(period.totalGasUsed, 0);
        assertGt(period.totalGasCost, 0);
    }

    function test_MultipleTransactions_AccumulateGas() public {
        bytes memory data1 = abi.encode("transaction 1");
        bytes memory data2 = abi.encode("transaction 2");

        // Process first transaction
        chain.processTransaction(data1);
        SyndicateFactory.AppchainGasPeriod memory periodAfterFirst = factory.getCurrentAppchainData(appchainId);

        assertGt(periodAfterFirst.totalGasUsed, 0);
        assertGt(periodAfterFirst.totalGasCost, 0);

        // Process second transaction
        chain.processTransaction(data2);
        SyndicateFactory.AppchainGasPeriod memory periodAfterSecond = factory.getCurrentAppchainData(appchainId);

        // Gas should have accumulated
        assertGt(periodAfterSecond.totalGasUsed, periodAfterFirst.totalGasUsed);
        assertGt(periodAfterSecond.totalGasCost, periodAfterFirst.totalGasCost);

        // Still in same epoch
        assertEq(factory.currentEpoch(), 0);
    }

    function test_GasPeriodAdvancement() public {
        bytes memory data = abi.encode("test transaction");

        // Process transaction in first epoch
        chain.processTransaction(data);
        SyndicateFactory.AppchainGasPeriod memory firstEpochData = factory.getCurrentAppchainData(appchainId);

        // Fast forward past period duration (30 days)
        vm.warp(block.timestamp + 30 days + 1);

        // Process transaction in new epoch - should trigger epoch advancement
        chain.processTransaction(data);

        // Should be in new epoch now
        assertEq(factory.currentEpoch(), 1);

        // New epoch should have gas tracked, old epoch should be finalized
        SyndicateFactory.AppchainGasPeriod memory newEpochData = factory.getCurrentAppchainData(appchainId);
        assertGt(newEpochData.totalGasUsed, 0);

        // Check previous epoch was finalized
        SyndicateFactory.AppchainGasPeriod memory previousEpochData = factory.getAppchainEpochData(0, appchainId);
        assertGt(previousEpochData.endTimestamp, 0); // Finalized
        assertEq(previousEpochData.totalGasUsed, firstEpochData.totalGasUsed);
    }

    function test_GetTotalGasFees_ReflectsCurrentPeriod() public {
        // Process some transactions
        chain.processTransaction(abi.encode("tx1"));
        chain.processTransaction(abi.encode("tx2"));

        SyndicateFactory.AppchainGasPeriod memory period = factory.getCurrentAppchainData(appchainId);

        // Total fees should be positive and based on actual gas prices
        assertGt(period.totalGasCost, 0);
    }

    function test_ViewFunctions_WorkCorrectly() public {
        // Before any transactions
        SyndicateFactory.AppchainGasPeriod memory emptyPeriod = factory.getCurrentAppchainData(appchainId);
        assertEq(emptyPeriod.totalGasUsed, 0);
        assertEq(emptyPeriod.totalGasCost, 0);
        assertFalse(factory.gasTrackingInitialized());

        // After processing a transaction
        chain.processTransaction(abi.encode("test"));

        SyndicateFactory.AppchainGasPeriod memory activePeriod = factory.getCurrentAppchainData(appchainId);
        assertGt(activePeriod.totalGasUsed, 0);
        assertGt(activePeriod.totalGasCost, 0);
        assertTrue(factory.gasTrackingInitialized());

        // Time remaining should be close to full period
        assertGt(factory.getCurrentEpochTimeRemaining(), 30 days - 100);
    }

    function test_Constants() public pure {
        // Period duration is defined in GasTracker as 30 days
        uint256 thirtyDays = 30 days;
        assertEq(thirtyDays, 2592000);
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
        assertTrue(factory.gasTrackingInitialized());
        SyndicateFactory.AppchainGasPeriod memory period = factory.getCurrentAppchainData(appchainId);
        assertGt(period.totalGasUsed, 0);
        assertGt(period.totalGasCost, 0);

        // Should be in first epoch
        assertEq(factory.currentEpoch(), 0);
    }
}
