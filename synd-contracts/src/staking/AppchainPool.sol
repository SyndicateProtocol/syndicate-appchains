// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RewardPoolBase} from "./RewardPoolBase.sol";
import {IGasDataProvider} from "./IGasDataProvider.sol";
import {ISyndStaking} from "./ISyndStaking.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./IPool.sol";

contract AppchainPool is IPool, RewardPoolBase {
    // amount already claimed per epoch/appchain
    mapping(uint256 => mapping(uint256 => uint256)) public claimed;

    error InvalidClaimer();

    constructor(address admin, address staking, address gas) RewardPoolBase(admin, staking, gas) {}

    function deposit(uint256 epochIndex) external payable override {
        _deposit(epochIndex);
    }

    function claim(uint256 epochIndex, uint256 appchainId, address destination) external nonReentrant {
        _preChecks(epochIndex);

        // Only the configured receiver can claim
        if (msg.sender != IGasDataProvider(address(gasDataProvider)).getAppchainRewardsReceiver(epochIndex, appchainId))
        {
            revert InvalidClaimer();
        }

        uint256 appchainTotal = _computeAppchainTotalReward(epochIndex, appchainId);
        uint256 already = claimed[epochIndex][appchainId];
        uint256 amount = appchainTotal > already ? appchainTotal - already : 0;
        if (amount == 0) revert ClaimNotAvailable();

        claimed[epochIndex][appchainId] = already + amount;
        Address.sendValue(payable(destination), amount);

        emit ClaimSuccess(epochIndex, appchainId, destination, amount);
    }

    function getClaimableAmount(uint256 epochIndex, uint256 appchainId) public returns (uint256) {
        uint256 appchainTotal = _computeAppchainTotalReward(epochIndex, appchainId);
        uint256 already = claimed[epochIndex][appchainId];
        return appchainTotal > already ? appchainTotal - already : 0;
    }
}
