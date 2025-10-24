// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";

/**
 * @title GasCounter
 * @notice Tracks gas consumption over 30-day epochs for reward calculation
 * @dev This contract provides gas tracking functionality that can be inherited by sequencing contracts.
 *      It automatically tracks gas usage and converts it to token costs for reward distribution.
 */
abstract contract GasCounter is EpochTracker {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Whether gas tracking is disabled
    /// @dev When true, gas tracking is bypassed for all functions using trackGasUsage modifier
    bool public gasTrackingDisabled;

    /// @notice Mapping from epoch index to total tokens used in gas fees for that epoch
    /// @dev Accumulates all gas costs (gasUsed * gasPrice) for each epoch
    mapping(uint256 epochIndex => uint256 tokensUsed) public tokensUsedPerEpoch;

    /*//////////////////////////////////////////////////////////////
                              MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Modifier that tracks gas usage for a function call
    /// @dev Automatically measures gas consumption and converts to token cost
    ///      Gas tracking can be disabled by setting gasTrackingDisabled to true
    modifier trackGasUsage() {
        if (gasTrackingDisabled) {
            _;
            return;
        }

        uint256 gasStart = gasleft();
        _;

        // workaround: certora thinks gasStart - gasleft() can underflow even though it is safe
        unchecked {
            _trackGas(gasStart - gasleft());
        }
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Internal function to track gas usage and accumulate costs
    /// @dev Converts gas usage to token cost using current gas price and adds to epoch total
    /// @param gasUsed Amount of gas consumed by the function call
    function _trackGas(uint256 gasUsed) internal {
        uint256 currentEpoch = getCurrentEpoch();

        // Calculate gas cost using current transaction gas price
        uint256 gasPrice = tx.gasprice;

        // WORKAROUND: estimate gas will give a wrong value when called with tx.gasprice 0
        // Use minimum price of 1 wei to ensure calculation doesn't fail
        if (gasPrice == 0) {
            gasPrice = 1;
        }

        // Add gas cost to current epoch total
        // Using unchecked for gas efficiency since gasUsed * gasPrice cannot realistically overflow
        unchecked {
            tokensUsedPerEpoch[currentEpoch] += gasUsed * gasPrice;
        }
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the total tokens used in gas fees for a given epoch
    /// @dev Returns the accumulated gas costs for the specified epoch
    /// @param epochIndex The epoch index to query
    /// @return The total tokens used in gas fees for the specified epoch
    /// @custom:example If epoch 1 had 1000 gas units at 20 gwei, returns 20000 wei
    function getTokensForEpoch(uint256 epochIndex) external view returns (uint256) {
        return tokensUsedPerEpoch[epochIndex];
    }
}
