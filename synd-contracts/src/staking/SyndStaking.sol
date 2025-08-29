// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {EpochTracker} from "./EpochTracker.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IUserPool} from "./IPool.sol";

/**
 * @title SyndStaking
 * @notice Main staking contract for SYND token
 * @dev Extends EpochTracker to provide comprehensive stake management across multiple dimensions
 *
 * This contract implements a stake tracking system that maintains historical
 * accounting while enabling efficient queries. It tracks stake across four dimensions:
 * - Global total stake across all users and appchains
 * - Per-user total stake across all appchains
 * - Per-appchain total stake across all users
 * - Per-user per-appchain stake
 *
 * The 5-variable pattern for each dimension ensures accurate historical accounting:
 * 1. Current Total - Present amount
 * 2. Historical Total - Per-epoch snapshots
 * 3. Epoch Additions - Amount added during specific epoch
 * 4. Epoch Withdrawals - Amount withdrawn during specific epoch
 * 5. Last Finalized Epoch - Tracks processed epochs
 *
 * Key features:
 * - Epoch-based stake accounting with 30-day periods
 * - Pro-rata stake sharing for partial epoch participation
 * - Multi-appchain support with restaking capabilities
 * - Delayed withdrawal system for security
 * - Efficient finalization system for historical queries
 */
contract SyndStaking is EpochTracker, ReentrancyGuard, Pausable, Ownable {
    /// @notice Total amount of SYND tokens staked across all users and appchains
    uint256 public totalStake;

    /// @notice Mapping from epoch index to total amount staked in that epoch
    mapping(uint256 epochIndex => uint256 total) public epochTotal;

    /// @notice Mapping from epoch index to amount staked during that specific epoch
    mapping(uint256 epochIndex => uint256 additions) public epochAdditions;

    /// @notice Mapping from epoch index to amount withdrawn during that specific epoch
    mapping(uint256 epochIndex => uint256 withdrawals) public epochWithdrawals;

    /// @notice Last finalized epoch index for global stake accounting
    uint256 public finalizedEpochCount;

    /// @notice Mapping from user address to their current total stake amount across all appchains
    mapping(address user => uint256 total) public userTotal;

    /// @notice Mapping from epoch index and user address to their stake amount in that epoch
    mapping(uint256 epochIndex => mapping(address user => uint256 total)) public epochUserTotal;

    /// @notice Mapping from epoch index and user address to amount staked during that specific epoch
    mapping(uint256 epochIndex => mapping(address user => uint256 additions)) public epochUserAdditions;

    /// @notice Mapping from epoch index and user address to amount withdrawn during that specific epoch
    mapping(uint256 epochIndex => mapping(address user => uint256 withdrawals)) public epochUserWithdrawals;

    /// @notice Mapping from user address to their last finalized epoch index
    mapping(address user => uint256 finalizedEpochCount) public userFinalizedEpochCount;

    /// @notice Mapping from appchain ID to total amount staked on that appchain across all users
    mapping(uint256 appchainId => uint256 total) public appchainTotal;

    /// @notice Mapping from epoch index and appchain ID to total amount staked on that appchain in that epoch
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 total)) public epochAppchainTotal;

    /// @notice Mapping from epoch index and appchain ID to amount staked on that appchain during that specific epoch
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 additions)) public epochAppchainAdditions;

    /// @notice Mapping from epoch index and appchain ID to amount withdrawn from that appchain during that specific epoch
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 withdrawals)) public epochAppchainWithdrawals;

    /// @notice Mapping from appchain ID to its last finalized epoch index
    mapping(uint256 appchainId => uint256 finalizedEpochCount) public appchainFinalizedEpochCount;

    /// @notice Mapping from user address and appchain ID to user's current stake amount on that specific appchain
    mapping(address user => mapping(uint256 appchainId => uint256 total)) public userAppchainTotal;

    /// @notice Mapping from epoch index, user address, and appchain ID to user's stake amount on that appchain in that epoch
    mapping(uint256 epochIndex => mapping(address user => mapping(uint256 appchainId => uint256 total))) public
        epochUserAppchainTotal;

    /// @notice Mapping from epoch index, user address, and appchain ID to amount staked by user on that appchain during that specific epoch
    mapping(uint256 epochIndex => mapping(address user => mapping(uint256 appchainId => uint256 additions))) public
        epochUserAppchainAdditions;

    /// @notice Mapping from epoch index, user address, and appchain ID to amount withdrawn by user from that appchain during that specific epoch
    mapping(uint256 epochIndex => mapping(address user => mapping(uint256 appchainId => uint256 withdrawals))) public
        epochUserAppchainWithdrawals;

    /// @notice Mapping from user address and appchain ID to their last finalized epoch index for that specific appchain
    mapping(address user => mapping(uint256 appchainId => uint256 finalizedEpochCount)) public
        userAppchainFinalizedEpochCount;

    /**
     * @notice Struct for claiming rewards
     * @param epochIndex The epoch index to claim rewards from
     * @param poolAddress The address of the pool to claim from
     */
    struct ClaimRequest {
        uint256 epochIndex;
        address poolAddress;
    }

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

    /// @notice Mapping from epoch index to total pro-rata stake share for that epoch (weighted by time remaining)
    mapping(uint256 epochIndex => uint256 stakeShare) public epochStakeShare;

    /// @notice Mapping from epoch index and user address to user's pro-rata stake share for that epoch (weighted by time remaining)
    mapping(uint256 epochIndex => mapping(address user => uint256 stakeShare)) public epochUserStakeShare;

    /**
     * @notice Emitted when a user stakes SYND tokens
     * @param epochIndex The epoch index when staking occurred
     * @param user The address of the user who staked
     * @param amount The amount of SYND tokens staked
     * @param appchainId The ID of the appchain where tokens were staked
     */
    event Stake(uint256 epochIndex, address user, uint256 amount, uint256 appchainId);

    /**
     * @notice Emitted when a user stages a transfer of stake from one appchain to another
     * @param epochIndex The epoch index when the transfer was staged
     * @param user The address of the user who staged the transfer
     * @param amount The amount of SYND tokens being transferred
     * @param fromAppchainId The ID of the source appchain
     * @param toAppchainId The ID of the destination appchain
     */
    event StakeTransfer(uint256 epochIndex, address user, uint256 amount, uint256 fromAppchainId, uint256 toAppchainId);

    /**
     * @notice Emitted when a withdrawal is initialized
     * @param user The address of the user who initiated withdrawal
     * @param appchainId The ID of the appchain where withdrawal was initiated
     * @param amount The amount of SYND tokens to be withdrawn
     */
    event WithdrawalInitialized(address user, uint256 appchainId, uint256 amount);

    /**
     * @notice Emitted when a withdrawal is completed
     * @param user The address of the user who completed withdrawal
     * @param destination The address where tokens were sent
     * @param amount The amount of SYND tokens withdrawn
     */
    event WithdrawalCompleted(address user, address destination, uint256 amount);

    /// @notice Error thrown when attempting to stake or withdraw zero amount
    error InvalidAmount();
    /// @notice Error thrown when attempting to use invalid appchain ID (0)
    error InvalidAppchainId();
    /// @notice Error thrown when user doesn't have sufficient stake for operation
    error InsufficientStake();
    /// @notice Error thrown when attempting to withdraw before withdrawal period is complete
    error WithdrawalNotReady();
    /// @notice Error thrown when withdrawal data is invalid or missing
    error InvalidWithdrawal();
    /// @notice Error thrown when no claims are provided
    error NoClaimsProvided();
    /// @notice Error thrown when input is invalid
    error InvalidInput();
    /// @notice Error thrown when total amount of staking does not match the amount of ETH sent
    error InvalidStakingAmount(uint256 totalAmount, uint256 sentAmount);

    /**
     * @notice Constructs the SyndStaking contract and sets the default admin (owner)
     * @param _defaultAdmin The address to be set as the contract owner
     */
    constructor(address _defaultAdmin) Ownable(_defaultAdmin) {}

    /**
     * @notice Pause the contract, disabling staking and withdrawal operations
     * @dev Only callable by the contract owner. Triggers the emergency stop mechanism.
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @notice Unpause the contract, re-enabling staking and withdrawal operations
     * @dev Only callable by the contract owner. Lifts the emergency stop mechanism.
     */
    function unpause() external onlyOwner {
        _unpause();
    }

    ///////////////////////
    // Staking functions
    ///////////////////////

    /**
     * @notice Stake SYND tokens for a specific appchain
     * @dev Automatically finalizes epochs if needed and calculates pro-rata stake share
     * @param appchainId The ID of the appchain to stake for (must be non-zero)
     */
    function stakeSynd(uint256 appchainId) external payable whenNotPaused {
        _stakeSynd(appchainId, msg.value);
    }

    function _stakeSynd(uint256 appchainId, uint256 amount) internal {
        if (amount == 0) {
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
        uint256 stakeShare = _calculateStakeShare(amount);
        epochStakeShare[epochIndex] += stakeShare;
        epochUserStakeShare[epochIndex][msg.sender] += stakeShare;

        epochAdditions[epochIndex] += amount;
        totalStake += amount;

        epochUserAdditions[epochIndex][msg.sender] += amount;
        userTotal[msg.sender] += amount;

        epochAppchainAdditions[epochIndex][appchainId] += amount;
        appchainTotal[appchainId] += amount;

        epochUserAppchainAdditions[epochIndex][msg.sender][appchainId] += amount;
        userAppchainTotal[msg.sender][appchainId] += amount;

        emit Stake(epochIndex, msg.sender, amount, appchainId);
    }

    /**
     * @notice Stake tokens across multiple appchains in a single transaction
     * @dev Iterates over the provided appchain IDs and amounts, calling `_stakeSynd` for each.
     *      The total sum of all amounts must equal `msg.value` or the transaction reverts.
     * @param appchainIds The list of appchain IDs to stake into
     * @param amounts The list of corresponding stake amounts for each appchain ID
     */
    function stakeMultipleAppchains(uint256[] calldata appchainIds, uint256[] calldata amounts)
        external
        payable
        whenNotPaused
    {
        if (appchainIds.length != amounts.length) {
            revert InvalidInput();
        }
        uint256 totalAmount = 0;
        for (uint256 i = 0; i < appchainIds.length; i++) {
            totalAmount += amounts[i];
            _stakeSynd(appchainIds[i], amounts[i]);
        }
        if (totalAmount != msg.value) {
            revert InvalidStakingAmount(totalAmount, msg.value);
        }
    }

    /**
     * @notice Calculate pro-rata stake share based on time remaining in current epoch
     * @dev Stake share is weighted by the fraction of epoch remaining when staking occurs
     * @param amount The amount of tokens being staked
     * @return The calculated stake share for the current epoch
     */
    function _calculateStakeShare(uint256 amount) internal view returns (uint256) {
        return (amount * (getEpochEnd(getCurrentEpoch()) - block.timestamp)) / EPOCH_DURATION;
    }

    /**
     * @notice Stage a transfer of stake from one appchain to another that will go into effect in the next epoch
     * @dev Moves existing stake between appchains without affecting total user stake
     * @param fromAppchainId The ID of the source appchain
     * @param toAppchainId The ID of the destination appchain
     * @param amount The amount of tokens to restake
     */
    function stageStakeTransfer(uint256 fromAppchainId, uint256 toAppchainId, uint256 amount)
        external
        payable
        nonReentrant
        whenNotPaused
    {
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

        emit StakeTransfer(epochIndex, msg.sender, amount, fromAppchainId, toAppchainId);
    }

    ///////////////////////
    // Finalization functions
    ///////////////////////

    /**
     * @notice Finalize global epoch accounting up to the current epoch
     * @dev Updates epoch totals by incorporating additions and withdrawals
     */
    function finalizeEpochs() public {
        uint256 currentEpoch = getCurrentEpoch();
        while (finalizedEpochCount < currentEpoch) {
            epochTotal[finalizedEpochCount] += totalStake;
            epochTotal[finalizedEpochCount] += epochWithdrawals[finalizedEpochCount];
            epochTotal[finalizedEpochCount] -= epochAdditions[finalizedEpochCount];
            finalizedEpochCount++;
        }
    }

    /**
     * @notice Finalize user epoch accounting up to the current epoch
     * @dev Updates user epoch totals by incorporating additions and withdrawals
     * @param user The address of the user to finalize epochs for
     */
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

    /**
     * @notice Finalize appchain epoch accounting up to the current epoch
     * @dev Updates appchain epoch totals by incorporating additions and withdrawals
     * @param appchainId The ID of the appchain to finalize epochs for
     */
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

    /**
     * @notice Finalize user-appchain epoch accounting up to the current epoch
     * @dev Updates user-appchain epoch totals by incorporating additions and withdrawals
     * @param user The address of the user
     * @param appchainId The ID of the appchain
     */
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

    /**
     * @notice Initialize a withdrawal for a specific amount from a specific appchain
     * @dev Withdrawals are delayed by one epoch for security. This function marks
     * the withdrawal but doesn't transfer tokens immediately.
     * @param appchainId The ID of the appchain to withdraw from
     * @param amount The amount of tokens to withdraw
     */
    function initializeWithdrawal(uint256 appchainId, uint256 amount) public {
        if (amount == 0) {
            revert InvalidAmount();
        }
        if (appchainId == 0) {
            revert InvalidAppchainId();
        }
        if (amount > userAppchainTotal[msg.sender][appchainId] || amount > userTotal[msg.sender]) {
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

    /**
     * @notice Initialize a withdrawal for the user's entire stake on a specific appchain
     * @dev Convenience function that calls initializeWithdrawal with the user's full stake amount
     * @param appchainId The ID of the appchain to withdraw from
     */
    function initializeWithdrawal(uint256 appchainId) external {
        initializeWithdrawal(appchainId, getWithdrawalAmount(msg.sender, appchainId));
    }

    /**
     * @notice Complete a withdrawal after the required epoch delay
     * @dev Can only be called after the withdrawal epoch has passed
     * @param epochIndex The epoch index when withdrawal was initialized
     * @param destination The address where tokens should be sent
     */
    function withdraw(uint256 epochIndex, address destination) public nonReentrant {
        if (epochIndex >= getCurrentEpoch()) {
            revert WithdrawalNotReady();
        }

        uint256 amount = epochUserWithdrawals[epochIndex][msg.sender];
        if (amount == 0) {
            revert InvalidWithdrawal();
        }

        finalizeUserEpochs(msg.sender);

        epochUserWithdrawals[epochIndex][msg.sender] = 0;

        Address.sendValue(payable(destination), amount);

        emit WithdrawalCompleted(msg.sender, destination, amount);
    }

    ///////////////////////
    // Claim functions
    ///////////////////////

    /**
     * @notice Claim rewards from multiple pools for the caller
     * @dev This function calls the claimFor function on each pool contract
     * @param claims Array of ClaimRequest structs containing claim details
     */
    function claimAllRewards(ClaimRequest[] calldata claims, address destination) external nonReentrant {
        if (claims.length == 0) {
            revert NoClaimsProvided();
        }

        for (uint256 i = 0; i < claims.length; i++) {
            IUserPool(claims[i].poolAddress).claimFor(claims[i].epochIndex, msg.sender, destination);
        }
    }

    ///////////////////////
    // View functions
    ///////////////////////

    /**
     * @notice Get the amount available for withdrawal for a specific user and appchain
     * @param user The address of the user
     * @param appchainId The ID of the appchain
     * @return The amount of tokens available for withdrawal
     */
    function getWithdrawalAmount(address user, uint256 appchainId) public view returns (uint256) {
        return userAppchainTotal[user][appchainId];
    }

    /**
     * @notice Get the total stake amount for a specific user in a specific epoch
     * @dev Returns finalized data if available, otherwise calculates from current state
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @return The user's total stake amount for the specified epoch
     */
    function getUserStake(uint256 epochIndex, address user) public view returns (uint256) {
        if (epochIndex >= userFinalizedEpochCount[user]) {
            return userTotal[user] + epochUserTotal[epochIndex][user] + epochUserWithdrawals[epochIndex][user]
                - epochUserAdditions[epochIndex][user];
        } else {
            return epochUserTotal[epochIndex][user];
        }
    }

    /**
     * @notice Get the pro-rata stake share for a specific user in a specific epoch
     * @dev Combines full stake with pro-rata stake share for complete epoch participation
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @return The user's complete stake share for the specified epoch
     */
    function getUserStakeShare(uint256 epochIndex, address user) public view returns (uint256) {
        return getUserStake(epochIndex, user) + epochUserStakeShare[epochIndex][user];
    }

    /**
     * @notice Get the total stake amount across all users in a specific epoch
     * @dev Returns finalized data if available, otherwise calculates from current state
     * @param epochIndex The epoch index to query
     * @return The total stake amount for the specified epoch
     */
    function getTotalStake(uint256 epochIndex) public view returns (uint256) {
        if (epochIndex >= finalizedEpochCount) {
            return totalStake + epochTotal[epochIndex] + epochWithdrawals[epochIndex] - epochAdditions[epochIndex];
        } else {
            return epochTotal[epochIndex];
        }
    }

    /**
     * @notice Get the total pro-rata stake share across all users in a specific epoch
     * @dev Combines full stake with pro-rata stake share for complete epoch participation
     * @param epochIndex The epoch index to query
     * @return The total complete stake share for the specified epoch
     */
    function getTotalStakeShare(uint256 epochIndex) public view returns (uint256) {
        return getTotalStake(epochIndex) + epochStakeShare[epochIndex];
    }

    /**
     * @notice Get the total stake amount for a specific appchain in a specific epoch
     * @dev Returns finalized data if available, otherwise calculates from current state
     * @param epochIndex The epoch index to query
     * @param appchainId The ID of the appchain
     * @return The total stake amount for the specified appchain and epoch
     */
    function getAppchainStake(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= appchainFinalizedEpochCount[appchainId]) {
            return appchainTotal[appchainId] + epochAppchainTotal[epochIndex][appchainId]
                + epochAppchainWithdrawals[epochIndex][appchainId] - epochAppchainAdditions[epochIndex][appchainId];
        } else {
            return epochAppchainTotal[epochIndex][appchainId];
        }
    }

    /**
     * @notice Get the stake amount for a specific user on a specific appchain in a specific epoch
     * @dev Returns finalized data if available, otherwise calculates from current state
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @param appchainId The ID of the appchain
     * @return The user's stake amount for the specified appchain and epoch
     */
    function getUserAppchainStake(uint256 epochIndex, address user, uint256 appchainId) public view returns (uint256) {
        if (epochIndex >= userAppchainFinalizedEpochCount[user][appchainId]) {
            return userAppchainTotal[user][appchainId] + epochUserAppchainTotal[epochIndex][user][appchainId]
                + epochUserAppchainWithdrawals[epochIndex][user][appchainId]
                - epochUserAppchainAdditions[epochIndex][user][appchainId];
        } else {
            return epochUserAppchainTotal[epochIndex][user][appchainId];
        }
    }

    ///////////////////////
    // Bulk functions
    ///////////////////////

    /**
     * @notice Initialize withdrawals for multiple appchains with specified amounts
     * @dev Reverts if the length of appchainIds and amounts arrays do not match
     * @param appchainIds The array of appchain IDs to initialize withdrawals for
     * @param amounts The array of withdrawal amounts corresponding to each appchain ID
     */
    function initializeWithdrawals(uint256[] calldata appchainIds, uint256[] calldata amounts) external nonReentrant {
        if (appchainIds.length != amounts.length) {
            revert InvalidWithdrawal();
        }

        for (uint256 i = 0; i < appchainIds.length; i++) {
            initializeWithdrawal(appchainIds[i], amounts[i]);
        }
    }

    /**
     * @notice Initialize withdrawals for multiple appchains for the caller, withdrawing the full available amount from each
     * @dev Calls initializeWithdrawal for each appchainId with the full withdrawal amount for msg.sender
     * @param appchainIds The array of appchain IDs to initialize withdrawals for
     */
    function initializeWithdrawals(uint256[] calldata appchainIds) external nonReentrant {
        for (uint256 i = 0; i < appchainIds.length; i++) {
            initializeWithdrawal(appchainIds[i], getWithdrawalAmount(msg.sender, appchainIds[i]));
        }
    }

    /**
     * @notice Withdraws funds for multiple epochs in a single transaction.
     * @dev Iterates through the provided epoch indices, finalizes user epochs, and transfers the total amount to the specified destination.
     *      Reverts if any epoch is not ready for withdrawal or if there is no withdrawal amount for an epoch.
     * @param epochIndices The array of epoch indices to withdraw from.
     * @param destination The address to receive the withdrawn funds.
     */
    function withdrawBulk(uint256[] calldata epochIndices, address destination) external {
        uint256 totalAmount = 0;
        for (uint256 i = 0; i < epochIndices.length; i++) {
            uint256 epochIndex = epochIndices[i];
            if (epochIndex >= getCurrentEpoch()) {
                revert WithdrawalNotReady();
            }

            uint256 amount = epochUserWithdrawals[epochIndex][msg.sender];
            if (amount == 0) {
                revert InvalidWithdrawal();
            }

            finalizeUserEpochs(msg.sender);
            epochUserWithdrawals[epochIndex][msg.sender] = 0;
            totalAmount += amount;
        }

        Address.sendValue(payable(destination), totalAmount);

        emit WithdrawalCompleted(msg.sender, destination, totalAmount);
    }
}
