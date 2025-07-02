// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title GasCounter
 * @notice Tracks gas consumption over 30-day periods for reward calculation
 * @dev This contract provides gas tracking functionality that can be inherited by sequencing contracts
 * @author Syndicate Protocol
 */
abstract contract GasCounter {
    /*//////////////////////////////////////////////////////////////
                                STRUCTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Represents a 30-day gas tracking period
    struct GasPeriod {
        uint256 startTimestamp; // When the period started
        uint256 endTimestamp; // When the period ended (0 if current)
        uint256 totalGasUsed; // Total gas consumed in this period
        bool finalized; // Whether period is closed
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Duration of each tracking period (30 days)
    uint256 public constant PERIOD_DURATION = 30 days;

    /// @notice Approximate gas price in SYND tokens (18 decimals)
    /// @dev hardcoded as 1 gwei
    uint256 public gasPriceInSynd = 1e9; // 1 gwei in SYND

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Index of the current active period
    uint256 public currentPeriodIndex;

    /// @notice Whether gas tracking has been initialized
    bool public gasTrackingInitialized;

    /// @notice Mapping of period index to gas period data
    mapping(uint256 => GasPeriod) public periods;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when gas is tracked for a transaction
    /// @param periodIndex The period in which gas was tracked
    /// @param gasUsed Amount of gas consumed
    /// @param gasPrice Gas price used for the transaction
    event GasTracked(uint256 indexed periodIndex, uint256 gasUsed, uint256 gasPrice);

    /// @notice Emitted when a period is finalized
    /// @param periodIndex The period that was finalized
    /// @param totalGasUsed Total gas consumed during the period
    /// @param duration Actual duration of the period
    event PeriodFinalized(uint256 indexed periodIndex, uint256 totalGasUsed, uint256 duration);

    /// @notice Emitted when a new period starts
    /// @param periodIndex The new period index
    /// @param startTimestamp When the period started
    event NewPeriodStarted(uint256 indexed periodIndex, uint256 startTimestamp);

    /*//////////////////////////////////////////////////////////////
                              MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Modifier that tracks gas usage for a function call
    modifier trackGasUsage() {
        uint256 gasStart = gasleft();
        _;
        uint256 gasUsed = gasStart - gasleft();

        // Add some gas for the tracking operations themselves
        gasUsed += 5000; // Approximate gas for tracking operations

        _trackGas(gasUsed);
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Initializes gas tracking with the first period
    function _initializeGasTracking() internal {
        if (!gasTrackingInitialized) {
            gasTrackingInitialized = true;
            currentPeriodIndex = 0;

            periods[0] =
                GasPeriod({startTimestamp: block.timestamp, endTimestamp: 0, totalGasUsed: 0, finalized: false});

            emit NewPeriodStarted(0, block.timestamp);
        }
    }

    /// @notice Internal function to track gas usage
    /// @param gasUsed Amount of gas consumed
    function _trackGas(uint256 gasUsed) internal {
        // Initialize if not already done
        if (!gasTrackingInitialized) {
            _initializeGasTracking();
        }

        // Check if current period has expired and advance if needed
        _advancePeriodIfNeeded();

        // Add gas to current period
        periods[currentPeriodIndex].totalGasUsed += gasUsed;

        emit GasTracked(currentPeriodIndex, gasUsed, tx.gasprice);
    }

    /// @notice Advances to a new period if the current one has expired
    function _advancePeriodIfNeeded() internal {
        GasPeriod storage currentPeriod = periods[currentPeriodIndex];

        // Check if current period has expired
        if (block.timestamp >= currentPeriod.startTimestamp + PERIOD_DURATION) {
            // Finalize current period
            currentPeriod.endTimestamp = currentPeriod.startTimestamp + PERIOD_DURATION;
            currentPeriod.finalized = true;

            emit PeriodFinalized(
                currentPeriodIndex,
                currentPeriod.totalGasUsed,
                currentPeriod.endTimestamp - currentPeriod.startTimestamp
            );

            // Start new period
            currentPeriodIndex++;
            periods[currentPeriodIndex] =
                GasPeriod({startTimestamp: block.timestamp, endTimestamp: 0, totalGasUsed: 0, finalized: false});

            emit NewPeriodStarted(currentPeriodIndex, block.timestamp);
        }
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the current period's gas usage data
    /// @return period The current gas period data
    function getCurrentPeriod() external view returns (GasPeriod memory period) {
        if (!gasTrackingInitialized) {
            return GasPeriod(0, 0, 0, false);
        }
        return periods[currentPeriodIndex];
    }

    /// @notice Get a specific period's gas usage data
    /// @param periodIndex The period to query
    /// @return period The gas period data
    function getPeriod(uint256 periodIndex) external view returns (GasPeriod memory period) {
        return periods[periodIndex];
    }

    /// @notice Get the total gas used in the current period
    /// @return totalGas Total gas consumed in current period
    function getCurrentPeriodGasUsed() external view returns (uint256 totalGas) {
        if (!gasTrackingInitialized) {
            return 0;
        }
        return periods[currentPeriodIndex].totalGasUsed;
    }

    /// @notice Calculate total SYND token cost for gas in current period
    /// @return totalCost Total cost in SYND wei for current period's gas usage
    function getTotalGasFees() external view returns (uint256 totalCost) {
        if (!gasTrackingInitialized) {
            return 0;
        }

        uint256 totalGasUsed = periods[currentPeriodIndex].totalGasUsed;
        return totalGasUsed * gasPriceInSynd;
    }

    /// @notice Get the time remaining in the current period
    /// @return timeRemaining Seconds remaining in current period (0 if expired)
    function getCurrentPeriodTimeRemaining() external view returns (uint256 timeRemaining) {
        if (!gasTrackingInitialized) {
            return 0;
        }

        GasPeriod memory currentPeriod = periods[currentPeriodIndex];
        uint256 periodEnd = currentPeriod.startTimestamp + PERIOD_DURATION;

        if (block.timestamp >= periodEnd) {
            return 0;
        }

        return periodEnd - block.timestamp;
    }

    /// @notice Check if gas tracking has been initialized
    /// @return initialized Whether gas tracking is active
    function isGasTrackingInitialized() external view returns (bool initialized) {
        return gasTrackingInitialized;
    }

    /// @notice Get the total number of periods (including current)
    /// @return totalPeriods Number of periods created
    function getTotalPeriods() external view returns (uint256 totalPeriods) {
        if (!gasTrackingInitialized) {
            return 0;
        }
        return currentPeriodIndex + 1;
    }
}
