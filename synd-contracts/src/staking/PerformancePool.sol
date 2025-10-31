// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import {RewardPoolBase} from "./RewardPoolBase.sol";
import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IUserPool} from "./interfaces/IPool.sol";

/**
 * @title PerformancePool
 * @notice Pool that distributes rewards to users based on their stake performance across appchains
 * @dev This contract allows users to claim rewards proportional to their stake share on specific appchains.
 *      Rewards are distributed based on both user stake and appchain performance metrics.
 * @dev Inherits from RewardPoolBase for shared reward calculation logic
 */
contract PerformancePool is IUserPool, RewardPoolBase {
    /// @notice Mapping from epoch index, appchain ID, and user address to amount already claimed
    /// @dev Tracks claimed amounts per user per appchain per epoch to prevent double-claiming
    mapping(uint256 epochIndex => mapping(uint256 appchainId => mapping(address user => uint256 claimed))) public
        claimed;

    /// @notice Error thrown when caller is not the authorized SyndStaking contract
    /// @dev Only the SyndStaking contract can call claimFor() function
    error UnauthorizedCaller();

    /**
     * @notice Constructor to initialize the PerformancePool
     * @param admin The address to be granted admin privileges
     * @param staking The address of the SyndStaking contract
     * @param gas The address of the gas data provider contract
     */
    constructor(address admin, address staking, address gas) RewardPoolBase(admin, staking, gas) {}

    /**
     * @notice Modifier that restricts function access to the SyndStaking contract only
     * @dev Used for claimFor() function to ensure only authorized contract can claim on behalf of users
     */
    modifier onlyStakingContract() {
        if (msg.sender != address(stakingContract)) {
            revert UnauthorizedCaller();
        }
        _;
    }

    /**
     * @notice Claim rewards for a specific user on a specific appchain in a specific epoch
     * @dev Users can claim their proportional share of rewards based on their stake on the appchain.
     *      Rewards are calculated as: (user stake / total appchain stake) * appchain total rewards
     * @param epochIndex The epoch index to claim rewards for
     * @param destination The destination address to send the rewards to
     * @param appchainId The appchain ID to claim rewards for
     */
    function claim(uint256 epochIndex, address destination, uint256 appchainId) external nonReentrant {
        _claim(epochIndex, msg.sender, destination, appchainId);
    }

    /**
     * @notice Variant of claim that reverts if diminishing factors need to be computed
     * @param epochIndex The epoch index to claim rewards for
     * @param appchainId The appchain ID to claim rewards for
     * @param destination The destination address to send the rewards to
     */
    function claimWithoutCompute(uint256 epochIndex, uint256 appchainId, address destination) external nonReentrant {
        require(remainingAppchainsPlusOne[epochIndex] == 1, RewardComputationNotComplete());
        _claim(epochIndex, msg.sender, destination, appchainId);
    }

    /**
     * @notice Claim rewards on behalf of a user (only callable by SyndStaking contract)
     * @dev This function allows the SyndStaking contract to claim rewards on behalf of users.
     *      This is used when users claim rewards through the main staking interface.
     * @param epochIndex The epoch index to claim rewards for
     * @param user The address of the user to claim rewards for
     * @param destination The destination address to send the rewards to
     * @param appchainId The appchain ID to claim rewards for
     * @custom:security Only the SyndStaking contract can call this function
     */
    function claimFor(uint256 epochIndex, address user, address destination, uint256 appchainId)
        external
        nonReentrant
        onlyStakingContract
    {
        require(remainingAppchainsPlusOne[epochIndex] == 1, RewardComputationNotComplete());
        _claim(epochIndex, user, destination, appchainId);
    }

    /**
     * @notice Internal function to process reward claims
     * @dev Handles the common logic for both claim() and claimFor() functions
     * @param epochIndex The epoch index to claim rewards for
     * @param user The address of the user claiming rewards
     * @param destination The destination address to send the rewards to
     * @param appchainId The appchain ID to claim rewards for
     */
    function _claim(uint256 epochIndex, address user, address destination, uint256 appchainId) internal {
        if (destination == address(0)) {
            revert InvalidDestination();
        }

        if (remainingAppchainsPlusOne[epochIndex] != 1) {
            computeDiminishingFactors(epochIndex, 0);
        }

        uint256 amount = getClaimableAmount(epochIndex, user, appchainId);
        if (amount == 0) revert ClaimNotAvailable();

        claimed[epochIndex][appchainId][user] += amount;
        Address.sendValue(payable(destination), amount);

        emit ClaimSuccess(epochIndex, appchainId, destination, amount);
    }

    /**
     * @notice Calculates the claimable reward amount for a user in a specific epoch
     * @dev Returns the amount of rewards the user can claim for the given epoch, based on their stake share and any previously claimed amount.
     *      The calculation is: (user stake on appchain / total appchain stake) * appchain total rewards - already claimed
     * @dev Uses integer division which may result in small precision loss (dust) when
     *      reward amounts are not evenly divisible. This is expected behavior to maintain
     *      gas efficiency. Dust amounts are typically negligible in normal operations.
     * @param epochIndex The epoch index to query
     * @param user The address of the user
     * @param appchainId The appchain id to query
     * @return The amount of rewards claimable by the user for the specified epoch
     * @custom:example If user has 10% of appchain stake and appchain earned 1000 tokens, user can claim 100 tokens
     * @custom:example If user already claimed 50 tokens, returns 50 (remaining claimable amount)
     */
    function getClaimableAmount(uint256 epochIndex, address user, uint256 appchainId) public view returns (uint256) {
        uint256 appchainTotal = getAppchainTotalReward(epochIndex, appchainId);
        if (appchainTotal == 0) return 0;

        uint256 userStaked = ISyndStaking(address(stakingContract)).getUserStake(epochIndex, user);
        if (userStaked == 0) return 0;

        uint256 appchainStaked = ISyndStaking(address(stakingContract)).getAppchainStake(epochIndex, appchainId);
        if (appchainStaked == 0) return 0;

        uint256 userShare = (appchainTotal * userStaked) / appchainStaked;
        uint256 already = claimed[epochIndex][appchainId][user];
        return userShare > already ? userShare - already : 0;
    }
}
