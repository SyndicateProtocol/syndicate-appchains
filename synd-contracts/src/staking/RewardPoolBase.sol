// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title RewardPoolBase
 * @notice Abstract base contract for reward distribution pools with diminishing returns algorithm
 * @dev This contract provides the core reward calculation logic using a sophisticated diminishing returns algorithm.
 *      It implements a system where appchains with higher dominance (fee share + stake share) receive
 *      proportionally less rewards to prevent centralization and encourage diversity.
 *
 * Key Features:
 * - Diminishing returns algorithm to prevent centralization
 * - Configurable weights for fee and stake multipliers
 * - Caching system for gas-efficient calculations
 * - Shared reward calculation logic across pool types
 *
 * Algorithm Overview:
 * 1. Calculate appchain dominance = (feeShare * feeMultiplier) + (stakeShare * stakeMultiplier)
 * 2. Apply diminishing factor = ln(1 + decayFactor * dominance)
 * 3. Distribute rewards proportionally based on diminishing factors
 *
 * Children implement:
 * - Claim surface (claim/claimFor functions)
 * - "claimed" accounting (tracking already claimed amounts)
 * - Pool-specific reward distribution logic
 */
abstract contract RewardPoolBase is ReentrancyGuard, Ownable {
    /// @notice Weight multiplier for gas fee contribution (40% by default)
    /// @dev Higher values give more weight to gas fee performance in reward calculation
    UD60x18 public feeMultiplier = ud(0.4e18);

    /// @notice Weight multiplier for stake contribution (20% by default)
    /// @dev Higher values give more weight to stake amount in reward calculation
    UD60x18 public stakeMultiplier = ud(0.2e18);

    /// @notice Decay factor for diminishing returns calculation (2.0 by default)
    /// @dev Higher values create stronger diminishing returns effect
    UD60x18 public decayFactor = ud(2e18);

    /// @notice Reference to the SyndStaking contract for stake queries
    ISyndStaking public immutable stakingContract;

    /// @notice Reference to the gas data provider for fee queries
    IGasDataProvider public immutable gasDataProvider;

    /// @notice Total reward deposited per epoch (in wei)
    /// @dev Accumulates all deposits for each epoch
    mapping(uint256 epochIndex => uint256 epochTotal) public epochTotal;

    /// @notice Cache for per-epoch/appchain diminishing factors
    /// @dev Stores calculated diminishing factors to avoid recalculation
    mapping(uint256 epochIndex => mapping(uint256 appchainId => UD60x18 diminishingFactor)) internal diminishingFactor;

    /// @notice Cache for per-epoch sum of diminishing factors across all appchains
    /// @dev Stores total diminishing factor sum for each epoch
    mapping(uint256 epochIndex => UD60x18 epochTotalDiminishingFactor) internal epochTotalDiminishingFactor;

    /// @notice Event emitted when rewards are deposited for an epoch
    /// @param epochIndex The epoch index for which rewards were deposited
    /// @param amount The amount of rewards deposited
    event EpochDeposit(uint256 indexed epochIndex, uint256 amount);

    /// @notice Event emitted when rewards are successfully claimed
    /// @param epochIndex The epoch index for which rewards were claimed
    /// @param appchainId The appchain ID for which rewards were claimed
    /// @param destination The address where rewards were sent
    /// @param amount The amount of rewards claimed
    event ClaimSuccess(
        uint256 indexed epochIndex, uint256 indexed appchainId, address indexed destination, uint256 amount
    );

    /// @notice Error thrown when attempting to claim from an unavailable epoch
    /// @dev Epoch must be past and have funding to be claimable
    error ClaimNotAvailable();

    /// @notice Error thrown when a zero address is provided
    /// @dev Used for constructor validation
    error ZeroAddress();

    /// @notice Error thrown when destination address is zero
    /// @dev Prevents sending rewards to zero address
    error InvalidDestination();

    /**
     * @notice Constructor to initialize the RewardPoolBase
     * @param _defaultAdmin The address to be granted admin privileges
     * @param _staking The address of the SyndStaking contract
     * @param _gas The address of the gas data provider contract
     */
    constructor(address _defaultAdmin, address _staking, address _gas) Ownable(_defaultAdmin) {
        if (_staking == address(0) || _gas == address(0)) revert ZeroAddress();
        stakingContract = ISyndStaking(_staking);
        gasDataProvider = IGasDataProvider(_gas);
    }

    /**
     * @notice Internal function to deposit rewards for an epoch
     * @dev Anyone can fund any epoch. Rewards are additive.
     * @param epochIndex The epoch index to deposit rewards for
     */
    function _deposit(uint256 epochIndex) internal {
        uint256 amount = msg.value;
        epochTotal[epochIndex] += amount;
        emit EpochDeposit(epochIndex, amount);
    }

    /*//////////////////////////////////////////////////////////////
                        SHARED MATH HELPERS (INTERNAL)
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Internal function to perform pre-claim validation checks
     * @dev Ensures epoch is past and has funding before allowing claims
     * @param epochIndex The epoch index to validate
     */
    function _preChecks(uint256 epochIndex) internal view {
        // must be a past epoch with funding
        if (epochTotal[epochIndex] == 0 || stakingContract.getCurrentEpoch() <= epochIndex) {
            revert ClaimNotAvailable();
        }
    }

    /**
     * @notice Calculate the diminishing factor for a specific appchain in an epoch
     * @dev Implements the core diminishing returns algorithm:
     *      1. Calculate fee share and stake share
     *      2. Combine into dominance score
     *      3. Apply diminishing factor = ln(1 + decayFactor * dominance)
     * @param epochIndex The epoch index to calculate for
     * @param appchainId The appchain ID to calculate for
     * @param totalStake Total stake across all appchains
     * @param totalGasFees Total gas fees across all appchains
     * @return The diminishing factor for the appchain
     */
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

    /**
     * @notice Calculate the sum of diminishing factors for all appchains in an epoch
     * @dev Iterates through all active appchains and sums their diminishing factors
     * @param epochIndex The epoch index to calculate for
     * @param totalStake Total stake across all appchains
     * @param totalGasFees Total gas fees across all appchains
     * @return The sum of all appchain diminishing factors
     */
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
     * @notice Calculate the total reward assigned to a specific appchain for an epoch
     * @dev Returns the total reward (wei) assigned to a specific appchain for an epoch
     *      BEFORE any further per-user or per-receiver splitting.
     *      Uses the diminishing returns algorithm to determine proportional rewards.
     * @param epochIndex The epoch index to calculate for
     * @param appchainId The appchain ID to calculate for
     * @return The total reward amount for the appchain in the epoch
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

    /*//////////////////////////////////////////////////////////////
                        ADMIN SETTERS (SHARED)
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set the fee multiplier for reward calculations
     * @dev Higher values give more weight to gas fee performance
     * @param _fee The new fee multiplier (in UD60x18 format)
     */
    function setFeeMultiplier(uint256 _fee) external onlyOwner {
        feeMultiplier = ud(_fee);
    }

    /**
     * @notice Set the stake multiplier for reward calculations
     * @dev Higher values give more weight to stake amount
     * @param _stake The new stake multiplier (in UD60x18 format)
     */
    function setStakeMultiplier(uint256 _stake) external onlyOwner {
        stakeMultiplier = ud(_stake);
    }

    /**
     * @notice Set the decay factor for diminishing returns
     * @dev Higher values create stronger diminishing returns effect
     * @param _decay The new decay factor (in UD60x18 format)
     */
    function setDecayFactor(uint256 _decay) external onlyOwner {
        decayFactor = ud(_decay);
    }
}
