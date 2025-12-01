package pkg

import (
	"context"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"maps"
	"math/big"
	"strings"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teetypes"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/log"
	"github.com/offchainlabs/nitro/arbnode"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/daprovider"
	"github.com/offchainlabs/nitro/execution"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
	"github.com/pkg/errors"
)

// ValidationData
//
// Need to keep this in sync with the nitro node's version
// https://github.com/SyndicateProtocol/nitro/blob/49f053e12ab56055473a1a7c68f57590df63bb5b/arbnode/synd_api.go#L23
type ValidationData struct {
	BatchStartBlockNum uint64
	BatchEndBlockNum   uint64
	BatchStartIndex    uint64
	BatchEndIndex      uint64
	DelayedMessages    [][]byte
	StartDelayedAcc    common.Hash
	PreimageData       [][]byte
}

var (
	messageDeliveredEventHash                common.Hash
	inboxMessageDeliveredEventHash           common.Hash
	inboxMessageDeliveredFromOriginEventHash common.Hash
	l2MessageFromOriginCallABI               abi.Method
	l2MessageFromOriginCallSelector          common.Hash
)

var ArbSysPrecompileAddress = common.HexToAddress("0x0000000000000000000000000000000000000064")

func init() {
	parsedIBridgeABI, err := bridgegen.IBridgeMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	messageDeliveredEventHash = parsedIBridgeABI.Events["MessageDelivered"].ID
	parsedIMessageProviderABI, err := bridgegen.IDelayedMessageProviderMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	inboxMessageDeliveredEventHash = parsedIMessageProviderABI.Events["InboxMessageDelivered"].ID
	inboxMessageDeliveredFromOriginEventHash = parsedIMessageProviderABI.Events["InboxMessageDeliveredFromOrigin"].ID
	parsedIInboxABI, err := bridgegen.IInboxMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	l2MessageFromOriginCallABI = parsedIInboxABI.Methods["sendL2MessageFromOrigin"]
	l2MessageFromOriginCallSelector = common.BytesToHash(l2MessageFromOriginCallABI.ID)
}

// once the target qty is reached or exceeded, getLogs stops fetching logs
// note that it may return more logs than the target qty or less if target qty logs do not exist
// if target qty is set to 0, all logs in the range are fetched
func getLogs(
	ctx context.Context,
	c *ethclient.Client,
	startBlock uint64,
	endBlock uint64,
	addresses []common.Address,
	topics [][]common.Hash,
	targetQty uint64,
) ([]types.Log, error) {
	if startBlock > endBlock {
		return nil, errors.New("start block > end block")
	}
	logs, err := c.FilterLogs(ctx, ethereum.FilterQuery{
		FromBlock: big.NewInt(int64(startBlock)),
		ToBlock:   big.NewInt(int64(endBlock)),
		Addresses: addresses,
		Topics:    topics,
	})
	// TODO (SEQ-1064): check if a valid error response object is received before retrying
	if err == nil {
		return logs, nil
	}
	if startBlock == endBlock {
		return nil, errors.Wrap(err, "start block == end block, cannot bisect further")
	}
	mid := (startBlock + endBlock) / 2
	logs, err = getLogs(ctx, c, mid+1, endBlock, addresses, topics, targetQty)
	if err != nil {
		return nil, err
	}
	if targetQty > 0 {
		if uint64(len(logs)) >= targetQty {
			return logs, nil
		}
		targetQty -= uint64(len(logs))
	}
	prevLogs, err := getLogs(ctx, c, startBlock, mid, addresses, topics, targetQty)
	if err != nil {
		return nil, err
	}
	prevLogs = append(prevLogs, logs...)

	return prevLogs, nil
}

func getBatches(
	ctx context.Context,
	c *ethclient.Client,
	sequencerInbox common.Address,
	start uint64,
	end uint64,
	startBlock uint64,
	endBlock uint64,
) ([][]byte, error) {
	inbox, err := arbnode.NewSequencerInbox(c, sequencerInbox, 0)
	if err != nil {
		return nil, err
	}
	// TODO (SEQ-1064): use custom client that auto bisects the log range
	batches, err := inbox.LookupBatchesInRange(ctx, big.NewInt(int64(startBlock)), big.NewInt(int64(endBlock)))
	if err != nil {
		return nil, err
	}
	var data [][]byte
	for _, batch := range batches {
		if batch.SequenceNumber >= start && batch.SequenceNumber <= end {
			// TODO (SEQ-1064): can this be sped up? probably not since the tx receipt needs to be fetched in general
			raw, err := batch.Serialize(ctx, c)
			if err != nil {
				return nil, err
			}
			data = append(data, raw)
		}
	}

	return data, nil
}

func loadBatchPreimageData(
	ctx context.Context,
	batch []byte,
	dapReaders []daprovider.Reader,
	preimages map[arbutil.PreimageType]map[common.Hash][]byte,
) error {
	// byte 40 is a flag byte that determines if the batch uses alt-DA
	if len(batch) <= 40 || batch[40] == 0 {
		return nil
	}
	if !daprovider.IsKnownHeaderByte(batch[40]) {
		return fmt.Errorf("unknown header byte 0x%02x", batch[40])
	}
	for _, dapReader := range dapReaders {
		if dapReader.IsValidHeaderByte(ctx, batch[40]) {
			// TODO (SEQ-1064): try to speed this up - can disable validation as well if it is slow.
			_, preimagesRecorded, err := dapReader.RecoverPayloadFromBatch(ctx, 0, common.Hash{}, batch, nil, true)
			if err != nil {
				return errors.Wrap(err, "failed to recover payload from batch - "+hex.EncodeToString(batch))
			}

			for ty, images := range preimagesRecorded {
				if preimages[ty] == nil {
					preimages[ty] = images
				} else {
					maps.Copy(preimages[ty], images)
				}
			}

			return nil
		}
	}
	log.Error("DAS reader mismatch",
		"headerByte", fmt.Sprintf("0x%02x", batch[40]),
		"availableReaders", len(dapReaders),
		"batchLength", len(batch))

	return fmt.Errorf("no DAS reader configured for header byte 0x%02x (have %d readers)",
		batch[40], len(dapReaders))
}

// if count is zero, get the latest delayed message accumulator
func GetMessageAcc(ctx context.Context, c *ethclient.Client, bridge common.Address, count uint64) (common.Hash, uint64, error) {
	// Memory slot in contract is 6
	slot := common.BigToHash(big.NewInt(6))
	if count == 0 {
		countBytes, err := c.StorageAt(ctx, bridge, slot, nil)
		if err != nil {
			return common.Hash{}, 0, errors.Wrap(err, "failed to fetch account data")
		}

		count = common.Hash(countBytes).Big().Uint64()
	}
	if count == 0 {
		return common.Hash{}, 0, errors.New("delayed acc count is zero")
	}
	slot = common.BigToHash(new(big.Int).Add(crypto.Keccak256Hash(slot[:]).Big(), big.NewInt(int64(count-1))))
	acc, err := c.StorageAt(ctx, bridge, slot, nil)
	if err != nil {
		return common.Hash{}, 0, errors.Wrap(err, "failed to get account from storage")
	}
	if common.Hash(acc).Cmp(common.Hash{}) == 0 {
		return common.Hash{}, 0, errors.New("acc value of zero found")
	}

	return common.Hash(acc), count - 1, nil
}

func GetDelayedMessages(
	ctx context.Context,
	c *ethclient.Client,
	bridge common.Address,
	start uint64,
	endAcc common.Hash,
	settlesToArbitrumRollup bool,
) (common.Hash, [][]byte, bool, error) {
	endBlock, err := c.BlockNumber(ctx)
	if err != nil {
		return common.Hash{}, nil, false, errors.Wrap(err, "failed to get block number")
	}
	acc, end, err := GetMessageAcc(ctx, c, bridge, 0)
	if err != nil {
		return common.Hash{}, nil, false, errors.Wrap(err, "failed to get message account data")
	}

	if acc != endAcc {
		logs, err := getLogs(
			ctx,
			c,
			0,
			endBlock,
			[]common.Address{bridge},
			[][]common.Hash{{messageDeliveredEventHash}, nil, {endAcc}},
			1)
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		if len(logs) != 1 {
			return common.Hash{}, nil, false, fmt.Errorf("unexpected number of logs found: got %d, expected 1", len(logs))
		}
		end = logs[0].Topics[1].Big().Uint64()
		if end == 0 {
			return common.Hash{}, nil, false, errors.New("unexpected message index 0 found")
		}
		end--
		endBlock = logs[0].BlockNumber
	}
	var dummy bool
	// include a dummy message if the start count is after the end message index
	indexes := []common.Hash{common.BigToHash(big.NewInt(int64(end)))}
	if start == end+1 {
		start--
		dummy = true
	} else {
		if start > end {
			return common.Hash{}, nil, false, fmt.Errorf("start message %d is after end %d", start, end)
		}
		if start < end {
			indexes = append(indexes, common.BigToHash(big.NewInt(int64(start))))
		}
	}
	logs, err := getLogs(
		ctx,
		c,
		0,
		endBlock,
		[]common.Address{bridge},
		[][]common.Hash{{messageDeliveredEventHash}, indexes},
		uint64(len(indexes)))
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	if len(logs) != len(indexes) {
		return common.Hash{}, nil, false,
			fmt.Errorf("unexpected number of logs found: got %d, expected %d", len(logs), len(indexes))
	}

	ibridge, err := bridgegen.NewBridge(bridge, c)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	seqInbox, err := ibridge.SequencerInbox(&bind.CallOpts{Context: ctx})
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	addrs := []common.Address{bridge, seqInbox}

	var i int64
	for {
		inbox, err := ibridge.AllowedDelayedInboxList(&bind.CallOpts{Context: ctx}, big.NewInt(i))
		if err != nil {
			if strings.Contains(err.Error(), vm.ErrExecutionReverted.Error()) {
				break
			}

			return common.Hash{}, nil, false, err
		}
		addrs = append(addrs, inbox)
		i++
	}
	if i == 0 {
		return common.Hash{}, nil, false, errors.New("no inbox addresses found")
	}

	logs, err = getLogs(ctx, c, logs[0].BlockNumber, logs[len(logs)-1].BlockNumber,
		addrs,
		[][]common.Hash{
			{messageDeliveredEventHash, inboxMessageDeliveredEventHash, inboxMessageDeliveredFromOriginEventHash},
		}, 0)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	if len(logs)%2 != 0 {
		for _, log := range logs {
			fmt.Println("LOG: ", log.Topics[0], log.TxHash, log.Address, log.Topics[1].Big().Uint64())
		}

		return common.Hash{}, nil, false, fmt.Errorf("even number of logs expected: got %d", len(logs))
	}

	iinbox, err := bridgegen.NewIDelayedMessageProvider(common.Address{}, c)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	var msgs [][]byte
	var prevAcc *common.Hash
	for i := 0; i < len(logs); i += 2 {
		msgDeliveredLog, err := ibridge.ParseMessageDelivered(logs[i])
		if err != nil {
			return common.Hash{}, nil, false, errors.Wrap(err, "failed to parse message delivered log")
		}

		inboxLog := logs[i+1]
		var logData []byte
		var logBlockNum uint64
		var logMsgIndex *big.Int

		switch inboxLog.Topics[0] {
		case inboxMessageDeliveredEventHash:
			dataLog, err := iinbox.ParseInboxMessageDelivered(inboxLog)
			if err != nil {
				return common.Hash{}, nil, false, errors.Wrap(err, "failed to parse message delivered log")
			}
			logData = dataLog.Data
			logBlockNum = dataLog.Raw.BlockNumber
			logMsgIndex = dataLog.MessageNum

		case inboxMessageDeliveredFromOriginEventHash:
			dataLog, err := iinbox.ParseInboxMessageDeliveredFromOrigin(inboxLog)
			if err != nil {
				return common.Hash{}, nil, false, errors.Wrap(err, "failed to parse message delivered from origin log")
			}
			// fetch the tx from the event
			tx, _, err := c.TransactionByHash(ctx, inboxLog.TxHash)
			if err != nil {
				return common.Hash{}, nil, false, errors.Wrap(err, fmt.Sprintf("failed to get tx by hash %s", inboxLog.TxHash.String()))
			}
			if len(tx.Data()) < 4 {
				return common.Hash{}, nil, false, errors.New("tx data too short")
			}
			if l2MessageFromOriginCallSelector.Cmp(common.BytesToHash(tx.Data()[:4])) != 0 {
				return common.Hash{}, nil, false, errors.New("invalid function selector")
			}
			args := make(map[string]interface{})
			err = l2MessageFromOriginCallABI.Inputs.UnpackIntoMap(args, tx.Data()[4:])
			if err != nil {
				return common.Hash{}, nil, false, errors.Wrap(err, "failed to parse inputs of sendL2MessageFromOrigin")
			}
			var ok bool
			logData, ok = args["messageData"].([]byte)
			if !ok {
				return common.Hash{}, nil, false, errors.New("failed to cast messageData to []byte")
			}
			logBlockNum = dataLog.Raw.BlockNumber
			logMsgIndex = dataLog.MessageNum
		}
		if msgDeliveredLog.MessageIndex.Cmp(logMsgIndex) != 0 {
			return common.Hash{}, nil, false, errors.New("event log msg index mismatch")
		}
		if msgDeliveredLog.Raw.BlockNumber != logBlockNum {
			return common.Hash{}, nil, false, errors.New("event log block number mismatch")
		}
		// skip events prior to the start one
		if msgDeliveredLog.MessageIndex.Cmp(big.NewInt(int64(start))) != 0 {
			continue
		}
		// exit once we have processed the end message
		if start > end {
			break
		}
		start++
		if prevAcc == nil {
			hash := common.Hash(msgDeliveredLog.BeforeInboxAcc)
			prevAcc = &hash
		}
		requestId := common.BigToHash(msgDeliveredLog.MessageIndex)

		msg := arbostypes.L1IncomingMessage{
			Header: &arbostypes.L1IncomingMessageHeader{
				Kind:        msgDeliveredLog.Kind,
				Poster:      msgDeliveredLog.Sender,
				BlockNumber: msgDeliveredLog.Raw.BlockNumber,
				Timestamp:   msgDeliveredLog.Timestamp,
				RequestId:   &requestId,
				L1BaseFee:   msgDeliveredLog.BaseFeeL1,
			},
			L2msg: logData,
		}

		if settlesToArbitrumRollup {
			block, err := c.BlockByHash(ctx, msgDeliveredLog.Raw.BlockHash)
			if err != nil {
				return common.Hash{}, nil, false, errors.Wrap(err, "failed to get block by hash")
			}

			// TODO revisit this

			// Override the block number with the L1 block number
			// It is used during contract execution in nitro rollups
			l1BlockNum := types.DeserializeHeaderExtraInformation(block.Header()).L1BlockNumber
			msg.Header.BlockNumber = l1BlockNum
		}

		data, err := msg.Serialize()
		if err != nil {
			return common.Hash{}, nil, false, err
		}

		msgs = append(msgs, data)
	}
	if start != end+1 || prevAcc == nil {
		return common.Hash{}, nil, false, fmt.Errorf("missing message: got %d, expected %d", start, end+1)
	}

	return *prevAcc, msgs, dummy, nil
}

func getNumBatches(batches []teetypes.SyndicateBatch, dmsgs [][]byte, setDelay uint64) uint64 {
	var batchCount uint64
	i := 0
	for _, b := range batches {
		hasMsg := false
		for i < len(dmsgs) {
			dmsgTimestamp := binary.BigEndian.
				Uint64(dmsgs[i][teetypes.DelayedMessageTimestampOffset : teetypes.DelayedMessageTimestampOffset+8])
			if dmsgTimestamp+setDelay > b.Timestamp {
				break
			}
			i++
			hasMsg = true
		}
		if b.Data != nil || hasMsg {
			batchCount++
		}
	}

	return batchCount
}

// FindBlock finds the last block <= L1 number. return an error if start is greater than L1 number.
func FindBlock(
	ctx context.Context,
	c *ethclient.Client,
	start uint64,
	l1Number uint64,
) (*execution.MessageResult, error) {
	end, err := c.BlockNumber(ctx)
	if err != nil {
		return nil, err
	}
	end++
	var extraInfo types.HeaderInfo
	for end-start > 1 {
		mid := (start + end) / 2
		header, err := c.HeaderByNumber(ctx, big.NewInt(int64(mid)))
		if err != nil {
			return nil, errors.Wrap(err, "failed to get block header by number")
		}
		extraInfo = types.DeserializeHeaderExtraInformation(header)
		if extraInfo.L1BlockNumber <= l1Number {
			start = mid
		} else {
			end = mid
		}
	}

	header, err := c.HeaderByNumber(ctx, big.NewInt(int64(start)))
	if err != nil {
		return nil, errors.Wrap(err, "failed to get block header by number")
	}
	extraInfo = types.DeserializeHeaderExtraInformation(header)
	if extraInfo.L1BlockNumber > l1Number {
		return nil, errors.New("block not found")
	}

	return &execution.MessageResult{BlockHash: header.Hash(), SendRoot: extraInfo.SendRoot}, nil
}
