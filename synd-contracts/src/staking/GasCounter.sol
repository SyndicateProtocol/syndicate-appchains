// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";

/**
 * @title GasCounter
 * @notice Tracks gas consumption over 30-day epoch for reward calculation
 * @dev This contract provides gas tracking functionality that can be inherited by sequencing contracts
 * @author Syndicate Protocol
 */
abstract contract GasCounter is EpochTracker {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Whether gas tracking is enabled
    bool public gasTrackingEnabled = true;

    /// @notice Mapping of epoch to gas data
    mapping(uint256 => uint256) public tokensUsedPerEpoch;

    /*//////////////////////////////////////////////////////////////
                ERRORS
    //////////////////////////////////////////////////////////////*/
    error GasTrackingAlreadyEnabled();
    error GasTrackingAlreadyDisabled();

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

        _trackGas(gasUsed);
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Internal function to track gas usage
    /// @param gasUsed Amount of gas consumed
    function _trackGas(uint256 gasUsed) internal {
        uint256 currentEpoch = getCurrentEpoch();

        // Calculate gas cost using current transaction gas price
        uint256 gasPrice = tx.gasprice;

        // WORKAROUND: estimate gas will give a wrong value when called with tx.gasprice 0
        if (gasPrice == 0) {
            gasPrice = 1;
        }

        // Add gas and cost to current epoch
        tokensUsedPerEpoch[currentEpoch] += gasUsed * gasPrice;
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice get the gas usage for a given epoch
    /// @param epoch The epoch to query
    function getTokensForEpoch(uint256 epoch) external view returns (uint256) {
        return tokensUsedPerEpoch[epoch];
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disable gas tracking if needed
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _disableGasTracking() internal {
        if (gasTrackingEnabled == false) {
            revert GasTrackingAlreadyDisabled();
        }
        gasTrackingEnabled = false;
    }

    /// @notice Enable gas tracking
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _enableGasTracking() internal {
        if (gasTrackingEnabled == true) {
            revert GasTrackingAlreadyEnabled();
        }
        gasTrackingEnabled = true;
    }
}
