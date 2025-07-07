// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {GasCounter} from "src/GasCounter.sol";

contract TestGasCounter is GasCounter {
    function trackGasPublic(uint256 gasUsed) external {
        _trackGas(gasUsed);
    }

    function initializeGasTrackingPublic() external {
        _initializeGasTracking();
    }

    // Note: Gas price is now dynamic based on tx.gasprice, no longer configurable

    function useTrackGasUsageModifier() external trackGasUsage {
        // Simple function to test the modifier
        uint256 dummy = 42;
        dummy = dummy + 1;
    }
}

contract GasCounterTest is Test {
    TestGasCounter public gasCounter;

    // Events for testing
    event GasTracked(uint256 indexed periodIndex, uint256 gasUsed, uint256 gasPrice);
    event PeriodFinalized(uint256 indexed periodIndex, uint256 totalGasUsed, uint256 duration);
    event NewPeriodStarted(uint256 indexed periodIndex, uint256 startTimestamp);
    event GasPriceUpdated(uint256 oldPrice, uint256 newPrice);

    function setUp() public {
        gasCounter = new TestGasCounter();
    }

    // ============ INITIALIZATION TESTS ============

    function test_InitialState() public view {
        assertFalse(gasCounter.isGasTrackingInitialized());
        assertEq(gasCounter.currentPeriodIndex(), 0);
        assertEq(gasCounter.getTotalPeriods(), 0);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 0);
        assertEq(gasCounter.getTotalGasFees(), 0);
        assertEq(gasCounter.getCumulativeGasFees(), 0);
        assertEq(gasCounter.cumulativeGasFees(), 0);
        // Gas price is now dynamic based on tx.gasprice
    }

    function test_InitializeGasTracking() public {
        vm.expectEmit(true, false, false, true);
        emit NewPeriodStarted(0, block.timestamp);

        gasCounter.initializeGasTrackingPublic();

        assertTrue(gasCounter.isGasTrackingInitialized());
        assertEq(gasCounter.currentPeriodIndex(), 0);
        assertEq(gasCounter.getTotalPeriods(), 1);

        GasCounter.GasPeriod memory period = gasCounter.getCurrentPeriod();
        assertEq(period.startTimestamp, block.timestamp);
        assertEq(period.endTimestamp, 0);
        assertEq(period.totalGasUsed, 0);
        assertEq(period.endTimestamp, 0); // Not finalized yet
    }

    function test_CannotInitializeTwice() public {
        gasCounter.initializeGasTrackingPublic();

        // Second initialization should not change anything
        uint256 firstTimestamp = gasCounter.getCurrentPeriod().startTimestamp;

        vm.warp(block.timestamp + 100);
        gasCounter.initializeGasTrackingPublic();

        assertEq(gasCounter.getCurrentPeriod().startTimestamp, firstTimestamp);
        assertEq(gasCounter.getTotalPeriods(), 1);
    }

    // ============ GAS TRACKING TESTS ============

    function test_TrackGas_InitializesAutomatically() public {
        vm.expectEmit(true, false, false, true);
        emit NewPeriodStarted(0, block.timestamp);

        vm.expectEmit(true, false, false, true);
        emit GasTracked(0, 1000, tx.gasprice);

        gasCounter.trackGasPublic(1000);

        assertTrue(gasCounter.isGasTrackingInitialized());
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 1000);
    }

    function test_TrackGas_AccumulatesInCurrentPeriod() public {
        gasCounter.initializeGasTrackingPublic();

        gasCounter.trackGasPublic(1000);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 1000);

        gasCounter.trackGasPublic(2000);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 3000);

        gasCounter.trackGasPublic(500);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 3500);
    }

    function test_TrackGasUsageModifier() public {
        vm.expectEmit(true, false, false, true);
        emit NewPeriodStarted(0, block.timestamp);

        // The modifier should track gas automatically
        gasCounter.useTrackGasUsageModifier();

        assertTrue(gasCounter.isGasTrackingInitialized());
        assertGt(gasCounter.getCurrentPeriodGasUsed(), 0);
    }

    // ============ PERIOD MANAGEMENT TESTS ============

    function test_PeriodAdvancement() public {
        gasCounter.initializeGasTrackingPublic();
        gasCounter.trackGasPublic(1000);

        // Fast forward past period duration
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);

        vm.expectEmit(true, false, false, true);
        emit PeriodFinalized(0, 1000, gasCounter.PERIOD_DURATION());

        vm.expectEmit(true, false, false, true);
        emit NewPeriodStarted(1, block.timestamp);

        gasCounter.trackGasPublic(2000);

        // Should be in new period now
        assertEq(gasCounter.currentPeriodIndex(), 1);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 2000);
        assertEq(gasCounter.getTotalPeriods(), 2);

        // Check previous period was finalized
        GasCounter.GasPeriod memory previousPeriod = gasCounter.getPeriod(0);
        assertGt(previousPeriod.endTimestamp, 0); // Finalized
        assertEq(previousPeriod.totalGasUsed, 1000);
        assertGt(previousPeriod.endTimestamp, 0);
    }

    function test_MultiplePeriodAdvancements() public {
        gasCounter.initializeGasTrackingPublic();

        // Track gas in first period
        gasCounter.trackGasPublic(1000);

        // Advance to second period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        gasCounter.trackGasPublic(2000);

        // Advance to third period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        gasCounter.trackGasPublic(3000);

        assertEq(gasCounter.currentPeriodIndex(), 2);
        assertEq(gasCounter.getTotalPeriods(), 3);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 3000);

        // Check all periods
        assertEq(gasCounter.getPeriod(0).totalGasUsed, 1000);
        assertEq(gasCounter.getPeriod(1).totalGasUsed, 2000);
        assertEq(gasCounter.getPeriod(2).totalGasUsed, 3000);

        assertGt(gasCounter.getPeriod(0).endTimestamp, 0); // Finalized
        assertGt(gasCounter.getPeriod(1).endTimestamp, 0); // Finalized
        assertEq(gasCounter.getPeriod(2).endTimestamp, 0); // Not finalized
    }

    // ============ VIEW FUNCTIONS TESTS ============

    function test_GetCurrentPeriodTimeRemaining() public {
        gasCounter.initializeGasTrackingPublic();

        // Should have close to full period remaining
        uint256 timeRemaining = gasCounter.getCurrentPeriodTimeRemaining();
        assertEq(timeRemaining, gasCounter.PERIOD_DURATION());

        // Fast forward half the period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() / 2);
        timeRemaining = gasCounter.getCurrentPeriodTimeRemaining();
        assertEq(timeRemaining, gasCounter.PERIOD_DURATION() / 2);

        // Fast forward exactly to period end (now we're in a conceptual new period)
        uint256 periodEnd = 1 + gasCounter.PERIOD_DURATION();
        vm.warp(periodEnd);
        timeRemaining = gasCounter.getCurrentPeriodTimeRemaining();
        // Due to conceptual current period logic, we're now at the start of a new period
        assertEq(timeRemaining, gasCounter.PERIOD_DURATION());
    }

    function test_GetTotalGasFees() public {
        gasCounter.initializeGasTrackingPublic();

        // Track some gas
        gasCounter.trackGasPublic(1000);

        uint256 expectedFees = 1000 * tx.gasprice;
        assertEq(gasCounter.getTotalGasFees(), expectedFees);

        // Track more gas
        gasCounter.trackGasPublic(500);
        expectedFees = 1500 * tx.gasprice;
        assertEq(gasCounter.getTotalGasFees(), expectedFees);
    }

    function test_GetTotalGasFees_WithCustomGasPrice() public {
        gasCounter.initializeGasTrackingPublic();

        // Set custom gas price for this transaction
        vm.txGasPrice(2e12); // 2000 gwei
        gasCounter.trackGasPublic(1000);

        uint256 expectedFees = 1000 * 2e12;
        assertEq(gasCounter.getTotalGasFees(), expectedFees);
    }

    function test_GetCurrentPeriod_Uninitialized() public view {
        GasCounter.GasPeriod memory period = gasCounter.getCurrentPeriod();
        assertEq(period.startTimestamp, 0);
        assertEq(period.endTimestamp, 0);
        assertEq(period.totalGasUsed, 0);
        assertEq(period.endTimestamp, 0); // Not finalized
    }

    // ============ GAS PRICE MANAGEMENT TESTS ============

    function test_DynamicGasPrice() public {
        gasCounter.initializeGasTrackingPublic();
        
        // Set a specific gas price
        vm.txGasPrice(5e12);
        gasCounter.trackGasPublic(1000);
        
        uint256 expectedFees = 1000 * 5e12;
        assertEq(gasCounter.getTotalGasFees(), expectedFees);
    }

    function test_UpdateGasPrice_AffectsFutureCalculations() public {
        gasCounter.initializeGasTrackingPublic();
        gasCounter.trackGasPublic(1000);

        uint256 feesWithOldPrice = gasCounter.getTotalGasFees();

        // Track more gas with different gas price
        vm.txGasPrice(tx.gasprice * 2); // Double the gas price
        gasCounter.trackGasPublic(1000);

        // Total should be old fees + new fees at higher price
        uint256 expectedNewFees = feesWithOldPrice + (1000 * tx.gasprice);
        assertEq(gasCounter.getTotalGasFees(), expectedNewFees);
    }

    // ============ EDGE CASES AND FUZZ TESTS ============

    function test_ZeroGasTracking() public {
        gasCounter.initializeGasTrackingPublic();

        gasCounter.trackGasPublic(0);

        assertEq(gasCounter.getCurrentPeriodGasUsed(), 0);
        assertEq(gasCounter.getTotalGasFees(), 0);
    }

    function testFuzz_GasTracking(uint256 gasAmount) public {
        // Bound gas amount to reasonable values
        gasAmount = bound(gasAmount, 0, 1e9);

        gasCounter.initializeGasTrackingPublic();
        gasCounter.trackGasPublic(gasAmount);

        assertEq(gasCounter.getCurrentPeriodGasUsed(), gasAmount);

        uint256 expectedFees = gasAmount * tx.gasprice;
        assertEq(gasCounter.getTotalGasFees(), expectedFees);
    }

    function testFuzz_MultipleGasTracking(uint256[] memory gasAmounts) public {
        // Limit array size and bound values
        vm.assume(gasAmounts.length <= 10);

        gasCounter.initializeGasTrackingPublic();

        uint256 totalExpected = 0;
        for (uint256 i = 0; i < gasAmounts.length; i++) {
            gasAmounts[i] = bound(gasAmounts[i], 0, 1e8);
            gasCounter.trackGasPublic(gasAmounts[i]);
            totalExpected += gasAmounts[i];
        }

        assertEq(gasCounter.getCurrentPeriodGasUsed(), totalExpected);
    }

    function testFuzz_GasPriceUpdate(uint256 gasPrice) public {
        // Bound to reasonable gas price values
        gasPrice = bound(gasPrice, 1, 1e18);

        gasCounter.initializeGasTrackingPublic();
        
        // Set custom gas price
        vm.txGasPrice(gasPrice);
        gasCounter.trackGasPublic(1000);

        uint256 expectedFees = 1000 * gasPrice;
        assertEq(gasCounter.getTotalGasFees(), expectedFees);
    }

    // ============ INTEGRATION TESTS ============

    function test_CompleteWorkflow() public {
        // Initialize and track gas in first period
        gasCounter.initializeGasTrackingPublic();
        gasCounter.trackGasPublic(1000);
        gasCounter.trackGasPublic(2000);

        uint256 firstPeriodGas = gasCounter.getCurrentPeriodGasUsed();
        assertEq(firstPeriodGas, 3000);

        // Advance to second period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        gasCounter.trackGasPublic(5000);

        // Check period advancement worked
        assertEq(gasCounter.currentPeriodIndex(), 1);
        assertEq(gasCounter.getCurrentPeriodGasUsed(), 5000);
        assertEq(gasCounter.getTotalPeriods(), 2);

        // Check previous period was finalized correctly
        GasCounter.GasPeriod memory previousPeriod = gasCounter.getPeriod(0);
        assertGt(previousPeriod.endTimestamp, 0); // Finalized
        assertEq(previousPeriod.totalGasUsed, 3000);

        // Check current period is active
        GasCounter.GasPeriod memory currentPeriod = gasCounter.getCurrentPeriod();
        assertEq(currentPeriod.endTimestamp, 0); // Not finalized
        assertEq(currentPeriod.totalGasUsed, 5000);
    }

    function test_PeriodConstants() public view {
        assertEq(gasCounter.PERIOD_DURATION(), 30 days);
        // Gas price is now dynamic based on tx.gasprice
    }

    // ============ CUMULATIVE GAS FEES TESTS ============

    function test_CumulativeGasFees_InitiallyZero() public view {
        assertEq(gasCounter.getCumulativeGasFees(), 0);
        assertEq(gasCounter.cumulativeGasFees(), 0);
    }

    function test_CumulativeGasFees_SinglePeriod() public {
        gasCounter.initializeGasTrackingPublic();
        
        // Track gas in current period
        gasCounter.trackGasPublic(1000);
        uint256 expectedFees = 1000 * tx.gasprice;
        
        // Cumulative should include current period (no completed periods yet)
        assertEq(gasCounter.getCumulativeGasFees(), expectedFees);
        assertEq(gasCounter.cumulativeGasFees(), 0); // No completed periods
        
        // Track more gas
        gasCounter.trackGasPublic(500);
        expectedFees = 1500 * tx.gasprice;
        assertEq(gasCounter.getCumulativeGasFees(), expectedFees);
        assertEq(gasCounter.cumulativeGasFees(), 0);
    }

    function test_CumulativeGasFees_MultiplePeriods() public {
        gasCounter.initializeGasTrackingPublic();
        
        // Track gas in first period
        gasCounter.trackGasPublic(1000);
        uint256 firstPeriodFees = 1000 * tx.gasprice;
        
        // Advance to second period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        gasCounter.trackGasPublic(2000);
        uint256 secondPeriodFees = 2000 * tx.gasprice;
        
        // Now cumulative should include completed first period + current second period
        assertEq(gasCounter.cumulativeGasFees(), firstPeriodFees); // Only completed periods
        assertEq(gasCounter.getCumulativeGasFees(), firstPeriodFees + secondPeriodFees); // All periods
        
        // Advance to third period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        gasCounter.trackGasPublic(3000);
        uint256 thirdPeriodFees = 3000 * tx.gasprice;
        
        // Now cumulative should include first + second completed periods + current third period
        assertEq(gasCounter.cumulativeGasFees(), firstPeriodFees + secondPeriodFees); // Two completed periods
        assertEq(gasCounter.getCumulativeGasFees(), firstPeriodFees + secondPeriodFees + thirdPeriodFees); // All periods
    }

    function test_CumulativeGasFees_WithDifferentGasPrices() public {
        gasCounter.initializeGasTrackingPublic();
        
        // Track gas with first gas price
        vm.txGasPrice(1e12); // 1000 gwei
        gasCounter.trackGasPublic(1000);
        uint256 firstPeriodFees = 1000 * 1e12;
        
        // Advance to second period with different gas price
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        vm.txGasPrice(2e12); // 2000 gwei
        gasCounter.trackGasPublic(1000);
        uint256 secondPeriodFees = 1000 * 2e12;
        
        assertEq(gasCounter.cumulativeGasFees(), firstPeriodFees);
        assertEq(gasCounter.getCumulativeGasFees(), firstPeriodFees + secondPeriodFees);
    }

    function test_GetGasFeesInRange_ValidRange() public view {
        uint256 startCumulative = 1000;
        uint256 endCumulative = 5000;
        uint256 expected = 4000;
        
        uint256 result = gasCounter.getGasFeesInRange(startCumulative, endCumulative);
        assertEq(result, expected);
    }

    function test_GetGasFeesInRange_ZeroRange() public view {
        uint256 cumulative = 1000;
        uint256 result = gasCounter.getGasFeesInRange(cumulative, cumulative);
        assertEq(result, 0);
    }

    function test_GetGasFeesInRange_RevertsOnInvalidRange() public {
        uint256 startCumulative = 5000;
        uint256 endCumulative = 1000;
        
        vm.expectRevert("GasCounter: invalid range");
        gasCounter.getGasFeesInRange(startCumulative, endCumulative);
    }

    function test_CumulativeGasFees_IntegrationWorkflow() public {
        gasCounter.initializeGasTrackingPublic();
        
        // Snapshot at start
        uint256 startSnapshot = gasCounter.getCumulativeGasFees();
        assertEq(startSnapshot, 0);
        
        // Track gas in first period
        gasCounter.trackGasPublic(1000);
        gasCounter.trackGasPublic(500);
        uint256 firstPeriodTotal = 1500 * tx.gasprice;
        
        // Mid-period snapshot
        uint256 midSnapshot = gasCounter.getCumulativeGasFees();
        assertEq(midSnapshot, firstPeriodTotal);
        
        // Advance to second period
        vm.warp(block.timestamp + gasCounter.PERIOD_DURATION() + 1);
        gasCounter.trackGasPublic(2000);
        uint256 secondPeriodFees = 2000 * tx.gasprice;
        
        // End snapshot
        uint256 endSnapshot = gasCounter.getCumulativeGasFees();
        assertEq(endSnapshot, firstPeriodTotal + secondPeriodFees);
        
        // Calculate fees for the entire duration
        uint256 totalFees = gasCounter.getGasFeesInRange(startSnapshot, endSnapshot);
        assertEq(totalFees, firstPeriodTotal + secondPeriodFees);
        
        // Calculate fees just for the second period
        uint256 secondPeriodOnlyFees = gasCounter.getGasFeesInRange(midSnapshot, endSnapshot);
        assertEq(secondPeriodOnlyFees, secondPeriodFees);
    }

    function testFuzz_CumulativeGasFees(uint256[] memory gasAmounts) public {
        // Limit array size and bound values
        vm.assume(gasAmounts.length <= 5);
        vm.assume(gasAmounts.length > 0);
        
        gasCounter.initializeGasTrackingPublic();
        
        uint256 expectedTotal = 0;
        for (uint256 i = 0; i < gasAmounts.length; i++) {
            gasAmounts[i] = bound(gasAmounts[i], 1, 1e7);
            gasCounter.trackGasPublic(gasAmounts[i]);
            expectedTotal += gasAmounts[i];
        }
        
        uint256 expectedFees = expectedTotal * tx.gasprice;
        assertEq(gasCounter.getCumulativeGasFees(), expectedFees);
        
        // No periods completed yet, so cumulative should be 0
        assertEq(gasCounter.cumulativeGasFees(), 0);
    }

    function testFuzz_GetGasFeesInRange(uint256 start, uint256 end) public view {
        // Ensure valid range
        vm.assume(end >= start);
        vm.assume(start <= type(uint256).max - end); // Avoid overflow
        
        uint256 result = gasCounter.getGasFeesInRange(start, end);
        assertEq(result, end - start);
    }
}
