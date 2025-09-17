// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

// TODO: Reconcile with IGasProvider interface when contract is ready
// Current work in progress https://github.com/SyndicateProtocol/syndicate-appchains/pull/780
interface IGasDataProvider {
    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256);
    function getTotalGasFees(uint256 epochIndex) external view returns (uint256);
    function getActiveAppchainIds(uint256 epochIndex) external view returns (uint256[] memory _chainIDs);
    function getAppchainRewardsReceiver(uint256 appchainId) external view returns (address);
}
