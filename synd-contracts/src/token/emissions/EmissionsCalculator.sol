// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

interface ISyndicateTokenMintable {
    function mint(address to, uint256 amount) external;
    function totalSupply() external view returns (uint256);
    function TOTAL_SUPPLY() external view returns (uint256);
}

/**
 * @title EmissionsCalculator
 * @notice Calculates and manages token emissions using piece-wise geometric change factor
 * @dev Implements a flexible emission system where change factors can be updated by governance
 *      while maintaining the 80M cap and 48-epoch limit constraints.
 *
 * Formula:
 * - For epoch t < 47: E_t = R_t * |1 - r_t| / |1 - P_t|
 * - For epoch 47: E_t = R_t (sweep remainder)
 * - SPECIAL CASE: When r_t = 1e18: E_t = R_t / (48 - t)
 *
 * Where:
 * - R_t = remaining supply = CAP - M (M = total minted so far)
 * - r_t = change factor for epoch t (0 < r, scaled by 1e18)
 * - P_t = cumulative product of change factors from epoch t to 47
 *
 * @author Syndicate Protocol
 */
contract EmissionsCalculator is AccessControl {
    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when an address is zero
    error ZeroAddress();

    /// @notice Thrown when an epoch is invalid (>= 48)
    error InvalidEpoch();

    /// @notice Thrown when all emissions are completed
    error EmissionsCompleted();

    /// @notice Thrown when change factor is invalid (0)
    error InvalidChangeFactor();

    /// @notice Thrown when trying to set change factor for past epochs
    error CannotModifyPastEpoch();

    /// @notice Thrown when the expected epoch doesn't match current epoch
    error EpochMismatch(uint256 expected, uint256 current);

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing change factors (typically governance)
    bytes32 public constant CHANGE_FACTOR_MANAGER_ROLE = keccak256("CHANGE_FACTOR_MANAGER_ROLE");

    /// @notice Role for triggering emissions
    bytes32 public constant EMISSIONS_ROLE = keccak256("EMISSIONS_ROLE");

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Total emission epochs: 48
    uint256 public constant TOTAL_EPOCHS = 48;

    /// @notice Total emissions cap: 80 million tokens
    uint256 public constant EMISSIONS_CAP = 80_000_000 * 10 ** 18;

    /// @notice Scaling factor for change factor calculations (1e18)
    uint256 public constant SCALE = 1e18;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The SyndicateToken contract for minting and supply queries
    ISyndicateTokenMintable public immutable syndicateToken;

    /// @notice Change factor for each epoch (scaled by 1e18)
    /// @dev r[epoch] where 0 < r, represented as r * 1e18
    mapping(uint256 epochIndex => uint256 changeFactor) public changeFactors;

    /// @notice Current epoch index (0-47)
    uint256 public currentEpoch;

    /// @notice Total emissions minted so far
    uint256 public totalEmitted;

    /// @notice Whether emissions have been initialized
    bool public initialized;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when change factor is updated
    event ChangeFactorSet(uint256 indexed epoch, uint256 changeFactor, address indexed setter);

    /// @notice Emitted when emissions are calculated and minted
    event EmissionMinted(uint256 indexed epoch, uint256 amount, uint256 remainingSupply, address indexed to);

    /// @notice Emitted when emissions are initialized
    event EmissionsInitialized(uint256 defaultChangeFactor);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the emissions calculator
     * @param _syndicateToken Address of the SyndicateToken contract
     * @param defaultAdmin Address that will have default admin privileges
     * @param changeFactorManager Address that can manage change factors
     */
    constructor(address _syndicateToken, address defaultAdmin, address changeFactorManager) {
        if (_syndicateToken == address(0)) revert ZeroAddress();
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (changeFactorManager == address(0)) revert ZeroAddress();

        syndicateToken = ISyndicateTokenMintable(_syndicateToken);

        // Grant roles
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(CHANGE_FACTOR_MANAGER_ROLE, changeFactorManager);
    }

    /*//////////////////////////////////////////////////////////////
                           INITIALIZATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize emissions with default change factor
     * @param defaultChangeFactor Default change factor for all epochs (scaled by 1e18)
     * @dev Can only be called once. Sets all epochs to the same initial change factor.
     */
    function initializeEmissions(uint256 defaultChangeFactor) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (initialized) revert EmissionsCompleted();
        if (defaultChangeFactor == 0) revert InvalidChangeFactor();

        initialized = true;

        // Set default change factor for all epochs
        for (uint256 i = 0; i < TOTAL_EPOCHS; i++) {
            changeFactors[i] = defaultChangeFactor;
        }

        emit EmissionsInitialized(defaultChangeFactor);
    }

    /*//////////////////////////////////////////////////////////////
                            CHANGE FACTOR MANAGEMENT
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set change factor for a specific epoch
     * @param epoch Epoch number (0-47)
     * @param changeFactor New change factor (scaled by 1e18, must be 0 < r)
     * @dev Only future epochs can be modified. This allows governance to adjust
     *      the emission curve while maintaining the mathematical constraints.
     */
    function setChangeFactor(uint256 epoch, uint256 changeFactor) external onlyRole(CHANGE_FACTOR_MANAGER_ROLE) {
        if (epoch >= TOTAL_EPOCHS) revert InvalidEpoch();
        if (epoch < currentEpoch) revert CannotModifyPastEpoch();
        if (changeFactor == 0) revert InvalidChangeFactor();

        changeFactors[epoch] = changeFactor;
        emit ChangeFactorSet(epoch, changeFactor, msg.sender);
    }

    /**
     * @notice Set change factors for multiple epochs at once
     * @param startEpoch Starting epoch number
     * @param changeFactorArray Array of change factors
     */
    function setChangeFactors(uint256 startEpoch, uint256[] calldata changeFactorArray)
        external
        onlyRole(CHANGE_FACTOR_MANAGER_ROLE)
    {
        for (uint256 i = 0; i < changeFactorArray.length; i++) {
            uint256 epoch = startEpoch + i;
            if (epoch >= TOTAL_EPOCHS) break;
            if (epoch < currentEpoch) continue;
            if (changeFactorArray[i] == 0) continue;

            changeFactors[epoch] = changeFactorArray[i];
            emit ChangeFactorSet(epoch, changeFactorArray[i], msg.sender);
        }
    }

    /*//////////////////////////////////////////////////////////////
                         EMISSION CALCULATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Calculate and mint emissions for current epoch
     * @param to Address to mint tokens to
     * @param expectedEpoch The epoch number that the caller expects to mint for
     * @dev Implements the piece-wise geometric change factor formula:
     *      E_t = R_t * |1 - r_t| / |1 - P_t| for t < 47
     *      E_t = R_t for t = 47 (final epoch sweeps remainder)
     *      E_t = R_t / (48 - t) for r_t = 1e18
     *      Requires expectedEpoch to match currentEpoch for synchronization
     */
    function calculateAndMintEmission(address to, uint256 expectedEpoch)
        external
        onlyRole(EMISSIONS_ROLE)
        returns (uint256)
    {
        if (!initialized) revert EmissionsCompleted();
        if (currentEpoch >= TOTAL_EPOCHS) revert EmissionsCompleted();
        if (to == address(0)) revert ZeroAddress();

        // Ensure epoch synchronization
        if (currentEpoch != expectedEpoch) revert EpochMismatch(expectedEpoch, currentEpoch);

        uint256 emissionAmount = getNextEmission();
        if (emissionAmount == 0) revert EmissionsCompleted();

        // Update state
        totalEmitted += emissionAmount;

        // Mint tokens
        syndicateToken.mint(to, emissionAmount);

        emit EmissionMinted(currentEpoch, emissionAmount, getRemainingSupply(), to);

        // Advance to next epoch
        currentEpoch++;

        return emissionAmount;
    }

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get remaining supply available for emissions
     * @return Amount of tokens remaining to be emitted
     */
    function getRemainingSupply() public view returns (uint256) {
        // R_t = CAP - M where M is total minted emissions so far
        uint256 totalSupply = syndicateToken.totalSupply();
        uint256 maxSupply = syndicateToken.TOTAL_SUPPLY();
        uint256 initialSupply = maxSupply - EMISSIONS_CAP;

        uint256 emittedSoFar = totalSupply > initialSupply ? totalSupply - initialSupply : 0;

        return EMISSIONS_CAP > emittedSoFar ? EMISSIONS_CAP - emittedSoFar : 0;
    }

    /**
     * @notice Calculate cumulative product P_t = r_t * r_(t+1) * ... * r_47
     * @param fromEpoch Starting epoch for the product calculation
     * @return Cumulative product of change factors (scaled by 1e18)
     */
    function calculateCumulativeProduct(uint256 fromEpoch) public view returns (uint256) {
        if (fromEpoch >= TOTAL_EPOCHS) return SCALE;

        uint256 product = SCALE;

        for (uint256 i = fromEpoch; i < TOTAL_EPOCHS; i++) {
            product = (product * changeFactors[i]) / SCALE;
        }

        return product;
    }

    /**
     * @notice Get emission amount for current epoch without minting
     * @return Emission amount that would be minted for current epoch
     */
    function getNextEmission() public view returns (uint256) {
        if (!initialized || currentEpoch >= TOTAL_EPOCHS) return 0;
        uint256 epochsLeft = TOTAL_EPOCHS - currentEpoch;

        uint256 remainingSupply = getRemainingSupply();

        // Final epoch (47): sweep all remaining tokens
        if (epochsLeft == 1) {
            return remainingSupply;
        }

        // Get the change factor for current epoch
        uint256 currentChangeFactor = changeFactors[currentEpoch];

        // Special case: when change factor equals SCALE (1.0), use linear distribution
        if (currentChangeFactor == SCALE) {
            return remainingSupply / epochsLeft;
        }

        // Calculate the cumulative product P_t from current epoch to end
        uint256 cumulativeProduct = calculateCumulativeProduct(currentEpoch);

        // Calculate |1 - P_t|
        uint256 productDifference = cumulativeProduct > SCALE ? cumulativeProduct - SCALE : SCALE - cumulativeProduct;

        // Use minimum denominator to avoid precision issues with near-zero values
        uint256 denominator = productDifference < 1000 ? 1000 : productDifference;

        // Calculate |1 - r_t| * remainingSupply
        uint256 numerator = currentChangeFactor > SCALE
            ? remainingSupply * (currentChangeFactor - SCALE)
            : remainingSupply * (SCALE - currentChangeFactor);

        // E_t = R_t * |1 - r_t| / |1 - P_t|
        return numerator / denominator;
    }

    /**
     * @notice Get change factor for a specific epoch
     * @param epoch Epoch number (0-47)
     * @return Change factor for the epoch (scaled by 1e18)
     */
    function getChangeFactor(uint256 epoch) external view returns (uint256) {
        if (epoch >= TOTAL_EPOCHS) revert InvalidEpoch();
        return changeFactors[epoch];
    }

    /**
     * @notice Check if all emissions have been completed
     * @return True if all 48 epochs have been processed
     */
    function isCompleted() external view returns (bool) {
        return currentEpoch >= TOTAL_EPOCHS;
    }

    /**
     * @notice Get emissions progress information
     * @return current Current epoch number
     * @return total Total epochs
     * @return emitted Total amount emitted so far
     * @return remaining Remaining supply for emissions
     * @return completed Whether emissions are completed
     */
    function getEmissionsInfo()
        external
        view
        returns (uint256 current, uint256 total, uint256 emitted, uint256 remaining, bool completed)
    {
        return (currentEpoch, TOTAL_EPOCHS, totalEmitted, getRemainingSupply(), currentEpoch >= TOTAL_EPOCHS);
    }
}
