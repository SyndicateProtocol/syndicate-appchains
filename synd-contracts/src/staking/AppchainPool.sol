// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {IPool} from "./interfaces/IPool.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

// TODO: Reconcile with IGasProvider interface when contract is ready
// Current work in progress https://github.com/SyndicateProtocol/syndicate-appchains/pull/780
interface IGasDataProvider {
    function getAppchainGasFees(uint256 epochIndex, uint256 appchainId) external view returns (uint256);
    function getTotalGasFees(uint256 epochIndex) external view returns (uint256);
    function getActiveAppchainIds(uint256 epochIndex) external view returns (uint256[] memory _chainIDs);
    function getAppchainRewardsReceiver(uint256 epochIndex, uint256 appchainId) external view returns (address);
}

/**
 * @title AppchainPool
 * @notice Contract for distributing rewards to stakers based on appchain stake share
 * @dev Implements a reward distribution system where users can claim rewards
 * based on their appchain stake share
 *
 * This contract allows anyone to deposit rewards for specific epochs,
 * and stakers can claim their rewards based on their appchain stake share.
 */
contract AppchainPool is IPool, ReentrancyGuard, Ownable {
    /// @notice Vesting duration (1 year = 365 days)
    uint256 public constant VESTING_DURATION = 365 days;

    // Weights and decay
    // fee multiplier x = 0.4, stake multiplier y = 0.2
    UD60x18 public feeMultiplier = ud(0.4e18);
    UD60x18 public stakeMultiplier = ud(0.2e18);
    // decay factor = 2
    UD60x18 public decayFactor = ud(2e18);

    /// @notice Reference to the SyndStaking contract for stake queries
    ISyndStaking public immutable stakingContract;

    /// @notice Reference to the GasDataProvider contract for gas data queries
    IGasDataProvider public immutable gasDataProvider;

    /// @notice Mapping from epoch index to total reward amount deposited for that epoch
    mapping(uint256 epochIndex => uint256 total) public epochTotal;

    /// @notice Mapping from epoch index and appchain id to the amount of rewards claimed
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 claimed)) public claimed;

    /// @notice Mapping from epoch index and appchain id to the diminishing factor for that appchain
    mapping(uint256 epochIndex => mapping(uint256 appchainId => UD60x18 diminishingFactor)) public diminishingFactor;

    /// @notice Mapping from epoch index to the total diminishing factor for all appchains
    mapping(uint256 epochIndex => UD60x18 epochTotalDiminishingFactor) public epochTotalDiminishingFactor;

    /**
     * @notice Emitted when rewards are deposited for a specific epoch
     * @param epochIndex The epoch index for which rewards were deposited
     * @param amount The amount of rewards deposited
     */
    event EpochDeposit(uint256 indexed epochIndex, uint256 amount);

    /**
     * @notice Emitted when a user successfully claims their rewards
     * @param epochIndex The epoch index for which rewards were claimed
     * @param appchainId The ID of the appchain
     * @param destination The address where rewards were sent
     * @param amount The amount of rewards claimed
     */
    event ClaimSuccess(
        uint256 indexed epochIndex, uint256 indexed appchainId, address indexed destination, uint256 amount
    );

    /// @notice Error thrown when trying to claim from an epoch with no rewards
    error ClaimNotAvailable();

    /// @notice Error thrown when trying to claim from an invalid sender
    error InvalidClaimer();

    /// @notice Error thrown when trying to claim from a zero address
    error ZeroAddress();

    /**
     * @notice Constructor to initialize the pool with staking contract and depositor
     * @param _defaultAdmin The address to be set as the contract owner
     * @param _stakingContract Address of the SyndStaking contract
     * @param _gasDataProvider Address of the GasDataProvider contract
     */
    constructor(address _defaultAdmin, address _stakingContract, address _gasDataProvider) Ownable(_defaultAdmin) {
        if (_stakingContract == address(0) || _gasDataProvider == address(0)) revert ZeroAddress();

        stakingContract = ISyndStaking(_stakingContract);
        gasDataProvider = IGasDataProvider(_gasDataProvider);
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
     * @notice Claim rewards for a specific epoch based on appchain's stake proportion
     *    @dev Appchains can claim repeatedly until fully claimed; rewards are based on stake/fees for the epoch.
     * @param epochIndex The epoch index for which to claim rewards
     * @param appchainId The ID of the appchain
     * @param destination The address where rewards should be sent
     */
    function claim(uint256 epochIndex, uint256 appchainId, address destination) external nonReentrant {
        if (epochTotal[epochIndex] == 0 || stakingContract.getCurrentEpoch() <= epochIndex) {
            revert ClaimNotAvailable();
        }

        if (msg.sender != gasDataProvider.getAppchainRewardsReceiver(epochIndex, appchainId)) {
            revert InvalidClaimer();
        }

        uint256 claimAmount = getClaimableAmount(epochIndex, appchainId);
        if (claimAmount == 0) {
            revert ClaimNotAvailable();
        }
        claimed[epochIndex][appchainId] += claimAmount;

        // Send $SYND to destination
        Address.sendValue(payable(destination), claimAmount);

        emit ClaimSuccess(epochIndex, appchainId, destination, claimAmount);
    }

    /**
     * @notice Calculates the full reward amount for an appchain in a specific epoch (without vesting)
     * @dev Returns the total amount of rewards the appchain earned for the given epoch, regardless of vesting
     * RewardAmount = (PoolAmount * AppchainDiminishingFactor) / AllAppchainsDiminishingFactor
     * Claimable = RewardAmount - AlreadyClaimed
     * @param epochIndex The epoch index for which to calculate rewards
     * @param appchainId The ID of the appchain
     * @return amount The full reward amount for the appchain in the epoch
     */
    function getFullRewardAmount(uint256 epochIndex, uint256 appchainId) public returns (uint256) {
        if (epochTotal[epochIndex] == 0 || stakingContract.getCurrentEpoch() <= epochIndex) {
            revert ClaimNotAvailable();
        }

        UD60x18 poolAmount = convert(epochTotal[epochIndex]);

        UD60x18 totalStake = convert(stakingContract.getTotalStake(epochIndex));
        if (totalStake.isZero()) return 0;

        // We need this value to be 0 or the function to revert when gas data is not ready yet
        UD60x18 totalGasFees = convert(gasDataProvider.getTotalGasFees(epochIndex));
        if (totalGasFees.isZero()) return 0;

        UD60x18 appchainDiminishingFactor =
            _getAppchainDiminishingFactor(epochIndex, appchainId, totalStake, totalGasFees);
        if (appchainDiminishingFactor.isZero()) return 0;

        UD60x18 allAppchainsDiminishingFactor = _getAllAppchainsDiminishingFactor(epochIndex, totalStake, totalGasFees);
        if (allAppchainsDiminishingFactor.isZero()) return 0;

        uint256 appchainReward = convert(poolAmount.mul(appchainDiminishingFactor).div(allAppchainsDiminishingFactor));

        return appchainReward;
    }

    /**
     * @notice Calculates the claimable reward amount for an appchain in a specific epoch (with vesting)
     * @dev Returns the amount of rewards the appchain can claim for the given epoch, considering vesting schedule
     * @param epochIndex The epoch index for which to calculate claimable rewards
     * @param appchainId The ID of the appchain
     * @return The claimable reward amount for the appchain in the epoch
     */
    function getClaimableAmount(uint256 epochIndex, uint256 appchainId) public returns (uint256) {
        uint256 fullReward = getFullRewardAmount(epochIndex, appchainId);
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

    /**
     * @dev Computes the appchain’s diminishing factor for an epoch.
     *      Uses stake share and gas-fee share with multipliers, then ln(1 + DECAY_FACTOR * dominance).
     */
    function _getAppchainDiminishingFactor(
        uint256 epochIndex,
        uint256 appchainId,
        UD60x18 totalStake,
        UD60x18 totalGasFees
    ) internal returns (UD60x18) {
        if (!diminishingFactor[epochIndex][appchainId].isZero()) {
            return diminishingFactor[epochIndex][appchainId];
        }

        UD60x18 appchainStake = convert(stakingContract.getAppchainStake(epochIndex, appchainId));
        UD60x18 appchainGasFees = convert(gasDataProvider.getAppchainGasFees(epochIndex, appchainId));

        UD60x18 appchainFeeShare = appchainGasFees.mul(feeMultiplier).div(totalGasFees);
        UD60x18 appchainStakeShare = appchainStake.mul(stakeMultiplier).div(totalStake);
        UD60x18 appchainDominanceFactor = appchainFeeShare.add(appchainStakeShare);
        UD60x18 appchainDiminishingFactor = (convert(1).add(decayFactor.mul(appchainDominanceFactor))).ln();
        if (appchainDiminishingFactor.isZero()) return convert(0);

        // Cache the result
        diminishingFactor[epochIndex][appchainId] = appchainDiminishingFactor;

        return appchainDiminishingFactor;
    }

    /**
     * @dev Computes the sum of the diminishing factors for all appchains for an epoch.
     */
    function _getAllAppchainsDiminishingFactor(uint256 epochIndex, UD60x18 totalStake, UD60x18 totalGasFees)
        internal
        returns (UD60x18)
    {
        // If it's already computed, return the cached result
        if (!epochTotalDiminishingFactor[epochIndex].isZero()) {
            return epochTotalDiminishingFactor[epochIndex];
        }

        uint256[] memory allAppchainIds = gasDataProvider.getActiveAppchainIds(epochIndex);
        UD60x18 allAppchainsDiminishingFactor = convert(0);
        for (uint256 i = 0; i < allAppchainIds.length;) {
            UD60x18 appchainDiminishingFactor =
                _getAppchainDiminishingFactor(epochIndex, allAppchainIds[i], totalStake, totalGasFees);
            allAppchainsDiminishingFactor = allAppchainsDiminishingFactor.add(appchainDiminishingFactor);
            unchecked {
                ++i;
            }
        }

        // Cache the result
        epochTotalDiminishingFactor[epochIndex] = allAppchainsDiminishingFactor;

        return allAppchainsDiminishingFactor;
    }

    /////// Setters for multipliers and decay factor ///////

    /**
     * @notice Set the fee multiplier
     * @param _feeMultiplier The new fee multiplier
     * @dev The fee multiplier is divided by 1e18
     * eg. If we want to set 0.4, we need to call with 0.4e18
     * @dev Changes will take effect for the next epoch
     */
    function setFeeMultiplier(uint256 _feeMultiplier) external onlyOwner {
        feeMultiplier = ud(_feeMultiplier);
    }

    /**
     * @notice Set the stake multiplier
     * @param _stakeMultiplier The new stake multiplier
     * @dev The stake multiplier is divided by 1e18
     * eg. If we want to set 0.2, we need to call with 0.2e18
     * @dev Changes will take effect for the next epoch
     */
    function setStakeMultiplier(uint256 _stakeMultiplier) external onlyOwner {
        stakeMultiplier = ud(_stakeMultiplier);
    }

    /**
     * @notice Set the decay factor
     * @param _decayFactor The new decay factor
     * @dev The decay factor is divided by 1e18
     * eg. If we want to set 2, we need to call with 2e18
     * @dev Changes will take effect for the next epoch
     */
    function setDecayFactor(uint256 _decayFactor) external onlyOwner {
        decayFactor = ud(_decayFactor);
    }
}
