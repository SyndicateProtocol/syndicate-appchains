// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";

interface ISyndStaking {
    function getUserStake(uint256 epochIndex, address user) external view returns (uint256);
    function getUserStakeShare(uint256 epochIndex, address user) external view returns (uint256);
    function getTotalStake(uint256 epochIndex) external view returns (uint256);
    function getTotalStakeShare(uint256 epochIndex) external view returns (uint256);
    function getAppchainStake(uint256 epochIndex, uint256 appchainId) external view returns (uint256);
    function getUserAppchainStake(uint256 epochIndex, address user, uint256 appchainId)
        external
        view
        returns (uint256);
}

contract SyndStaking is EpochTracker, ISyndStaking {
    /**
     * Stake Tracking System Overview
     *
     * This contract uses a 4-variable pattern to track stake across different dimensions.
     * The pattern ensures accurate historical accounting while enabling efficient queries.
     *
     * For each tracked value, we maintain:
     * 1. Current Total - The present amount (e.g., totalStake, userTotal)
     * 2. Historical Total - Per-epoch snapshots (e.g., epochTotal, epochUserTotal)
     * 3. Epoch Additions - Amount added during specific epoch (e.g., epochAdditions, epochUserAdditions)
     * 4. Last Finalized Epoch - Tracks which epochs have been fully processed
     *
     * The key insight: stake added during an epoch doesn't count for that epoch's rewards
     * since it wasn't present for the entire epoch duration. Epoch additions let us
     * exclude new stake from the current epoch while including it in future epochs.
     *
     * We track 4 different stake dimensions:
     * - Total Stake: Global stake across all users and appchains
     * - User Total Stake: Individual user's stake across all appchains
     * - Appchain Stake: Total stake for specific appchain across all users
     * - User Appchain Stake: Individual user's stake for specific appchain
     *
     * Each dimension uses the same 4-variable pattern for consistent accounting.
     */

    // Total amount staked
    uint256 public totalStake;
    // Epoch => total amount staked
    mapping(uint256 => uint256) public epochTotal;
    // Epoch => amount staked during the epoch
    mapping(uint256 => uint256) public epochAdditions;
    // Last finalized epoch
    uint256 public finalizedEpoch;

    // User => current stake amount
    mapping(address => uint256) public userTotal;
    // Epoch => user => stake amount
    mapping(uint256 => mapping(address => uint256)) public epochUserTotal;
    // Epoch => user => amount staked during the epoch
    mapping(uint256 => mapping(address => uint256)) public epochUserAdditions;
    // User => last finalized epoch
    mapping(address => uint256) public userFinalizedEpoch;

    // User => appchain => total amount staked
    mapping(address => mapping(uint256 => uint256)) public userAppchainTotal;
    // Epoch => user => appchain => amount staked during the epoch
    mapping(uint256 => mapping(address => mapping(uint256 => uint256))) public epochUserAppchainTotal;
    // Epoch => user => appchain => amount staked during the epoch
    mapping(uint256 => mapping(address => mapping(uint256 => uint256))) public epochUserAppchainAdditions;
    // User => appchain => last finalized epoch
    mapping(address => mapping(uint256 => uint256)) public userAppchainFinalizedEpoch;

    // Appchain => total amount staked
    mapping(uint256 => uint256) public appchainTotal;
    // Epoch => appchain => total amount staked
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainTotal;
    // Epoch => appchain => amount staked during the epoch
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainAdditions;
    // Last finalized epoch for each appchain
    mapping(uint256 => uint256) public appchainFinalizedEpoch;

    /*
     * Pro-Rata Stake Tracking:
     * Some rewards require pro-rata accounting where stake added mid-epoch receives
     * partial credit based on time remaining. For example, staking halfway through
     * an epoch might give 50% of that epoch's rewards.
     * 
     * For this we track 2 additional variables:
     * - epochStakeShare: Total pro-rata stake for an epoch (weighted by time)
     * - epochUserStakeShare: Per-user pro-rata stake for an epoch (weighted by time)
     * 
     * These are added to the totals from the 4-variable pattern above to get
     * complete stake accounting for both full-epoch and partial-epoch rewards.
    */

    // Epoch => amount staked during the epoch weighted by time left in the epoch
    mapping(uint256 => uint256) public epochStakeShare;
    // Epoch => user => amount staked during the epoch weighted by time left in the epoch
    mapping(uint256 => mapping(address => uint256)) public epochUserStakeShare;

    // User => appchain => epoch index of withdrawal initialization
    mapping(address => mapping(uint256 => uint256)) public userAppchainWithdrawalInitialized;

    event Stake(uint256 epochIndex, address user, uint256 amount, uint256 appchainId);
    event WithdrawalInitialized(address user, uint256 appchainId, uint256 timestamp);
    event WithdrawalCompleted(address user, address destination, uint256 amount);
    event Log(string message, uint256 index);

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
        if (msg.value == 0) {
            revert InvalidStakeAmount();
        }
        if (appchainId == 0) {
            revert InvalidAppchainId();
        }
        if (userAppchainWithdrawalInitialized[msg.sender][appchainId] != 0) {
            revert WithdrawalAlreadyInitialized();
        }

        // Finalize epoch accounting if needed
        uint256 epochIndex = getCurrentEpoch();
        if (finalizedEpoch < epochIndex) {
            finalizeEpochs();
        }
        if (appchainFinalizedEpoch[appchainId] < epochIndex) {
            finalizeAppchainEpochs(appchainId);
        }
        if (userFinalizedEpoch[msg.sender] < epochIndex) {
            finalizeUserEpochs(msg.sender);
        }
        if (userAppchainFinalizedEpoch[msg.sender][appchainId] < epochIndex) {
            finalizeUserAppchainEpochs(msg.sender, appchainId);
        }

        // Calculate stake share for current epoch
        uint256 stakeShare = calculateStakeShare(msg.value);
        epochStakeShare[epochIndex] += stakeShare;
        epochUserStakeShare[epochIndex][msg.sender] += stakeShare;

        epochAdditions[epochIndex] += msg.value;
        totalStake += msg.value;

        epochUserAdditions[epochIndex][msg.sender] += msg.value;
        userTotal[msg.sender] += msg.value;

        epochAppchainAdditions[epochIndex][appchainId] += msg.value;
        appchainTotal[appchainId] += msg.value;

        epochUserAppchainAdditions[epochIndex][msg.sender][appchainId] += msg.value;
        userAppchainTotal[msg.sender][appchainId] += msg.value;

        emit Stake(epochIndex, msg.sender, msg.value, appchainId);
    }

    function calculateStakeShare(uint256 amount) internal view returns (uint256) {
        return (amount * (getEpochEnd(getCurrentEpoch()) - block.timestamp)) / EPOCH_DURATION;
    }

    ///////////////////////
    // Finalization functions
    ///////////////////////

    function finalizeUserEpochs(address user) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = userFinalizedEpoch[user];
        while (index < currentEpoch) {
            // We only track stake that was present the entire epoch, so we need to subtract the stake that was added during the epoch
            epochUserTotal[index][user] += userTotal[user];
            epochUserTotal[index][user] -= epochUserAdditions[index][user];
            index++;
        }
        userFinalizedEpoch[user] = index;
    }

    function finalizeUserAppchainEpochs(address user, uint256 appchainId) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = userAppchainFinalizedEpoch[user][appchainId];
        while (index < currentEpoch) {
            // We only track stake that was present the entire epoch, so we need to subtract the stake that was added during the epoch
            epochUserAppchainTotal[index][user][appchainId] =
                userAppchainTotal[user][appchainId] - epochUserAppchainAdditions[index][user][appchainId];
            index++;
        }
        userAppchainFinalizedEpoch[user][appchainId] = index;
    }

    function finalizeEpochs() public {
        uint256 currentEpoch = getCurrentEpoch();
        while (finalizedEpoch < currentEpoch) {
            epochTotal[finalizedEpoch] += totalStake;
            // We only track stake that was present the entire epoch, so we need to subtract the stake that was added during the epoch
            epochTotal[finalizedEpoch] -= epochAdditions[finalizedEpoch];
            finalizedEpoch++;
        }
    }

    function finalizeAppchainEpochs(uint256 appchainId) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = appchainFinalizedEpoch[appchainId];
        while (index < currentEpoch) {
            epochAppchainTotal[index][appchainId] += appchainTotal[appchainId];
            // We only track stake that was present the entire epoch, so we need to subtract the stake that was added during the epoch
            epochAppchainTotal[index][appchainId] -= epochAppchainAdditions[index][appchainId];
            index++;
        }
        appchainFinalizedEpoch[appchainId] = index;
    }

    ///////////////////////
    // Withdrawal functions
    ///////////////////////

    function initializeWithdrawal(uint256 appchainId) external {
        if (userAppchainWithdrawalInitialized[msg.sender][appchainId] != 0) {
            revert WithdrawalAlreadyInitialized();
        }

        uint256 currentEpoch = getCurrentEpoch();
        if (finalizedEpoch < currentEpoch) {
            finalizeEpochs();
        }
        if (appchainFinalizedEpoch[appchainId] < currentEpoch) {
            finalizeAppchainEpochs(appchainId);
        }
        if (userFinalizedEpoch[msg.sender] < currentEpoch) {
            finalizeUserEpochs(msg.sender);
        }

        userAppchainWithdrawalInitialized[msg.sender][appchainId] = currentEpoch;

        // Pre-emptively remove stake from current totals and add to the epoch totals
        // This is to make sure we have accurate accounting regardless of the withdrawal completion time
        uint256 amount = userAppchainTotal[msg.sender][appchainId];
        totalStake -= amount;
        userTotal[msg.sender] -= amount;
        appchainTotal[appchainId] -= amount;

        epochTotal[currentEpoch] += amount;
        epochUserTotal[currentEpoch][msg.sender] += amount;
        epochAppchainTotal[currentEpoch][appchainId] += amount;
        epochUserAppchainTotal[currentEpoch][msg.sender][appchainId] += amount;

        emit WithdrawalInitialized(msg.sender, appchainId, block.timestamp);
    }

    function withdraw(uint256 appchainId, address destination) external {
        if (userAppchainWithdrawalInitialized[msg.sender][appchainId] == 0) {
            revert WithdrawalNotInitialized();
        }

        if (getCurrentEpoch() <= userAppchainWithdrawalInitialized[msg.sender][appchainId]) {
            revert WithdrawalNotReady();
        }

        finalizeUserAppchainEpochs(msg.sender, appchainId);

        uint256 amount = userAppchainTotal[msg.sender][appchainId];
        userAppchainTotal[msg.sender][appchainId] = 0;
        userAppchainWithdrawalInitialized[msg.sender][appchainId] = 0;

        payable(destination).transfer(amount);

        emit WithdrawalCompleted(msg.sender, destination, amount);
    }

    ///////////////////////
    // View functions
    ///////////////////////

    function getUserStake(uint256 epochIndex, address user) public view returns (uint256) {
        if (epochIndex >= userFinalizedEpoch[user]) {
            return userTotal[user] + epochUserTotal[epochIndex][user] - epochUserAdditions[epochIndex][user];
        } else {
            return epochUserTotal[epochIndex][user];
        }
    }

    function getUserStakeShare(uint256 epochIndex, address user) public view returns (uint256) {
        return getUserStake(epochIndex, user) + epochUserStakeShare[epochIndex][user];
    }

    function getTotalStake(uint256 epochIndex) public view returns (uint256) {
        if (epochIndex >= finalizedEpoch) {
            return totalStake + epochTotal[epochIndex] - epochAdditions[epochIndex];
        } else {
            return epochTotal[epochIndex];
        }
    }

    function getTotalStakeShare(uint256 epochIndex) public view returns (uint256) {
        return getTotalStake(epochIndex) + epochStakeShare[epochIndex];
    }

    function getAppchainStake(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= appchainFinalizedEpoch[appchainId]) {
            return appchainTotal[appchainId] + epochAppchainTotal[epochIndex][appchainId]
                - epochAppchainAdditions[epochIndex][appchainId];
        } else {
            return epochAppchainTotal[epochIndex][appchainId];
        }
    }

    function getUserAppchainStake(uint256 epochIndex, address user, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= userAppchainFinalizedEpoch[user][appchainId]) {
            return userAppchainTotal[user][appchainId] + epochUserAppchainTotal[epochIndex][user][appchainId]
                - epochUserAppchainAdditions[epochIndex][user][appchainId];
        } else {
            return epochUserAppchainTotal[epochIndex][user][appchainId];
        }
    }
}
