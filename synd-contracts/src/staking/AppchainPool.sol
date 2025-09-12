// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {RewardPoolBase} from "./RewardPoolBase.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IPool} from "./interfaces/IPool.sol";
import {UD60x18, convert} from "@prb/math/src/UD60x18.sol";

contract AppchainPool is IPool, RewardPoolBase {
    /// @notice The duration of the vesting period in seconds
    uint256 public constant VESTING_DURATION = 365 days;

    // amount already claimed per epoch/appchain
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 claimed)) public claimed;

    error InvalidClaimer();

    constructor(address admin, address staking, address gas) RewardPoolBase(admin, staking, gas) {}

    function deposit(uint256 epochIndex) external payable override nonReentrant {
        _deposit(epochIndex);
    }

    function claim(uint256 epochIndex, uint256 appchainId, address destination) external nonReentrant {
        _preChecks(epochIndex);

        // Only the configured receiver can claim
        if (msg.sender != IGasDataProvider(address(gasDataProvider)).getAppchainRewardsReceiver(epochIndex, appchainId))
        {
            revert InvalidClaimer();
        }
        if (destination == address(0)) {
            revert InvalidDestination();
        }

        uint256 amount = getClaimableAmount(epochIndex, appchainId);
        if (amount == 0) revert ClaimNotAvailable();

        claimed[epochIndex][appchainId] += amount;
        Address.sendValue(payable(destination), amount);

        emit ClaimSuccess(epochIndex, appchainId, destination, amount);
    }

    /**
     * @notice Calculates the full reward amount for an appchain in a specific epoch (without vesting)
     * @dev Returns the total amount of rewards the appchain earned for the given epoch, regardless of vesting
     * @param epochIndex The epoch index for which to calculate rewards
     * @param appchainId The ID of the appchain
     * @return amount The full reward amount for the appchain in the epoch
     */
    function getFullRewardAmount(uint256 epochIndex, uint256 appchainId) public returns (uint256) {
        return _computeAppchainTotalReward(epochIndex, appchainId);
    }

    /**
     * @notice Calculates the claimable reward amount for an appchain in a specific epoch (with vesting)
     * @dev Returns the amount of rewards the appchain can claim for the given epoch, considering vesting schedule
     * @dev Uses integer division which may result in small precision loss (dust) when
     *      reward amounts are not evenly divisible. This is expected behavior to maintain
     *      gas efficiency. Dust amounts are typically negligible in normal operations.
     * @param epochIndex The epoch index for which to calculate claimable rewards
     * @param appchainId The ID of the appchain
     * @return The claimable reward amount for the appchain in the epoch
     */
    function getClaimableAmount(uint256 epochIndex, uint256 appchainId) public returns (uint256) {
        uint256 fullReward = _computeAppchainTotalReward(epochIndex, appchainId);
        if (fullReward == 0) {
            return 0;
        }

        uint256 alreadyClaimed = claimed[epochIndex][appchainId];
        uint256 vestedAmount = getVestedAmount(epochIndex, fullReward);

        return vestedAmount - alreadyClaimed;
    }

    /**
     * @notice Calculates the vested amount for a given epoch and reward amount
     * @dev Implements linear vesting over 1 year (365 days) after the epoch ends
     * @param epochIndex The epoch index
     * @param fullReward The full reward amount for the epoch
     * @return The vested amount as of the current time
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
