package enclave

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"io"
	"strings"

	"github.com/SyndicateProtocol/synd-appchains/synd-enclave/teetypes"
	"github.com/andybalholm/brotli"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/offchainlabs/nitro/arbos"
	"github.com/offchainlabs/nitro/arbos/arbostypes"
	"github.com/offchainlabs/nitro/arbstate"
	"github.com/offchainlabs/nitro/daprovider"
)

var allowedSeqMsgs = map[byte]struct{}{
	arbos.L2MessageKind_UnsignedUserTx: {},
	arbos.L2MessageKind_ContractTx:     {},
	arbos.L2MessageKind_Batch:          {},
	arbos.L2MessageKind_SignedTx:       {},
}

// TODO: make sure spurious errors eg out of memory are not returned by the brotli reader and rlp decoder
// These functions should panic if the compressed data is valid but decoding fails
func processEvent(data []byte) [][]byte {
	if len(data) == 0 {
		return nil
	}
	if _, ok := allowedSeqMsgs[data[0]]; !ok {
		panic(fmt.Errorf("unexpected event header byte: %d", data[0]))
	}
	if data[0] != arbos.L2MessageKind_Batch {
		return [][]byte{data}
	}
	r := brotli.NewReader(bytes.NewReader(data[1:]))
	data, err := io.ReadAll(r)
	if err != nil {
		return nil
	}
	var txs [][]byte
	if err := rlp.DecodeBytes(data, &txs); err != nil {
		return nil
	}
	for i := range txs {
		txs[i] = append([]byte{arbos.L2MessageKind_SignedTx}, txs[i]...)
	}
	return txs
}

func buildBatch(txs [][]byte, ts uint64, blockNum uint64) ([]byte, error) {
	var data []byte

	if ts != 0 {
		segment, err := rlp.EncodeToBytes(ts)
		if err != nil {
			return nil, err
		}
		segment, err = rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindAdvanceTimestamp}, segment...))
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	if blockNum != 0 {
		segment, err := rlp.EncodeToBytes(blockNum)
		if err != nil {
			return nil, err
		}
		segment, err = rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindAdvanceL1BlockNumber}, segment...))
		if err != nil {
			return nil, err
		}
		data = append(data, segment...)
	}

	var l2Message []byte
	if len(txs) == 1 {
		l2Message = append(l2Message, txs[0]...)
	} else {
		l2Message = append(l2Message, arbos.L2MessageKind_Batch)
		var sizeBuf [8]byte
		for _, tx := range txs {
			binary.BigEndian.PutUint64(sizeBuf[:], uint64(len(tx)))
			l2Message = append(l2Message, sizeBuf[:]...)
			l2Message = append(l2Message, tx...)
		}
	}
	if len(l2Message) > arbostypes.MaxL2MessageSize {
		return nil, errors.New("l2message too long")
	}
	segment, err := rlp.EncodeToBytes(append([]byte{arbstate.BatchSegmentKindL2Message}, l2Message...))
	if err != nil {
		return nil, err
	}
	data = append(data, segment...)

	var buffer bytes.Buffer
	writer := brotli.NewWriterLevel(&buffer, brotli.BestSpeed)
	lenWritten, err := writer.Write(data)
	if err != nil {
		return nil, err
	}
	if lenWritten != len(data) {
		return nil, errors.New("write failed")
	}
	if err := writer.Close(); err != nil {
		return nil, err
	}
	return append([]byte{daprovider.BrotliMessageHeaderByte}, buffer.Bytes()...), nil
}

type SyndicateAccumulator struct {
	Address  common.Address
	Batches  []teetypes.SyndicateBatch
	BlockNum uint64
}

var TransactionProcessedEvent abi.Event

func init() {
	abi, err := abi.JSON(strings.NewReader(`[{"type":"event","name":"TransactionProcessed","inputs":[{"name":"sender","type":"address","indexed":true,"internalType":"address"},{"name":"data","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]`))
	if err != nil {
		panic(err)
	}
	TransactionProcessedEvent = abi.Events["TransactionProcessed"]
}

func (s *SyndicateAccumulator) ProcessBlock(block *types.Block, receipts types.Receipts) error {
	if s.BlockNum > 0 && s.BlockNum+1 != block.NumberU64() {
		return errors.New("unexpected block number")
	}
	s.BlockNum = block.NumberU64()
	var txs [][]byte
	for _, receipt := range receipts {
		for _, log := range receipt.Logs {
			if log.Address == s.Address && log.Topics[0] == TransactionProcessedEvent.ID {
				args, err := TransactionProcessedEvent.Inputs.Unpack(log.Data)
				if err != nil {
					return fmt.Errorf("failed to decode event: %w", err)
				}
				if len(args) != 1 {
					return errors.New("failed to decode event: unexpected number of args")
				}
				data, ok := args[0].([]byte)
				if !ok {
					return errors.New("failed to decode event: arg0 is not []byte")
				}
				txs = append(txs, processEvent(data)...)
			}
		}
	}
	var data []byte
	if len(txs) > 0 {
		var err error
		data, err = buildBatch(txs, block.Time(), block.NumberU64())
		if err != nil {
			return err
		}
	}
	s.Batches = append(s.Batches, teetypes.SyndicateBatch{
		Timestamp: block.Time(),
		Data:      data,
	})
	return nil
}
