// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./SyndStaking.sol";

/**
 * @title BasePool
 * @notice Contract for distributing rewards to stakers based on their stake proportion
 * @dev Implements a pro-rata reward distribution system where users can claim rewards
 * based on their stake share in a specific epoch
 *
 * This contract allows a designated depositor to deposit rewards for specific epochs,
 * and stakers can claim their proportional share of those rewards based on their
 * stake in the SyndStaking contract.
 */
contract BasePool {
    /// @notice The address authorized to deposit rewards into the pool
    address public immutable depositor;

    /// @notice Reference to the SyndStaking contract for stake queries
    ISyndStaking public immutable stakingContract;

    /// @notice Mapping from epoch index to total reward amount deposited for that epoch
    mapping(uint256 epochIndex => uint256 total) public epochTotal;
    /// @notice Mapping from epoch index and user address to whether they have claimed rewards
    mapping(uint256 epochIndex => mapping(address user => bool claimed)) public claimed;

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

    /// @notice Error thrown when a user tries to claim rewards without having any stake
    error NotStaked();
    /// @notice Error thrown when a user tries to claim rewards they have already claimed
    error AlreadyClaimed();
    /// @notice Error thrown when trying to claim from an epoch with no rewards
    error ClaimNotAvailable();
    /// @notice Error thrown when a non-depositor tries to deposit rewards
    error DepositNotAllowed();

    /**
     * @notice Constructor to initialize the pool with staking contract and depositor
     * @param _stakingContract Address of the SyndStaking contract
     * @param _depositor Address authorized to deposit rewards
     */
    constructor(address _stakingContract, address _depositor) {
        stakingContract = ISyndStaking(_stakingContract);
        depositor = _depositor;
    }

    /// @notice Restricts function access to the authorized depositor only
    modifier onlyDepositor() {
        if (msg.sender != depositor) {
            revert DepositNotAllowed();
        }
        _;
    }

    /**
     * @notice Deposit rewards for a specific epoch
     * @dev Only the designated depositor can call this function
     * @param epochIndex The epoch index for which rewards are being deposited
     */
    function deposit(uint256 epochIndex) external payable onlyDepositor {
        uint256 amount = msg.value;
        epochTotal[epochIndex] += amount;

        emit EpochDeposit(epochIndex, amount);
    }

    /**
     * @notice Claim rewards for a specific epoch based on user's stake proportion
     * @dev Users can only claim once per epoch and must have stake in the SyndStaking contract
     * @param epochIndex The epoch index for which to claim rewards
     * @param destination The address where rewards should be sent
     */
    function claim(uint256 epochIndex, address destination) external {
        if (epochTotal[epochIndex] == 0) {
            revert ClaimNotAvailable();
        }

        if (claimed[epochIndex][msg.sender]) {
            revert AlreadyClaimed();
        }
        claimed[epochIndex][msg.sender] = true;

        uint256 amount = stakingContract.getUserStake(epochIndex, msg.sender);
        if (amount == 0) {
            revert NotStaked();
        }
        uint256 total = stakingContract.getTotalStake(epochIndex);

        // Calculate the amount of synd to claim
        uint256 claimAmount = (epochTotal[epochIndex] * amount) / total;

        // Send synd to user
        payable(destination).transfer(claimAmount);

        emit ClaimSuccess(epochIndex, msg.sender, destination, claimAmount);
    }
}
