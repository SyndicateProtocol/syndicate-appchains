package pkg

import (
	"context"
	"encoding/binary"
	"errors"
	"fmt"
	"math/big"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/enclave"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/log"
	"github.com/offchainlabs/nitro/arbnode"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbstate/daprovider"
	"github.com/offchainlabs/nitro/arbutil"
	"github.com/offchainlabs/nitro/execution"
	"github.com/offchainlabs/nitro/solgen/go/bridgegen"
)

// keep in sync with the nitro node
type ValidationData struct {
	BatchStartBlockNum uint64
	BatchEndBlockNum   uint64
	BatchStartIndex    uint64
	BatchEndIndex      uint64
	DelayedMessages    [][]byte
	StartDelayedAcc    common.Hash
	PreimageData       [][]byte
}

var messageDeliveredID common.Hash
var inboxMessageDeliveredID common.Hash

func init() {
	parsedIBridgeABI, err := bridgegen.IBridgeMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	messageDeliveredID = parsedIBridgeABI.Events["MessageDelivered"].ID
	parsedIMessageProviderABI, err := bridgegen.IDelayedMessageProviderMetaData.GetAbi()
	if err != nil {
		panic(err)
	}
	inboxMessageDeliveredID = parsedIMessageProviderABI.Events["InboxMessageDelivered"].ID
}

func getLogs(ctx context.Context, c *ethclient.Client, startBlock uint64, endBlock uint64, addresses []common.Address, topics [][]common.Hash, maxQty uint64) ([]types.Log, error) {
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
		return nil, err
	}
	mid := (startBlock + endBlock) / 2
	logs, err = getLogs(ctx, c, mid+1, endBlock, addresses, topics, maxQty)
	if err != nil {
		return nil, err
	}
	if maxQty > 0 && uint64(len(logs)) >= maxQty {
		return logs, nil
	}
	prevLogs, err := getLogs(ctx, c, startBlock, mid, addresses, topics, maxQty)
	if err != nil {
		return nil, err
	}
	prevLogs = append(prevLogs, logs...)
	return prevLogs, nil
}

func getBatches(ctx context.Context, c *ethclient.Client, sequencerInbox common.Address, start uint64, end uint64, startBlock uint64, endBlock uint64) ([][]byte, error) {
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
func getBatchPreimageData(ctx context.Context, batch []byte, dapReaders []daprovider.Reader, preimages map[common.Hash][]byte) error {
	if len(batch) > 40 {
		for _, dapReader := range dapReaders {
			if dapReader != nil && dapReader.IsValidHeaderByte(batch[40]) {
				preimageRecorder := func(key common.Hash, value []byte, _ arbutil.PreimageType) {
					preimages[key] = value
				}
				// TODO (SEQ-1064): try to speed this up - can disable validation as well if it is slow.
				_, err := dapReader.RecoverPayloadFromBatch(ctx, 0, common.Hash{}, batch, preimageRecorder, true)
				if err != nil {
					// Matches the way keyset validation was done inside DAS readers i.e logging the error
					//  But other daproviders might just want to return the error
					if errors.Is(err, daprovider.ErrSeqMsgValidation) && daprovider.IsDASMessageHeaderByte(batch[40]) {
						log.Error(err.Error())
					} else {
						return err
					}
				}
				return nil
			}
		}
		if daprovider.IsDASMessageHeaderByte(batch[40]) {
			log.Error("No DAS Reader configured, but sequencer message found with DAS header")
		}
	}
	return nil
}

// if count is zero, fetches the count instead
func getMessageAcc(ctx context.Context, c *ethclient.Client, bridge common.Address, count uint64) (common.Hash, uint64, error) {
	slot := common.BigToHash(big.NewInt(6))
	if count == 0 {
		countBytes, err := c.StorageAt(ctx, bridge, slot, nil)
		if err != nil {
			return common.Hash{}, 0, err
		}
		count = common.Hash(countBytes).Big().Uint64()
	}
	if count == 0 {
		return common.Hash{}, 0, errors.New("delayed acc count is zero")
	}
	slot = common.BigToHash(new(big.Int).Add(crypto.Keccak256Hash(slot[:]).Big(), big.NewInt(int64(count-1))))
	acc, err := c.StorageAt(ctx, bridge, slot, nil)
	if err != nil {
		return common.Hash{}, 0, err
	}
	if common.Hash(acc).Cmp(common.Hash{}) == 0 {
		return common.Hash{}, 0, errors.New("acc value of zero found")
	}
	return common.Hash(acc), count - 1, nil
}

func getDelayedMessages(ctx context.Context, c *ethclient.Client, bridge common.Address, inbox common.Address, start uint64, endAcc common.Hash) (common.Hash, [][]byte, bool, error) {
	endBlock, err := c.BlockNumber(ctx)
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	acc, end, err := getMessageAcc(ctx, c, bridge, 0)
	if err != nil {
		return common.Hash{}, nil, false, err
	}

	if acc != endAcc {
		logs, err := getLogs(ctx, c, 0, endBlock, []common.Address{bridge}, [][]common.Hash{[]common.Hash{messageDeliveredID}, nil, []common.Hash{endAcc}}, 1)
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
	logs, err := getLogs(ctx, c, 0, endBlock, []common.Address{bridge}, [][]common.Hash{[]common.Hash{messageDeliveredID}, indexes}, uint64(len(indexes)))
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	if len(logs) != len(indexes) {
		return common.Hash{}, nil, false, fmt.Errorf("unexpected number of logs found: got %d, expected %d", len(logs), len(indexes))
	}

	logs, _ = getLogs(ctx, c, logs[0].BlockNumber, logs[len(logs)-1].BlockNumber,
		[]common.Address{bridge, inbox},
		[][]common.Hash{
			[]common.Hash{messageDeliveredID, inboxMessageDeliveredID},
		}, 0)

	ibridge, err := bridgegen.NewIBridge(bridge, c)
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	iinbox, err := bridgegen.NewIDelayedMessageProvider(inbox, c)
	if err != nil {
		return common.Hash{}, nil, false, err
	}
	if len(logs)%2 != 0 {
		return common.Hash{}, nil, false, errors.New("even number of logs expected")
	}

	var msgs [][]byte
	var prevAcc *common.Hash
	for i := 0; i < len(logs); i += 2 {
		log, err := ibridge.ParseMessageDelivered(logs[i])
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		dataLog, err := iinbox.ParseInboxMessageDelivered(logs[i+1])
		if err != nil {
			return common.Hash{}, nil, false, err
		}
		if log.MessageIndex.Cmp(dataLog.MessageNum) != 0 {
			return common.Hash{}, nil, false, errors.New("event log msg index mismatch")
		}
		if log.Raw.BlockNumber != dataLog.Raw.BlockNumber {
			return common.Hash{}, nil, false, errors.New("event log block number mismatch")
		}
		// skip events prior to the start one
		if log.MessageIndex.Cmp(big.NewInt(int64(start))) != 0 {
			continue
		}
		// exit once we have processed the end message
		if start > end {
			break
		}
		start++
		if prevAcc == nil {
			hash := common.Hash(log.BeforeInboxAcc)
			prevAcc = &hash
		}
		requestId := common.BigToHash(log.MessageIndex)
		msg := arbostypes.L1IncomingMessage{
			Header: &arbostypes.L1IncomingMessageHeader{
				Kind:        log.Kind,
				Poster:      log.Sender,
				BlockNumber: log.Raw.BlockNumber,
				Timestamp:   log.Timestamp,
				RequestId:   &requestId,
				L1BaseFee:   log.BaseFeeL1,
			},
			L2msg: dataLog.Data,
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

func getNumBatches(batches []enclave.SyndicateBatch, dmsgs [][]byte, setDelay uint64) uint64 {
	var batchCount uint64
	i := 0
	for _, b := range batches {
		hasMsg := false
		for i < len(dmsgs) && binary.BigEndian.Uint64(dmsgs[i][enclave.DelayedMessageTimestampOffset:enclave.DelayedMessageTimestampOffset+8])+setDelay <= b.Timestamp {
			i++
			hasMsg = true
		}
		if b.Data != nil || hasMsg {
			batchCount++
		}
	}
	return batchCount
}

// find the last block <= l1 number. return an error if start is greater than l1 number.
func findBlock(ctx context.Context, c *ethclient.Client, start uint64, l1Number uint64) (*execution.MessageResult, error) {
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
			return nil, err
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
		return nil, err
	}
	extraInfo = types.DeserializeHeaderExtraInformation(header)
	if extraInfo.L1BlockNumber > l1Number {
		return nil, errors.New("block not found")
	}
	return &execution.MessageResult{BlockHash: header.Hash(), SendRoot: extraInfo.SendRoot}, nil
}
