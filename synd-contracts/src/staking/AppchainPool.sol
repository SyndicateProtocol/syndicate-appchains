// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RewardPoolBase} from "./RewardPoolBase.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./interfaces/IPool.sol";
import {UD60x18, convert} from "@prb/math/src/UD60x18.sol";
import {IEmissionsReceiver} from "./interfaces/IEmissionsReceiver.sol";

/**
 * @title AppchainPool
 * @notice Pool that provides rewards to appchains for specific epochs based on their contribution to the network
 * @dev This contract distributes rewards to appchains with a 1-year linear vesting schedule.
 *      Only the configured rewards receiver for each appchain can claim rewards.
 *
 * Key Features:
 * - 1-year linear vesting for all rewards
 * - Appchain-specific reward distribution
 * - Authorization-based claiming (only configured receivers)
 * - Epoch-based reward accumulation
 *
 * Vesting Schedule:
 * - Rewards vest linearly over 365 days after epoch ends
 * - No vesting until epoch is complete
 * - Full vesting after 1 year from epoch end
 *
 * @dev Inherits from RewardPoolBase for shared reward calculation logic
 */
contract AppchainPool is RewardPoolBase {
    /// @notice The duration of the vesting period in seconds (1 year)
    /// @dev All rewards are subject to this vesting schedule
    uint256 public constant VESTING_DURATION = 365 days;

    IEmissionsReceiver public immutable emissionsReceiver;

    /// @notice Mapping from epoch index and appchain ID to amount already claimed
    /// @dev Tracks claimed amounts to prevent double-claiming
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 claimed)) public claimed;

    /// @notice Error thrown when the caller is not the configured rewards receiver for the appchain
    /// @dev Only the address returned by appchainRewardsReceiver() can claim for an appchain
    error InvalidClaimer();

    // @notice Error thrown when a contract does not have code
    error NoCode();

    /**
     * @notice Constructor to initialize the AppchainPool
     * @param admin The address to be granted admin privileges
     * @param staking The address of the SyndStaking contract
     * @param gas The address of the gas data provider contract
     * @param emissions The address of the emissions receiver contract
     */
    constructor(address admin, address staking, address gas, address emissions) RewardPoolBase(admin, staking, gas) {
        require(emissions.code.length > 0, NoCode());
        emissionsReceiver = IEmissionsReceiver(emissions);
    }

    /**
     * @notice Claim rewards for a specific appchain in a specific epoch
     * @dev Only the configured rewards receiver for the appchain can claim.
     *      Rewards are subject to the 1-year vesting schedule.
     * @param epochIndex The epoch index to claim rewards for
     * @param appchainId The appchain ID to claim rewards for
     * @param destination The destination address to send the rewards to
     * @custom:security Only the configured rewards receiver can call this function
     */
    function claim(uint256 epochIndex, uint256 appchainId, address destination) external nonReentrant {
        // Only the configured receiver can claim.
        require(msg.sender == emissionsReceiver.appchainEmissionsReceiver(appchainId), InvalidClaimer());

        require(destination != address(0), InvalidDestination());

        uint256 amount = getClaimableAmount(epochIndex, appchainId);
        if (amount == 0) revert ClaimNotAvailable();

        claimed[epochIndex][appchainId] += amount;
        Address.sendValue(payable(destination), amount);

        emit ClaimSuccess(epochIndex, appchainId, destination, amount);
    }

    /**
     * @notice Calculates the claimable reward amount for an appchain in a specific epoch (with vesting)
     * @dev Returns the amount of rewards the appchain can claim for the given epoch, considering vesting schedule.
     *      This is the actual amount that can be claimed after applying vesting and subtracting already claimed amounts.
     * @dev Uses integer division which may result in small precision loss (dust) when
     *      reward amounts are not evenly divisible. This is expected behavior to maintain
     *      gas efficiency. Dust amounts are typically negligible in normal operations.
     * @param epochIndex The epoch index for which to calculate claimable rewards
     * @param appchainId The ID of the appchain
     * @return The claimable reward amount for the appchain in the epoch
     * @custom:example If 1000 tokens earned, 6 months passed (50% vested), 200 already claimed, returns 300
     */
    function getClaimableAmount(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        uint256 fullReward = getAppchainTotalReward(epochIndex, appchainId);
        if (fullReward == 0) {
            return 0;
        }

        uint256 alreadyClaimed = claimed[epochIndex][appchainId];
        uint256 vestedAmount = getVestedAmount(epochIndex, fullReward);

        return vestedAmount - alreadyClaimed;
    }

    /**
     * @notice Calculates the vested amount for a given epoch and reward amount
     * @dev Implements linear vesting over 1 year (365 days) after the epoch ends.
     *      No vesting occurs until the epoch is complete.
     * @param epochIndex The epoch index
     * @param fullReward The full reward amount for the epoch
     * @return The vested amount as of the current time
     * @custom:example If epoch ended 6 months ago and fullReward is 1000, returns 500
     * @custom:example If epoch hasn't ended yet, returns 0
     * @custom:example If epoch ended 2 years ago, returns fullReward (fully vested)
     */
    function getVestedAmount(uint256 epochIndex, uint256 fullReward) public view returns (uint256) {
        uint256 epochEnd = stakingContract.getEpochEnd(epochIndex);

        uint256 currentTime = block.timestamp;
        if (epochEnd >= currentTime) {
            return 0;
        }

        uint256 timeElapsed = currentTime - epochEnd;

        // If vesting period is complete, return full amount
        if (timeElapsed >= VESTING_DURATION) {
            return fullReward;
        }

        // Calculate linear vesting: (timeElapsed / VESTING_DURATION) * fullReward
        uint256 vestedAmount = (fullReward * timeElapsed) / VESTING_DURATION;

        return vestedAmount;
    }
}
