// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title ILegacyAppchain
/// @notice Interface for legacy appchain contracts that need to be migrated
/// @dev Used to extract gas usage data from old appchain implementations during migration
interface ILegacyAppchain {
    /// @notice Get the total gas tokens used for a specific epoch
    /// @param epoch The epoch number to query
    /// @return The amount of gas tokens used in the specified epoch
    function getTokensForEpoch(uint256 epoch) external view returns (uint256);
}
