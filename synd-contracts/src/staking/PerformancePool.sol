// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RewardPoolBase} from "./RewardPoolBase.sol";
import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IUserPool} from "./interfaces/IPool.sol";

contract PerformancePool is IUserPool, RewardPoolBase {
    // amount already claimed per epoch/appchain/user
    mapping(uint256 epochIndex => mapping(uint256 appchainId => mapping(address user => uint256 claimed))) public
        claimed;

    error UnauthorizedCaller();

    constructor(address admin, address staking, address gas) RewardPoolBase(admin, staking, gas) {}

    modifier onlyStakingContract() {
        if (msg.sender != address(stakingContract)) {
            revert UnauthorizedCaller();
        }
        _;
    }

    function deposit(uint256 epochIndex) external payable override nonReentrant {
        _deposit(epochIndex);
    }

    function claim(uint256 epochIndex, address destination, uint256 appchainId) external nonReentrant {
        _claim(epochIndex, msg.sender, destination, appchainId);
    }

    function claimFor(uint256 epochIndex, address user, address destination, uint256 appchainId)
        external
        nonReentrant
        onlyStakingContract
    {
        _claim(epochIndex, user, destination, appchainId);
    }

    function _claim(uint256 epochIndex, address user, address destination, uint256 appchainId) internal {
        _preChecks(epochIndex);
        if (destination == address(0)) {
            revert InvalidDestination();
        }

        uint256 amount = getClaimableAmount(epochIndex, user, appchainId);
        if (amount == 0) revert ClaimNotAvailable();

        claimed[epochIndex][appchainId][user] += amount;
        Address.sendValue(payable(destination), amount);

        emit ClaimSuccess(epochIndex, appchainId, destination, amount);
    }

    function getClaimableAmount(uint256 epochIndex, address user, uint256 appchainId) public returns (uint256) {
        uint256 appchainTotal = _computeAppchainTotalReward(epochIndex, appchainId);
        if (appchainTotal == 0) return 0;

        uint256 userStaked = ISyndStaking(address(stakingContract)).getUserStakeShare(epochIndex, user);
        if (userStaked == 0) return 0;

        uint256 appchainStaked = ISyndStaking(address(stakingContract)).getAppchainStake(epochIndex, appchainId);
        if (appchainStaked == 0) return 0;

        uint256 userShare = (appchainTotal * userStaked) / appchainStaked;
        uint256 already = claimed[epochIndex][appchainId][user];
        return userShare > already ? userShare - already : 0;
    }
}
