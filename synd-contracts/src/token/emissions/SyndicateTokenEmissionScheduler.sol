// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IBridgeProxy} from "../interfaces/IBridgeProxy.sol";

interface ISyndicateTokenMintable {
    function mint(address to, uint256 amount) external;
}

/**
 * @title SyndicateTokenEmissionScheduler
 * @notice Manages the emission schedule and distribution of SYND tokens
 * @dev This contract handles all emissions logic and calls the SyndicateToken contract
 *      to mint tokens according to a predefined schedule. It includes automated
 *      emissions distribution to L2 chains via configurable bridge proxies.
 *
 * Key Features:
 * - Automated emissions over 48 epochs (~4 years) with decay schedule
 * - Modular bridge architecture supporting multiple bridge providers
 * - Comprehensive access controls and emergency mechanisms
 * - Emissions tracking and validation against total supply limits
 *
 * Emissions Schedule:
 * - 48 epochs of 30 days each (~4 years total)
 * - Decreasing amounts per epoch in 8 periods of 6 epochs each
 * - Automated distribution via configurable bridge proxy
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
contract SyndicateTokenEmissionScheduler is AccessControl, Pausable, ReentrancyGuard {
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

    /// @notice The SyndicateToken contract that will receive minted emissions
    ISyndicateTokenMintable public immutable syndicateToken;

    /// @notice Emission amounts for each epoch (fixed schedule with decay)
    uint256[TOTAL_EPOCHS] public emissionSchedule;

    /// @notice Timestamp when emissions were started
    uint256 public emissionsStartTime;

    /// @notice Current emission epoch (0-based)
    uint256 public currentEpoch;

    /// @notice Total amount of emissions minted so far
    uint256 public totalEmissionsMinted;

    /// @notice Bridge proxy contract for cross-chain emissions distribution
    IBridgeProxy public bridgeProxy;

    /// @notice Bridge-specific configuration data
    bytes public bridgeData;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when emissions are started
     * @param startTime The timestamp when emissions began
     */
    event EmissionsStarted(uint256 startTime);

    /**
     * @notice Emitted when emission tokens are minted and bridged
     * @param epoch The epoch number that was minted
     * @param amount The amount of tokens minted
     * @param destination The bridge proxy that received the tokens
     */
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);

    /**
     * @notice Emitted when the bridge proxy is updated
     * @param oldProxy The previous bridge proxy address
     * @param newProxy The new bridge proxy address
     */
    event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);

    /**
     * @notice Emitted when bridge configuration data is updated
     * @param oldData The previous bridge data
     * @param newData The new bridge data
     */
    event BridgeDataUpdated(bytes oldData, bytes newData);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the emission scheduler contract
     * @param _syndicateToken Address of the SyndicateToken contract
     * @param defaultAdmin Address that will have default admin privileges
     * @param emissionsManager Address that can manage emissions
     * @param pauser Address that can pause the contract in emergencies
     */
    constructor(address _syndicateToken, address defaultAdmin, address emissionsManager, address pauser) {
        // Input validation
        if (_syndicateToken == address(0)) revert ZeroAddress();
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (emissionsManager == address(0)) revert ZeroAddress();
        if (pauser == address(0)) revert ZeroAddress();

        syndicateToken = ISyndicateTokenMintable(_syndicateToken);

        // Grant roles
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(EMISSIONS_MANAGER_ROLE, emissionsManager);
        _grantRole(BRIDGE_MANAGER_ROLE, defaultAdmin);
        _grantRole(PAUSER_ROLE, pauser);

        // Initialize the emission schedule
        _initializeEmissionSchedule();
    }

    /*//////////////////////////////////////////////////////////////
                            EMISSIONS LOGIC
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Start the emissions process
     * @dev Can only be called once. Requires bridge proxy to be configured.
     *      Only callable by addresses with EMISSIONS_MANAGER_ROLE.
     */
    function startEmissions() external onlyRole(EMISSIONS_MANAGER_ROLE) {
        if (emissionsStartTime != 0) revert EmissionsAlreadyStarted();
        if (address(bridgeProxy) == address(0)) revert BridgeNotConfigured();

        emissionsStartTime = block.timestamp;

        emit EmissionsStarted(block.timestamp);
    }

    /**
     * @notice Mint emission tokens and bridge them to L2
     * @dev This function can only be called by the emissions manager.
     *      It mints tokens based on the current epoch schedule and bridges them
     *      using the configured bridge proxy. Includes comprehensive safety checks.
     */
    function mintEmission() external whenNotPaused nonReentrant onlyRole(EMISSIONS_MANAGER_ROLE) {
        // Validate emissions state
        if (emissionsStartTime == 0) revert EmissionsNotStarted();
        if (emissionsEnded()) revert AllEmissionsCompleted();

        // Calculate epochs elapsed since start
        uint256 epochsSinceStart = (block.timestamp - emissionsStartTime) / EPOCH_DURATION;
        // Check if enough time has passed (with buffer to prevent MEV)
        if (block.timestamp < getNextEmissionTime() - EMISSION_BUFFER_TIME) {
            revert EmissionTooEarly();
        }

        // Calculate how many epochs to mint (handles catching up if behind)
        uint256 epochsToMint = epochsSinceStart - currentEpoch;
        if (epochsToMint > TOTAL_EPOCHS - currentEpoch) {
            epochsToMint = TOTAL_EPOCHS - currentEpoch;
        }

        // Calculate total amount to mint
        uint256 totalToMint = 0;
        for (uint256 i = 0; i < epochsToMint; i++) {
            totalToMint += emissionSchedule[currentEpoch + i];
        }

        // Update state before external calls
        currentEpoch += epochsToMint;
        totalEmissionsMinted += totalToMint;

        // Mint tokens to this contract
        syndicateToken.mint(address(this), totalToMint);

        // Approve the bridge proxy to spend our tokens
        IERC20 tokenInterface = IERC20(address(syndicateToken));
        tokenInterface.forceApprove(address(bridgeProxy), totalToMint);

        // Bridge the tokens using the configured proxy
        bridgeProxy.executeBridge(address(syndicateToken), totalToMint, bridgeData);

        emit EmissionMinted(currentEpoch, totalToMint, address(bridgeProxy));
    }

    /*//////////////////////////////////////////////////////////////
                      BRIDGE PROXY CONFIGURATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set the bridge proxy for emissions distribution
     * @dev Only callable by addresses with BRIDGE_MANAGER_ROLE
     *
     * @param _bridgeProxy The new bridge proxy contract implementing IBridgeProxy
     */
    function setBridgeProxy(IBridgeProxy _bridgeProxy) external onlyRole(BRIDGE_MANAGER_ROLE) {
        if (address(_bridgeProxy) == address(0)) revert ZeroAddress();

        IBridgeProxy oldProxy = bridgeProxy;
        bridgeProxy = _bridgeProxy;

        emit BridgeProxyUpdated(address(oldProxy), address(_bridgeProxy));
    }

    /**
     * @notice Set bridge configuration data
     * @dev This data is passed to the bridge proxy during execution.
     *      Format depends on the specific bridge implementation.
     *      Only callable by addresses with BRIDGE_MANAGER_ROLE.
     *
     * @param _bridgeData Encoded bridge configuration data
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
     * @notice Get current bridge configuration
     * @return proxy The current bridge proxy contract
     * @return data The current bridge configuration data
     */
    function getBridgeConfiguration() external view returns (IBridgeProxy proxy, bytes memory data) {
        return (bridgeProxy, bridgeData);
    }

    /**
     * @notice Get the complete emission schedule
     * @return The array of emission amounts for each epoch
     */
    function getEmissionSchedule() external view returns (uint256[TOTAL_EPOCHS] memory) {
        return emissionSchedule;
    }

    /**
     * @notice Get comprehensive information about the current epoch
     * @return epoch Current epoch number
     * @return nextEmissionTime Timestamp when next emission can be minted
     * @return nextEmissionAmount Amount of tokens in the next emission
     * @return canMintEmission Whether emission can currently be minted
     */
    function getCurrentEpochInfo()
        external
        view
        returns (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission)
    {
        if (emissionsStartTime == 0) {
            return (0, 0, 0, false);
        }

        uint256 epochsSinceStart = (block.timestamp - emissionsStartTime) / EPOCH_DURATION;
        uint256 nextEmissionTimestamp = emissionsStartTime + ((currentEpoch + 1) * EPOCH_DURATION);
        uint256 nextAmount = currentEpoch < TOTAL_EPOCHS ? emissionSchedule[currentEpoch] : 0;
        bool canMint = emissionsStartTime > 0 && epochsSinceStart > currentEpoch && currentEpoch < TOTAL_EPOCHS
            && block.timestamp >= nextEmissionTimestamp - EMISSION_BUFFER_TIME && !paused();

        return (currentEpoch, nextEmissionTimestamp, nextAmount, canMint);
    }

    /**
     * @notice Check if all emissions have been completed
     * @return True if all 48 epochs have been minted
     */
    function emissionsEnded() public view returns (bool) {
        return currentEpoch >= TOTAL_EPOCHS;
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
        return emissionsStartTime + ((currentEpoch + 1) * EPOCH_DURATION);
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

    /*//////////////////////////////////////////////////////////////
                          INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the emission schedule with predefined decay pattern
     * @dev Creates a decreasing emission schedule over 8 periods of 6 epochs each.
     *      Each period has a lower emission rate than the previous one.
     */
    function _initializeEmissionSchedule() private {
        // Define emission amounts per period (6 epochs each)
        // These amounts are designed to distribute 100M tokens over 48 epochs
        // with a decreasing pattern to incentivize early adoption
        uint256[8] memory emissionAmounts = [
            uint256(6_780_550 * 10 ** 18), // Epochs 1-6: Highest emissions
            uint256(4_068_330 * 10 ** 18), // Epochs 7-12
            uint256(2_441_000 * 10 ** 18), // Epochs 13-18
            uint256(1_464_600 * 10 ** 18), // Epochs 19-24
            uint256(878_760 * 10 ** 18), // Epochs 25-30
            uint256(527_260 * 10 ** 18), // Epochs 31-36
            uint256(316_350 * 10 ** 18), // Epochs 37-42
            uint256(189_810 * 10 ** 18) // Epochs 43-48: Lowest emissions
        ];

        // Fill the emission schedule array
        for (uint256 period = 0; period < 8; period++) {
            for (uint256 epoch = 0; epoch < 6; epoch++) {
                uint256 scheduleIndex = period * 6 + epoch;
                if (scheduleIndex < TOTAL_EPOCHS) {
                    emissionSchedule[scheduleIndex] = emissionAmounts[period];
                }
            }
        }
    }
}
