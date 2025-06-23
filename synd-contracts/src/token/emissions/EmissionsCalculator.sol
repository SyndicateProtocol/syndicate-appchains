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
 * @notice Calculates and manages token emissions using piece-wise geometric decay
 * @dev Implements a flexible emission system where decay factors can be updated by governance
 *      while maintaining the 100M cap and 48-epoch limit constraints.
 *
 * Formula:
 * - For epoch t < 47: E_t = R_t * (1 - r_t) / (1 - P_t)
 * - For epoch 47: E_t = R_t (sweep remainder)
 *
 * Where:
 * - R_t = remaining supply = CAP - M (M = total minted so far)
 * - r_t = decay factor for epoch t (0 < r < 1, scaled by 1e18)
 * - P_t = cumulative product of decay factors from epoch t to 47
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

    /// @notice Thrown when decay factor is invalid (0 or >= 1e18)
    error InvalidDecayFactor();

    /// @notice Thrown when trying to set decay for past epochs
    error CannotModifyPastEpoch();

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing decay factors (typically governance)
    bytes32 public constant DECAY_MANAGER_ROLE = keccak256("DECAY_MANAGER_ROLE");

    /// @notice Role for triggering emissions
    bytes32 public constant EMISSIONS_ROLE = keccak256("EMISSIONS_ROLE");

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Total emission epochs: 48
    uint256 public constant TOTAL_EPOCHS = 48;

    /// @notice Total emissions cap: 100 million tokens
    uint256 public constant EMISSIONS_CAP = 100_000_000 * 10 ** 18;

    /// @notice Scaling factor for decay calculations (1e18)
    uint256 public constant SCALE = 1e18;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The SyndicateToken contract for minting and supply queries
    ISyndicateTokenMintable public immutable syndicateToken;

    /// @notice Decay factor for each epoch (scaled by 1e18)
    /// @dev r[epoch] where 0 < r < 1, represented as r * 1e18
    mapping(uint256 => uint256) public decayFactors;

    /// @notice Current epoch index (0-47)
    uint256 public currentEpoch;

    /// @notice Total emissions minted so far
    uint256 public totalEmitted;

    /// @notice Whether emissions have been initialized
    bool public initialized;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when decay factor is updated
    event DecayFactorSet(uint256 indexed epoch, uint256 decayFactor, address indexed setter);

    /// @notice Emitted when emissions are calculated and minted
    event EmissionMinted(uint256 indexed epoch, uint256 amount, uint256 remainingSupply, address indexed to);

    /// @notice Emitted when emissions are initialized
    event EmissionsInitialized(uint256 defaultDecayFactor);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the emissions calculator
     * @param _syndicateToken Address of the SyndicateToken contract
     * @param defaultAdmin Address that will have default admin privileges
     * @param decayManager Address that can manage decay factors
     */
    constructor(address _syndicateToken, address defaultAdmin, address decayManager) {
        if (_syndicateToken == address(0)) revert ZeroAddress();
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (decayManager == address(0)) revert ZeroAddress();

        syndicateToken = ISyndicateTokenMintable(_syndicateToken);

        // Grant roles
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(DECAY_MANAGER_ROLE, decayManager);
        _grantRole(EMISSIONS_ROLE, defaultAdmin);
    }

    /*//////////////////////////////////////////////////////////////
                           INITIALIZATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize emissions with default decay factor
     * @param defaultDecayFactor Default decay factor for all epochs (scaled by 1e18)
     * @dev Can only be called once. Sets all epochs to the same initial decay factor.
     */
    function initializeEmissions(uint256 defaultDecayFactor) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (initialized) revert EmissionsCompleted();
        if (defaultDecayFactor == 0 || defaultDecayFactor >= SCALE) revert InvalidDecayFactor();

        initialized = true;

        // Set default decay factor for all epochs
        for (uint256 i = 0; i < TOTAL_EPOCHS; i++) {
            decayFactors[i] = defaultDecayFactor;
        }

        emit EmissionsInitialized(defaultDecayFactor);
    }

    /*//////////////////////////////////////////////////////////////
                            DECAY MANAGEMENT
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set decay factor for a specific epoch
     * @param epoch Epoch number (0-47)
     * @param decayFactor New decay factor (scaled by 1e18, must be 0 < r < 1e18)
     * @dev Only future epochs can be modified. This allows governance to adjust
     *      the emission curve while maintaining the mathematical constraints.
     */
    function setDecayFactor(uint256 epoch, uint256 decayFactor) external onlyRole(DECAY_MANAGER_ROLE) {
        if (epoch >= TOTAL_EPOCHS) revert InvalidEpoch();
        if (epoch < currentEpoch) revert CannotModifyPastEpoch();
        if (decayFactor == 0 || decayFactor >= SCALE) revert InvalidDecayFactor();

        decayFactors[epoch] = decayFactor;
        emit DecayFactorSet(epoch, decayFactor, msg.sender);
    }

    /**
     * @notice Set decay factors for multiple epochs at once
     * @param startEpoch Starting epoch number
     * @param decayFactorArray Array of decay factors
     */
    function setDecayFactors(uint256 startEpoch, uint256[] calldata decayFactorArray)
        external
        onlyRole(DECAY_MANAGER_ROLE)
    {
        for (uint256 i = 0; i < decayFactorArray.length; i++) {
            uint256 epoch = startEpoch + i;
            if (epoch >= TOTAL_EPOCHS) break;
            if (epoch < currentEpoch) continue;
            if (decayFactorArray[i] == 0 || decayFactorArray[i] >= SCALE) continue;

            decayFactors[epoch] = decayFactorArray[i];
            emit DecayFactorSet(epoch, decayFactorArray[i], msg.sender);
        }
    }

    /*//////////////////////////////////////////////////////////////
                         EMISSION CALCULATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Calculate and mint emissions for current epoch
     * @param to Address to mint tokens to
     * @dev Implements the piece-wise geometric decay formula:
     *      E_t = R_t * (1 - r_t) / (1 - P_t) for t < 47
     *      E_t = R_t for t = 47 (final epoch sweeps remainder)
     */
    function calculateAndMintEmission(address to) external onlyRole(EMISSIONS_ROLE) returns (uint256) {
        if (!initialized) revert EmissionsCompleted();
        if (currentEpoch >= TOTAL_EPOCHS) revert EmissionsCompleted();
        if (to == address(0)) revert ZeroAddress();

        // Calculate remaining supply (R_t)
        uint256 remainingSupply = getRemainingSupply();

        uint256 emissionAmount;

        if (currentEpoch == TOTAL_EPOCHS - 1) {
            // Final epoch: sweep all remaining tokens
            emissionAmount = remainingSupply;
        } else {
            // Calculate emission using geometric decay formula
            uint256 rt = decayFactors[currentEpoch];
            uint256 pt = calculateCumulativeProduct(currentEpoch);

            // E_t = R_t * (1 - r_t) / (1 - P_t)
            // Note: We need to be careful with precision in fixed-point arithmetic
            uint256 numerator = remainingSupply * (SCALE - rt);
            uint256 denominator = SCALE - pt;

            emissionAmount = numerator / denominator;
        }

        // Update state
        totalEmitted += emissionAmount;

        // Mint tokens
        syndicateToken.mint(to, emissionAmount);

        emit EmissionMinted(currentEpoch, emissionAmount, remainingSupply - emissionAmount, to);

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
     * @return Cumulative product of decay factors (scaled by 1e18)
     */
    function calculateCumulativeProduct(uint256 fromEpoch) public view returns (uint256) {
        if (fromEpoch >= TOTAL_EPOCHS) return SCALE;

        uint256 product = SCALE;

        for (uint256 i = fromEpoch; i < TOTAL_EPOCHS; i++) {
            product = (product * decayFactors[i]) / SCALE;
        }

        return product;
    }

    /**
     * @notice Preview emission amount for current epoch without minting
     * @return Emission amount that would be minted for current epoch
     */
    function previewCurrentEmission() external view returns (uint256) {
        if (!initialized || currentEpoch >= TOTAL_EPOCHS) return 0;

        uint256 remainingSupply = getRemainingSupply();

        if (currentEpoch == TOTAL_EPOCHS - 1) {
            return remainingSupply;
        }

        uint256 rt = decayFactors[currentEpoch];
        uint256 pt = calculateCumulativeProduct(currentEpoch);

        uint256 numerator = remainingSupply * (SCALE - rt);
        uint256 denominator = SCALE - pt;

        return numerator / denominator;
    }

    /**
     * @notice Get decay factor for a specific epoch
     * @param epoch Epoch number (0-47)
     * @return Decay factor for the epoch (scaled by 1e18)
     */
    function getDecayFactor(uint256 epoch) external view returns (uint256) {
        if (epoch >= TOTAL_EPOCHS) revert InvalidEpoch();
        return decayFactors[epoch];
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
