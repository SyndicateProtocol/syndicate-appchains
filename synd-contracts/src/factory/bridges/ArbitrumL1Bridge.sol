// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {IL1Bridge} from "../interfaces/IL1Bridge.sol";
import {IInbox} from "@arbitrum/nitro-contracts/src/bridge/IInbox.sol";

/// @title ArbitrumL1Bridge
/// @notice Arbitrum implementation of L1→L2 bridge messaging
/// @dev Uses Arbitrum's Inbox for retryable tickets
contract ArbitrumL1Bridge is IL1Bridge {
    /// @notice The Arbitrum Inbox contract
    IInbox public immutable inbox;

    /// @notice Thrown when insufficient ETH is provided for the retryable ticket
    error InsufficientValue();

    /// @notice Initializes the Arbitrum bridge adapter
    /// @param _inbox The Arbitrum Inbox contract address
    constructor(address _inbox) {
        inbox = IInbox(_inbox);
    }

    /// @notice Sends a cross-chain message to Arbitrum L2
    /// @param target The L2 contract address to call
    /// @param data The calldata to send to the target
    /// @param gasLimit The gas limit for L2 execution
    /// @param maxFeePerGas The max fee per gas for L2
    /// @return messageId The retryable ticket ID
    function sendMessage(address target, bytes calldata data, uint256 gasLimit, uint256 maxFeePerGas)
        external
        payable
        override
        returns (uint256 messageId)
    {
        if (msg.value == 0) revert InsufficientValue();

        // Send retryable ticket to L2
        // msg.value should cover: maxSubmissionCost + (gasLimit * maxFeePerGas)
        messageId = inbox.createRetryableTicket{value: msg.value}(
            target, // destination
            0, // l2CallValue (no ETH sent to target)
            msg.value, // maxSubmissionCost (use all msg.value for submission)
            msg.sender, // excessFeeRefundAddress
            msg.sender, // callValueRefundAddress
            gasLimit, // gasLimit
            maxFeePerGas, // maxFeePerGas
            data // data
        );

        return messageId;
    }
}
