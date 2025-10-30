// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @title IL1Bridge
/// @notice Interface for L1→L2 bridge messaging
/// @dev Different implementations for Arbitrum, Optimism/Base, etc.
interface IL1Bridge {
    /// @notice Sends a cross-chain message to L2
    /// @param target The L2 contract address to call
    /// @param data The calldata to send to the target
    /// @param gasLimit The gas limit for L2 execution
    /// @param maxFeePerGas The max fee per gas for L2
    /// @return messageId A unique identifier for the cross-chain message
    function sendMessage(address target, bytes calldata data, uint256 gasLimit, uint256 maxFeePerGas)
        external
        payable
        returns (uint256 messageId);
}
