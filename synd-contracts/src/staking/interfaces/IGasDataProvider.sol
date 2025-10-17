// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title IGasDataProvider
/// @notice Interface for providing gas usage data and appchain information for reward calculations
/// @dev This interface is used by reward pools to access gas fee data and appchain configurations
interface IGasDataProvider {
    /**
     * @notice Get the gas fees paid by a specific appchain in a given epoch
     * @param epochIndex The epoch index to query
     * @param appchainId The ID of the appchain
     * @return The amount of gas fees paid by the appchain in the specified epoch
     */
    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256);

    /**
     * @notice Get the total gas fees across all appchains in a given epoch
     * @param epochIndex The epoch index to query
     * @return The total amount of gas fees paid across all appchains in the specified epoch
     */
    function getTotalGasFees(uint256 epochIndex) external view returns (uint256);

    /**
     * @notice Get the list of active appchain IDs for a given epoch
     * @param epochIndex The epoch index to query
     * @return _chainIDs Array of appchain IDs that were active in the specified epoch
     */
    function getAppchainIds(uint256 epochIndex) external view returns (uint256[] memory _chainIDs);

    function getAppchainCount(uint256 epochIndex) external view returns (uint256);

    // when count is zero, returns the full list of results
    // count can be greater than the number of appchains that exist
    function getAppchainInfo(uint256 epochIndex, uint256 startIndex, uint256 count)
        external
        view
        returns (uint256[] memory chainId, uint256[] memory gasUsed);
}
