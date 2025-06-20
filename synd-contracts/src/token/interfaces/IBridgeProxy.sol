// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/**
 * @title IBridgeProxy
 * @notice Generic interface for all bridge proxy implementations
 * @dev This interface provides a standardized way for the SyndicateToken contract
 *      to interact with different bridge providers without knowing their specific
 *      implementation details.
 *
 * @author Syndicate Protocol
 * @custom:security-contact security@syndicate.io
 */
interface IBridgeProxy {
    /**
     * @notice Execute a bridge operation using the configured bridge target and parameters
     * @dev This function should handle all bridge-specific logic internally, including
     *      token approvals, fee calculations, and the actual bridge call.
     *
     * Requirements:
     * - Only authorized callers should be able to execute bridge operations
     * - The function should revert if the bridge operation fails
     * - Token approvals should be handled safely
     *
     * @param token The token contract address to bridge
     * @param amount The amount of tokens to bridge
     * @param dynamicData Additional bridge-specific data (ABI-encoded parameters)
     *                    This can include recipient addresses, gas limits, chain IDs, etc.
     */
    function executeBridge(address token, uint256 amount, bytes calldata dynamicData) external;

    /**
     * @notice Get information about this bridge proxy
     * @dev Useful for monitoring and debugging bridge configurations
     *
     * @return name Human-readable name of the bridge (e.g., "Optimism Bridge")
     * @return target The actual bridge contract address that this proxy calls
     * @return active Whether this bridge is currently active and accepting transactions
     */
    function getBridgeInfo() external view returns (string memory name, address target, bool active);
}
