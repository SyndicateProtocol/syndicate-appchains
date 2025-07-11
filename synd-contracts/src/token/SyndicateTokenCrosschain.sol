// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateToken} from "./SyndicateToken.sol";
import {IERC7802} from "./crosschain/interfaces/IERC7802.sol";
import {IBridgeRateLimiter} from "./crosschain/interfaces/IBridgeRateLimiter.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

/**
 * @title SyndicateTokenCrosschain
 * @notice Crosschain-compatible Syndicate Protocol token
 * @dev Extends the existing SyndicateToken with crosschain capabilities including:
 *      - ERC7802 compatibility for SuperChain and modern bridges
 *      - Bridge rate limiting and access controls
 *      - Same contract address across all chains via deterministic deployment
 *      - Full compatibility with existing emission scheduler and governance
 *
 * Key Features:
 * - All existing SyndicateToken functionality (governance, permits, emissions, etc.)
 * - Native crosschain mint/burn functions for authorized bridges
 * - Rate limiting per bridge to prevent abuse
 * - Emergency controls for bridge management
 * - ERC165 interface detection for bridge compatibility
 *
 */
contract SyndicateTokenCrosschain is SyndicateToken, IERC7802, IBridgeRateLimiter {
    using EnumerableSet for EnumerableSet.AddressSet;

    /*//////////////////////////////////////////////////////////////
                                  ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing bridge configurations
    bytes32 public constant BRIDGE_MANAGER_ROLE = keccak256("BRIDGE_MANAGER_ROLE");

    /// @notice Role for allocating emission budgets to bridges (typically emission scheduler)
    bytes32 public constant EMISSION_BUDGET_MANAGER_ROLE = keccak256("EMISSION_BUDGET_MANAGER_ROLE");

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Mapping of bridge address to bridge configuration
    mapping(address => BridgeConfig) public bridgeConfigs;

    /// @notice Set of all configured bridges for enumeration and O(1) operations
    EnumerableSet.AddressSet private bridges;

    /// @notice Mapping of bridge address to emission budget allocated
    mapping(address => uint256) public bridgeEmissionBudgets;

    /// @notice Mapping of bridge -> hour -> mint usage for sliding window rate limiting
    mapping(address => mapping(uint256 => uint256)) public hourlyMintUsage;

    /// @notice Mapping of bridge -> hour -> burn usage for sliding window rate limiting
    mapping(address => mapping(uint256 => uint256)) public hourlyBurnUsage;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a bridge is added to the system
    /// @param bridge Bridge address
    /// @param dailyMintLimit Initial daily mint limit
    /// @param dailyBurnLimit Initial daily burn limit
    event BridgeAdded(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);

    /// @notice Emitted when a bridge is removed from the system
    /// @param bridge Bridge address
    event BridgeRemoved(address indexed bridge);

    /// @notice Emitted when emission budget is allocated to a bridge
    /// @param bridge Bridge address
    /// @param amount Amount of emission budget allocated
    event EmissionBudgetAllocated(address indexed bridge, uint256 amount);

    /// @notice Emitted when emission budget is consumed by a bridge
    /// @param bridge Bridge address
    /// @param amount Amount of emission budget consumed
    event EmissionBudgetConsumed(address indexed bridge, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the crosschain Syndicate token
     * @param defaultAdmin Address that will have default admin privileges
     * @param syndTreasuryAddress Address to receive the initial token mint
     */
    constructor(address defaultAdmin, address syndTreasuryAddress) SyndicateToken(defaultAdmin, syndTreasuryAddress) {
        // Grant bridge manager role to admin
        _grantRole(BRIDGE_MANAGER_ROLE, defaultAdmin);

        // Grant emission budget manager role to admin (typically transferred to emission scheduler)
        _grantRole(EMISSION_BUDGET_MANAGER_ROLE, defaultAdmin);
    }

    /*//////////////////////////////////////////////////////////////
                        CROSSCHAIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IERC7802
    function crosschainMint(address to, uint256 amount) external override {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        // Check that we don't exceed the total supply
        if (totalSupply() + amount > TOTAL_SUPPLY) {
            revert ExceedsTotalSupply();
        }

        // Check if bridge is authorized and has sufficient limits
        _validateAndUseMintLimit(msg.sender, amount);

        // Check and consume emission budget
        _validateAndConsumeEmissionBudget(msg.sender, amount);

        // Check if transfers are locked and caller is not authorized to bypass
        if (transfersLocked() && !hasRole(AIRDROP_MANAGER_ROLE, msg.sender)) {
            revert TransfersLocked();
        }

        // Mint tokens
        _mint(to, amount);

        emit CrosschainMint(to, amount, msg.sender);
    }

    /// @inheritdoc IERC7802
    function crosschainBurn(address from, uint256 amount) external override {
        if (from == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        // Check if bridge is authorized and has sufficient limits
        _validateAndUseBurnLimit(msg.sender, amount);

        // Handle allowance if caller is not the token owner and there is an allowance
        if (msg.sender != from) {
            uint256 currentAllowance = allowance(from, msg.sender);
            if (currentAllowance != 0) {
                // If there's an allowance, consume it
                _spendAllowance(from, msg.sender, amount);
            }
            // If no allowance, trusted bridges can still burn (this is the crosschain bridge pattern)
        }

        // Burn tokens
        _burn(from, amount);

        emit CrosschainBurn(from, amount, msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                        BRIDGE MANAGEMENT
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IBridgeRateLimiter
    function setBridgeLimits(address bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit)
        external
        override
        onlyRole(BRIDGE_MANAGER_ROLE)
    {
        if (bridge == address(0)) revert ZeroAddress();

        // Prevent bridge manager from adding themselves as a bridge
        if (bridge == msg.sender) revert CannotAddSelfAsBridge();

        // Require bridge to be a contract (not an EOA)
        if (bridge.code.length == 0) revert BridgeMustBeContract();

        // Verify that the limits are not unreasonably high (allow max uint256 as "unlimited")
        if (dailyMintLimit != type(uint256).max && dailyMintLimit > TOTAL_SUPPLY) revert UnreasonableMintLimit();
        if (dailyBurnLimit != type(uint256).max && dailyBurnLimit > TOTAL_SUPPLY) revert UnreasonableBurnLimit();

        // Add bridge to set if not already added
        if (bridges.add(bridge)) {
            emit BridgeAdded(bridge, dailyMintLimit, dailyBurnLimit);
        }

        // Set bridge configuration
        bridgeConfigs[bridge] =
            BridgeConfig({dailyMintLimit: dailyMintLimit, dailyBurnLimit: dailyBurnLimit, isActive: true});

        emit BridgeLimitsSet(bridge, dailyMintLimit, dailyBurnLimit);
    }

    /// @inheritdoc IBridgeRateLimiter
    function setBridgeActive(address bridge, bool isActive) external override onlyRole(BRIDGE_MANAGER_ROLE) {
        if (bridge == address(0)) revert ZeroAddress();
        if (!bridges.contains(bridge)) revert UnauthorizedBridge(bridge);

        bridgeConfigs[bridge].isActive = isActive;
        emit BridgeActiveStatusChanged(bridge, isActive);
    }

    /**
     * @notice Remove a bridge from the system
     * @param bridge Bridge address to remove
     * @dev Only callable by bridge manager role
     */
    function removeBridge(address bridge) external onlyRole(BRIDGE_MANAGER_ROLE) {
        if (bridge == address(0)) revert ZeroAddress();
        if (!bridges.remove(bridge)) revert UnauthorizedBridge(bridge);

        // Remove from mapping
        delete bridgeConfigs[bridge];

        emit BridgeRemoved(bridge);
    }

    /**
     * @notice Allocate emission budget to a bridge
     * @param bridge Bridge address to allocate budget to
     * @param amount Amount of emission budget to allocate
     * @dev Only callable by emission budget manager role (typically emission scheduler)
     */
    function allocateEmissionBudget(address bridge, uint256 amount) external onlyRole(EMISSION_BUDGET_MANAGER_ROLE) {
        if (bridge == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (!bridges.contains(bridge)) revert UnauthorizedBridge(bridge);

        bridgeEmissionBudgets[bridge] += amount;
        emit EmissionBudgetAllocated(bridge, amount);
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Validate bridge authorization and consume mint limit using sliding window
     * @param bridge Bridge address
     * @param amount Amount to mint
     */
    function _validateAndUseMintLimit(address bridge, uint256 amount) internal {
        BridgeConfig storage config = bridgeConfigs[bridge];

        // Check if bridge is authorized
        if (!bridges.contains(bridge) || !config.isActive) {
            revert UnauthorizedBridge(bridge);
        }

        // Calculate total usage in the past 24 hours using sliding window
        uint256 currentHour = block.timestamp / 1 hours;
        uint256 totalUsageIn24Hours = 0;

        // Sum usage over the past 24 hourly buckets (sliding window)
        for (uint256 i = 0; i < 24; i++) {
            if (currentHour >= i) {
                totalUsageIn24Hours += hourlyMintUsage[bridge][currentHour - i];
            }
        }

        // Check if adding this amount would exceed the daily limit
        if (totalUsageIn24Hours + amount > config.dailyMintLimit) {
            uint256 available =
                config.dailyMintLimit > totalUsageIn24Hours ? config.dailyMintLimit - totalUsageIn24Hours : 0;
            revert InsufficientMintLimit(bridge, amount, available);
        }

        // Record the new usage in the current hourly bucket
        hourlyMintUsage[bridge][currentHour] += amount;
    }

    /**
     * @notice Validate bridge authorization and consume burn limit using sliding window
     * @param bridge Bridge address
     * @param amount Amount to burn
     */
    function _validateAndUseBurnLimit(address bridge, uint256 amount) internal {
        BridgeConfig storage config = bridgeConfigs[bridge];

        // Check if bridge is authorized
        if (!bridges.contains(bridge) || !config.isActive) {
            revert UnauthorizedBridge(bridge);
        }

        // Calculate total usage in the past 24 hours using sliding window
        uint256 currentHour = block.timestamp / 1 hours;
        uint256 totalUsageIn24Hours = 0;

        // Sum usage over the past 24 hourly buckets (sliding window)
        for (uint256 i = 0; i < 24; i++) {
            if (currentHour >= i) {
                totalUsageIn24Hours += hourlyBurnUsage[bridge][currentHour - i];
            }
        }

        // Check if adding this amount would exceed the daily limit
        if (totalUsageIn24Hours + amount > config.dailyBurnLimit) {
            uint256 available =
                config.dailyBurnLimit > totalUsageIn24Hours ? config.dailyBurnLimit - totalUsageIn24Hours : 0;
            revert InsufficientBurnLimit(bridge, amount, available);
        }

        // Record the new usage in the current hourly bucket
        hourlyBurnUsage[bridge][currentHour] += amount;
    }

    /**
     * @notice Validate bridge emission budget and consume it
     * @param bridge Bridge address
     * @param amount Amount to consume from emission budget
     */
    function _validateAndConsumeEmissionBudget(address bridge, uint256 amount) internal {
        // Check if bridge has sufficient emission budget
        uint256 availableBudget = bridgeEmissionBudgets[bridge];
        if (amount > availableBudget) {
            revert InsufficientEmissionBudget();
        }

        // Consume the emission budget
        bridgeEmissionBudgets[bridge] -= amount;
        emit EmissionBudgetConsumed(bridge, amount);
    }

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IBridgeRateLimiter
    function getBridgeConfig(address bridge) external view override returns (BridgeConfig memory config) {
        return bridgeConfigs[bridge];
    }

    /// @inheritdoc IBridgeRateLimiter
    function getAvailableMintLimit(address bridge) external view override returns (uint256 available) {
        BridgeConfig memory config = bridgeConfigs[bridge];

        // Calculate total usage in the past 24 hours using sliding window
        uint256 currentHour = block.timestamp / 1 hours;
        uint256 totalUsageIn24Hours = 0;

        // Sum usage over the past 24 hourly buckets (sliding window)
        for (uint256 i = 0; i < 24; i++) {
            if (currentHour >= i) {
                totalUsageIn24Hours += hourlyMintUsage[bridge][currentHour - i];
            }
        }

        // Return remaining limit
        return config.dailyMintLimit > totalUsageIn24Hours ? config.dailyMintLimit - totalUsageIn24Hours : 0;
    }

    /// @inheritdoc IBridgeRateLimiter
    function getAvailableBurnLimit(address bridge) external view override returns (uint256 available) {
        BridgeConfig memory config = bridgeConfigs[bridge];

        // Calculate total usage in the past 24 hours using sliding window
        uint256 currentHour = block.timestamp / 1 hours;
        uint256 totalUsageIn24Hours = 0;

        // Sum usage over the past 24 hourly buckets (sliding window)
        for (uint256 i = 0; i < 24; i++) {
            if (currentHour >= i) {
                totalUsageIn24Hours += hourlyBurnUsage[bridge][currentHour - i];
            }
        }

        // Return remaining limit
        return config.dailyBurnLimit > totalUsageIn24Hours ? config.dailyBurnLimit - totalUsageIn24Hours : 0;
    }

    /// @inheritdoc IBridgeRateLimiter
    function isBridgeAuthorized(address bridge) external view override returns (bool authorized) {
        BridgeConfig memory config = bridgeConfigs[bridge];
        return bridges.contains(bridge) && config.isActive;
    }

    /**
     * @notice Get total number of configured bridges
     * @return count Number of bridges
     */
    function getBridgeCount() external view returns (uint256 count) {
        return bridges.length();
    }

    /**
     * @notice Get bridge address at index
     * @param index Index in bridges set
     * @return bridge Bridge address
     */
    function getBridgeAtIndex(uint256 index) external view returns (address bridge) {
        require(index < bridges.length(), "SyndicateTokenCrosschain: index out of bounds");
        return bridges.at(index);
    }

    /**
     * @notice Get all configured bridges
     * @return allBridges Array of bridge addresses
     */
    function getAllBridges() external view returns (address[] memory allBridges) {
        return bridges.values();
    }

    /**
     * @notice Get emission budget for a bridge
     * @param bridge Bridge address
     * @return budget Available emission budget for the bridge
     */
    function getEmissionBudget(address bridge) external view returns (uint256 budget) {
        return bridgeEmissionBudgets[bridge];
    }

    /*//////////////////////////////////////////////////////////////
                        INTERFACE SUPPORT
    //////////////////////////////////////////////////////////////*/

    /// @notice Check if contract supports an interface
    /// @param interfaceId Interface identifier to check
    /// @return true if interface is supported
    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(AccessControl, IERC165)
        returns (bool)
    {
        return interfaceId == type(IERC7802).interfaceId || interfaceId == type(IBridgeRateLimiter).interfaceId
            || super.supportsInterface(interfaceId);
    }
}
