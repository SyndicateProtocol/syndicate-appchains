// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title GasTracker
 * @notice Centralized gas tracking for multiple appchains across 30-day periods
 * @dev Tracks gas consumption per appchain and provides aggregate views
 * @author Syndicate Protocol
 */
contract GasTracker {
    /*//////////////////////////////////////////////////////////////
                                STRUCTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Represents a 30-day gas tracking period for a specific appchain
    struct AppchainGasPeriod {
        uint256 startTimestamp; // When the period started
        uint256 endTimestamp; // When the period ended (0 if current)
        uint256 totalGasUsed; // Total gas consumed in this period
        uint256 totalGasCost; // Total gas cost in SYND wei for this period
    }

    /// @notice Aggregate gas data across all appchains for a period
    struct AggregateGasPeriod {
        uint256 totalGasUsed; // Total gas across all appchains
        uint256 totalGasCost; // Total cost across all appchains
        uint256 activeAppchains; // Number of appchains that had activity
    }

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Duration of each tracking period (30 days)
    uint256 public constant PERIOD_DURATION = 30 days;

    /// @notice Global epoch counter (increments every 30 days)
    uint256 public currentEpoch;

    /// @notice When the current epoch started
    uint256 public currentEpochStart;

    /// @notice Whether gas tracking has been initialized
    bool public gasTrackingInitialized;

    /// @notice Whether gas tracking is enabled
    bool public gasTrackingEnabled = true;

    /// @notice Mapping of epoch => appchainId => gas period data
    mapping(uint256 => mapping(uint256 => AppchainGasPeriod)) public appchainPeriods;

    /// @notice Mapping of epoch => aggregate gas data
    mapping(uint256 => AggregateGasPeriod) public aggregatePeriods;

    /// @notice Cumulative gas fees across all completed epochs for each appchain
    mapping(uint256 => uint256) public cumulativeGasFeesByAppchain;

    /// @notice Total cumulative gas fees across all appchains and epochs
    uint256 public totalCumulativeGasFees;

    /// @notice Set of appchain IDs that have been active (for enumeration)
    mapping(uint256 => bool) public knownAppchains;
    uint256[] public appchainList;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when gas is tracked for an appchain
    /// @param epoch The epoch in which gas was tracked
    /// @param appchainId The appchain that consumed gas
    /// @param gasUsed Amount of gas consumed
    /// @param gasPrice Gas price used for the transaction
    event GasTracked(uint256 indexed epoch, uint256 indexed appchainId, uint256 gasUsed, uint256 gasPrice);

    /// @notice Emitted when an epoch is finalized
    /// @param epoch The epoch that was finalized
    /// @param totalGasUsed Total gas consumed during the epoch across all appchains
    /// @param activeAppchains Number of appchains that had activity
    event EpochFinalized(uint256 indexed epoch, uint256 totalGasUsed, uint256 activeAppchains);

    /// @notice Emitted when a new epoch starts
    /// @param epoch The new epoch number
    /// @param startTimestamp When the epoch started
    event NewEpochStarted(uint256 indexed epoch, uint256 startTimestamp);

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    error GasTrackingDisabled();
    error InvalidAppchainId();
    error InvalidGasAmount();

    /*//////////////////////////////////////////////////////////////
                        INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Initializes gas tracking with the first epoch
    function _initializeGasTracking() internal {
        if (!gasTrackingInitialized) {
            gasTrackingInitialized = true;
            currentEpoch = 0;
            currentEpochStart = block.timestamp;

            emit NewEpochStarted(0, block.timestamp);
        }
    }

    /// @notice Advances to a new epoch if the current one has expired
    function _advanceEpochIfNeeded() internal {
        if (block.timestamp >= currentEpochStart + PERIOD_DURATION) {
            // Finalize current epoch
            AggregateGasPeriod storage currentAggregate = aggregatePeriods[currentEpoch];

            // Finalize all active appchain periods for current epoch
            uint256 epochEndTime = currentEpochStart + PERIOD_DURATION;
            for (uint256 i = 0; i < appchainList.length; i++) {
                uint256 appchainId = appchainList[i];
                AppchainGasPeriod storage appchainPeriod = appchainPeriods[currentEpoch][appchainId];
                if (appchainPeriod.startTimestamp > 0 && appchainPeriod.endTimestamp == 0) {
                    appchainPeriod.endTimestamp = epochEndTime;
                    // Add to cumulative for this appchain
                    cumulativeGasFeesByAppchain[appchainId] += appchainPeriod.totalGasCost;
                }
            }

            // Add completed epoch's total cost to cumulative
            totalCumulativeGasFees += currentAggregate.totalGasCost;

            emit EpochFinalized(currentEpoch, currentAggregate.totalGasUsed, currentAggregate.activeAppchains);

            // Start new epoch
            currentEpoch++;
            currentEpochStart = block.timestamp;

            emit NewEpochStarted(currentEpoch, block.timestamp);
        }
    }

    /// @notice Adds an appchain to the known list if not already present
    function _registerAppchain(uint256 appchainId) internal {
        if (!knownAppchains[appchainId]) {
            knownAppchains[appchainId] = true;
            appchainList.push(appchainId);
        }
    }

    /*//////////////////////////////////////////////////////////////
                        EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Tracks gas usage for a specific appchain
    /// @param appchainId The appchain that consumed gas
    /// @param gasUsed Amount of gas consumed
    function trackGas(uint256 appchainId, uint256 gasUsed) public {
        if (!gasTrackingEnabled) revert GasTrackingDisabled();
        if (appchainId == 0) revert InvalidAppchainId();
        if (gasUsed == 0) revert InvalidGasAmount();

        // Initialize if not already done
        if (!gasTrackingInitialized) {
            _initializeGasTracking();
        }

        // Check if current epoch has expired and advance if needed
        _advanceEpochIfNeeded();

        // Register appchain if new
        _registerAppchain(appchainId);

        // Calculate gas cost using current transaction gas price
        uint256 gasPrice = tx.gasprice;
        if (gasPrice == 0) {
            gasPrice = 1; // Workaround for estimate gas calls
        }
        uint256 gasCost = gasUsed * gasPrice;

        // Get or initialize appchain period for current epoch
        AppchainGasPeriod storage appchainPeriod = appchainPeriods[currentEpoch][appchainId];
        if (appchainPeriod.startTimestamp == 0) {
            appchainPeriod.startTimestamp = currentEpochStart;
        }

        // Update appchain-specific tracking
        appchainPeriod.totalGasUsed += gasUsed;
        appchainPeriod.totalGasCost += gasCost;

        // Update aggregate tracking
        AggregateGasPeriod storage aggregate = aggregatePeriods[currentEpoch];

        aggregate.totalGasUsed += gasUsed;
        aggregate.totalGasCost += gasCost;

        // Track unique active appchains this epoch
        if (appchainPeriod.totalGasUsed == gasUsed) {
            // First activity for this appchain this epoch
            aggregate.activeAppchains++;
        }

        emit GasTracked(currentEpoch, appchainId, gasUsed, gasPrice);
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Get aggregate gas data for a specific epoch
    /// @param epoch The epoch to query
    /// @return aggregate The aggregate gas data
    function getEpochAggregate(uint256 epoch) external view returns (AggregateGasPeriod memory aggregate) {
        return aggregatePeriods[epoch];
    }

    /// @notice Get gas data for a specific appchain in a specific epoch
    /// @param epoch The epoch to query
    /// @param appchainId The appchain to query
    /// @return period The appchain gas period data
    function getAppchainEpochData(uint256 epoch, uint256 appchainId)
        external
        view
        returns (AppchainGasPeriod memory period)
    {
        return appchainPeriods[epoch][appchainId];
    }

    /// @notice Get current epoch aggregate gas data
    /// @return aggregate Current epoch aggregate data
    function getCurrentEpochAggregate() external view returns (AggregateGasPeriod memory aggregate) {
        return aggregatePeriods[currentEpoch];
    }

    /// @notice Get gas data for a specific appchain in the current epoch
    /// @param appchainId The appchain to query
    /// @return period Current epoch data for the appchain
    function getCurrentAppchainData(uint256 appchainId) external view returns (AppchainGasPeriod memory period) {
        return appchainPeriods[currentEpoch][appchainId];
    }

    /// @notice Get all known appchain IDs
    /// @return appchains Array of appchain IDs that have been active
    function getKnownAppchains() external view returns (uint256[] memory appchains) {
        return appchainList;
    }

    /// @notice Get total cumulative gas fees for a specific appchain
    /// @param appchainId The appchain to query
    /// @return totalFees Cumulative fees across all completed epochs plus current epoch
    function getAppchainCumulativeFees(uint256 appchainId) external view returns (uint256 totalFees) {
        return cumulativeGasFeesByAppchain[appchainId] + appchainPeriods[currentEpoch][appchainId].totalGasCost;
    }

    /// @notice Get total cumulative gas fees across all appchains
    /// @return totalFees Total cumulative fees
    function getTotalCumulativeFees() external view returns (uint256 totalFees) {
        return totalCumulativeGasFees + aggregatePeriods[currentEpoch].totalGasCost;
    }

    /// @notice Get time remaining in current epoch
    /// @return timeRemaining Seconds remaining (0 if expired)
    function getCurrentEpochTimeRemaining() external view returns (uint256 timeRemaining) {
        if (!gasTrackingInitialized) return 0;

        uint256 epochEnd = currentEpochStart + PERIOD_DURATION;
        if (block.timestamp >= epochEnd) return 0;

        return epochEnd - block.timestamp;
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disable gas tracking
    function _disableGasTracking() internal {
        gasTrackingEnabled = false;
    }

    /// @notice Enable gas tracking
    function _enableGasTracking() internal {
        gasTrackingEnabled = true;
    }
}
