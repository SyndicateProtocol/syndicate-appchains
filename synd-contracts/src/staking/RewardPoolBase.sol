// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev Abstract base that holds:
 *  - weights (feeMultiplier/stakeMultiplier/decayFactor)
 *  - references (staking, gas provider)
 *  - shared mappings: epochTotal, diminishing-factor caches
 *  - deposit()
 *  - diminishing-factor math and caching
 *  - a helper to compute appchain total reward for an epoch
 *
 * Children implement the claim surface and "claimed" accounting.
 */
abstract contract RewardPoolBase is ReentrancyGuard, Ownable {
    // Weights and decay
    UD60x18 public feeMultiplier = ud(0.4e18);
    UD60x18 public stakeMultiplier = ud(0.2e18);
    UD60x18 public decayFactor = ud(2e18);

    ISyndStaking public immutable stakingContract;
    IGasDataProvider public immutable gasDataProvider;

    // Total reward deposited per epoch (in wei)
    mapping(uint256 epochIndex => uint256 epochTotal) public epochTotal;

    // Cache: per-epoch/appchain diminishing factor
    mapping(uint256 epochIndex => mapping(uint256 appchainId => UD60x18 diminishingFactor)) internal diminishingFactor;

    // Cache: per-epoch sum of diminishing factors across all appchains
    mapping(uint256 epochIndex => UD60x18 epochTotalDiminishingFactor) internal epochTotalDiminishingFactor;

    // Events / Errors reused by children
    event EpochDeposit(uint256 indexed epochIndex, uint256 amount);
    event ClaimSuccess(
        uint256 indexed epochIndex, uint256 indexed appchainId, address indexed destination, uint256 amount
    );

    error ClaimNotAvailable();
    error ZeroAddress();

    constructor(address _defaultAdmin, address _staking, address _gas) Ownable(_defaultAdmin) {
        if (_staking == address(0) || _gas == address(0)) revert ZeroAddress();
        stakingContract = ISyndStaking(_staking);
        gasDataProvider = IGasDataProvider(_gas);
    }

    // Anyone can fund an epoch
    function _deposit(uint256 epochIndex) internal {
        uint256 amount = msg.value;
        epochTotal[epochIndex] += amount;
        emit EpochDeposit(epochIndex, amount);
    }

    // ----- Shared math helpers (internal) -----

    function _preChecks(uint256 epochIndex) internal view {
        // must be a past epoch with funding
        if (epochTotal[epochIndex] == 0 || stakingContract.getCurrentEpoch() <= epochIndex) {
            revert ClaimNotAvailable();
        }
    }

    function _getAppchainDiminishingFactor(
        uint256 epochIndex,
        uint256 appchainId,
        UD60x18 totalStake,
        UD60x18 totalGasFees
    ) internal returns (UD60x18) {
        UD60x18 cached = diminishingFactor[epochIndex][appchainId];
        if (!cached.isZero()) return cached;

        UD60x18 appchainStake = convert(stakingContract.getAppchainStake(epochIndex, appchainId));
        UD60x18 appchainGasFees = convert(gasDataProvider.getAppchainGasFees(epochIndex, appchainId));

        UD60x18 feeShare = appchainGasFees.mul(feeMultiplier).div(totalGasFees);
        UD60x18 stakeShare = appchainStake.mul(stakeMultiplier).div(totalStake);
        UD60x18 dominance = feeShare.add(stakeShare);

        UD60x18 df = (convert(1).add(decayFactor.mul(dominance))).ln();
        if (df.isZero()) return convert(0);

        diminishingFactor[epochIndex][appchainId] = df;
        return df;
    }

    function _getAllAppchainsDiminishingFactor(uint256 epochIndex, UD60x18 totalStake, UD60x18 totalGasFees)
        internal
        returns (UD60x18)
    {
        UD60x18 cached = epochTotalDiminishingFactor[epochIndex];
        if (!cached.isZero()) return cached;

        uint256[] memory ids = gasDataProvider.getActiveAppchainIds(epochIndex);
        UD60x18 sum = convert(0);
        for (uint256 i = 0; i < ids.length;) {
            sum = sum.add(_getAppchainDiminishingFactor(epochIndex, ids[i], totalStake, totalGasFees));
            unchecked {
                ++i;
            }
        }
        epochTotalDiminishingFactor[epochIndex] = sum;
        return sum;
    }

    /**
     * @dev Returns the total reward (wei) assigned to a specific appchain for an epoch
     *      BEFORE any further per-user or per-receiver splitting.
     *      Reverts with ClaimNotAvailable when data is not ready.
     */
    function _computeAppchainTotalReward(uint256 epochIndex, uint256 appchainId) internal returns (uint256) {
        _preChecks(epochIndex);

        UD60x18 poolAmount = convert(epochTotal[epochIndex]);

        UD60x18 totalStake = convert(stakingContract.getTotalStake(epochIndex));
        if (totalStake.isZero()) return 0;

        UD60x18 totalGasFees = convert(gasDataProvider.getTotalGasFees(epochIndex));
        if (totalGasFees.isZero()) return 0;

        UD60x18 df = _getAppchainDiminishingFactor(epochIndex, appchainId, totalStake, totalGasFees);
        if (df.isZero()) return 0;

        UD60x18 dfSum = _getAllAppchainsDiminishingFactor(epochIndex, totalStake, totalGasFees);
        if (dfSum.isZero()) return 0;

        return convert(poolAmount.mul(df).div(dfSum));
    }

    // Admin setters (shared)
    function setFeeMultiplier(uint256 _fee) external onlyOwner {
        feeMultiplier = ud(_fee);
    }

    function setStakeMultiplier(uint256 _stake) external onlyOwner {
        stakeMultiplier = ud(_stake);
    }

    function setDecayFactor(uint256 _decay) external onlyOwner {
        decayFactor = ud(_decay);
    }
}
