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
     * This contract uses a 5-variable pattern to track stake across different dimensions.
     * The pattern ensures accurate historical accounting while enabling efficient queries.
     *
     * For each tracked value, we maintain:
     * 1. Current Total - The present amount (e.g., totalStake, userTotal)
     * 2. Historical Total - Per-epoch snapshots (e.g., epochTotal, epochUserTotal)
     * 3. Epoch Additions - Amount added during specific epoch (e.g., epochAdditions, epochUserAdditions)
     * 4. Epoch Withdrawals - Amount withdrawn during specific epoch (e.g., epochWithdrawals, epochUserWithdrawals)
     * 5. Last Finalized Epoch - Tracks which epochs have been fully processed
     *
     * The key insight: stake added during an epoch doesn't count for that epoch's rewards, while
     * withdrawals are included in the epoch they are initialized in. Epoch additions let us
     * exclude new stake from the current epoch while including it in future epochs and vice versa
     * for withdrawals.
     *
     * We track 4 different stake dimensions:
     * - Total Stake: Global stake across all users and appchains
     * - User Total Stake: Individual user's stake across all appchains
     * - Appchain Stake: Total stake for specific appchain across all users
     * - User Appchain Stake: Individual user's stake for specific appchain
     *
     * Each dimension uses the same 5-variable pattern for consistent accounting.
     */

    // Total amount staked
    uint256 public totalStake;
    // Epoch => total amount staked
    mapping(uint256 => uint256) public epochTotal;
    // Epoch => amount staked during the epoch
    mapping(uint256 => uint256) public epochAdditions;
    // Epoch => amount withdrawn during the epoch
    mapping(uint256 => uint256) public epochWithdrawals;
    // Last finalized epoch
    uint256 public finalizedEpochCount;

    // User => current stake amount
    mapping(address => uint256) public userTotal;
    // Epoch => user => stake amount
    mapping(uint256 => mapping(address => uint256)) public epochUserTotal;
    // Epoch => user => amount staked during the epoch
    mapping(uint256 => mapping(address => uint256)) public epochUserAdditions;
    // Epoch => user => amount withdrawn during the epoch
    mapping(uint256 => mapping(address => uint256)) public epochUserWithdrawals;
    // User => last finalized epoch
    mapping(address => uint256) public userFinalizedEpochCount;

    // Appchain => total amount staked
    mapping(uint256 => uint256) public appchainTotal;
    // Epoch => appchain => total amount staked
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainTotal;
    // Epoch => appchain => amount staked during the epoch
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainAdditions;
    // Epoch => appchain => amount withdrawn during the epoch
    mapping(uint256 => mapping(uint256 => uint256)) public epochAppchainWithdrawals;
    // Last finalized epoch for each appchain
    mapping(uint256 => uint256) public appchainFinalizedEpochCount;

    // User => appchain => total amount staked
    mapping(address => mapping(uint256 => uint256)) public userAppchainTotal;
    // Epoch => user => appchain => amount staked during the epoch
    mapping(uint256 => mapping(address => mapping(uint256 => uint256))) public epochUserAppchainTotal;
    // Epoch => user => appchain => amount staked during the epoch
    mapping(uint256 => mapping(address => mapping(uint256 => uint256))) public epochUserAppchainAdditions;
    // Epoch => user => appchain => amount withdrawn during the epoch
    mapping(uint256 => mapping(address => mapping(uint256 => uint256))) public epochUserAppchainWithdrawals;
    // User => appchain => last finalized epoch
    mapping(address => mapping(uint256 => uint256)) public userAppchainFinalizedEpochCount;

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

    event Stake(uint256 epochIndex, address user, uint256 amount, uint256 appchainId);
    event Restake(uint256 epochIndex, address user, uint256 amount, uint256 fromAppchainId, uint256 toAppchainId);
    event WithdrawalInitialized(address user, uint256 appchainId, uint256 amount);
    event WithdrawalCompleted(address user, address destination, uint256 amount);

    error InvalidAmount();
    error InvalidAppchainId();
    error InsufficientStake();
    error WithdrawalNotReady();
    error InvalidWithdrawal();

    constructor(uint256 _startTimestamp) EpochTracker(_startTimestamp) {}

    ///////////////////////
    // Staking functions
    ///////////////////////

    function stakeSynd(uint256 appchainId) external payable {
        if (msg.value == 0) {
            revert InvalidAmount();
        }
        if (appchainId == 0) {
            revert InvalidAppchainId();
        }

        // Finalize epoch accounting if needed
        uint256 epochIndex = getCurrentEpoch();
        if (finalizedEpochCount < epochIndex) {
            finalizeEpochs();
        }
        if (userFinalizedEpochCount[msg.sender] < epochIndex) {
            finalizeUserEpochs(msg.sender);
        }
        if (appchainFinalizedEpochCount[appchainId] < epochIndex) {
            finalizeAppchainEpochs(appchainId);
        }
        if (userAppchainFinalizedEpochCount[msg.sender][appchainId] < epochIndex) {
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

    function restakeSynd(uint256 fromAppchainId, uint256 toAppchainId, uint256 amount) external payable {
        if (amount == 0) {
            revert InvalidAmount();
        }
        if (fromAppchainId == 0 || toAppchainId == 0) {
            revert InvalidAppchainId();
        }
        if (userAppchainTotal[msg.sender][fromAppchainId] < amount) {
            revert InsufficientStake();
        }

        uint256 epochIndex = getCurrentEpoch();
        if (userFinalizedEpochCount[msg.sender] < epochIndex) {
            finalizeUserEpochs(msg.sender);
        }
        if (appchainFinalizedEpochCount[fromAppchainId] < epochIndex) {
            finalizeAppchainEpochs(fromAppchainId);
        }
        if (userAppchainFinalizedEpochCount[msg.sender][fromAppchainId] < epochIndex) {
            finalizeUserAppchainEpochs(msg.sender, fromAppchainId);
        }

        epochUserAppchainTotal[epochIndex][msg.sender][fromAppchainId] += amount;
        epochAppchainTotal[epochIndex][fromAppchainId] += amount;
        userAppchainTotal[msg.sender][fromAppchainId] -= amount;
        appchainTotal[fromAppchainId] -= amount;

        epochUserAppchainAdditions[epochIndex][msg.sender][toAppchainId] += amount;
        epochAppchainAdditions[epochIndex][toAppchainId] += amount;
        userAppchainTotal[msg.sender][toAppchainId] += amount;
        appchainTotal[toAppchainId] += amount;

        emit Restake(epochIndex, msg.sender, amount, fromAppchainId, toAppchainId);
    }

    ///////////////////////
    // Finalization functions
    ///////////////////////

    function finalizeEpochs() public {
        uint256 currentEpoch = getCurrentEpoch();
        while (finalizedEpochCount < currentEpoch) {
            epochTotal[finalizedEpochCount] += totalStake;
            epochTotal[finalizedEpochCount] += epochWithdrawals[finalizedEpochCount];
            epochTotal[finalizedEpochCount] -= epochAdditions[finalizedEpochCount];
            finalizedEpochCount++;
        }
    }

    function finalizeUserEpochs(address user) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = userFinalizedEpochCount[user];
        while (index < currentEpoch) {
            epochUserTotal[index][user] += userTotal[user];
            epochUserTotal[index][user] += epochUserWithdrawals[index][user];
            epochUserTotal[index][user] -= epochUserAdditions[index][user];
            index++;
        }
        userFinalizedEpochCount[user] = index;
    }

    function finalizeAppchainEpochs(uint256 appchainId) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = appchainFinalizedEpochCount[appchainId];
        while (index < currentEpoch) {
            epochAppchainTotal[index][appchainId] += appchainTotal[appchainId];
            epochAppchainTotal[index][appchainId] += epochAppchainWithdrawals[index][appchainId];
            epochAppchainTotal[index][appchainId] -= epochAppchainAdditions[index][appchainId];
            index++;
        }
        appchainFinalizedEpochCount[appchainId] = index;
    }

    function finalizeUserAppchainEpochs(address user, uint256 appchainId) public {
        uint256 currentEpoch = getCurrentEpoch();
        uint256 index = userAppchainFinalizedEpochCount[user][appchainId];
        while (index < currentEpoch) {
            epochUserAppchainTotal[index][user][appchainId] += userAppchainTotal[user][appchainId];
            epochUserAppchainTotal[index][user][appchainId] += epochUserAppchainWithdrawals[index][user][appchainId];
            epochUserAppchainTotal[index][user][appchainId] -= epochUserAppchainAdditions[index][user][appchainId];
            index++;
        }
        userAppchainFinalizedEpochCount[user][appchainId] = index;
    }

    ///////////////////////
    // Withdrawal functions
    ///////////////////////
    function initializeWithdrawal(uint256 appchainId, uint256 amount) public {
        if (amount == 0) {
            revert InvalidAmount();
        }
        if (appchainId == 0) {
            revert InvalidAppchainId();
        }
        if (userAppchainTotal[msg.sender][appchainId] < amount) {
            revert InsufficientStake();
        }

        uint256 currentEpoch = getCurrentEpoch();
        if (finalizedEpochCount < currentEpoch) {
            finalizeEpochs();
        }
        if (userFinalizedEpochCount[msg.sender] < currentEpoch) {
            finalizeUserEpochs(msg.sender);
        }
        if (appchainFinalizedEpochCount[appchainId] < currentEpoch) {
            finalizeAppchainEpochs(appchainId);
        }
        if (userAppchainFinalizedEpochCount[msg.sender][appchainId] < currentEpoch) {
            finalizeUserAppchainEpochs(msg.sender, appchainId);
        }

        epochWithdrawals[currentEpoch] += amount;
        epochUserWithdrawals[currentEpoch][msg.sender] += amount;
        epochAppchainWithdrawals[currentEpoch][appchainId] += amount;
        epochUserAppchainWithdrawals[currentEpoch][msg.sender][appchainId] += amount;

        totalStake -= amount;
        userTotal[msg.sender] -= amount;
        appchainTotal[appchainId] -= amount;
        userAppchainTotal[msg.sender][appchainId] -= amount;

        emit WithdrawalInitialized(msg.sender, appchainId, amount);
    }

    function initializeWithdrawal(uint256 appchainId) external {
        initializeWithdrawal(appchainId, getWithdrawalAmount(msg.sender, appchainId));
    }

    function withdraw(uint256 epochIndex, address destination) external {
        if (epochIndex >= getCurrentEpoch()) {
            revert WithdrawalNotReady();
        }

        uint256 amount = epochUserWithdrawals[epochIndex][msg.sender];
        if (amount == 0) {
            revert InvalidWithdrawal();
        }

        finalizeUserEpochs(msg.sender);

        epochUserWithdrawals[epochIndex][msg.sender] = 0;

        payable(destination).transfer(amount);

        emit WithdrawalCompleted(msg.sender, destination, amount);
    }

    ///////////////////////
    // View functions
    ///////////////////////

    function getWithdrawalAmount(address user, uint256 appchainId) public view returns (uint256) {
        return userAppchainTotal[user][appchainId];
    }

    function getUserStake(uint256 epochIndex, address user) public view returns (uint256) {
        if (epochIndex >= userFinalizedEpochCount[user]) {
            return userTotal[user] + epochUserTotal[epochIndex][user] + epochUserWithdrawals[epochIndex][user]
                - epochUserAdditions[epochIndex][user];
        } else {
            return epochUserTotal[epochIndex][user];
        }
    }

    function getUserStakeShare(uint256 epochIndex, address user) public view returns (uint256) {
        return getUserStake(epochIndex, user) + epochUserStakeShare[epochIndex][user];
    }

    function getTotalStake(uint256 epochIndex) public view returns (uint256) {
        if (epochIndex >= finalizedEpochCount) {
            return totalStake + epochTotal[epochIndex] + epochWithdrawals[epochIndex] - epochAdditions[epochIndex];
        } else {
            return epochTotal[epochIndex];
        }
    }

    function getTotalStakeShare(uint256 epochIndex) public view returns (uint256) {
        return getTotalStake(epochIndex) + epochStakeShare[epochIndex];
    }

    function getAppchainStake(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= appchainFinalizedEpochCount[appchainId]) {
            return appchainTotal[appchainId] + epochAppchainTotal[epochIndex][appchainId]
                + epochAppchainWithdrawals[epochIndex][appchainId] - epochAppchainAdditions[epochIndex][appchainId];
        } else {
            return epochAppchainTotal[epochIndex][appchainId];
        }
    }

    function getUserAppchainStake(uint256 epochIndex, address user, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= userAppchainFinalizedEpochCount[user][appchainId]) {
            return userAppchainTotal[user][appchainId] + epochUserAppchainTotal[epochIndex][user][appchainId]
                + epochUserAppchainWithdrawals[epochIndex][user][appchainId]
                - epochUserAppchainAdditions[epochIndex][user][appchainId];
        } else {
            return epochUserAppchainTotal[epochIndex][user][appchainId];
        }
    }
}
