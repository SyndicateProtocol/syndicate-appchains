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
        uint256 totalGasCost; // Total gas cost in SYND wei for this period
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Duration of each tracking period (30 days)
    uint256 public constant PERIOD_DURATION = 30 days;
    uint256 public constant TRACKING_OVERHEAD = 5000; // Approximate gas overhead for tracking operations


    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Index of the current active period
    uint256 public currentPeriodIndex;

    /// @notice Whether gas tracking has been initialized
    bool public gasTrackingInitialized;

    /// @notice Whether gas tracking is enabled (can be disabled for formal verification)
    bool public gasTrackingEnabled = true;

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
        if (!gasTrackingEnabled) {
            _;
            return;
        }

        uint256 gasStart = gasleft();
        _;
        uint256 gasUsed = gasStart - gasleft();

        // Add some gas for the tracking operations themselves
        gasUsed += TRACKING_OVERHEAD; // Approximate gas for tracking operations

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

            periods[0] = GasPeriod({
            startTimestamp: block.timestamp,
            endTimestamp: 0,
            totalGasUsed: 0,
            totalGasCost: 0
        });

            emit NewPeriodStarted(0, block.timestamp);
        }
    }

    /// @notice Internal function to track gas usage
    /// @param gasUsed Amount of gas consumed
    function _trackGas(uint256 gasUsed) internal {
        if (!gasTrackingEnabled) {
            return;
        }

        // Initialize if not already done
        if (!gasTrackingInitialized) {
            _initializeGasTracking();
        }

        // Check if current period has expired and advance if needed
        _advancePeriodIfNeeded();

        // Calculate gas cost using current transaction gas price
        uint256 gasCost = gasUsed * tx.gasprice;
        
        // Add gas and cost to current period
        periods[currentPeriodIndex].totalGasUsed += gasUsed;
        periods[currentPeriodIndex].totalGasCost += gasCost;

        emit GasTracked(currentPeriodIndex, gasUsed, tx.gasprice);
    }

    /// @notice Advances to a new period if the current one has expired
    function _advancePeriodIfNeeded() internal {
        GasPeriod storage currentPeriod = periods[currentPeriodIndex];

        // Check if current period has expired
        if (block.timestamp >= currentPeriod.startTimestamp + PERIOD_DURATION) {
            // Finalize current period
            currentPeriod.endTimestamp = currentPeriod.startTimestamp + PERIOD_DURATION;

            emit PeriodFinalized(
                currentPeriodIndex,
                currentPeriod.totalGasUsed,
                currentPeriod.endTimestamp - currentPeriod.startTimestamp
            );

            // Start new period
            currentPeriodIndex++;
            periods[currentPeriodIndex] = GasPeriod({
                startTimestamp: block.timestamp,
                endTimestamp: 0,
                totalGasUsed: 0,
                totalGasCost: 0
            });

            emit NewPeriodStarted(currentPeriodIndex, block.timestamp);
        }
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Internal helper to get the conceptual current period
    /// @dev Handles case where stored current period has expired due to inactivity
    /// @return period The conceptual current period
    function _getConceptualCurrentPeriod() private view returns (GasPeriod memory period) {
        if (!gasTrackingInitialized) {
            return GasPeriod(0, 0, 0, 0);
        }
        
        GasPeriod memory storedPeriod = periods[currentPeriodIndex];
        
        // Check if the stored current period has expired
        if (block.timestamp >= storedPeriod.startTimestamp + PERIOD_DURATION) {
            // The stored period has expired, return a conceptual current period
            // Calculate when the actual current period would start
            uint256 periodsElapsed = (block.timestamp - storedPeriod.startTimestamp) / PERIOD_DURATION;
            uint256 currentPeriodStart = storedPeriod.startTimestamp + (periodsElapsed * PERIOD_DURATION);
            
            return GasPeriod({
                startTimestamp: currentPeriodStart,
                endTimestamp: 0, // Not finalized
                totalGasUsed: 0, // No activity yet
                totalGasCost: 0  // No cost yet
            });
        }
        
        return storedPeriod;
    }

    /// @notice Get the current period's gas usage data
    /// @dev If the stored current period has expired due to inactivity, returns a conceptual current period
    /// @return period The current gas period data
    function getCurrentPeriod() external view returns (GasPeriod memory period) {
        return _getConceptualCurrentPeriod();
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
        return _getConceptualCurrentPeriod().totalGasUsed;
    }

    /// @notice Get total gas cost in SYND wei for current period
    /// @return totalCost Total cost in SYND wei for current period's gas usage
    function getTotalGasFees() external view returns (uint256 totalCost) {
        return _getConceptualCurrentPeriod().totalGasCost;
    }

    /// @notice Get the time remaining in the current period
    /// @return timeRemaining Seconds remaining in current period (0 if expired)
    function getCurrentPeriodTimeRemaining() external view returns (uint256 timeRemaining) {
        GasPeriod memory currentPeriod = _getConceptualCurrentPeriod();
        
        if (currentPeriod.startTimestamp == 0) {
            return 0; // Not initialized
        }
        
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

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disable gas tracking if needed
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _disableGasTracking() internal {
        gasTrackingEnabled = false;
    }

    /// @notice Enable gas tracking
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _enableGasTracking() internal {
        gasTrackingEnabled = true;
    }
}
