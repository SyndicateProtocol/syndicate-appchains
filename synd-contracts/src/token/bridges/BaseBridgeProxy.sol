// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {IBridgeProxy} from "../interfaces/IBridgeProxy.sol";

/**
 * @title BaseBridgeProxy
 * @notice Base implementation contract for all bridge proxy implementations
 * @dev This abstract contract provides common security features, access controls,
 *      and safety mechanisms that all bridge implementations should have.
 *
 * Features:
 * - Role-based access control with separate admin and caller roles
 * - Reentrancy protection for all bridge operations
 * - Daily transfer limits with automatic reset
 * - Maximum single transfer limits
 * - Emergency pause functionality
 * - Safe token handling with automatic approval management
 * - Token recovery mechanisms for stuck funds
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
abstract contract BaseBridgeProxy is IBridgeProxy, AccessControl, ReentrancyGuard {
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a zero address is provided where a valid address is required
    error ZeroAddress();

    /// @notice Thrown when a zero amount is provided for a bridge operation
    error ZeroAmount();

    /// @notice Thrown when a bridge call fails with a specific reason
    /// @param reason The failure reason returned by the bridge
    error BridgeCallFailed(string reason);

    /// @notice Thrown when trying to use a bridge that is currently inactive
    error BridgeNotActive();

    /// @notice Thrown when an unauthorized address tries to call a restricted function
    error UnauthorizedCaller();

    /// @notice Thrown when a transfer amount exceeds the configured limits
    error ExcessiveAmount();

    /*//////////////////////////////////////////////////////////////
                                 ROLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Role for bridge administration (configuration, pause/unpause)
    bytes32 public constant BRIDGE_ADMIN_ROLE = keccak256("BRIDGE_ADMIN_ROLE");

    /// @notice Role for executing bridge operations (typically the SyndicateToken contract)
    bytes32 public constant BRIDGE_CALLER_ROLE = keccak256("BRIDGE_CALLER_ROLE");

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The actual bridge contract address that this proxy calls
    address public bridgeTarget;

    /// @notice Whether this bridge is currently active and accepting transactions
    bool public bridgeActive;

    /// @notice Maximum amount that can be transferred in a single transaction
    uint256 public maxSingleTransfer;

    /// @notice Maximum amount that can be transferred in a 24-hour period
    uint256 public dailyLimit;

    /// @notice Amount already transferred today
    //#olympix-ignore-uninitialized-state-variable
    uint256 public dailyUsed;

    /// @notice The day number when daily limits were last reset (block.timestamp / 1 days)
    uint256 public lastResetDay;

    /// @notice Human-readable name of this bridge
    string public bridgeName;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when the bridge target address is updated
     * @param oldTarget The previous bridge target address
     * @param newTarget The new bridge target address
     */
    event BridgeTargetUpdated(address indexed oldTarget, address indexed newTarget);

    /**
     * @notice Emitted when the bridge active status is changed
     * @param active The new active status
     */
    event BridgeStatusUpdated(bool active);

    /**
     * @notice Emitted when a bridge operation is successfully executed
     * @param token The token that was bridged
     * @param amount The amount that was bridged
     * @param target The bridge contract that was called
     */
    event BridgeExecuted(address indexed token, uint256 amount, address indexed target);

    /**
     * @notice Emitted when the daily limit is updated
     * @param oldLimit The previous daily limit
     * @param newLimit The new daily limit
     */
    event DailyLimitUpdated(uint256 oldLimit, uint256 newLimit);

    /**
     * @notice Emitted when daily limits are reset
     * @param day The new day number
     * @param previousUsed The amount used in the previous day
     */
    event DailyLimitReset(uint256 day, uint256 previousUsed);

    /*//////////////////////////////////////////////////////////////
                               MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @dev Ensures the bridge is active before allowing operations
    modifier onlyActiveBridge() {
        if (!bridgeActive) revert BridgeNotActive();
        _;
    }

    /// @dev Ensures only authorized callers can execute bridge operations
    modifier onlyAuthorizedCaller() {
        if (!hasRole(BRIDGE_CALLER_ROLE, msg.sender)) revert UnauthorizedCaller();
        _;
    }

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initialize the base bridge proxy
     * @param admin Address that will have admin privileges
     * @param caller Address that will be able to call executeBridge (typically SyndicateToken)
     * @param _bridgeName Human-readable name for this bridge
     * @param _bridgeTarget The actual bridge contract address
     * @param _maxSingleTransfer Maximum amount per single transaction
     * @param _dailyLimit Maximum amount per 24-hour period
     */
    // #olympix-ignore-no-parameter-validation-in-constructor
    constructor(
        address admin,
        address caller,
        string memory _bridgeName, //#olympix-ignore-no-parameter-validation-in-constructor
        address _bridgeTarget, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _maxSingleTransfer, //#olympix-ignore-no-parameter-validation-in-constructor
        uint256 _dailyLimit //#olympix-ignore-no-parameter-validation-in-constructor
    ) {
        if (admin == address(0) || caller == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_CALLER_ROLE, caller);

        bridgeName = _bridgeName;
        bridgeTarget = _bridgeTarget;
        maxSingleTransfer = _maxSingleTransfer;
        dailyLimit = _dailyLimit;
        bridgeActive = true;
        lastResetDay = block.timestamp / 1 days;
    }

    /*//////////////////////////////////////////////////////////////
                          BRIDGE EXECUTION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Execute a bridge operation with comprehensive safety checks
     * @dev This function implements the IBridgeProxy interface and provides common
     *      security checks before delegating to bridge-specific implementation
     *
     * @param token The token to bridge
     * @param amount The amount to bridge
     * @param dynamicData Bridge-specific data
     */
    function executeBridge(address token, uint256 amount, bytes calldata dynamicData)
        external
        virtual
        override
        nonReentrant
        onlyActiveBridge
        onlyAuthorizedCaller
    {
        if (token == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (amount > maxSingleTransfer) revert ExcessiveAmount();

        _updateDailyLimits();
        if (dailyUsed + amount > dailyLimit) revert ExcessiveAmount();

        // Transfer tokens from caller to this contract
        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);

        // Execute bridge-specific logic
        _executeBridgeCall(token, amount, dynamicData);

        dailyUsed += amount;
        emit BridgeExecuted(token, amount, bridgeTarget);
    }

    /**
     * @notice Abstract function that must be implemented by specific bridge contracts
     * @dev This function contains the actual bridge-specific logic for each implementation
     *
     * @param token The token to bridge
     * @param amount The amount to bridge
     * @param dynamicData Bridge-specific parameters
     */
    function _executeBridgeCall(address token, uint256 amount, bytes calldata dynamicData) internal virtual;

    /*//////////////////////////////////////////////////////////////
                          INTERNAL HELPERS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Update daily transfer limits, resetting if a new day has started
     * @dev This function is called before each transfer to ensure limits are current
     */
    function _updateDailyLimits() internal {
        uint256 currentDay = block.timestamp / 1 days;
        if (currentDay > lastResetDay) {
            emit DailyLimitReset(currentDay, dailyUsed);
            dailyUsed = 0;
            lastResetDay = currentDay;
        }
    }

    /*//////////////////////////////////////////////////////////////
                        ADMIN CONFIGURATION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Set the bridge target contract address
     * @dev Only callable by bridge admin
     *
     * @param target The new bridge target address
     */
    function setBridgeTarget(address target) external onlyRole(BRIDGE_ADMIN_ROLE) {
        if (target == address(0)) revert ZeroAddress();
        address oldTarget = bridgeTarget;
        bridgeTarget = target;
        emit BridgeTargetUpdated(oldTarget, target);
    }

    /**
     * @notice Set the bridge active status
     * @dev Use this to pause/unpause the bridge in emergencies
     *
     * @param active Whether the bridge should be active
     */
    function setBridgeActive(bool active) external onlyRole(BRIDGE_ADMIN_ROLE) {
        bridgeActive = active;
        emit BridgeStatusUpdated(active);
    }

    /**
     * @notice Set the daily transfer limit
     * @dev This limit resets every 24 hours
     *
     * @param limit The new daily limit in token units
     */
    function setDailyLimit(uint256 limit) external onlyRole(BRIDGE_ADMIN_ROLE) {
        uint256 oldLimit = dailyLimit;
        dailyLimit = limit;
        emit DailyLimitUpdated(oldLimit, limit);
    }

    /**
     * @notice Set the maximum single transfer amount
     * @dev This prevents any single transaction from being too large
     *
     * @param amount The new maximum single transfer amount
     */
    function setMaxSingleTransfer(uint256 amount) external onlyRole(BRIDGE_ADMIN_ROLE) {
        maxSingleTransfer = amount;
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get information about this bridge proxy
     * @dev Implementation of IBridgeProxy interface
     *
     * @return name Human-readable bridge name
     * @return target The bridge contract address
     * @return active Whether the bridge is currently active
     */
    function getBridgeInfo() external view override returns (string memory name, address target, bool active) {
        return (bridgeName, bridgeTarget, bridgeActive);
    }

    /**
     * @notice Get current daily usage statistics
     * @return used Amount used today
     * @return limit Daily limit
     * @return remaining Amount remaining for today
     */
    function getDailyUsage() external view returns (uint256 used, uint256 limit, uint256 remaining) {
        return (dailyUsed, dailyLimit, dailyLimit > dailyUsed ? dailyLimit - dailyUsed : 0);
    }

    /*//////////////////////////////////////////////////////////////
                          EMERGENCY FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emergency function to recover stuck tokens
     * @dev Only callable by DEFAULT_ADMIN_ROLE in case tokens get stuck in the contract
     *
     * @param token The token to recover
     * @param amount The amount to recover
     * @param to The address to send the recovered tokens to
     */
    function recoverTokens(address token, uint256 amount, address to) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        IERC20(token).safeTransfer(to, amount);
    }
}
