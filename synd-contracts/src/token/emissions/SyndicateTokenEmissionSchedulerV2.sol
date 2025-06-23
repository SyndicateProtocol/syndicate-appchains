// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {EmissionsCalculator} from "./EmissionsCalculator.sol";
import {IBridgeProxy} from "../interfaces/IBridgeProxy.sol";

/**
 * @title SyndicateTokenEmissionSchedulerV2
 * @notice Manages token emissions using the new EmissionsCalculator with piece-wise geometric decay
 * @dev This contract handles emission timing, bridging, and coordination with the EmissionsCalculator.
 *      It maintains the same external interface as V1 while using the new flexible emission formula.
 *
 * Key Features:
 * - Time-based emission epochs (30 days each)
 * - Integration with EmissionsCalculator for flexible decay factors
 * - Automated L2 bridging via configurable bridge proxies
 * - Comprehensive access controls and emergency mechanisms
 * - Backward compatibility with existing emission scheduler interface
 *
 * @author Syndicate Protocol
 */
contract SyndicateTokenEmissionSchedulerV2 is AccessControl, Pausable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when an address is zero
    error ZeroAddress();

    /// @notice Thrown when trying to start emissions that have already been started
    error EmissionsAlreadyStarted();

    /// @notice Thrown when trying to perform emissions operations before they're started
    error EmissionsNotStarted();

    /// @notice Thrown when trying to mint emissions after all epochs are completed
    error AllEmissionsCompleted();

    /// @notice Thrown when trying to start emissions without a configured bridge
    error BridgeNotConfigured();

    /// @notice Thrown when trying to mint emissions too early (before buffer time)
    error EmissionTooEarly();

    /// @notice Thrown when calculator is not initialized
    error CalculatorNotInitialized();

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing emissions (start, pause, resume)
    bytes32 public constant EMISSIONS_MANAGER_ROLE = keccak256("EMISSIONS_MANAGER_ROLE");

    /// @notice Role for emergency pausing functionality
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @notice Role for managing bridge configuration
    bytes32 public constant BRIDGE_MANAGER_ROLE = keccak256("BRIDGE_MANAGER_ROLE");

    /*//////////////////////////////////////////////////////////////
                               CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Duration of each emission epoch: 30 days
    uint256 public constant EPOCH_DURATION = 30 days;

    /// @notice Total number of emission epochs: 48 (~4 years)
    uint256 public constant TOTAL_EPOCHS = 48;

    /// @notice Buffer time before scheduled emission to prevent MEV/timing issues
    uint256 public constant EMISSION_BUFFER_TIME = 1 hours;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The EmissionsCalculator that handles emission calculations
    EmissionsCalculator public immutable emissionsCalculator;

    /// @notice Timestamp when emissions were started
    uint256 public emissionsStartTime;

    /// @notice Bridge proxy contract for cross-chain emissions distribution
    IBridgeProxy public bridgeProxy;

    /// @notice Bridge-specific configuration data
    bytes public bridgeData;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when emissions are started
    event EmissionsStarted(uint256 startTime);

    /// @notice Emitted when emission tokens are minted and bridged
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);

    /// @notice Emitted when the bridge proxy is updated
    event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);

    /// @notice Emitted when bridge configuration data is updated
    event BridgeDataUpdated(bytes oldData, bytes newData);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the emission scheduler contract
     * @param _emissionsCalculator Address of the EmissionsCalculator contract
     * @param defaultAdmin Address that will have default admin privileges
     * @param emissionsManager Address that can manage emissions
     * @param pauser Address that can pause the contract in emergencies
     */
    constructor(address _emissionsCalculator, address defaultAdmin, address emissionsManager, address pauser) {
        // Input validation
        if (_emissionsCalculator == address(0)) revert ZeroAddress();
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (emissionsManager == address(0)) revert ZeroAddress();
        if (pauser == address(0)) revert ZeroAddress();

        emissionsCalculator = EmissionsCalculator(_emissionsCalculator);

        // Grant roles
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(EMISSIONS_MANAGER_ROLE, emissionsManager);
        _grantRole(BRIDGE_MANAGER_ROLE, defaultAdmin);
        _grantRole(PAUSER_ROLE, pauser);
    }

    /*//////////////////////////////////////////////////////////////
                            EMISSIONS LOGIC
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Start the emissions process
     * @dev Can only be called once. Requires bridge proxy to be configured and calculator initialized.
     */
    function startEmissions() external onlyRole(EMISSIONS_MANAGER_ROLE) {
        if (emissionsStartTime != 0) revert EmissionsAlreadyStarted();
        if (address(bridgeProxy) == address(0)) revert BridgeNotConfigured();
        if (!emissionsCalculator.initialized()) revert CalculatorNotInitialized();

        emissionsStartTime = block.timestamp;

        emit EmissionsStarted(block.timestamp);
    }

    /**
     * @notice Mint emission tokens and bridge them to L2
     * @dev This function can only be called by the emissions manager.
     *      It calculates emissions using the EmissionsCalculator and bridges them.
     */
    function mintEmission() external whenNotPaused nonReentrant onlyRole(EMISSIONS_MANAGER_ROLE) {
        // Validate emissions state
        if (emissionsStartTime == 0) revert EmissionsNotStarted();
        if (emissionsEnded()) revert AllEmissionsCompleted();

        // Check timing constraints
        uint256 currentEpoch_ = getCurrentEpoch();
        uint256 calculatorEpoch = emissionsCalculator.currentEpoch();

        // Ensure we're not minting too early
        if (block.timestamp < getNextEmissionTime() - EMISSION_BUFFER_TIME) {
            revert EmissionTooEarly();
        }

        // Ensure we don't skip epochs or mint the same epoch twice
        if (calculatorEpoch != currentEpoch_) {
            // Allow catching up if we're behind, but prevent double-minting
            if (calculatorEpoch > currentEpoch_) {
                revert AllEmissionsCompleted();
            }
        }

        // Calculate and mint emission to this contract
        uint256 emissionAmount = emissionsCalculator.calculateAndMintEmission(address(this));

        // Get token interface for bridging
        IERC20 tokenInterface = IERC20(address(emissionsCalculator.syndicateToken()));

        // Approve the bridge proxy to spend our tokens
        tokenInterface.forceApprove(address(bridgeProxy), emissionAmount);

        // Bridge the tokens using the configured proxy
        bridgeProxy.executeBridge(address(tokenInterface), emissionAmount, bridgeData);

        emit EmissionMinted(calculatorEpoch, emissionAmount, address(bridgeProxy));
    }

    /*//////////////////////////////////////////////////////////////
                      BRIDGE PROXY CONFIGURATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set the bridge proxy for emissions distribution
     * @param _bridgeProxy The new bridge proxy contract implementing IBridgeProxy
     */
    function setBridgeProxy(IBridgeProxy _bridgeProxy) external onlyRole(BRIDGE_MANAGER_ROLE) {
        if (address(_bridgeProxy) == address(0)) revert ZeroAddress();

        address oldProxy = address(bridgeProxy);
        bridgeProxy = _bridgeProxy;

        emit BridgeProxyUpdated(oldProxy, address(_bridgeProxy));
    }

    /**
     * @notice Set bridge-specific configuration data
     * @param _bridgeData Encoded bridge configuration parameters
     */
    function setBridgeData(bytes calldata _bridgeData) external onlyRole(BRIDGE_MANAGER_ROLE) {
        bytes memory oldData = bridgeData;
        bridgeData = _bridgeData;

        emit BridgeDataUpdated(oldData, _bridgeData);
    }

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get comprehensive emission status information
     * @return epoch Current epoch number based on time elapsed
     * @return nextEmissionTime Timestamp when next emission can be minted
     * @return nextEmissionAmount Preview of next emission amount
     * @return canMintEmission Whether emission can be minted now
     */
    function getCurrentEpochInfo()
        external
        view
        returns (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission)
    {
        if (emissionsStartTime == 0) {
            return (0, 0, 0, false);
        }

        uint256 currentTimeEpoch = getCurrentEpoch();
        uint256 calculatorEpoch = emissionsCalculator.currentEpoch();

        uint256 nextEmissionTimestamp = getNextEmissionTime();
        uint256 nextAmount = 0;

        if (!emissionsEnded() && calculatorEpoch == currentTimeEpoch) {
            nextAmount = emissionsCalculator.previewCurrentEmission();
        }

        bool canMint = emissionsStartTime > 0 && currentTimeEpoch >= calculatorEpoch && !emissionsEnded()
            && block.timestamp >= nextEmissionTimestamp - EMISSION_BUFFER_TIME && !paused();

        return (currentTimeEpoch, nextEmissionTimestamp, nextAmount, canMint);
    }

    /**
     * @notice Get current epoch based on time elapsed since start
     * @return Current epoch number (0-47)
     */
    function getCurrentEpoch() public view returns (uint256) {
        if (emissionsStartTime == 0) return 0;
        return (block.timestamp - emissionsStartTime) / EPOCH_DURATION;
    }

    /**
     * @notice Check if all emissions have been completed
     * @return True if all 48 epochs have been processed
     */
    function emissionsEnded() public view returns (bool) {
        return emissionsCalculator.isCompleted();
    }

    /**
     * @notice Check if emissions have been started
     * @return True if startEmissions() has been called
     */
    function emissionsStarted() public view returns (bool) {
        return emissionsStartTime > 0;
    }

    /**
     * @notice Get the timestamp when the next emission can be minted
     * @return Timestamp of the next emission opportunity
     */
    function getNextEmissionTime() public view returns (uint256) {
        if (emissionsStartTime == 0) return 0;
        uint256 calculatorEpoch = emissionsCalculator.currentEpoch();
        return emissionsStartTime + ((calculatorEpoch + 1) * EPOCH_DURATION);
    }

    /**
     * @notice Get total emissions minted so far
     * @return Total amount of tokens emitted
     */
    function totalEmissionsMinted() external view returns (uint256) {
        return emissionsCalculator.totalEmitted();
    }

    /**
     * @notice Get current emission epoch from calculator
     * @return Current epoch from EmissionsCalculator
     */
    function currentEpoch() external view returns (uint256) {
        return emissionsCalculator.currentEpoch();
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
