// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";

interface ISyndStaking {
    function getStakeDetails(uint256 epochIndex, address user) external view returns (uint256, uint256);
    function getWeightedStakeDetails(uint256 epochIndex, address user) external view returns (uint256, uint256);
}

contract SyndStaking is EpochTracker, ISyndStaking {
    uint256 public totalStaked;
    // User => current stake amount
    mapping(address => uint256) public stake;
    // User => epoch index of withdrawal initialization
    mapping(address => uint256) public withdrawalInitialized;

    // Epoch => total amount staked
    mapping(uint256 => uint256) public epochTotal;
    // Epoch => amount staked during the epoch
    mapping(uint256 => uint256) public epochAdditions;
    // Epoch => amount staked during the epoch weighted by time left in the epoch
    mapping(uint256 => uint256) public epochStakeShare;
    // Last finalized epoch
    uint256 public lastFinalizedEpoch;

    // Epoch => user => stake amount
    mapping(uint256 => mapping(address => uint256)) public userTotal;
    // Epoch => user => amount staked during the epoch
    mapping(uint256 => mapping(address => uint256)) public userAdditions;
    // Epoch => user => amount staked during the epoch weighted by time left in the epoch
    mapping(uint256 => mapping(address => uint256)) public userStakeShare;
    // User => last finalized epoch
    mapping(address => uint256) public userLastFinalizedEpoch;

    event Stake(uint256 epochIndex, address user, uint256 amount);
    event WithdrawalInitialized(uint256 timestamp);
    event WithdrawalCompleted(uint256 amount);

    error InvalidStakeAmount();
    error WithdrawalAlreadyInitialized();
    error WithdrawalNotInitialized();
    error WithdrawalNotReady();

    constructor(uint256 _startTimestamp) EpochTracker(_startTimestamp) {}

    ///////////////////////
    // Staking functions
    ///////////////////////

    function stakeSynd() external payable {
        uint256 epochIndex = getCurrentEpoch();

        if (msg.value == 0) {
            revert InvalidStakeAmount();
        }

        if (withdrawalInitialized[msg.sender] != 0) {
            revert WithdrawalAlreadyInitialized();
        }

        if (lastFinalizedEpoch < epochIndex) {
            finalizeEpochs();
        }

        if (userLastFinalizedEpoch[msg.sender] < epochIndex) {
            finalizeUserEpoch(msg.sender);
        }

        uint256 weight = weightStake(msg.value);

        userAdditions[epochIndex][msg.sender] += msg.value;
        userStakeShare[epochIndex][msg.sender] += weight;
        stake[msg.sender] += msg.value;

        epochAdditions[epochIndex] += msg.value;
        epochStakeShare[epochIndex] += weight;
        totalStaked += msg.value;

        emit Stake(epochIndex, msg.sender, msg.value);
    }

    function weightStake(uint256 amount) internal view returns (uint256) {
        return (amount * (getEpochEnd(getCurrentEpoch()) - block.timestamp)) / EPOCH_DURATION;
    }

    ///////////////////////
    // Finalization functions
    ///////////////////////

    function finalizeUserEpoch(address user) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = userLastFinalizedEpoch[user];
        if (index < currentEpoch) {
            userTotal[index][user] = stake[user] - userAdditions[index][user];
            index++;
        }
        userLastFinalizedEpoch[user] = index;
    }

    function finalizeEpochs() public {
        uint256 currentEpoch = getCurrentEpoch();
        while (lastFinalizedEpoch < currentEpoch) {
            epochTotal[lastFinalizedEpoch] += totalStaked - epochAdditions[lastFinalizedEpoch];
            lastFinalizedEpoch++;
        }
    }

    ///////////////////////
    // Withdrawal functions
    ///////////////////////

    function initializeWithdrawal() external {
        if (withdrawalInitialized[msg.sender] != 0) {
            revert WithdrawalAlreadyInitialized();
        }

        uint256 epochIndex = getCurrentEpoch();
        withdrawalInitialized[msg.sender] = epochIndex;

        uint256 amount = stake[msg.sender];
        totalStaked -= amount;
        epochTotal[epochIndex] += amount;

        emit WithdrawalInitialized(block.timestamp);
    }

    function withdraw() external {
        if (withdrawalInitialized[msg.sender] == 0) {
            revert WithdrawalNotInitialized();
        }

        if (getCurrentEpoch() <= withdrawalInitialized[msg.sender]) {
            revert WithdrawalNotReady();
        }

        uint256 amount = stake[msg.sender];
        stake[msg.sender] = 0;
        withdrawalInitialized[msg.sender] = 0;

        payable(msg.sender).transfer(amount);

        emit WithdrawalCompleted(amount);
    }

    ///////////////////////
    // View functions
    ///////////////////////

    function getUserStake(uint256 epochIndex, address user) public view returns (uint256) {
        if (withdrawalInitialized[user] != 0 && withdrawalInitialized[user] < epochIndex) {
            return 0;
        } else if (epochIndex >= userLastFinalizedEpoch[user]) {
            return stake[user] - userAdditions[epochIndex][user];
        } else {
            return userTotal[epochIndex][user];
        }
    }

    function getTotalStake(uint256 epochIndex) public view returns (uint256) {
        if (epochIndex >= lastFinalizedEpoch) {
            return totalStaked + epochTotal[epochIndex] - epochAdditions[epochIndex];
        } else {
            return epochTotal[epochIndex];
        }
    }

    function getStakeDetails(uint256 epochIndex, address user)
        external
        view
        returns (uint256 stakeAmount, uint256 totalStake)
    {
        return (getUserStake(epochIndex, user), getTotalStake(epochIndex));
    }

    function getWeightedStakeDetails(uint256 epochIndex, address user)
        external
        view
        returns (uint256 weightedStake, uint256 totalWeightedStake)
    {
        return (
            getUserStake(epochIndex, user) + userStakeShare[epochIndex][user],
            getTotalStake(epochIndex) + epochStakeShare[epochIndex]
        );
    }
}
