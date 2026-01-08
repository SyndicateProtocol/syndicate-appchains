// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";

/// @custom:storage-location erc7201:syndicate.storage.GasCounter
struct GasCounterStorage {
    /// @notice Whether gas tracking is enabled
    bool gasTrackingEnabled;
    /// @notice Mapping of epoch to gas data
    mapping(uint256 => uint256) tokensUsedPerEpoch;
}

/**
 * @title GasCounter
 * @notice Tracks gas consumption over 30-day epochs for reward calculation
 * @dev This contract provides gas tracking functionality that can be inherited by sequencing contracts.
 *      It automatically tracks gas usage and converts it to token costs for reward distribution.
 */
abstract contract GasCounter is EpochTracker {
    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLE VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    // cast index-erc7201 syndicate.storage.GasCounter
    bytes32 public constant GAS_COUNTER_STORAGE_LOCATION =
        0x119494e47c2426a6072fc6072ec5c5d5ae865a3372fd102c643c18e978b14800;

    function _getGasCounterStorage() internal pure returns (GasCounterStorage storage $) {
        assembly {
            $.slot := GAS_COUNTER_STORAGE_LOCATION
        }
    }

    function gasTrackingEnabled() public view returns (bool) {
        GasCounterStorage storage $ = _getGasCounterStorage();
        return $.gasTrackingEnabled;
    }

    function tokensUsedPerEpoch(uint256 epoch) public view returns (uint256) {
        GasCounterStorage storage $ = _getGasCounterStorage();
        return $.tokensUsedPerEpoch[epoch];
    }

    /*//////////////////////////////////////////////////////////////
                ERRORS
    //////////////////////////////////////////////////////////////*/
    error GasTrackingAlreadyEnabled();
    error GasTrackingAlreadyDisabled();

    /*//////////////////////////////////////////////////////////////
                              MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Modifier that tracks gas usage for a function call
    /// @dev Automatically measures gas consumption and converts to token cost
    ///      Gas tracking can be disabled by setting gasTrackingDisabled to true
    modifier trackGasUsage() {
        if (!gasTrackingEnabled()) {
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

        // Add gas and cost to current epoch
        _getGasCounterStorage().tokensUsedPerEpoch[currentEpoch] += gasUsed * gasPrice;
    }

    /*//////////////////////////////////////////////////////////////
                           VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice get the gas usage for a given epoch
    /// @param epoch The epoch to query
    function getTokensForEpoch(uint256 epoch) external view returns (uint256) {
        return tokensUsedPerEpoch(epoch);
    }

    /*//////////////////////////////////////////////////////////////
                         ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Disable gas tracking if needed
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _disableGasTracking() internal {
        if (gasTrackingEnabled() == false) {
            revert GasTrackingAlreadyDisabled();
        }
        _getGasCounterStorage().gasTrackingEnabled = false;
    }

    /// @notice Enable gas tracking
    /// @dev This is an internal function that should be exposed by inheriting contracts with proper access control
    function _enableGasTracking() internal {
        if (gasTrackingEnabled() == true) {
            revert GasTrackingAlreadyEnabled();
        }
        _getGasCounterStorage().gasTrackingEnabled = true;
    }
}
