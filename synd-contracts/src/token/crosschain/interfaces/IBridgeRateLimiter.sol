// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title IBridgeRateLimiter
/// @notice Interface for managing bridge rate limits and permissions
interface IBridgeRateLimiter {
    /*//////////////////////////////////////////////////////////////
                                STRUCTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Bridge configuration structure
    struct BridgeConfig {
        uint256 dailyMintLimit; // Maximum tokens that can be minted per day
        uint256 dailyBurnLimit; // Maximum tokens that can be burned per day
        bool isActive; // Whether bridge is active
    }

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when bridge limits are updated
    /// @param bridge Bridge address
    /// @param dailyMintLimit New daily mint limit
    /// @param dailyBurnLimit New daily burn limit
    event BridgeLimitsSet(address indexed bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit);

    /// @notice Emitted when bridge active status changes
    /// @param bridge Bridge address
    /// @param isActive New active status
    event BridgeActiveStatusChanged(address indexed bridge, bool isActive);

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when bridge is not authorized
    error UnauthorizedBridge(address bridge);

    /// @notice Thrown when bridge has insufficient mint limit
    error InsufficientMintLimit(address bridge, uint256 requested, uint256 available);

    /// @notice Thrown when bridge has insufficient burn limit
    error InsufficientBurnLimit(address bridge, uint256 requested, uint256 available);

    /// @notice Thrown when bridge is not active
    error BridgeNotActive(address bridge);

    /// @notice Thrown when trying to add self as bridge
    error CannotAddSelfAsBridge();

    /// @notice Thrown when bridge address is not a contract
    error BridgeMustBeContract();

    /// @notice Thrown when bridge has insufficient emission budget
    error InsufficientEmissionBudget();

    /// @notice Thrown when mint limit is unreasonably high
    error UnreasonableMintLimit();

    /// @notice Thrown when burn limit is unreasonably high
    error UnreasonableBurnLimit();

    /*//////////////////////////////////////////////////////////////
                            CORE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Set minting and burning limits for a bridge
    /// @param bridge Bridge address
    /// @param dailyMintLimit Maximum tokens that can be minted per day
    /// @param dailyBurnLimit Maximum tokens that can be burned per day
    function setBridgeLimits(address bridge, uint256 dailyMintLimit, uint256 dailyBurnLimit) external;

    /// @notice Set active status for a bridge
    /// @param bridge Bridge address
    /// @param isActive Whether bridge should be active
    function setBridgeActive(address bridge, bool isActive) external;

    /*//////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Get bridge configuration
    /// @param bridge Bridge address
    /// @return config Bridge configuration struct
    function getBridgeConfig(address bridge) external view returns (BridgeConfig memory config);

    /// @notice Get current available mint limit for a bridge
    /// @param bridge Bridge address
    /// @return available Available mint limit
    function getAvailableMintLimit(address bridge) external view returns (uint256 available);

    /// @notice Get current available burn limit for a bridge
    /// @param bridge Bridge address
    /// @return available Available burn limit
    function getAvailableBurnLimit(address bridge) external view returns (uint256 available);

    /// @notice Check if bridge is authorized and active
    /// @param bridge Bridge address
    /// @return authorized True if bridge is authorized and active
    function isBridgeAuthorized(address bridge) external view returns (bool authorized);
}
