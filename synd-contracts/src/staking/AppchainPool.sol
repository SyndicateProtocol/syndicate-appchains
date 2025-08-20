// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./SyndStaking.sol";
import {IPool} from "./IPool.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";

/**
 * @title AppchainPool
 * @notice Contract for distributing rewards to stakers based on appchain stake share
 * @dev Implements a reward distribution system where users can claim rewards
 * based on their appchain stake share
 *
 * This contract allows anyone to deposit rewards for specific epochs,
 * and stakers can claim their rewards based on their appchain stake share.
 */
contract AppchainPool is IPool {
    /// @notice Reference to the SyndStaking contract for stake queries
    ISyndStaking public immutable stakingContract;

    /// @notice Mapping from epoch index to total reward amount deposited for that epoch
    mapping(uint256 epochIndex => uint256 total) public epochTotal;

    /// @notice Mapping from epoch index and user address to the amount of rewards claimed
    mapping(uint256 epochIndex => mapping(address user => uint256 claimed)) public claimed;

    /**
     * @notice Emitted when rewards are deposited for a specific epoch
     * @param epochIndex The epoch index for which rewards were deposited
     * @param amount The amount of rewards deposited
     */
    event EpochDeposit(uint256 epochIndex, uint256 amount);

    /**
     * @notice Emitted when a user successfully claims their rewards
     * @param epochIndex The epoch index for which rewards were claimed
     * @param user The address of the user who claimed rewards
     * @param destination The address where rewards were sent
     * @param amount The amount of rewards claimed
     */
    event ClaimSuccess(uint256 epochIndex, address user, address destination, uint256 amount);

    /// @notice Error thrown when trying to claim from an epoch with no rewards
    error ClaimNotAvailable();

    /**
     * @notice Constructor to initialize the pool with staking contract and depositor
     * @param _stakingContract Address of the SyndStaking contract
     */
    constructor(address _stakingContract) {
        stakingContract = ISyndStaking(_stakingContract);
    }

    /**
     * @notice Deposit rewards for a specific epoch
     * @dev Since rewards are additive, we dont care who deposits
     * @param epochIndex The epoch index for which rewards are being deposited
     */
    function deposit(uint256 epochIndex) external payable {
        uint256 amount = msg.value;
        epochTotal[epochIndex] += amount;

        emit EpochDeposit(epochIndex, amount);
    }

    /**
     * @notice Claim rewards for a specific epoch based on user's stake proportion for a specific appchain
     * @dev Users can only claim once per epoch and must have stake in the SyndStaking contract
     * @param epochIndex The epoch index for which to claim rewards
     * @param destination The address where rewards should be sent
     * @param appchainId The ID of the appchain
     */
    function claim(uint256 epochIndex, address destination, uint256 appchainId) external {
        if (epochTotal[epochIndex] == 0 || stakingContract.getCurrentEpoch() <= epochIndex) {
            revert ClaimNotAvailable();
        }

        uint256 claimAmount = getClaimableAmount(epochIndex, msg.sender, appchainId);
        if (claimAmount == 0) {
            revert ClaimNotAvailable();
        }
        claimed[epochIndex][msg.sender] += claimAmount;

        // Send synd to destination
        Address.sendValue(payable(destination), claimAmount);

        emit ClaimSuccess(epochIndex, msg.sender, destination, claimAmount);
    }

    /**
     * @notice Calculates the claimable reward amount for a user in a specific epoch for a specific appchain
     * @dev Returns the amount of rewards the user can claim for the given epoch, based on their stake share and any previously claimed amount.
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @param appchainId The ID of the appchain
     * @return The amount of rewards claimable by the user for the specified epoch
     */
    function getClaimableAmount(uint256 epochIndex, address user, uint256 appchainId) public view returns (uint256) {
        uint256 appchain_pool_amount = epochTotal[epochIndex];
        if (appchain_pool_amount == 0) {
            return 0;
        }

        uint256 user_appchain_stake = stakingContract.getUserAppchainStake(epochIndex, user, appchainId);
        if (user_appchain_stake == 0) {
            return 0;
        }

        uint256 total_appchain_stake = stakingContract.getAppchainStake(epochIndex, appchainId);
        if (total_appchain_stake == 0) {
            return 0;
        }

        uint256 user_appchain_reward = (appchain_pool_amount * user_appchain_stake) / total_appchain_stake;
        return user_appchain_reward - claimed[epochIndex][user];
    }
}
