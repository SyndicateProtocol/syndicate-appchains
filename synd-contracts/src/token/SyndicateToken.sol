// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";

// Interface for Optimism StandardBridge
interface IStandardBridge {
    function bridgeERC20To(
        address _localToken,
        address _remoteToken,
        address _to,
        uint256 _amount,
        uint32 _minGasLimit,
        bytes memory _extraData
    ) external;
}

contract SyndicateToken is ERC20, AccessControl, ERC20Permit {
    bytes32 public constant EMISSIONS_MANAGER_ROLE = keccak256("EMISSIONS_MANAGER_ROLE");

    uint256 public constant TOTAL_SUPPLY = 100_000_000 * 10 ** 18; // 100M tokens
    uint256 public constant INITIAL_MINT_SUPPLY = 90_000_000 * 10 ** 18; // 90M tokens (90%)
    uint256 public constant EMISSIONS_SUPPLY = 10_000_000 * 10 ** 18; // 10M tokens (10%)
    uint256 public constant EPOCH_DURATION = 30 days;
    uint256 public constant TOTAL_EPOCHS = 48;

    // Emission amounts per epoch (in tokens, will be multiplied by 10**18)
    uint256 public constant EMISSION_AMOUNT_1 = 678_055 * 10 ** 18; // Epochs 1-6
    uint256 public constant EMISSION_AMOUNT_2 = 406_833 * 10 ** 18; // Epochs 7-12
    uint256 public constant EMISSION_AMOUNT_3 = 244_100 * 10 ** 18; // Epochs 13-18
    uint256 public constant EMISSION_AMOUNT_4 = 146_460 * 10 ** 18; // Epochs 19-24
    uint256 public constant EMISSION_AMOUNT_5 = 87_876 * 10 ** 18; // Epochs 25-30
    uint256 public constant EMISSION_AMOUNT_6 = 52_726 * 10 ** 18; // Epochs 31-36
    uint256 public constant EMISSION_AMOUNT_7 = 31_635 * 10 ** 18; // Epochs 37-42
    uint256 public constant EMISSION_AMOUNT_8 = 18_981 * 10 ** 18; // Epochs 43-48

    // Emissions state
    bool public emissionsStarted;
    uint256 public emissionsStartTime;
    uint256 public currentEpoch;
    uint256 public totalEmissionsMinted;

    // Bridge configuration
    address public bridgeAddress; // The Optimism StandardBridge contract address
    address public l2TokenAddress; // The remote token address on L2
    address public l2DestinationAddress; // The destination address on L2 (splitter contract)
    uint32 public bridgeGasLimit; // Gas limit for bridge transaction
    bytes public bridgeExtraData; // Extra data for bridge transaction

    // Emission schedule (tokens per epoch in wei)
    uint256[TOTAL_EPOCHS] public emissionSchedule;

    // Events
    event EmissionsStarted(uint256 startTime);
    event EmissionMinted(uint256 epoch, uint256 amount, address indexed l2DestinationAddress);
    event BridgeConfigurationUpdated(
        address indexed bridgeAddress,
        address indexed l2TokenAddress,
        address indexed l2DestinationAddress,
        uint32 gasLimit
    );

    constructor(address defaultAdmin, address syndFoundationAddress, address emissionsManager)
        ERC20("Syndicate", "SYND")
        ERC20Permit("Syndicate")
    {
        require(defaultAdmin != address(0), "Default admin cannot be zero address");
        require(syndFoundationAddress != address(0), "Foundation address cannot be zero address");
        require(emissionsManager != address(0), "Emissions manager cannot be zero address");

        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(EMISSIONS_MANAGER_ROLE, emissionsManager);

        // Mint initial supply to foundation
        _mint(syndFoundationAddress, INITIAL_MINT_SUPPLY);

        _initializeEmissionSchedule();
    }

    /**
     * @notice Start the emissions process (can only be called once)
     */
    function startEmissions() external onlyRole(EMISSIONS_MANAGER_ROLE) {
        require(!emissionsStarted, "Emissions already started");
        require(bridgeAddress != address(0), "Bridge address not set");
        require(l2TokenAddress != address(0), "L2 token address not set");
        require(l2DestinationAddress != address(0), "L2 destination address not set");
        require(bridgeGasLimit > 0, "Bridge gas limit not set");

        emissionsStarted = true;
        emissionsStartTime = block.timestamp;

        emit EmissionsStarted(block.timestamp);
    }

    /**
     * @notice Mint emission tokens and bridge them to L2
     * Can be called by anyone once the time for an epoch has passed
     */
    function mintEmission() external {
        require(emissionsStarted, "Emissions not started");
        require(currentEpoch < TOTAL_EPOCHS, "All emissions completed");

        uint256 epochsSinceStart = (block.timestamp - emissionsStartTime) / EPOCH_DURATION;
        require(epochsSinceStart > currentEpoch, "Current epoch already minted");

        // Calculate how many epochs we can mint (in case we're behind)
        uint256 epochsToMint = epochsSinceStart - currentEpoch;
        if (epochsToMint > TOTAL_EPOCHS - currentEpoch) {
            epochsToMint = TOTAL_EPOCHS - currentEpoch;
        }

        uint256 totalToMint = 0;
        for (uint256 i = 0; i < epochsToMint; i++) {
            totalToMint += emissionSchedule[currentEpoch + i];
        }

        require(totalEmissionsMinted + totalToMint <= EMISSIONS_SUPPLY, "Cannot exceed emissions supply");

        currentEpoch += epochsToMint;
        totalEmissionsMinted += totalToMint;

        // Mint tokens to this contract first
        _mint(address(this), totalToMint);

        // Approve the bridge to spend the tokens
        _approve(address(this), bridgeAddress, totalToMint);

        // Bridge the tokens to L2
        IStandardBridge(bridgeAddress).bridgeERC20To(
            address(this), // _localToken (this contract)
            l2TokenAddress, // _remoteToken (L2 token address)
            l2DestinationAddress, // _to (destination on L2)
            totalToMint, // _amount
            bridgeGasLimit, // _minGasLimit
            bridgeExtraData // _extraData
        );

        emit EmissionMinted(currentEpoch - 1, totalToMint, l2DestinationAddress);
    }

    /**
     * @notice Set the bridge configuration for L2 emissions
     * @param _bridgeAddress The Optimism StandardBridge contract address
     * @param _l2TokenAddress The remote token address on L2
     * @param _l2DestinationAddress The destination address on L2 (splitter contract)
     * @param _gasLimit The gas limit for bridge transactions
     * @param _extraData Extra data for bridge transactions
     */
    function setBridgeConfiguration(
        address _bridgeAddress,
        address _l2TokenAddress,
        address _l2DestinationAddress,
        uint32 _gasLimit,
        bytes calldata _extraData
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_bridgeAddress != address(0), "Bridge address cannot be zero");
        require(_l2TokenAddress != address(0), "L2 token address cannot be zero");
        require(_l2DestinationAddress != address(0), "L2 destination address cannot be zero");
        require(_gasLimit > 0, "Gas limit must be greater than zero");

        bridgeAddress = _bridgeAddress;
        l2TokenAddress = _l2TokenAddress;
        l2DestinationAddress = _l2DestinationAddress;
        bridgeGasLimit = _gasLimit;
        bridgeExtraData = _extraData;

        emit BridgeConfigurationUpdated(_bridgeAddress, _l2TokenAddress, _l2DestinationAddress, _gasLimit);
    }

    /**
     * @notice Get bridge configuration
     * @return bridgeAddr The current bridge address
     * @return l2TokenAddr The L2 token address
     * @return l2DestAddr The L2 destination address
     * @return gasLimit The bridge gas limit
     * @return extraData The bridge extra data
     */
    function getBridgeConfiguration()
        external
        view
        returns (address bridgeAddr, address l2TokenAddr, address l2DestAddr, uint32 gasLimit, bytes memory extraData)
    {
        return (bridgeAddress, l2TokenAddress, l2DestinationAddress, bridgeGasLimit, bridgeExtraData);
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
     * @return epoch Current epoch number
     * @return nextEmissionTime Timestamp when next emission can be minted
     * @return nextEmissionAmount Amount of tokens for next emission
     * @return canMintEmission Whether emission can be minted now
     */
    function getCurrentEpochInfo()
        external
        view
        returns (uint256 epoch, uint256 nextEmissionTime, uint256 nextEmissionAmount, bool canMintEmission)
    {
        if (!emissionsStarted) {
            return (0, 0, 0, false);
        }

        uint256 epochsSinceStart = (block.timestamp - emissionsStartTime) / EPOCH_DURATION;
        uint256 nextEmissionTimestamp = emissionsStartTime + ((currentEpoch + 1) * EPOCH_DURATION);
        uint256 nextAmount = currentEpoch < TOTAL_EPOCHS ? emissionSchedule[currentEpoch] : 0;
        bool canMint = epochsSinceStart > currentEpoch && currentEpoch < TOTAL_EPOCHS;

        return (currentEpoch, nextEmissionTimestamp, nextAmount, canMint);
    }

    /**
     * @notice Get total emissions remaining
     * @return Amount of tokens still to be emitted
     */
    function getRemainingEmissions() external view returns (uint256) {
        return EMISSIONS_SUPPLY - totalEmissionsMinted;
    }

    /**
     * @notice Check if all emissions have been completed
     * @return True if all epochs have been minted
     */
    function isEmissionComplete() external view returns (bool) {
        return currentEpoch >= TOTAL_EPOCHS;
    }

    /**
     * @dev Initialize the emission schedule according to the decay table
     */
    function _initializeEmissionSchedule() private {
        // Epochs 1-6: EMISSION_AMOUNT_1 tokens each
        for (uint256 i = 0; i < 6; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_1;
        }

        // Epochs 7-12: EMISSION_AMOUNT_2 tokens each
        for (uint256 i = 6; i < 12; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_2;
        }

        // Epochs 13-18: EMISSION_AMOUNT_3 tokens each
        for (uint256 i = 12; i < 18; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_3;
        }

        // Epochs 19-24: EMISSION_AMOUNT_4 tokens each
        for (uint256 i = 18; i < 24; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_4;
        }

        // Epochs 25-30: EMISSION_AMOUNT_5 tokens each
        for (uint256 i = 24; i < 30; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_5;
        }

        // Epochs 31-36: EMISSION_AMOUNT_6 tokens each
        for (uint256 i = 30; i < 36; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_6;
        }

        // Epochs 37-42: EMISSION_AMOUNT_7 tokens each
        for (uint256 i = 36; i < 42; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_7;
        }

        // Epochs 43-48: EMISSION_AMOUNT_8 tokens each
        for (uint256 i = 42; i < 48; i++) {
            emissionSchedule[i] = EMISSION_AMOUNT_8;
        }
    }
}
