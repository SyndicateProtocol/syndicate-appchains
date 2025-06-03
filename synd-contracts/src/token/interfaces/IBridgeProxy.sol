// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

interface IBridgeProxy {
    /**
     * @notice Execute bridge operation using configured target and calldata
     * @param token The token to bridge
     * @param amount The amount to bridge
     * @param dynamicData Additional data to append/replace in calldata
     */
    function executeBridge(address token, uint256 amount, bytes calldata dynamicData) external;
}
