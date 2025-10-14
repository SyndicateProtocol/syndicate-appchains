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

    mapping(uint256 epochIndex => uint256) public remainingAppchainsPlusOne;

    /// @notice Cache for per-epoch/appchain diminishing factors
    /// @dev Stores calculated diminishing factors to avoid recalculation
    mapping(uint256 epochIndex => mapping(uint256 appchainId => UD60x18 diminishingFactor)) public diminishingFactor;

    /// @notice Cache for per-epoch sum of diminishing factors across all appchains
    /// @dev Stores total diminishing factor sum for each epoch
    mapping(uint256 epochIndex => UD60x18 epochTotalDiminishingFactor) public epochTotalDiminishingFactor;

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

    // Compute appchain reward factors
    // To check if the computation is complete, call remainingAppchainsPlusOne(epochIndex)
    // and ensure the return value is 1.
    // It also returns true when computations are complete.
    function computeDiminishingFactors(uint256 epochIndex, uint256 count) external returns (bool) {
        uint256 remainingPlusOne = remainingAppchainsPlusOne[epochIndex];
        require(remainingPlusOne != 1, "all diminishing factors computed");

        if (remainingPlusOne == 0) {
            remainingPlusOne = gasDataProvider.getAppchainCount(epochIndex) + 1;
            remainingAppchainsPlusOne[epochIndex] = remainingPlusOne;
            if (remainingPlusOne == 1) {
                return true;
            }
        }

        if (count == 0 || count >= remainingPlusOne) {
            count = remainingPlusOne - 1;
        }

        remainingAppchainsPlusOne[epochIndex] -= count;

        UD60x18 totalStake = convert(stakingContract.getTotalStake(epochIndex));
        if (totalStake.isZero()) {
            remainingAppchainsPlusOne[epochIndex] = 1;
            return true;
        }

        UD60x18 totalGasFees = convert(gasDataProvider.getTotalGasFees(epochIndex));
        if (totalGasFees.isZero()) {
            remainingAppchainsPlusOne[epochIndex] = 1;
            return true;
        }

        uint256[] memory appchainId;
        uint256[] memory appchainGasFees;
        (appchainId, appchainGasFees) =
            gasDataProvider.getAppchainInfo(epochIndex, remainingAppchainsPlusOne[epochIndex] - 1, count);
        UD60x18 dfSum = epochTotalDiminishingFactor[epochIndex];
        for (uint256 i = 0; i < count; i++) {
            UD60x18 appchainStake = convert(stakingContract.getAppchainStake(epochIndex, appchainId[i]));
            UD60x18 feeShare = convert(appchainGasFees[i]).mul(feeMultiplier).div(totalGasFees);
            UD60x18 stakeShare = appchainStake.mul(stakeMultiplier).div(totalStake);
            UD60x18 dominance = feeShare.add(stakeShare);
            UD60x18 df = (convert(1).add(decayFactor.mul(dominance))).ln();
            if (!df.isZero()) {
                diminishingFactor[epochIndex][appchainId[i]] = df;
                dfSum = dfSum.add(df);
            }
        }
        epochTotalDiminishingFactor[epochIndex] = dfSum;

        return remainingAppchainsPlusOne[epochIndex] == 1;
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
    function getAppchainTotalReward(uint256 epochIndex, uint256 appchainId) public view returns (uint256) {
        require(remainingAppchainsPlusOne[epochIndex] == 1 && epochTotal[epochIndex] > 0, ClaimNotAvailable());
        UD60x18 df = diminishingFactor[epochIndex][appchainId];
        if (df.isZero()) return 0;
        return convert(convert(epochTotal[epochIndex]).mul(df).div(epochTotalDiminishingFactor[epochIndex]));
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
