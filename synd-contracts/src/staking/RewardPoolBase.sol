// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {ISyndStaking} from "./interfaces/ISyndStaking.sol";
import {IGasDataProvider} from "./interfaces/IGasDataProvider.sol";
import {UD60x18, ud, convert} from "@prb/math/src/UD60x18.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {EpochTracker} from "./EpochTracker.sol";
import {IPool} from "src/staking/interfaces/IPool.sol";

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
abstract contract RewardPoolBase is ReentrancyGuard, Ownable, EpochTracker, IPool {
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

    /// @notice Index to keep track of the pre-computed appchains
    /// @dev Stores an internal index for ability to pre-compute the diminishing factors for an epoch in batches
    mapping(uint256 epochIndex => uint256 preComputeIndex) public preComputeIndex;

    /// @notice Constant to indicate that the pre-compute is complete
    /// @dev Used to indicate that the pre-compute is complete for an epoch
    uint256 public constant PRE_COMPUTE_COMPLETE = type(uint256).max;

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

    receive() external payable {
        deposit(getCurrentEpoch());
    }

    // legacy function to satisfy the IPool interface
    // depositing to past or future epochs is not recommended
    function deposit(uint256 epoch) public payable {
        epochTotal[epoch] += msg.value;
        emit EpochDeposit(epoch, msg.value);
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
        if (epochTotal[epochIndex] == 0 || getCurrentEpoch() <= epochIndex) {
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
     * @dev Returns the sum of all appchain diminishing factors for an epoch
     *      If the computation is not complete, it will try to compute the remaining appchains
     * @param epochIndex The epoch index to calculate for
     * @return The sum of all appchain diminishing factors
     */
    function _getAllAppchainsDiminishingFactor(uint256 epochIndex) internal returns (UD60x18) {
        if (!preComputeDiminishingFactors(epochIndex, 0)) {
            return convert(0);
        }

        return epochTotalDiminishingFactor[epochIndex];
    }

    /**
     * @notice Pre-compute the diminishing factors for an epoch
     * @dev Pre-compute the diminishing factors for an epoch
     * @param epochIndex The epoch index to compute for
     * @param _batchSize The batch size to compute for (0 to compute all)
     * @return isComplete Whether the computation is complete
     */
    function preComputeDiminishingFactors(uint256 epochIndex, uint256 _batchSize) public returns (bool isComplete) {
        if (preComputeIndex[epochIndex] == PRE_COMPUTE_COMPLETE) {
            return true;
        }

        UD60x18 totalStake = convert(stakingContract.getTotalStake(epochIndex));
        if (totalStake.isZero()) return false;

        UD60x18 totalGasFees = convert(gasDataProvider.getTotalGasFees(epochIndex));
        if (totalGasFees.isZero()) return false;

        // If batch size is not 0, get an extra appchain to check if the pre-compute is complete
        uint256 batchSize = _batchSize == 0 ? _batchSize : _batchSize + 1;
        uint256[] memory ids = gasDataProvider.getAppchainIds(epochIndex, preComputeIndex[epochIndex], batchSize);
        uint256 length = ids.length;
        // If we got the full batch, we need to subtract 1 from the end index to not count the extra appchain
        uint256 endIndex = length == batchSize ? length - 1 : length;

        for (uint256 i = 0; i < endIndex; i++) {
            epochTotalDiminishingFactor[epochIndex] = epochTotalDiminishingFactor[epochIndex].add(
                _getAppchainDiminishingFactor(epochIndex, ids[i], totalStake, totalGasFees)
            );
        }

        // If _batchSize is 0 or we are at the end of the appchains, set the pre-compute index to complete
        if (_batchSize == 0 || endIndex == length) {
            preComputeIndex[epochIndex] = PRE_COMPUTE_COMPLETE;
            return true;
        } else {
            preComputeIndex[epochIndex] += endIndex;
            return false;
        }
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

        UD60x18 dfSum = _getAllAppchainsDiminishingFactor(epochIndex);
        if (dfSum.isZero()) return 0;

        UD60x18 df = diminishingFactor[epochIndex][appchainId];
        if (df.isZero()) return 0;

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
