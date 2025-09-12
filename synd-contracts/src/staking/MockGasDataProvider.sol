// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";

contract MockGasDataProvider is IGasDataProvider {
    uint256 public constant DEFAULT_APPCHAIN_FEES = 250;
    uint256 public constant DEFAULT_TOTAL_FEES = 1000;
    uint256[] public DEFAULT_APPCHAIN_IDS = [510002, 510003, 510004, 510005];
    address public constant DEFAULT_REWARDS_RECEIVER = 0x4834a3c778F2d005B28a18c68d580Cc7F68c5Cbf;
    uint256[] public appchainIds;
    uint256 public totalFees;
    mapping(uint256 appchainId => uint256 appchainFees) public appchainFees;
    mapping(uint256 appchainId => address rewardsReceiver) public rewardsReceivers;

    function getAppchainGasFees(uint256, uint256 appchainId) external view returns (uint256) {
        if (appchainFees[appchainId] == 0) {
            return DEFAULT_APPCHAIN_FEES;
        }
        return appchainFees[appchainId];
    }

    function getTotalGasFees(uint256) external view returns (uint256) {
        if (totalFees == 0) {
            return DEFAULT_TOTAL_FEES;
        }
        return totalFees;
    }

    function getActiveAppchainIds(uint256) external view returns (uint256[] memory) {
        if (appchainIds.length == 0) {
            return DEFAULT_APPCHAIN_IDS;
        }
        return appchainIds;
    }

    function getAppchainRewardsReceiver(uint256, uint256 appchainId) external view returns (address) {
        if (rewardsReceivers[appchainId] == address(0)) {
            return DEFAULT_REWARDS_RECEIVER;
        }
        return rewardsReceivers[appchainId];
    }

    function setData(uint256[] memory _appchainIds, uint256[] memory _appchainFees) external {
        if (appchainIds.length != _appchainFees.length) {
            revert("length mismatch");
        }
        // Clear existing appchain fees
        for (uint256 i = 0; i < appchainIds.length; i++) {
            delete appchainFees[appchainIds[i]];
        }

        // Set new appchain fees and ids
        appchainIds = _appchainIds;
        uint256 _totalFees = 0;
        for (uint256 i = 0; i < _appchainFees.length; i++) {
            appchainFees[appchainIds[i]] = _appchainFees[i];
            _totalFees += _appchainFees[i];
        }

        // Set total fees
        totalFees = _totalFees;
    }

    function setRewardsReceiver(uint256 appchainId, address rewardsReceiver) external {
        rewardsReceivers[appchainId] = rewardsReceiver;
    }
}
