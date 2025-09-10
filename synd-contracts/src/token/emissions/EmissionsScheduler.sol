// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {EmissionsCalculator} from "./EmissionsCalculator.sol";
import {EpochTracker} from "../../staking/EpochTracker.sol";

// TODO: Use the real Relayer contract when everything is ready
interface IRelayer {
    function relay(address destinationL3, uint256 epochIndex) external;
}

/**
 * @title EmissionsScheduler
 * @notice Manages token emissions using the EmissionsCalculator with piece-wise geometric decay
 * @dev This contract handles emission timing, bridging, and coordination with the EmissionsCalculator.
 *
 * Key Features:
 * - Integration with EmissionsCalculator for flexible decay factors
 * - Automated bridging of funds through the relayer contract to the commons L3 chain
 * - Emissions must be minted in order, and only once per epoch
 * - Emissions can only be minted for past epochs and the current epoch that have not been minted yet
 *
 * @author Syndicate Protocol
 */
contract EmissionsScheduler is AccessControl, Pausable, ReentrancyGuard, EpochTracker {
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when an address is zero
    error ZeroAddress();

    /// @notice Thrown when trying to mint emissions for an invalid epoch
    error InvalidEpoch();

    /// @notice Thrown when trying to mint emissions after all epochs are completed
    error AllEmissionsCompleted();

    /// @notice Thrown when trying to mint emissions for an epoch that's already been minted
    error EpochAlreadyMinted();

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for emergency pausing functionality
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The EmissionsCalculator that handles emission calculations
    EmissionsCalculator public immutable emissionsCalculator;

    /// @notice The epoch index when emissions started
    uint256 public immutable epochStartIndex;

    /// @notice The Relayer contract
    IRelayer public relayer;

    /// @notice The destination address for the relayer (ON THE COMMONS L3 CHAIN)
    address public relayDestinationL3;

    /// @notice Tracks which epochs have been minted
    mapping(uint256 epochIndex => bool isMinted) public epochMinted;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when emission tokens are minted
    event EmissionMinted(uint256 epoch, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the emission scheduler contract
     * @param _epochStartIndex The epoch index when emissions started
     * @param _emissionsCalculator Address of the EmissionsCalculator contract
     * @param _relayer Address of the Relayer contract
     * @param _relayDestinationL3 Address of the relay destination on the commons L3 chain
     * @param defaultAdmin Address that will have default admin privileges
     * @param pauser Address that can pause the contract in emergencies
     */
    constructor(
        uint256 _epochStartIndex,
        address _emissionsCalculator,
        address _relayer,
        address _relayDestinationL3,
        address defaultAdmin,
        address pauser
    ) {
        // Input validation
        // Epoch start index must be greater than 0
        if (_epochStartIndex == 0) revert InvalidEpoch();
        if (_emissionsCalculator == address(0)) revert ZeroAddress();
        if (_relayer == address(0)) revert ZeroAddress();
        if (_relayDestinationL3 == address(0)) revert ZeroAddress();
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (pauser == address(0)) revert ZeroAddress();

        emissionsCalculator = EmissionsCalculator(_emissionsCalculator);

        relayer = IRelayer(_relayer);
        relayDestinationL3 = _relayDestinationL3;

        // Grant roles
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(PAUSER_ROLE, pauser);

        epochStartIndex = _epochStartIndex;
    }

    /**
     * @notice Set the relay destination
     * @dev This function can only be called by the DEFAULT_ADMIN_ROLE.
     * NOTE: THIS IS AN ADDRESS ON THE COMMONS L3 CHAIN.
     * @param _relayDestinationL3 The new relay destination
     */
    function setRelayDestinationL3(address _relayDestinationL3) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (_relayDestinationL3 == address(0)) revert ZeroAddress();
        relayDestinationL3 = _relayDestinationL3;
    }

    /**
     * @notice Set the relayer contract address
     * @dev This function can only be called by the DEFAULT_ADMIN_ROLE.
     * @param _relayer The new relayer contract address
     */
    function setRelayer(address _relayer) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (_relayer == address(0)) revert ZeroAddress();
        relayer = IRelayer(_relayer);
    }

    /**
     * @notice Mint emission tokens and bridge them to the commons L3 chain
     * @dev This function can be called by anyone.
     *      The epoch index is the relative epoch index to the start epoch index.
     *      The epoch index must be the next epoch index to be emitted.
     *      The epoch index must be equal to or less than the current epoch index.
     *      The epoch index must not have been minted yet.
     * @param epochIndex The relative epoch index to mint emissions for
     */
    function mintEmission(uint256 epochIndex) external whenNotPaused nonReentrant {
        // Validate emissions state
        if (emissionsEnded()) revert AllEmissionsCompleted();
        if (epochMinted[epochIndex]) revert EpochAlreadyMinted();
        if (epochIndex != emissionsCalculator.currentEpoch()) revert InvalidEpoch();
        if (epochIndex + epochStartIndex > getCurrentEpoch()) revert InvalidEpoch();

        // Mark epoch as minted
        epochMinted[epochIndex] = true;

        // Calculate and mint emission to this contract, passing expected epoch for synchronization
        uint256 emissionAmount = emissionsCalculator.calculateAndMintEmission(address(relayer), epochIndex);

        // Bridge the emission to the commons L3 chain
        relayer.relay(relayDestinationL3, epochIndex + epochStartIndex);

        emit EmissionMinted(epochIndex, emissionAmount);
    }

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Check if all emissions have been completed
     * @return True if all 48 epochs have been processed
     */
    function emissionsEnded() public view returns (bool) {
        return emissionsCalculator.isCompleted();
    }

    /**
     * @notice Check if emissions have been started
     * @return True if current epoch is greater than or equal to the start epoch index
     */
    function emissionsStarted() public view returns (bool) {
        return epochStartIndex <= getCurrentEpoch();
    }

    /**
     * @notice Get total emissions minted so far
     * @return Total amount of tokens emitted
     */
    function totalEmissionsMinted() external view returns (uint256) {
        return emissionsCalculator.totalEmitted();
    }

    /*//////////////////////////////////////////////////////////////
                          EMERGENCY CONTROLS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Pause the entire contract in emergency situations
     * @dev This pauses all emissions operations. Only callable by PAUSER_ROLE.
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpause the contract
     * @dev Only callable by DEFAULT_ADMIN_ROLE to ensure careful consideration.
     */
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }
}
