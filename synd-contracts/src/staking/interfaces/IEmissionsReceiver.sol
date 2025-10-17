// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface IEmissionsReceiver {
    function appchainEmissionsReceiver(uint256 appchain) external view returns (address receiver);
    function setAppchainEmissionsReceiver(uint256 chainID, address receiver) external;
}
