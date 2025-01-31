// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "./IBridge.sol";
import "./ISequencerInbox.sol";
import "./IDelayedMessageProvider.sol";
import "../libraries/MessageTypes.sol";
import "../libraries/Error.sol";

contract Rollup {
    // TODO: set these values properly
    uint64 public constant maxDataSize = 117964;
    uint64 public constant delayBlocks = 7200;
    uint64 public constant futureBlocks = 12;
    uint64 public constant delaySeconds = 86400;
    uint64 public constant futureSeconds = 3600;

    // IBridge.sol
    bytes32[] public delayedInboxAccs;

    // IBridge.sol
    bytes32[] public sequencerInboxAccs;

    // ISequencerInbox.sol
    uint256 public totalDelayedMessagesRead;

    constructor(uint256 chainId, string memory chainConfig) {
        require(bytes(chainConfig).length > 0, "EMPTY_CHAIN_CONFIG");
        uint8 initMsgVersion = 1;
        uint256 currentDataCost = block.basefee;
        /*
        if (ArbitrumChecker.runningOnArbitrum()) {
            currentDataCost += ArbGasInfo(address(0x6c)).getL1BaseFeeEstimate();
        }
        */
        bytes memory initMsg = abi.encodePacked(chainId, initMsgVersion, currentDataCost, chainConfig);
        deliverMessage(INITIALIZATION_MSG_TYPE, address(0), initMsg);
    }

    // IBridge.sol
    function delayedMessageCount() external view returns (uint256) {
        return delayedInboxAccs.length;
    }

    // IBridge.sol
    function sequencerMessageCount() external view returns (uint256) {
        return sequencerInboxAccs.length;
    }

    // ISequencerInbox.sol
    function batchCount() external view returns (uint256) {
        return sequencerInboxAccs.length;
    }

    // ISequencerInbox.sol
    function inboxAccs(uint256 index) external view returns (bytes32) {
        return sequencerInboxAccs[index];
    }

    function postBatch(bytes calldata data) public {
        uint256 afterDelayedMessagesRead = delayedInboxAccs.length;
        (bytes32 dataHash, IBridge.TimeBounds memory timeBounds) = formCallDataHash(data, afterDelayedMessagesRead);
        uint256 seqMessageIndex = sequencerInboxAccs.length;
        bytes32 beforeAcc = 0;
        if (seqMessageIndex > 0) {
            beforeAcc = sequencerInboxAccs[seqMessageIndex - 1];
        }
        bytes32 delayedAcc = 0;
        if (afterDelayedMessagesRead > 0) {
            delayedAcc = delayedInboxAccs[afterDelayedMessagesRead - 1];
        }
        bytes32 acc = keccak256(abi.encodePacked(beforeAcc, dataHash, delayedAcc));
        sequencerInboxAccs.push(acc);
        totalDelayedMessagesRead = afterDelayedMessagesRead;
        emit ISequencerInbox.SequencerBatchDelivered(
            seqMessageIndex,
            beforeAcc,
            acc,
            delayedAcc,
            totalDelayedMessagesRead,
            timeBounds,
            IBridge.BatchDataLocation.SeparateBatchEvent
        );
        emit ISequencerInbox.SequencerBatchData(seqMessageIndex, data);
    }

    function depositEth(address src, address dest, uint256 value) external {
        deliverMessage(L1MessageType_ethDeposit, src, abi.encodePacked(dest, value));
    }

    // remember to alias the sender with AddressAliasHelper.applyL1ToL2Alias() before calling this function
    // Inbox.sol
    function deliverMessage(uint8 kind, address sender, bytes memory messageData) public {
        uint256 count = delayedInboxAccs.length;
        bytes32 messageDataHash = keccak256(messageData);
        bytes32 messageHash = keccak256(
            abi.encodePacked(
                kind, sender, uint64(block.number), uint64(block.timestamp), count, block.basefee, messageDataHash
            )
        );
        bytes32 prevAcc = 0;
        if (count > 0) {
            prevAcc = delayedInboxAccs[count - 1];
        }
        delayedInboxAccs.push(keccak256(abi.encodePacked(prevAcc, messageHash)));
        emit IBridge.MessageDelivered(
            count, prevAcc, address(this), kind, sender, messageDataHash, block.basefee, uint64(block.timestamp)
        );
        emit IDelayedMessageProvider.InboxMessageDelivered(count, messageData);
    }

    uint256 internal constant HEADER_LENGTH = 40;

    // SequencerInbox.sol
    function formCallDataHash(bytes calldata data, uint256 afterDelayedMessagesRead)
        internal
        view
        returns (bytes32, IBridge.TimeBounds memory)
    {
        uint256 fullDataLen = HEADER_LENGTH + data.length;
        if (fullDataLen > maxDataSize) revert DataTooLarge(fullDataLen, maxDataSize);

        IBridge.TimeBounds memory timeBounds;
        if (block.timestamp > delaySeconds) {
            timeBounds.minTimestamp = uint64(block.timestamp) - delaySeconds;
        }
        timeBounds.maxTimestamp = uint64(block.timestamp) + futureSeconds;
        if (block.number > delayBlocks) {
            timeBounds.minBlockNumber = uint64(block.number) - delayBlocks;
        }
        timeBounds.maxBlockNumber = uint64(block.number) + futureBlocks;

        bytes memory header = abi.encodePacked(
            timeBounds.minTimestamp,
            timeBounds.maxTimestamp,
            timeBounds.minBlockNumber,
            timeBounds.maxBlockNumber,
            uint64(afterDelayedMessagesRead)
        );
        // This must always be true from the packed encoding
        assert(header.length == HEADER_LENGTH);
        return (keccak256(bytes.concat(header, data)), timeBounds);
    }
}
