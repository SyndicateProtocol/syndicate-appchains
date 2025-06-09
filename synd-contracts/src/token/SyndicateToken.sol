// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {AbstractXERC20} from "./AbstractXERC20.sol";
import {IBridgeProxy} from "./interfaces/IBridgeProxy.sol";

/**
 * @title SyndicateToken
 * @notice Syndicate Token with emissions and XERC20 functionality
 * @dev This is the main token contract deployed on Ethereum (L1)
 *      It includes emissions logic for distributing tokens to L2s
 */
contract SyndicateToken is AbstractXERC20, Pausable {
    // Custom errors for emissions logic
    error EmissionsAlreadyStarted();
    error EmissionsNotStarted();
    error EmissionsNotActive();
    error AllEmissionsCompleted();
    error EpochAlreadyMinted();
    error ExceedsEmissionsSupply();
    error BridgeNotConfigured();
    error ZeroGasLimit();
    error InvalidEpoch();
    error EmissionTooEarly();

    // Role definitions for emissions
    bytes32 public constant EMISSIONS_MANAGER_ROLE = keccak256("EMISSIONS_MANAGER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    // Token supply constants
    uint256 public constant TOTAL_SUPPLY = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 public constant INITIAL_MINT_SUPPLY = 90_000_000 * 10 ** 18; // 90M tokens (90%) - Initial distribution to foundation
    uint256 public constant EMISSIONS_SUPPLY = 10_000_000 * 10 ** 18; // 10M tokens (10%) - Reserved for emissions over 4 years

    // Emission timing constants
    uint256 public constant EPOCH_DURATION = 30 days; // 30 days per epoch
    uint256 public constant TOTAL_EPOCHS = 48; // 48 epochs = ~4 years total

    // Emission buffer to prevent edge cases
    uint256 public constant EMISSION_BUFFER_TIME = 1 hours; // 1 hour buffer before next emission

    // Emission amounts per epoch (in wei) - organized by 6-epoch periods with decay
    uint256[TOTAL_EPOCHS] public emissionSchedule;

    // Emissions state
    //#olympix-ignore-uninitialized-state-variable
    bool public emissionsActive; // Covers both started and can be paused
    uint256 public emissionsStartTime; //#olympix-ignore-uninitialized-state-variable
    uint256 public currentEpoch; //#olympix-ignore-uninitialized-state-variable
    uint256 public totalEmissionsMinted; //#olympix-ignore-uninitialized-state-variable

    // Bridge configuration for upgradeable bridge logic
    address public bridgeProxy; // The bridge proxy contract address
    //#olympix-ignore-uninitialized-state-variable
    bytes public bridgeData; // Encoded bridge configuration data

    // Events - Emissions
    event EmissionsStarted(uint256 startTime);
    event EmissionsPaused();
    event EmissionsResumed();
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed destination);
    event BridgeProxyUpdated(address indexed oldProxy, address indexed newProxy);
    event BridgeDataUpdated(bytes oldData, bytes newData);

    constructor(address defaultAdmin, address syndFoundationAddress, address emissionsManager, address pauser)
        AbstractXERC20("Syndicate", "SYND")
    {
        if (defaultAdmin == address(0)) revert ZeroAddress();
        if (syndFoundationAddress == address(0)) revert ZeroAddress();
        if (emissionsManager == address(0)) revert ZeroAddress();
        if (pauser == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(EMISSIONS_MANAGER_ROLE, emissionsManager);
        _grantRole(BRIDGE_MANAGER_ROLE, defaultAdmin);
        _grantRole(PAUSER_ROLE, pauser);

        // Mint initial supply to foundation
        _mint(syndFoundationAddress, INITIAL_MINT_SUPPLY);

        _initializeEmissionSchedule();
    }

    // =============================================================================
    // EMISSIONS LOGIC
    // =============================================================================

    /**
     * @notice Start the emissions process (can only be called once)
     */
    function startEmissions() external onlyRole(EMISSIONS_MANAGER_ROLE) {
        if (emissionsStartTime != 0) revert EmissionsAlreadyStarted();
        if (bridgeProxy == address(0)) revert BridgeNotConfigured();

        emissionsActive = true;
        emissionsStartTime = block.timestamp;

        emit EmissionsStarted(block.timestamp);
    }

    /**
     * @notice Pause emissions (emergency function)
     */
    function pauseEmissions() external onlyRole(PAUSER_ROLE) {
        emissionsActive = false;
        emit EmissionsPaused();
    }

    /**
     * @notice Resume emissions
     */
    function resumeEmissions() external onlyRole(EMISSIONS_MANAGER_ROLE) {
        if (emissionsStartTime == 0) revert EmissionsNotStarted();
        emissionsActive = true;
        emit EmissionsResumed();
    }

    /**
     * @notice Mint emission tokens and bridge them to L2
     * @dev This function can only be called by the emissions manager
     * @dev It mints tokens based on the current epoch and bridges them using the configured bridge proxy
     */
    function mintEmission() external whenNotPaused nonReentrant onlyRole(EMISSIONS_MANAGER_ROLE) {
        if (!emissionsActive) revert EmissionsNotActive();
        if (emissionsEnded()) revert AllEmissionsCompleted();

        uint256 epochsSinceStart = (block.timestamp - emissionsStartTime) / EPOCH_DURATION;
        if (epochsSinceStart <= currentEpoch) revert EpochAlreadyMinted();
        if (block.timestamp < getNextEmissionTime() - EMISSION_BUFFER_TIME) revert EmissionTooEarly();

        // Calculate how many epochs we can mint (in case we're behind)
        uint256 epochsToMint = epochsSinceStart - currentEpoch;
        if (epochsToMint > TOTAL_EPOCHS - currentEpoch) {
            epochsToMint = TOTAL_EPOCHS - currentEpoch;
        }

        uint256 totalToMint = 0;
        for (uint256 i = 0; i < epochsToMint; i++) {
            totalToMint += emissionSchedule[currentEpoch + i];
        }

        if (totalEmissionsMinted + totalToMint > EMISSIONS_SUPPLY) {
            revert ExceedsEmissionsSupply();
        }

        currentEpoch += epochsToMint;
        totalEmissionsMinted += totalToMint;

        // Mint tokens to this contract first
        _mint(address(this), totalToMint);

        // Bridge the tokens using the proxy
        _approve(address(this), bridgeProxy, totalToMint);
        IBridgeProxy(bridgeProxy).executeBridge(address(this), totalToMint, bridgeData);

        emit EmissionMinted(currentEpoch, totalToMint, address(0));
    }

    // =============================================================================
    // BRIDGE PROXY CONFIGURATION
    // =============================================================================

    /**
     * @notice Set the bridge proxy for upgradeable bridge logic
     * @param _bridgeProxy The bridge proxy contract address
     */
    function setBridgeProxy(address _bridgeProxy) external onlyRole(BRIDGE_MANAGER_ROLE) {
        if (_bridgeProxy == address(0)) revert ZeroAddress();

        address oldProxy = bridgeProxy;
        bridgeProxy = _bridgeProxy;

        emit BridgeProxyUpdated(oldProxy, _bridgeProxy);
    }

    /**
     * @notice Set bridge configuration data
     * @param _bridgeData Encoded bridge configuration data
     */
    function setBridgeData(bytes calldata _bridgeData) external onlyRole(BRIDGE_MANAGER_ROLE) {
        bytes memory oldData = bridgeData;
        bridgeData = _bridgeData;

        emit BridgeDataUpdated(oldData, _bridgeData);
    }

    // =============================================================================
    // VIEW FUNCTIONS
    // =============================================================================

    /**
     * @notice Get bridge configuration
     * @return proxy The current bridge proxy address
     * @return data The bridge configuration data
     */
    function getBridgeConfiguration() external view returns (address proxy, bytes memory data) {
        return (bridgeProxy, bridgeData);
    }

    /**
     * @notice Get the complete emission schedule
     * @return The array of emission amounts per epoch
     */
    function getEmissionSchedule() external view returns (uint256[TOTAL_EPOCHS] memory) {
        return emissionSchedule;
    }

    /**
     * @notice Get current epoch information
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
        bool canMint = emissionsActive && epochsSinceStart > currentEpoch && currentEpoch < TOTAL_EPOCHS;

        return (currentEpoch, nextEmissionTimestamp, nextAmount, canMint);
    }

    /**
     * @notice Get total emissions remaining
     */
    function getRemainingEmissions() external view returns (uint256) {
        return EMISSIONS_SUPPLY - totalEmissionsMinted;
    }

    /**
     * @notice Check if all emissions have been completed
     */
    function emissionsEnded() public view returns (bool) {
        return currentEpoch >= TOTAL_EPOCHS;
    }

    /**
     * @notice Check if emissions have been started
     */
    function emissionsStarted() public view returns (bool) {
        return emissionsStartTime > 0;
    }

    /**
     * @notice Get next emission time
     */
    function getNextEmissionTime() public view returns (uint256) {
        if (emissionsStartTime == 0) return 0;
        return emissionsStartTime + ((currentEpoch + 1) * EPOCH_DURATION);
    }

    // =============================================================================
    // EMERGENCY CONTROLS
    // =============================================================================

    /**
     * @notice Pause the contract (emergency function)
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpause the contract
     */
    function unpause() external onlyRole(DEFAULT_ADMIN_ROLE) {
        _unpause();
    }

    // =============================================================================
    // INTERNAL FUNCTIONS
    // =============================================================================

    /**
     * @dev Initialize the emission schedule according to the decay table
     */
    function _initializeEmissionSchedule() private {
        // Define emission amounts per period (6 epochs each)
        uint256[8] memory emissionAmounts = [
            uint256(678_055 * 10 ** 18), // Epochs 1-6
            uint256(406_833 * 10 ** 18), // Epochs 7-12
            uint256(244_100 * 10 ** 18), // Epochs 13-18
            uint256(146_460 * 10 ** 18), // Epochs 19-24
            uint256(87_876 * 10 ** 18), // Epochs 25-30
            uint256(52_726 * 10 ** 18), // Epochs 31-36
            uint256(31_635 * 10 ** 18), // Epochs 37-42
            uint256(18_981 * 10 ** 18) // Epochs 43-48
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
