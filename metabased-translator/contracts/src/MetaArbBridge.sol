pragma solidity 0.8.25;


contract MetaArbBridge {

    bytes32[] public delayedInboxAccs;
    bytes32[] public sequencerInboxAccs;

    uint256 public override sequencerReportedSubMessageCount;

    struct TimeBounds {
        uint64 minTimestamp;
        uint64 maxTimestamp;
        uint64 minBlockNumber;
        uint64 maxBlockNumber;
    }

    enum BatchDataLocation {
        TxInput,
        SeparateBatchEvent,
        NoData,
        Blob
    }

    event SequencerBatchDelivered(
        uint256 indexed batchSequenceNumber,
        bytes32 indexed beforeAcc,
        bytes32 indexed afterAcc,
        bytes32 delayedAcc,
        uint256 afterDelayedMessagesRead,
        TimeBounds timeBounds,
        BatchDataLocation dataLocation
    );
    event SequencerBatchData(uint256 indexed batchSequenceNumber, bytes data);


    function addSequencerBatch(
        uint256 sequenceNumber,
        bytes calldata data
    ) external {
        IBridge.TimeBounds memory bounds = TimeBounds({
            minTimestamp: 10,
            maxTimestamp: 10,
            minBlockNumber: 10,
            maxBlockNumber: 10
        });
        uint256 afterDelayedMessagesRead = delayedInboxAccs.length;

        bytes memory header = abi.encodePacked(
            bounds.minTimestamp,
            bounds.maxTimestamp,
            bounds.minBlockNumber,
            bounds.maxBlockNumber,
            uint64(afterDelayedMessagesRead)
        );
        bytes32 dataHash = keccak256(bytes.concat(header, data));


        BatchDataLocation dataLocation = BatchDataLocation.TxInput;

        bytes32 beforeAcc = sequencerInboxAccs[sequencerInboxAccs.length - 1];
        bytes32 delayedAcc = delayedInboxAccs[delayedInboxAccs.length - 1];
        bytes32 afterAcc = keccak256(abi.encodePacked(beforeAcc, dataHash, delayedAcc));

        sequencerInboxAccs.push(afterAcc);
        sequencerReportedSubMessageCount++;
        
        emit SequencerBatchDelivered(
            sequenceNumber,
            beforeAcc,
            afterAcc,
            delayedAcc,
            afterDelayedMessagesRead,
            bounds,
            dataLocation
        );
        emit SequencerBatchData(sequenceNumber, data);
    }

    function addDelayedMessage(bytes32 delayedAcc) external {
        delayedInboxAccs.push(delayedAcc);
    }
}
