// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./ISyndStaking.sol";
import {IPool} from "./IPool.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";

interface AppchainFactory {
    function getAppchainIds() external view returns (uint256[] memory _chainIDs);
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
contract AppchainPool is IPool {
    // Weights and decay
    // fee multiplier x = 0.4, stake multiplier y = 0.2
    UD60x18 immutable FEE_MULTIPLIER = ud(0.4e18);
    UD60x18 immutable STAKING_MULTIPLIER = ud(0.2e18);
    // decay factor = 2
    UD60x18 immutable DECAY_FACTOR = ud(2e18);

    /// @notice Reference to the SyndStaking contract for stake queries
    ISyndStaking public immutable stakingContract;

    /// @notice Reference to the AppchainFactory contract for appchain queries
    AppchainFactory public immutable appchainFactory;

    /// @notice Mapping from epoch index to total reward amount deposited for that epoch
    mapping(uint256 epochIndex => uint256 total) public epochTotal;

    /// @notice Mapping from epoch index and appchain id to the amount of rewards claimed
    mapping(uint256 epochIndex => mapping(uint256 appchainId => uint256 claimed)) public claimed;

    /**
     * @notice Emitted when rewards are deposited for a specific epoch
     * @param epochIndex The epoch index for which rewards were deposited
     * @param amount The amount of rewards deposited
     */
    event EpochDeposit(uint256 epochIndex, uint256 amount);

    /**
     * @notice Emitted when a user successfully claims their rewards
     * @param epochIndex The epoch index for which rewards were claimed
     * @param appchainId The ID of the appchain
     * @param destination The address where rewards were sent
     * @param amount The amount of rewards claimed
     */
    event ClaimSuccess(uint256 epochIndex, uint256 appchainId, address destination, uint256 amount);

    /// @notice Error thrown when trying to claim from an epoch with no rewards
    error ClaimNotAvailable();

    /**
     * @notice Constructor to initialize the pool with staking contract and depositor
     * @param _stakingContract Address of the SyndStaking contract
     */
    constructor(address _appchainFactory, address _stakingContract) {
        appchainFactory = AppchainFactory(_appchainFactory);
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
     * @notice Claim rewards for a specific epoch based on appchain's stake proportion
     * @dev Appchains can only claim once per epoch and must have stake in the SyndStaking contract
     * @param epochIndex The epoch index for which to claim rewards
     * @param destination The address where rewards should be sent
     * @param appchainId The ID of the appchain
     */
    function claim(uint256 epochIndex, address destination, uint256 appchainId) external {
        if (epochTotal[epochIndex] == 0 || stakingContract.getCurrentEpoch() <= epochIndex) {
            revert ClaimNotAvailable();
        }

        uint256 claimAmount = getClaimableAmount(epochIndex, appchainId);
        if (claimAmount == 0) {
            revert ClaimNotAvailable();
        }
        claimed[epochIndex][appchainId] += claimAmount;

        // Send synd to destination
        Address.sendValue(payable(destination), claimAmount);

        emit ClaimSuccess(epochIndex, appchainId, destination, claimAmount);
    }

    /**
     * @notice Calculates the claimable reward amount for an appchain in a specific epoch
     * @dev Returns the amount of rewards the appchain can claim for the given epoch, based on their stake share and any previously claimed amount.
     * @param epochIndex The epoch index to query
     * @param appchainId The ID of the appchain
     * @return The amount of rewards claimable by the appchain for the specified epoch
     */
    function getClaimableAmount(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        UD60x18 poolAmount = convert(epochTotal[epochIndex]);
        if (poolAmount.isZero()) return 0;

        /// TOTAL STAKING & GAS FEES
        UD60x18 totalStake = convert(stakingContract.getTotalStake(epochIndex));
        if (totalStake.isZero()) return 0;

        UD60x18 totalGasFees = convert(100); // TODO: get from real gas contract - ask Jorge
        if (totalGasFees.isZero()) return 0;

        UD60x18 appchainDiminishingFactor =
            _getAppchainDiminishingFactor(epochIndex, appchainId, totalStake, totalGasFees);
        if (appchainDiminishingFactor.isZero()) return 0;
        UD60x18 allAppchainsDiminishingFactor = _getAllAppchainsDiminishingFactor(epochIndex, totalStake, totalGasFees);
        if (allAppchainsDiminishingFactor.isZero()) return 0;

        uint256 appchainReward = convert(poolAmount.mul(appchainDiminishingFactor).div(allAppchainsDiminishingFactor));
        uint256 alreadyClaimed = claimed[epochIndex][appchainId];

        return appchainReward > alreadyClaimed ? appchainReward - alreadyClaimed : 0;
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
    ) internal view returns (UD60x18) {
        UD60x18 appchainStake = convert(stakingContract.getAppchainStake(epochIndex, appchainId));
        UD60x18 appchainGasFees = convert(20); // TODO: get from real gas contract - ask Jorge

        UD60x18 appchainFeeShare = appchainGasFees.mul(FEE_MULTIPLIER).div(totalGasFees);
        UD60x18 appchainStakeShare = appchainStake.mul(STAKING_MULTIPLIER).div(totalStake);
        UD60x18 appchainDominanceFactor = appchainFeeShare.add(appchainStakeShare);
        UD60x18 appchainDiminishingFactor = (convert(1).add(DECAY_FACTOR.mul(appchainDominanceFactor))).ln();
        return appchainDiminishingFactor;
    }

    // TODO: IS THIS TOO EXPENSIVE TO COMPUTE?
    /**
     * @dev Computes the total diminishing factor for all appchains for an epoch.
     */
    function _getAllAppchainsDiminishingFactor(uint256 epochIndex, UD60x18 totalStake, UD60x18 totalGasFees)
        internal
        view
        returns (UD60x18)
    {
        uint256[] memory allAppchainIds = appchainFactory.getAppchainIds();
        UD60x18 allAppchainsDiminishingFactor = convert(0);
        for (uint256 i = 0; i < allAppchainIds.length; i++) {
            UD60x18 appchainDiminishingFactor =
                _getAppchainDiminishingFactor(epochIndex, allAppchainIds[i], totalStake, totalGasFees);
            allAppchainsDiminishingFactor = allAppchainsDiminishingFactor.add(appchainDiminishingFactor);
        }
        return allAppchainsDiminishingFactor;
    }
}
