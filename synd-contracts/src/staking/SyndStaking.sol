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

    // User => current backed appchain id
    mapping(address => uint256) public userCurrentAppchain;
    // Epoch => user => appchain id
    mapping(uint256 => mapping(address => uint256)) public userAppchain;

    // Appchain => total amount staked
    mapping(uint256 => uint256) public appchainTotal;
    // Epoch => appchain => total amount staked
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainTotal;
    // Epoch => appchain => amount staked during the epoch
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainAdditions;
    // Last finalized epoch for each appchain
    mapping(uint256 => uint256) public appchainLastFinalizedEpoch;

    event Stake(uint256 epochIndex, address user, uint256 amount, uint256 appchainId);
    event WithdrawalInitialized(address user, uint256 timestamp);
    event WithdrawalCompleted(address user, uint256 amount);

    error InvalidStakeAmount();
    error InvalidAppchainId();
    error WithdrawalAlreadyInitialized();
    error WithdrawalNotInitialized();
    error WithdrawalNotReady();

    constructor(uint256 _startTimestamp) EpochTracker(_startTimestamp) {}

    ///////////////////////
    // Staking functions
    ///////////////////////

    function stakeSynd(uint256 appchainId) external payable {
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

        if (appchainLastFinalizedEpoch[appchainId] < epochIndex) {
            finalizeAppchainEpochs(appchainId);
        }

        if (stake[msg.sender] == 0) {
            userCurrentAppchain[msg.sender] = appchainId;
        } else if (userCurrentAppchain[msg.sender] != appchainId) {
            revert InvalidAppchainId();
        }

        uint256 weight = weightStake(msg.value);

        userAdditions[epochIndex][msg.sender] += msg.value;
        userStakeShare[epochIndex][msg.sender] += weight;
        stake[msg.sender] += msg.value;

        epochAdditions[epochIndex] += msg.value;
        epochStakeShare[epochIndex] += weight;
        totalStaked += msg.value;

        epochAppchainAdditions[epochIndex][appchainId] += msg.value;
        appchainTotal[appchainId] += msg.value;

        emit Stake(epochIndex, msg.sender, msg.value, appchainId);
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
            userAppchain[index][user] = userCurrentAppchain[user];
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

    function finalizeAppchainEpochs(uint256 appchainId) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = appchainLastFinalizedEpoch[appchainId];
        while (index < currentEpoch) {
            epochAppchainTotal[index][appchainId] +=
                appchainTotal[appchainId] - epochAppchainAdditions[index][appchainId];
            index++;
        }
        appchainLastFinalizedEpoch[appchainId] = index;
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

        // Pre-emptively add the user's stake to the epoch total
        // This is to make sure the user's stake
        uint256 amount = stake[msg.sender];
        totalStaked -= amount;
        epochTotal[epochIndex] += amount;

        uint256 appchainId = userCurrentAppchain[msg.sender];
        appchainTotal[appchainId] -= amount;
        epochAppchainTotal[epochIndex][appchainId] += amount;

        emit WithdrawalInitialized(msg.sender, block.timestamp);
    }

    function withdraw() external {
        if (withdrawalInitialized[msg.sender] == 0) {
            revert WithdrawalNotInitialized();
        }

        if (getCurrentEpoch() <= withdrawalInitialized[msg.sender]) {
            revert WithdrawalNotReady();
        }

        finalizeUserEpoch(msg.sender);

        uint256 amount = stake[msg.sender];

        stake[msg.sender] = 0;
        withdrawalInitialized[msg.sender] = 0;
        userCurrentAppchain[msg.sender] = 0;

        payable(msg.sender).transfer(amount);

        emit WithdrawalCompleted(msg.sender, amount);
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

    function getAppchainStake(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= appchainLastFinalizedEpoch[appchainId]) {
            return appchainTotal[appchainId] + epochAppchainTotal[epochIndex][appchainId]
                - epochAppchainAdditions[epochIndex][appchainId];
        } else {
            return epochAppchainTotal[epochIndex][appchainId];
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
