// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {SyndicateToken} from "./SyndicateToken.sol";
import {IERC7802} from "./crosschain/interfaces/IERC7802.sol";
import {IBridgeRateLimiter} from "./crosschain/interfaces/IBridgeRateLimiter.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

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
    /*//////////////////////////////////////////////////////////////
                                  ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for managing bridge configurations
    bytes32 public constant BRIDGE_MANAGER_ROLE = keccak256("BRIDGE_MANAGER_ROLE");

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Mapping of bridge address to bridge configuration
    mapping(address => BridgeConfig) public bridgeConfigs;

    /// @notice List of all configured bridges for enumeration
    address[] public bridges;

    /// @notice Mapping to track if address is in bridges array
    mapping(address => bool) public isBridgeAdded;

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

        // Add bridge to array if not already added
        if (!isBridgeAdded[bridge]) {
            bridges.push(bridge);
            isBridgeAdded[bridge] = true;
            emit BridgeAdded(bridge, dailyMintLimit, dailyBurnLimit);
        }

        // Reset daily usage when limits change
        bridgeConfigs[bridge] = BridgeConfig({
            dailyMintLimit: dailyMintLimit,
            dailyBurnLimit: dailyBurnLimit,
            lastMintTimestamp: block.timestamp,
            lastBurnTimestamp: block.timestamp,
            currentMintUsed: 0,
            currentBurnUsed: 0,
            isActive: true
        });

        emit BridgeLimitsSet(bridge, dailyMintLimit, dailyBurnLimit);
    }

    /// @inheritdoc IBridgeRateLimiter
    function setBridgeActive(address bridge, bool isActive) external override onlyRole(BRIDGE_MANAGER_ROLE) {
        if (bridge == address(0)) revert ZeroAddress();
        if (!isBridgeAdded[bridge]) revert UnauthorizedBridge(bridge);

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
        if (!isBridgeAdded[bridge]) revert UnauthorizedBridge(bridge);

        // Remove from mapping
        delete bridgeConfigs[bridge];
        isBridgeAdded[bridge] = false;

        // Remove from array (swap with last element and pop)
        for (uint256 i = 0; i < bridges.length; i++) {
            if (bridges[i] == bridge) {
                bridges[i] = bridges[bridges.length - 1];
                bridges.pop();
                break;
            }
        }

        emit BridgeRemoved(bridge);
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Validate bridge authorization and consume mint limit
     * @param bridge Bridge address
     * @param amount Amount to mint
     */
    function _validateAndUseMintLimit(address bridge, uint256 amount) internal {
        BridgeConfig storage config = bridgeConfigs[bridge];

        // Check if bridge is authorized
        if (!isBridgeAdded[bridge] || !config.isActive) {
            revert UnauthorizedBridge(bridge);
        }

        // Reset daily usage if a new day has started
        if (block.timestamp >= config.lastMintTimestamp + 1 days) {
            config.currentMintUsed = 0;
            config.lastMintTimestamp = block.timestamp;
        }

        // Check if sufficient limit available
        uint256 available = config.dailyMintLimit - config.currentMintUsed;
        if (amount > available) {
            revert InsufficientMintLimit(bridge, amount, available);
        }

        // Consume the limit
        config.currentMintUsed += amount;
    }

    /**
     * @notice Validate bridge authorization and consume burn limit
     * @param bridge Bridge address
     * @param amount Amount to burn
     */
    function _validateAndUseBurnLimit(address bridge, uint256 amount) internal {
        BridgeConfig storage config = bridgeConfigs[bridge];

        // Check if bridge is authorized
        if (!isBridgeAdded[bridge] || !config.isActive) {
            revert UnauthorizedBridge(bridge);
        }

        // Reset daily usage if a new day has started
        if (block.timestamp >= config.lastBurnTimestamp + 1 days) {
            config.currentBurnUsed = 0;
            config.lastBurnTimestamp = block.timestamp;
        }

        // Check if sufficient limit available
        uint256 available = config.dailyBurnLimit - config.currentBurnUsed;
        if (amount > available) {
            revert InsufficientBurnLimit(bridge, amount, available);
        }

        // Consume the limit
        config.currentBurnUsed += amount;
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

        // If a new day has started, return full limit
        if (block.timestamp >= config.lastMintTimestamp + 1 days) {
            return config.dailyMintLimit;
        }

        // Return remaining limit for today
        return config.dailyMintLimit - config.currentMintUsed;
    }

    /// @inheritdoc IBridgeRateLimiter
    function getAvailableBurnLimit(address bridge) external view override returns (uint256 available) {
        BridgeConfig memory config = bridgeConfigs[bridge];

        // If a new day has started, return full limit
        if (block.timestamp >= config.lastBurnTimestamp + 1 days) {
            return config.dailyBurnLimit;
        }

        // Return remaining limit for today
        return config.dailyBurnLimit - config.currentBurnUsed;
    }

    /// @inheritdoc IBridgeRateLimiter
    function isBridgeAuthorized(address bridge) external view override returns (bool authorized) {
        BridgeConfig memory config = bridgeConfigs[bridge];
        return isBridgeAdded[bridge] && config.isActive;
    }

    /**
     * @notice Get total number of configured bridges
     * @return count Number of bridges
     */
    function getBridgeCount() external view returns (uint256 count) {
        return bridges.length;
    }

    /**
     * @notice Get bridge address at index
     * @param index Index in bridges array
     * @return bridge Bridge address
     */
    function getBridgeAtIndex(uint256 index) external view returns (address bridge) {
        require(index < bridges.length, "SyndicateTokenCrosschain: index out of bounds");
        return bridges[index];
    }

    /**
     * @notice Get all configured bridges
     * @return allBridges Array of bridge addresses
     */
    function getAllBridges() external view returns (address[] memory allBridges) {
        return bridges;
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
