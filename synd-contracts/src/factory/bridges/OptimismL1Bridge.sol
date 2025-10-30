// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IL1Bridge} from "../interfaces/IL1Bridge.sol";
import {ICrossDomainMessenger} from
    "eigenlayer-middleware/lib/openzeppelin-contracts/contracts/vendor/optimism/ICrossDomainMessenger.sol";

/// @title OptimismL1Bridge
/// @notice Optimism/Base implementation of L1→L2 bridge messaging
/// @dev Uses Optimism's CrossDomainMessenger (also works for Base and other OP Stack chains)
contract OptimismL1Bridge is IL1Bridge {
    /// @notice The Optimism CrossDomainMessenger contract
    ICrossDomainMessenger public immutable messenger;

    /// @notice Counter for generating unique message IDs
    /// @dev Optimism's sendMessage doesn't return an ID, so we track our own
    uint256 private messageCounter;

    /// @notice Emitted when a message is sent
    /// @param messageId Our internal message ID
    /// @param target The L2 target address
    /// @param data The calldata sent
    /// @param gasLimit The gas limit for L2 execution
    event MessageSent(uint256 indexed messageId, address indexed target, bytes data, uint256 gasLimit);

    /// @notice Initializes the Optimism bridge adapter
    /// @param _messenger The Optimism CrossDomainMessenger contract address
    constructor(address _messenger) {
        messenger = ICrossDomainMessenger(_messenger);
    }

    /// @notice Sends a cross-chain message to Optimism/Base L2
    /// @param target The L2 contract address to call
    /// @param data The calldata to send to the target
    /// @param gasLimit The gas limit for L2 execution
    /// @param maxFeePerGas Not used for Optimism (kept for interface compatibility)
    /// @return messageId Our internal message ID
    function sendMessage(address target, bytes calldata data, uint256 gasLimit, uint256 maxFeePerGas)
        external
        payable
        override
        returns (uint256 messageId)
    {
        // Optimism uses uint32 for gas limit
        require(gasLimit <= type(uint32).max, "Gas limit too high");

        // Generate our own message ID
        messageId = ++messageCounter;

        // Send message via CrossDomainMessenger
        // Note: Optimism doesn't require ETH payment upfront like Arbitrum
        // The relayer fee model is different
        messenger.sendMessage(target, data, uint32(gasLimit));

        emit MessageSent(messageId, target, data, gasLimit);

        return messageId;
    }
}
